use crate::tokens::{Tok, TokenKind};
use crate::value::Value;
use crate::runtime::Vm;

/// Minimal Pratt-style expression parser:
/// supports + - * /, parentheses, and string concatenation via '+'.

#[derive(Clone)]
pub struct Cursor<'a> {
    pub toks: &'a [Tok],
    pub i: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(toks: &'a [Tok]) -> Self { Self { toks, i: 0 } }
    pub fn peek(&self) -> Option<&'a Tok> { self.toks.get(self.i) }
    pub fn next(&mut self) -> Option<&'a Tok> { let t = self.toks.get(self.i); if t.is_some() { self.i+=1; } t }
}

pub fn parse_expression(cur: &mut Cursor) -> Option<Value> {
    parse_term(cur).and_then(|mut lhs| {
        while let Some(tok) = cur.peek() {
            match tok {
                Tok::Symbol('+') => { cur.next();
                    let rhs = parse_term(cur)?;
                    lhs = match (lhs, rhs) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                        (Value::Str(a), Value::Str(b)) => Value::Str(a + &b),
                        (Value::Str(a), Value::Number(b)) => Value::Str(a + &b.to_string()),
                        (Value::Number(a), Value::Str(b)) => Value::Str(a.to_string() + &b),
                    };
                }
                Tok::Symbol('-') => { cur.next(); let rhs = parse_term(cur)?; lhs = Value::Number(lhs.as_number() - rhs.as_number()); }
                _ => break,
            }
        }
        Some(lhs)
    })
}

/// Variant that resolves identifiers through VM variables.
pub fn parse_expression_with_vm(cur: &mut Cursor, vm: &Vm) -> Option<Value> {
    parse_term_with_vm(cur, vm).and_then(|mut lhs| {
        while let Some(tok) = cur.peek() {
            match tok {
                Tok::Symbol('+') => { cur.next();
                    let rhs = parse_term_with_vm(cur, vm)?;
                    lhs = match (lhs, rhs) {
                        (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
                        (Value::Str(a), Value::Str(b)) => Value::Str(a + &b),
                        (Value::Str(a), Value::Number(b)) => Value::Str(a + &b.to_string()),
                        (Value::Number(a), Value::Str(b)) => Value::Str(a.to_string() + &b),
                    };
                }
                Tok::Symbol('-') => { cur.next(); let rhs = parse_term_with_vm(cur, vm)?; lhs = Value::Number(lhs.as_number() - rhs.as_number()); }
                _ => break,
            }
        }
        Some(lhs)
    })
}

fn parse_term(cur: &mut Cursor) -> Option<Value> {
    parse_factor(cur).and_then(|mut lhs| {
        while let Some(tok) = cur.peek() {
            match tok {
                Tok::Symbol('*') => { cur.next(); let rhs = parse_factor(cur)?; lhs = Value::Number(lhs.as_number() * rhs.as_number()); }
                Tok::Symbol('/') => { cur.next(); let rhs = parse_factor(cur)?; lhs = Value::Number(lhs.as_number() / rhs.as_number()); }
                _ => break,
            }
        }
        Some(lhs)
    })
}

fn parse_term_with_vm(cur: &mut Cursor, vm: &Vm) -> Option<Value> {
    parse_factor_with_vm(cur, vm).and_then(|mut lhs| {
        while let Some(tok) = cur.peek() {
            match tok {
                Tok::Symbol('*') => { cur.next(); let rhs = parse_factor_with_vm(cur, vm)?; lhs = Value::Number(lhs.as_number() * rhs.as_number()); }
                Tok::Symbol('/') => { cur.next(); let rhs = parse_factor_with_vm(cur, vm)?; lhs = Value::Number(lhs.as_number() / rhs.as_number()); }
                _ => break,
            }
        }
        Some(lhs)
    })
}

fn parse_factor(cur: &mut Cursor) -> Option<Value> {
    match cur.next()? {
        Tok::Number(n) => Some(Value::Number(*n)),
        Tok::String(s) => Some(Value::Str(s.clone())),
        Tok::Symbol('(') => {
            let v = parse_expression(cur)?;
            if !matches!(cur.next(), Some(Tok::Symbol(')'))) { return None; }
            Some(v)
        }
        Tok::Symbol('-') => {
            let v = parse_factor(cur)?;
            Some(Value::Number(-v.as_number()))
        }
        _ => None,
    }
}

fn parse_factor_with_vm(cur: &mut Cursor, vm: &Vm) -> Option<Value> {
    match cur.next()? {
        Tok::Number(n) => Some(Value::Number(*n)),
        Tok::String(s) => Some(Value::Str(s.clone())),
        Tok::Ident(name) => {
            // Variables default to 0 if undefined (common BASIC behavior)
            match vm.vars.get(name) {
                Some(v) => Some(v.clone()),
                None => Some(Value::Number(0.0)),
            }
        }
        Tok::Symbol('(') => {
            let v = parse_expression_with_vm(cur, vm)?;
            if !matches!(cur.next(), Some(Tok::Symbol(')'))) { return None; }
            Some(v)
        }
        Tok::Symbol('-') => {
            let v = parse_factor_with_vm(cur, vm)?;
            Some(Value::Number(-v.as_number()))
        }
        _ => None,
    }
}

