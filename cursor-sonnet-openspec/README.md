# BASIC M6502 Interpreter (Rust Edition)

A Rust implementation of the classic Microsoft BASIC 6502 interpreter.

## Features

### Implemented

- ✅ **Lexical Analysis (Tokenizer)**: Complete tokenization of BASIC source code
  - All 27 statement keywords
  - All 22 built-in functions
  - Numbers (integer, float, scientific notation)
  - Strings and identifiers
  - All operators and separators
  - Colon statement separator for multi-statement lines

- ✅ **Syntax Parsing (Parser)**: Full expression and statement parsing
  - Expression parsing with correct operator precedence
  - All control flow statements (IF, FOR, GOTO, GOSUB)
  - All I/O statements (PRINT, INPUT)
  - Array and function support
  - Multi-statement lines (colon-separated)

- ✅ **AST (Abstract Syntax Tree)**: Complete AST data structures
  - Expression nodes
  - Statement nodes
  - Program line representation

- ✅ **Error Handling**: Comprehensive error types
  - Syntax errors
  - Runtime errors
  - Type mismatch errors

- ✅ **Runtime Environment**: Program execution engine
  - Program storage (BTreeMap)
  - Execution state management
  - Call stack (GOSUB/FOR loops)
  - Jump control (GOTO/GOSUB)
  - Commands (NEW, RUN, STOP, CONT)

- ✅ **Variable System**: Variable storage and management
  - Variable types (numbers, strings)
  - Simple variables (HashMap)
  - Multi-dimensional arrays
  - Type checking
  - Implicit array creation
  - CLEAR command

- ✅ **Execution Engine**: Expression evaluation and statement execution
  - Expression evaluator (arithmetic, relational, logical, string operations)
  - Built-in functions (math: SGN, INT, ABS, SQR, SIN, COS, TAN, ATN, LOG, EXP)
  - Built-in functions (string: LEN, ASC, CHR$, STR$, VAL, LEFT$, RIGHT$, MID$)
  - Statement execution (LET, DIM, END, STOP, NEW, CLEAR)
  - Control flow (PRINT, GOTO, IF...THEN, GOSUB/RETURN)

- ✅ **I/O System**: Input/Output and data handling
  - INPUT statement (basic input, prompts, multiple variables, error handling)
  - DATA/READ/RESTORE mechanism
  - PRINT with separators (comma, semicolon)
  - TAB() and SPC() functions

- ✅ **Interactive REPL**: Command-line interface with line editing
  - Rustyline integration (line editing, history, Home/End, arrows)
  - Command history (persistent `.basic_history` file)
  - Direct mode and program mode
  - LIST, RUN, NEW, CONT commands
  - Ctrl+C interrupt handling
  - Ctrl+D exit

- ✅ **File I/O**: Program persistence
  - SAVE command (save program to text file)
  - LOAD command (load program from text file)
  - Text-based file format (can be edited manually)
  - Automatic program parsing on load

### Planned

- ⏳ **Advanced Features**: DEF FN, RND function, GET statement

## Testing

All implemented features have comprehensive unit tests:

```bash
cargo test
```

Current test status: **178 tests passing** ✅

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run
```

The REPL supports:
- **Program mode**: Enter lines with line numbers (e.g., `10 PRINT "HELLO"`)
- **Direct mode**: Execute commands immediately (e.g., `PRINT 2+2`)
- **LIST**: Display the current program
- **RUN**: Execute the program
- **NEW**: Clear the program
- **CONT**: Continue after STOP or Ctrl+C
- **SAVE "filename.bas"**: Save program to file
- **LOAD "filename.bas"**: Load program from file
- **Ctrl+C**: Interrupt running program
- **Ctrl+D**: Exit the REPL
- **Arrow keys**: Navigate and edit commands (via rustyline)
- **History**: Up/Down arrows to browse command history

## Language Features

### Statements (27)
END, FOR, NEXT, DATA, INPUT, DIM, READ, LET, GOTO, RUN, IF, RESTORE, GOSUB, RETURN, REM, STOP, ON, NULL, WAIT, LOAD, SAVE, DEF, POKE, PRINT, CONT, LIST, CLEAR, GET, NEW

### Functions (22)
SGN, INT, ABS, USR, FRE, POS, SQR, RND, LOG, EXP, COS, SIN, TAN, ATN, PEEK, LEN, STR$, VAL, ASC, CHR$, LEFT$, RIGHT$, MID$

### Operators
- Arithmetic: +, -, *, /, ^ (power)
- Relational: =, <>, <, >, <=, >=
- Logical: AND, OR, NOT

## Architecture

- `src/error.rs`: Error types
- `src/token.rs`: Token definitions
- `src/tokenizer.rs`: Lexical analyzer
- `src/ast.rs`: AST data structures
- `src/parser.rs`: Syntax parser
- `src/runtime.rs`: Execution engine
- `src/variables.rs`: Variable management
- `src/functions.rs`: Built-in functions (planned)
- `src/operators.rs`: Operator implementations (planned)
- `src/io.rs`: I/O system (planned)

## License

This project is a modern reimplementation of the classic Microsoft BASIC 6502 interpreter.

## Development Progress

See `openspec/changes/implement-basic-interpreter/tasks.md` for detailed task tracking.

