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

- v0.5.0
  - Flow control and functions:
    - END/STOP to halt execution.
    - Functions: LEN, CHR$, ASC, VAL, STR$, ABS, INT, LEFT$, RIGHT$, MID$, RND.
  - Unified error formatting in program mode: `?ERROR IN <line>` when possible.
  - Test snippet:
    ```text
    10 INPUT "N?" ; N
    20 PRINT LEN ( STR$ ( N ) ) , CHR$ ( 65 ) , ASC ( "A" )
    30 IF N THEN STOP
    35 PRINT "NO STOP"
    40 END
    RUN
    ```
  - Test snippet (string ops and random):
    ```text
    10 PRINT ABS(-3), INT(3.9)
    20 PRINT LEFT$("HELLO",2), RIGHT$("HELLO",3)
    30 PRINT MID$("HELLO",2,2)
    40 PRINT RND(1)
    RUN
    ```
  - Test snippet (ON ... GOSUB):
    ```text
    10 A=2
    20 ON A GOSUB 100,200
    30 PRINT "BACK": END
    100 PRINT "ONE": RETURN
    200 PRINT "TWO": RETURN
    RUN
    ```

- v0.6.0
  - SAVE/LOAD program as plain text (LIST-like format).
  - Test snippet:
    ```text
    10 PRINT "HELLO"
    SAVE "prog.bas"
    NEW
    LOAD "prog.bas"
    LIST
    RUN
    ```

- v0.7.0
  - Flow control: STOP prints `?BREAK IN <line>`; CONT resumes from next line.
  - Math functions: SGN, SIN, COS, TAN, ATN, SQR, EXP, LOG.
  - String helpers: SPACE$, INSTR; improved STR$/VAL formatting (trim, drop trailing .0).
  - Test snippet (STOP/CONT):
    ```text
    10 PRINT "A": STOP
    20 PRINT "B"
    RUN
    CONT
    ```
  - Test snippet (functions):
    ```text
    PRINT SGN(-3), SGN(0), SGN(2)
    PRINT SIN(0), COS(0)
    PRINT SQR(4), EXP(1)
    PRINT "[";SPACE$(5);"]"
    PRINT INSTR("HELLO","LL"), INSTR("HELLO","X"), INSTR("HELLO","L",3)
    PRINT STR$(2), STR$(2.5), VAL("  3.14 ")
    ```

- v0.8.0
  - Ctrl-C handling aligned with STOP/CONT semantics:
    - While running: `^C` triggers `?BREAK IN <line>`, remembers next line for CONT, halts.
    - At READY: prints `^C` and continues.
    - Implementation via `ctrlc` crate + cooperative checks inside VM run loop.
  - Test snippet:
    ```text
    10 PRINT "LOOP"
    20 GOTO 10
    RUN
    ^C
    CONT
    ```

- v0.9.0
  - Unified error handling polish:
    - Program mode prints `?ERROR IN <line>`; READY mode prints `?ERROR`.
    - Expression-level errors surfaced immediately; no placeholder output (e.g., no stray `0`).
    - Arrays: `UNDEFINED ARRAY` vs `BAD SUBSCRIPT` (non-positive/out-of-range) distinguished.
    - PRINT: incomplete expressions (e.g., `PRINT (`) raise `?SYNTAX ERROR`.
    - Program input: support multiple numbered lines in one input using `:` (e.g., `10 ... :20 ...`).
  - Test snippets:
    ```text
    REM Incomplete PRINT expression (program mode)
    10 PRINT (
    RUN

    REM Out-of-data across multiple numbered lines in one input
    10 DATA 1:20 READ A:30 READ B:40 PRINT B
    LIST
    RUN

    REM Arrays: BAD SUBSCRIPT vs UNDEFINED ARRAY (program mode)
    10 DIM A(2):20 PRINT A(3)
    RUN
    NEW
    10 PRINT A(1)
    RUN

    REM Direct mode (READY): errors printed immediately
    DIM A(2)
    PRINT A(3)
    PRINT A(1)
    PRINT (
    ```

- v1.0.0
  - REM 注释支持：遇到 `REM` 后，行内剩余内容视为注释（不支持单引号 `'` 简写）。
  - 统一错误处理与直接模式/程序模式输出行为稳定，准备发布 1.0。
  - 测试片段：
    ```text
    10 REM this is a comment
    20 PRINT 1
    LIST
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

- Deferred (post-1.0)
  - TXTTAB-like compact program storage (byte-packed crunched text + line index)
  - PEEK/POKE/USR hooks (virtual memory model and safe host callbacks)

## License
This subproject is licensed under the MIT License (see `LICENSE`).

Note: The repository root contains `LICENSE` from Microsoft (MIT) for the original 6502 BASIC assembly source. If you redistribute portions of that original source or derivative files, include that original license as required. This Rust code is © 2025 by zipxing@hotmail.com under MIT.
