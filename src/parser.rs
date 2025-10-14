use crate::runtime::Vm;
use crate::tokens::Tok;
use crate::value::Value;

/// Minimal Pratt-style expression parser:
/// supports + - * /, parentheses, and string concatenation via '+'.

#[derive(Clone)]
pub struct Cursor<'a> {
    pub toks: &'a [Tok],
    pub i: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(toks: &'a [Tok]) -> Self {
        Self { toks, i: 0 }
    }
    pub fn peek(&self) -> Option<&'a Tok> {
        self.toks.get(self.i)
    }
    pub fn next(&mut self) -> Option<&'a Tok> {
        let t = self.toks.get(self.i);
        if t.is_some() {
            self.i += 1;
        }
        t
    }
}

#[allow(dead_code)]
pub fn parse_expression(cur: &mut Cursor) -> Option<Value> {
    parse_term(cur).and_then(|mut lhs| {
        while let Some(tok) = cur.peek() {
            match tok {
                Tok::Symbol('+') => {
                    cur.next();
                    let rhs = parse_term(cur)?;
                    lhs = match (lhs, rhs) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                        (Value::Str(a), Value::Str(b)) => Value::Str(a + &b),
                        (Value::Str(a), Value::Number(b)) => Value::Str(a + &b.to_string()),
                        (Value::Number(a), Value::Str(b)) => Value::Str(a.to_string() + &b),
                        (Value::Tab(_), v) => v,
                        (v, Value::Tab(_)) => v,
                    };
                }
                Tok::Symbol('-') => {
                    cur.next();
                    let rhs = parse_term(cur)?;
                    lhs = match (lhs, rhs) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
                        (Value::Tab(_), v) => v,
                        (v, Value::Tab(_)) => v,
                        (a, b) => Value::Number(a.as_number() - b.as_number()),
                    };
                }
                _ => break,
            }
        }
        Some(lhs)
    })
}

/// Variant that resolves identifiers through VM variables.
pub fn parse_expression_with_vm(cur: &mut Cursor, vm: &mut Vm) -> Option<Value> {
    parse_term_with_vm(cur, vm).and_then(|mut lhs| {
        while let Some(tok) = cur.peek() {
            match tok {
                Tok::Symbol('+') => {
                    cur.next();
                    let rhs = parse_term_with_vm(cur, vm)?;
                    lhs = match (lhs, rhs) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                        (Value::Str(a), Value::Str(b)) => Value::Str(a + &b),
                        (Value::Str(a), Value::Number(b)) => Value::Str(a + &b.to_string()),
                        (Value::Number(a), Value::Str(b)) => Value::Str(a.to_string() + &b),
                        (Value::Tab(_), v) => v,
                        (v, Value::Tab(_)) => v,
                    };
                }
                Tok::Symbol('-') => {
                    cur.next();
                    let rhs = parse_term_with_vm(cur, vm)?;
                    lhs = Value::Number(lhs.as_number() - rhs.as_number());
                }
                _ => break,
            }
        }
        Some(lhs)
    })
}

#[allow(dead_code)]
fn parse_term(cur: &mut Cursor) -> Option<Value> {
    parse_factor(cur).and_then(|mut lhs| {
        while let Some(tok) = cur.peek() {
            match tok {
                Tok::Symbol('*') => {
                    cur.next();
                    let rhs = parse_factor(cur)?;
                    lhs = Value::Number(lhs.as_number() * rhs.as_number());
                }
                Tok::Symbol('/') => {
                    cur.next();
                    let rhs = parse_factor(cur)?;
                    lhs = Value::Number(lhs.as_number() / rhs.as_number());
                }
                _ => break,
            }
        }
        Some(lhs)
    })
}

fn parse_term_with_vm(cur: &mut Cursor, vm: &mut Vm) -> Option<Value> {
    parse_factor_with_vm(cur, vm).and_then(|mut lhs| {
        while let Some(tok) = cur.peek() {
            match tok {
                Tok::Symbol('*') => {
                    cur.next();
                    let rhs = parse_factor_with_vm(cur, vm)?;
                    lhs = match (lhs, rhs) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
                        (Value::Str(a), Value::Str(b)) => Value::Str(format!("{}{}", a, b)),
                        (Value::Tab(_), v) => v,
                        (v, Value::Tab(_)) => v,
                        (a, b) => Value::Number(a.as_number() * b.as_number()),
                    };
                }
                Tok::Symbol('/') => {
                    cur.next();
                    let rhs = parse_factor_with_vm(cur, vm)?;
                    lhs = match (lhs, rhs) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
                        (Value::Tab(_), v) => v,
                        (v, Value::Tab(_)) => v,
                        (a, b) => Value::Number(a.as_number() / b.as_number()),
                    };
                }
                _ => break,
            }
        }
        Some(lhs)
    })
}

#[allow(dead_code)]
fn parse_factor(cur: &mut Cursor) -> Option<Value> {
    match cur.next()? {
        Tok::Number(n) => Some(Value::Number(*n)),
        Tok::String(s) => Some(Value::Str(s.clone())),
        Tok::Symbol('(') => {
            let v = parse_expression(cur)?;
            if !matches!(cur.next(), Some(Tok::Symbol(')'))) {
                return None;
            }
            Some(v)
        }
        Tok::Symbol('-') => {
            let v = parse_factor(cur)?;
            Some(Value::Number(-v.as_number()))
        }
        _ => None,
    }
}

