use crate::tokens::{lookup_keyword_upper, Tok, TokenKind};

/// Crunch-like lexer:
/// - collapse reserved words into a single Keyword token; otherwise follow BASIC-ish rules.
/// - simplified: does not special-case REM here; handle in parser/executor if needed.
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
                if i < b.len() { i += 1; }
            }
            _ => {
                out.push(Tok::Symbol(c));
                i += 1;
            }
        }
    }
    out
}

/// Parse an optional leading line number. Returns (line_no, rest).
pub fn take_leading_line_number(src: &str) -> Option<(u16, &str)> {
    let s = src.trim_start();
    let bytes = s.as_bytes();
    let mut i = 0usize;
    let mut has_digit = false;
    while i < bytes.len() {
        let ch = bytes[i] as char;
        if ch.is_ascii_digit() { has_digit = true; i += 1; } else { break; }
    }
    if !has_digit { return None; }
    let ln: u16 = s[..i].parse().ok()?;
    Some((ln, &s[i..]))
}

