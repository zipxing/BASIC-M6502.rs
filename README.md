# BASIC-M6502.rs

A modern Rust recreation of Microsoft 6502 BASIC (8K v1.1) semantics and behavior, built for learning and portability.

## Status
- Initial REPL: supports LET assignments, PRINT, and basic expressions (+ - * /, parentheses, string concatenation via `+`).
- Program storage: insert/delete numbered lines; placeholders for LIST/RUN.

## Build & Run
```bash
cd BASIC-M6502.rs
cargo run
```

Examples:
```text
PRINT 1+2*3
LET A=10
A=20
PRINT "HELLO"+123
```

## Architecture
- Lexer: crunch-like tokenizer mapping reserved words to tokens.
- Parser: minimal Pratt-style expression parser.
- Program: BTreeMap-based line storage (Rust-side replacement for TXTTAB chain).
- Runtime: simple VM state with variable table; statements dispatch.

## Roadmap
- LIST/RUN/NEW/CLEAR and execution cursor (NEWSTT semantics)
- Control flow: IF/THEN, GOTO, GOSUB/RETURN, FOR/NEXT
- DATA/READ/RESTORE, INPUT; string functions subset
- Compatibility polish and tests

## License
This subproject is licensed under the MIT License (see `LICENSE`).

Note: The repository root contains `LICENSE` from Microsoft (MIT) for the original 6502 BASIC assembly source. If you redistribute portions of that original source or derivative files, include that original license as required. This Rust code is © 2025 by zipxing@hotmail.com under MIT.
