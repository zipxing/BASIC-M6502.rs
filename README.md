# BASIC-M6502.rs

A modern Rust recreation of Microsoft 6502 BASIC (8K v1.1) semantics and behavior, built for learning and portability.

## Status
- Initial REPL: supports LET assignments, PRINT, and basic expressions (+ - * /, parentheses, string concatenation via `+`).
- Program storage: insert/delete numbered lines; placeholders for LIST/RUN.

## Versions
- v0.1.0
  - Minimal REPL: LET assignments, PRINT, numeric/string literals, basic expressions (+ - * /, parentheses, '+' concatenation).
  - Program storage (insert/delete lines by number).
  - Test snippet (direct mode):
    ```text
    PRINT 1+2*3
    LET A=10
    A=20
    PRINT "HELLO"+123
    ```
- v0.2.0
  - Program mode: LIST/RUN/NEW/CLEAR implemented; direct mode supports colon-separated statements.
  - PRINT supports variables; '?' recognized as PRINT.
  - GOTO <line> supported in the run loop; minimal IF ... THEN (branch to line or run inline immediate statement).
  - Test snippet (store then run):
    ```text
    10 A=1:PRINT A
    20 A=A+1
    30 IF 5-A THEN 20
    40 PRINT "DONE"
    LIST
    RUN
    ```
  - Test snippet (GOTO):
    ```text
    10 PRINT "START"
    20 GOTO 40
    30 PRINT "THIS WILL BE SKIPPED"
    40 PRINT "END"
    LIST
    RUN
    ```

- v0.3.0
  - Control flow additions: GOSUB/RETURN; FOR/NEXT with optional STEP and variable matching on NEXT.
  - VM keeps simple stacks for subroutines and loops; RETURN without GOSUB and NEXT without FOR report errors.
  - Test snippet (GOSUB/RETURN):
    ```text
    10 GOSUB 100
    20 PRINT "MAIN": END
    100 PRINT "SUB": RETURN
    LIST
    RUN
    ```
  - Test snippet (FOR/NEXT):
    ```text
    10 FOR I=1 TO 3
    20 PRINT I
    30 NEXT I
    LIST
    RUN
    ```

- v0.4.0
  - Data and input features:
    - DATA/READ/RESTORE with a program-wide data pointer.
    - RUN now auto CLEAR variables and RESTORE the data pointer.
    - READ at end of data raises `?OUT OF DATA` and halts the run.
    - INPUT with optional prompt; numeric inputs must parse, else `?REDO FROM START` and re-prompt.
  - Strict type checking (classic BASIC semantics):
    - Variables without `$` are numeric; with `$` are string.
    - LET/READ/INPUT enforce types; mismatches raise `?TYPE MISMATCH`.
  - Test snippet (DATA/READ/RESTORE):
    ```text
    10 DATA 1,2,3,"HI"
    20 READ A,B$
    30 PRINT A,B$
    40 READ A
    50 PRINT A
    60 RESTORE
    70 READ A
    80 PRINT A
    LIST
    RUN
    ```
  - Test snippet (INPUT):
    ```text
    10 INPUT "NAME,AGE? "; N$,A
    20 PRINT N$,A
    RUN
    ```

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