fn parse_factor_with_vm(cur: &mut Cursor, vm: &mut Vm) -> Option<Value> {
    match cur.next()? {
        Tok::Number(n) => Some(Value::Number(*n)),
        Tok::String(s) => Some(Value::Str(s.clone())),
        Tok::Ident(name) => {
            // Function call? e.g., LEN(x), CHR$(n), ASC(s), VAL(s), STR$(n)
            if let Some(Tok::Symbol('(')) = cur.peek() {
                cur.next();
                // Support 1-3 arguments (for MID$)
                let arg1 = parse_expression_with_vm(cur, vm)?;
                let mut args: Vec<Value> = vec![arg1];
                while let Some(Tok::Symbol(',')) = cur.peek() {
                    cur.next();
                    if let Some(v) = parse_expression_with_vm(cur, vm) {
                        args.push(v);
                    } else {
                        return None;
                    }
                }
                if !matches!(cur.next(), Some(Tok::Symbol(')'))) {
                    return None;
                }
                let up = name.to_ascii_uppercase();
                let res = match up.as_str() {
                    "LEN" => {
                        let a0 = args.remove(0);
                        match a0 {
                            Value::Str(s) => Value::Number(s.chars().count() as f64),
                            Value::Number(n) => Value::Number(n.to_string().chars().count() as f64),
                            Value::Tab(_) => Value::Number(0.0),
                        }
                    }
                    "CHR$" => {
                        let code = args.get(0).unwrap().as_number();
                        let ch = (code as i64).clamp(0, 255) as u8 as char;
                        Value::Str(ch.to_string())
                    }
                    "ASC" => match args.remove(0) {
                        Value::Str(s) => {
                            let copt = s.chars().next();
                            Value::Number(copt.map(|c| c as u32 as f64).unwrap_or(0.0))
                        }
                        Value::Number(n) => Value::Number(n as f64),
                        Value::Tab(_) => Value::Number(0.0),
                    },
                    "VAL" => match args.remove(0) {
                        Value::Str(s) => s
                            .parse::<f64>()
                            .map(Value::Number)
                            .unwrap_or(Value::Number(0.0)),
                        Value::Number(n) => Value::Number(n),
                        Value::Tab(_) => Value::Number(0.0),
                    },
                    "STR$" => match args.remove(0) {
                        Value::Number(n) => Value::Str(n.to_string()),
                        Value::Str(s) => Value::Str(s),
                        Value::Tab(_) => Value::Str(String::new()),
                    },
                    "ABS" => Value::Number(args.remove(0).as_number().abs()),
                    "INT" => Value::Number(args.remove(0).as_number().trunc()),
                    "LEFT$" => {
                        let s = match args.get(0) {
                            Some(Value::Str(s)) => s.clone(),
                            v => v
                                .as_ref()
                                .map(|x| {
                                    if let Value::Number(n) = x {
                                        n.to_string()
                                    } else {
                                        String::new()
                                    }
                                })
                                .unwrap_or_default(),
                        };
                        let n = args.get(1).map(|v| v.as_number() as usize).unwrap_or(0);
                        Value::Str(s.chars().take(n).collect())
                    }
                    "RIGHT$" => {
                        let s = match args.get(0) {
                            Some(Value::Str(s)) => s.clone(),
                            v => v
                                .as_ref()
                                .map(|x| {
                                    if let Value::Number(n) = x {
                                        n.to_string()
                                    } else {
                                        String::new()
                                    }
                                })
                                .unwrap_or_default(),
                        };
                        let n = args.get(1).map(|v| v.as_number() as usize).unwrap_or(0);
                        let len = s.chars().count();
                        let take = n.min(len);
                        Value::Str(s.chars().skip(len - take).collect())
                    }
                    "MID$" => {
                        // MID$(s, start[, len])  start 1-based
                        let s = match args.get(0) {
                            Some(Value::Str(s)) => s.clone(),
                            v => v
                                .as_ref()
                                .map(|x| {
                                    if let Value::Number(n) = x {
                                        n.to_string()
                                    } else {
                                        String::new()
                                    }
                                })
                                .unwrap_or_default(),
                        };
                        let start = args.get(1).map(|v| v.as_number() as isize).unwrap_or(1);
                        let len_opt = args.get(2).map(|v| v.as_number() as usize);
                        let chars: Vec<char> = s.chars().collect();
                        if start <= 0 {
                            return Some(Value::Str(String::new()));
                        }
                        let idx0 = (start as usize).saturating_sub(1);
                        if idx0 >= chars.len() {
                            return Some(Value::Str(String::new()));
                        }
                        let slice = if let Some(l) = len_opt {
                            &chars[idx0..(idx0 + l).min(chars.len())]
                        } else {
                            &chars[idx0..]
                        };
                        Value::Str(slice.iter().collect())
                    }
                    "RND" => {
                        // RND() -> 0..1; RND(n) mimic: if n<=0 reseed (simple), else new value
                        if let Some(v) = args.get(0) {
                            if v.as_number() <= 0.0 { /* no-op reseed */ }
                        }
                        Value::Number(vm.next_rand())
                    }
                    "TAB" => {
                        let n = args.get(0).map(|v| v.as_number() as usize).unwrap_or(0);
                        Value::Tab(n)
                    }
                    _ => return None,
                };
                Some(res)
            } else {
                // Variable value; defaults to 0 if undefined
                match vm.vars.get(name) {
                    Some(v) => Some(v.clone()),
                    None => Some(Value::Number(0.0)),
                }
            }
        }
        Tok::Symbol('(') => {
            let v = parse_expression_with_vm(cur, vm)?;
            if !matches!(cur.next(), Some(Tok::Symbol(')'))) {
                return None;
            }
            Some(v)
        }
        Tok::Symbol('-') => {
            let v = parse_factor_with_vm(cur, vm)?;
            Some(Value::Number(-v.as_number()))
        }
        _ => None,
    }
}
