# Interactive Mode Demo

## Features Enabled with rustyline

### 1. Command History
- Press `↑` to navigate to previous commands
- Press `↓` to navigate forward
- History is saved to `.basic_history` file
- Persistent across sessions

### 2. Line Editing
- `←` / `→`: Move cursor
- `Ctrl-A` / `Home`: Jump to beginning of line
- `Ctrl-E` / `End`: Jump to end of line
- `Ctrl-K`: Delete from cursor to end of line
- `Ctrl-U`: Delete entire line
- `Alt-B`: Move back one word
- `Alt-F`: Move forward one word
- `Backspace` / `Delete`: Delete characters

### 3. Tab Completion
Type a few letters and press `Tab` to see available completions:
- `PR` + Tab → `PRINT`
- `GO` + Tab → Shows `GOSUB`, `GOTO`
- `LE` + Tab → Shows `LEFT$`, `LEN`, `LET`

All BASIC keywords, commands, and functions are available for completion.

### 4. Mode Detection
The program automatically detects if it's running in:
- **Interactive mode** (terminal): Uses rustyline with all features
- **Batch mode** (pipe/redirect): Uses simple stdin for compatibility

## Testing Interactive Mode

Run the program without input redirection:
```bash
cargo run
```

You'll see:
```
M6502 BASIC (Rust) — interactive REPL; type HELP for help
Features: Command history (↑/↓), line editing, Tab completion
READY. 
```

Try these:
1. Type: `10 PRINT "HELLO"`
2. Press ↑ to recall the command
3. Edit it to: `10 PRINT "WORLD"`
4. Type `PR` and press Tab to see completion
5. Type `HELP` to see all commands

## Testing Batch Mode

Run with input redirection:
```bash
cargo run < test_batch.txt
```

Or with pipes:
```bash
echo "PRINT 123" | cargo run
```

The program will automatically use simple stdin mode for compatibility.

## History File

Commands are saved to `.basic_history` in the current directory.
You can view/edit this file to see or modify your command history.

## Ctrl-C Behavior

- **At READY prompt**: Prints `^C` and continues (doesn't exit)
- **During RUN**: Triggers `?BREAK IN <line>`, use `CONT` to resume
- **Ctrl-D at prompt**: Exits cleanly with "BYE"

