# Changelog v1.1.0 - Enhanced Interactive Mode

## 🎉 New Features

### Interactive Mode with rustyline

The REPL now provides a modern, user-friendly interactive experience:

#### 1. **Command History** 📜
- Navigate through previous commands with ↑/↓ arrow keys
- History persists across sessions in `.basic_history` file
- Automatic deduplication of repeated commands

#### 2. **Line Editing** ✏️
Full cursor control and editing capabilities:
- `←` / `→`: Move cursor character by character
- `Ctrl-A` / `Home`: Jump to line start
- `Ctrl-E` / `End`: Jump to line end
- `Ctrl-K`: Delete from cursor to end
- `Ctrl-U`: Delete entire line
- `Alt-B` / `Alt-F`: Move by word
- Standard Backspace/Delete

#### 3. **Tab Completion** ⭐
Smart keyword completion for BASIC:
- Type partial keyword + Tab to see matches
- All BASIC statements, commands, and functions included
- Example: `PR` + Tab → Shows `PRINT`
- Example: `LE` + Tab → Shows `LEFT$`, `LEN`, `LET`

Completions include:
- **Statements**: PRINT, LET, INPUT, IF, THEN, GOTO, GOSUB, RETURN, FOR, NEXT, etc.
- **Commands**: RUN, LIST, NEW, CLEAR, HELP, CONT, SAVE, LOAD
- **Functions**: Math (ABS, INT, SGN, SQR, SIN, COS, etc.)
- **Functions**: String (LEN, LEFT$, RIGHT$, MID$, CHR$, ASC, etc.)

#### 4. **Smart Mode Detection** 🔍
Automatically switches between modes:
- **Interactive mode** (terminal): Full rustyline features
- **Batch mode** (pipes/files): Simple stdin for compatibility

```bash
# Interactive mode
cargo run
→ Full features enabled

# Batch mode
cargo run < script.bas
→ Simple stdin, works with pipes
```

#### 5. **Improved Ctrl-C Handling** 🛑
- **At READY prompt**: Shows `^C`, doesn't exit
- **During RUN**: Triggers `?BREAK IN <line>`, use CONT
- **Ctrl-D**: Clean exit with "BYE" message

## 🔧 Technical Changes

### Dependencies
- Added `rustyline = "17.0.1"` for interactive features (compatible with 14.x-17.x)

### Code Structure
- `run_interactive_mode()`: rustyline-based REPL
- `run_batch_mode()`: stdin-based for compatibility
- `BasicHelper`: Implements completion, validation, highlighting, hinting
- Automatic terminal detection using `is_terminal()`

### Files Modified
- `src/main.rs`: Complete rewrite of input handling
- `Cargo.toml`: Added rustyline dependency
- `README.md`: Updated with v1.1.0 features

### New Files
- `INTERACTIVE_DEMO.md`: Comprehensive feature guide
- `CHANGELOG_v1.1.md`: This file

## 📊 Impact

### User Experience
- **Before**: Basic line input, no history, no editing
- **After**: Full-featured REPL comparable to Python/Node.js

### Binary Size
- Increase: ~300-500KB (rustyline and dependencies)
- Acceptable trade-off for significantly improved UX

### Compilation Time
- Increase: ~10-20 seconds (additional dependencies)
- One-time cost during development

### Compatibility
- ✅ Maintains full backward compatibility
- ✅ Batch mode works identically to v1.0
- ✅ All existing features preserved

## 🎯 Usage Examples

### Before (v1.0.0)
```bash
$ cargo run
M6502 BASIC (Rust) — initial REPL; type HELP for help
READY. PRINT "HELLO"
HELLO
READY. [No way to recall previous command]
READY. [Have to retype everything]
```

### After (v1.1.0)
```bash
$ cargo run
M6502 BASIC (Rust) — interactive REPL; type HELP for help
Features: Command history (↑/↓), line editing, Tab completion
READY. PRINT "HELLO"
HELLO
READY. [Press ↑]
READY. PRINT "HELLO"  [Auto-filled!]
READY. [Edit to] PRINT "WORLD"
WORLD
READY. PR[Tab]
PRINT  [Completion shown]
```

## 🚀 Future Enhancements (Possible)

- Syntax highlighting for keywords
- Smart hints based on context
- Multi-line editing for complex statements
- Customizable key bindings
- Configurable history size

## 🔍 Code Quality Improvements (Minor Update)

### Compilation Warnings Eliminated
- **Removed unused code**: Deleted unused `Result<T>` type alias from `errors.rs`
- **Fixed dead code warning**: Added `#[allow(dead_code)]` annotation for `line_no` field in `ProgramLine` struct
- **Added LIST command support**: Fixed missing "LIST" keyword mapping in lexer - now fully functional in REPL
- **Clean compilation**: Zero warnings in both debug and release builds

### Testing
```bash
# Verify LIST command works
echo -e "10 PRINT \"TEST\"\n20 PRINT 123\nLIST" | cargo run
```

## 🙏 Acknowledgments

This enhancement brings cursor-gpt5 BASIC to modern REPL standards while maintaining the simplicity and compatibility of the original design.

---

**Version**: 1.1.0  
**Release Date**: 2025-10-27  
**Author**: zipxing@hotmail.com  
**License**: MIT

