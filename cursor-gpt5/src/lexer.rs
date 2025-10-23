use crate::tokens::{lookup_keyword_upper, Tok, TokenKind};

/// Crunch-like lexer:
/// - collapse reserved words into a single Keyword token; otherwise follow BASIC-ish rules.
/// - special-case REM: once seen, treat the rest of the line as a comment and stop.
pub fn crunch(src: &str) -> Vec<Tok> {
    let mut out = Vec::new();
    let mut i = 0usize;
    let b = src.as_bytes();
    while i < b.len() {
        let c = b[i] as char;
        match c {
            ' ' | '\t' => {
                i += 1;
            }
            '?' => {
                // Alias: '?' is PRINT
                i += 1;
                out.push(Tok::Keyword(TokenKind::Print));
            }
            '0'..='9' => {
                let start = i;
                i += 1;
                while i < b.len() && (b[i] as char).is_ascii_digit() {
                    i += 1;
                }
                if i < b.len() && (b[i] as char) == '.' {
                    i += 1;
                    while i < b.len() && (b[i] as char).is_ascii_digit() {
                        i += 1;
                    }
                }
                let n: f64 = src[start..i].parse().unwrap_or(0.0);
                out.push(Tok::Number(n));
            }
            'A'..='Z' | '_' | 'a'..='z' => {
                let start = i;
                i += 1;
                while i < b.len() {
                    let ch = b[i] as char;
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' {
                        i += 1;
                    } else {
                        break;
                    }
                }
                let word = &src[start..i];
                let upper = word.to_ascii_uppercase();
                if let Some(kw) = lookup_keyword_upper(&upper) {
                    if kw == TokenKind::Rem {
                        out.push(Tok::Keyword(kw));
                        // Stop lexing remainder (comment to end of line)
                        break;
                    }
                    out.push(Tok::Keyword(kw));
                } else {
                    out.push(Tok::Ident(upper));
                }
            }
            '"' => {
                // String literal
                i += 1;
                let start = i;
                while i < b.len() && (b[i] as char) != '"' {
                    i += 1;
                }
                let s = src[start..i].to_string();
                out.push(Tok::String(s));
                if i < b.len() {
                    i += 1;
                }
            }
            _ => {
                out.push(Tok::Symbol(c));
                i += 1;
            }
        }
    }
    // Post-process: merge "GO TO" into GOTO for compatibility
    let mut merged: Vec<Tok> = Vec::new();
    let mut j = 0usize;
    while j < out.len() {
        if j + 1 < out.len() {
            let is_go = matches!(&out[j], Tok::Ident(s) if s == "GO");
            let is_to_kw = matches!(&out[j+1], Tok::Keyword(TokenKind::To));
            let is_to_id = matches!(&out[j+1], Tok::Ident(s) if s == "TO");
            if is_go && (is_to_kw || is_to_id) {
                merged.push(Tok::Keyword(TokenKind::Goto));
                j += 2;
                continue;
            }
        }
        merged.push(out[j].clone());
        j += 1;
    }
    merged
}

/// Parse an optional leading line number. Returns (line_no, rest).
pub fn take_leading_line_number(src: &str) -> Option<(u16, &str)> {
    let s = src.trim_start();
    let bytes = s.as_bytes();
    let mut i = 0usize;
    let mut has_digit = false;
    while i < bytes.len() {
        let ch = bytes[i] as char;
        if ch.is_ascii_digit() {
            has_digit = true;
            i += 1;
        } else {
            break;
        }
    }
    if !has_digit {
        return None;
    }
    let ln: u16 = s[..i].parse().ok()?;
    Some((ln, &s[i..]))
}
