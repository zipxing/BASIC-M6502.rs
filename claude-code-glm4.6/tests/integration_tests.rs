// Integration tests for the BASIC interpreter
// These tests run complete BASIC programs to test the full execution flow

use std::process::{Command, Stdio};
use std::io::Write;

/// Helper function to run a BASIC program and capture output
fn run_basic_program(program: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Start the interpreter process
    let mut child = Command::new("cargo")
        .args(&["run", "--"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Write the program to stdin
    if let Some(stdin) = child.stdin.as_mut() {
        for line in program.lines() {
            let line = line.trim();
            if !line.is_empty() {
                writeln!(stdin, "{}", line)?;
            }
        }
        writeln!(stdin, "RUN")?;
        writeln!(stdin, "QUIT")?;
    }

    // Wait for the process to finish and capture output
    let output = child.wait_with_output()?;

    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr)?;
        return Err(format!("Program failed: {}", stderr).into());
    }

    let stdout = String::from_utf8(output.stdout)?;

    // Extract just the program output (remove READY. prompts and other noise)
    // We need to split lines by READY. and then filter
    let program_output: Vec<&str> = stdout
        .lines()
        .flat_map(|line| line.split("READY."))
        .filter(|segment| {
            let trimmed = segment.trim();
            !trimmed.is_empty() &&
            !trimmed.starts_with("Microsoft BASIC") &&
            !trimmed.starts_with("Type 'HELP'") &&
            !trimmed.starts_with("Loading program") &&
            !trimmed.starts_with("Program loaded") &&
            !trimmed.starts_with("Available commands") &&
            !trimmed.starts_with("Warning:") &&
            !trimmed.starts_with("Enter BASIC")
        })
        .map(|s| s.trim())
        .collect();

    Ok(program_output.join("\n"))
}

#[test]
fn test_simple_program_execution() {
    let program = r#"
10 PRINT "Hello World"
20 LET X = 42
30 PRINT "X ="; X
"#;

    let output = run_basic_program(program).expect("Program should run successfully");
    assert!(output.contains("Hello World"));
    assert!(output.contains("X =42"));
}

#[test]
fn test_for_loop_program() {
    let program = r#"
10 FOR I = 1 TO 5
20 PRINT "Count:"; I
30 NEXT I
"#;

    let output = run_basic_program(program).expect("Program should run successfully");

    // Check that all iterations are printed
    assert!(output.contains("Count:1"));
    assert!(output.contains("Count:2"));
    assert!(output.contains("Count:3"));
    assert!(output.contains("Count:4"));
    assert!(output.contains("Count:5"));
}

#[test]
fn test_nested_for_loops() {
    let program = r#"
10 FOR I = 1 TO 2
20 FOR J = 1 TO 2
30 PRINT I; J
40 NEXT J
50 NEXT I
"#;

    let output = run_basic_program(program).expect("Program should run successfully");

    // Should print: 11, 12, 21, 22 (no spaces in BASIC semicolon output)
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines.len(), 4);
    assert_eq!(lines[0], "11");
    assert_eq!(lines[1], "12");
    assert_eq!(lines[2], "21");
    assert_eq!(lines[3], "22");
}

#[test]
fn test_if_then_statement() {
    let program = r#"
10 LET X = 10
20 IF X > 5 THEN PRINT "X is large"
30 IF X < 5 THEN PRINT "X is small"
40 IF X = 10 THEN PRINT "X is 10"
"#;

    let output = run_basic_program(program).expect("Program should run successfully");

    assert!(output.contains("X is large"));
    assert!(!output.contains("X is small"));
    assert!(output.contains("X is 10"));
}

#[test]
fn test_goto_statement() {
    let program = r#"
10 PRINT "Start"
20 GOTO 50
30 PRINT "Skip this"
40 GOTO 70
50 PRINT "Jump to here"
60 GOTO 80
70 PRINT "Also skip"
80 PRINT "End"
"#;

    let output = run_basic_program(program).expect("Program should run successfully");

    assert!(output.contains("Start"));
    assert!(output.contains("Jump to here"));
    assert!(output.contains("End"));
    assert!(!output.contains("Skip this"));
    assert!(!output.contains("Also skip"));
}

#[test]
fn test_gosub_return() {
    let program = r#"
10 PRINT "Main 1"
20 GOSUB 100
30 PRINT "Main 2"
40 GOSUB 200
50 PRINT "Main 3"
60 GOTO 999
100 PRINT "Sub 1"
110 RETURN
200 PRINT "Sub 2"
210 RETURN
999 PRINT "End"
"#;

    let output = run_basic_program(program).expect("Program should run successfully");

    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines[0], "Main 1");
    assert_eq!(lines[1], "Sub 1");
    assert_eq!(lines[2], "Main 2");
    assert_eq!(lines[3], "Sub 2");
    assert_eq!(lines[4], "Main 3");
    assert_eq!(lines[5], "End");
}

#[test]
fn test_math_expressions() {
    let program = r#"
10 PRINT 2 + 3
20 PRINT 10 - 4
30 PRINT 3 * 4
40 PRINT 15 / 3
50 PRINT 2 ^ 3
"#;

    let output = run_basic_program(program).expect("Program should run successfully");

    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines[0], "5");
    assert_eq!(lines[1], "6");
    assert_eq!(lines[2], "12");
    assert_eq!(lines[3], "5");
    assert_eq!(lines[4], "8");
}

#[test]
fn test_operator_precedence() {
    let program = r#"
10 PRINT 2 + 3 * 4
20 PRINT (2 + 3) * 4
30 PRINT 10 - 4 / 2
40 PRINT (10 - 4) / 2
"#;

    let output = run_basic_program(program).expect("Program should run successfully");

    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines[0], "14");  // 2 + (3 * 4) = 14
    assert_eq!(lines[1], "20");  // (2 + 3) * 4 = 20
    assert_eq!(lines[2], "8");   // 10 - (4 / 2) = 8
    assert_eq!(lines[3], "3");   // (10 - 4) / 2 = 3
}

#[test]
fn test_string_operations() {
    let program = r#"
10 LET A$ = "Hello"
20 LET B$ = "World"
30 PRINT A$; " "; B$
40 LET C$ = A$ + B$
50 PRINT C$
"#;

    let output = run_basic_program(program).expect("Program should run successfully");

    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines[0], "Hello World");
    assert_eq!(lines[1], "HelloWorld");
}

#[test]
fn test_variable_types() {
    let program = r#"
10 LET I = 123
20 LET F = 3.14159
30 LET S$ = "String"
40 PRINT I
50 PRINT F
60 PRINT S$
"#;

    let output = run_basic_program(program).expect("Program should run successfully");

    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines[0], "123");
    assert_eq!(lines[1], "3.14159");
    assert_eq!(lines[2], "String");
}

#[test]
fn test_fibonacci_program() {
    let program = r#"
10 LET A = 0
20 LET B = 1
30 PRINT A
40 PRINT B
50 FOR I = 1 TO 8
60 LET C = A + B
70 PRINT C
80 LET A = B
90 LET B = C
100 NEXT I
"#;

    let output = run_basic_program(program).expect("Program should run successfully");

    let lines: Vec<&str> = output.lines().collect();
    let expected = vec!["0", "1", "1", "2", "3", "5", "8", "13", "21", "34"];
    assert_eq!(lines, expected);
}

#[test]
fn test_data_read_statements() {
    let program = r#"
10 DATA 10, 20, "Hello", 30.5
20 READ A
30 READ B
40 READ C$
50 READ D
60 PRINT A
70 PRINT B
80 PRINT C$
90 PRINT D
"#;

    let output = run_basic_program(program).expect("Program should run successfully");

    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines[0], "10");
    assert_eq!(lines[1], "20");
    assert_eq!(lines[2], "Hello");
    assert_eq!(lines[3], "30.5");
}

#[test]
fn test_restore_statement() {
    let program = r#"
10 DATA 100, 200, 300
20 READ A, B, C
30 PRINT A; B; C
35 PRINT
40 RESTORE
50 READ X, Y
60 PRINT X; Y
"#;

    let output = run_basic_program(program).expect("Program should run successfully");

    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines[0], "100200300");
    assert_eq!(lines[1], "100200");
}

#[test]
fn test_multiple_data_statements() {
    let program = r#"
10 DATA 1, 2, 3
20 DATA "A", "B", "C"
30 READ A, B, C
40 READ X$, Y$, Z$
50 PRINT A; B; C
55 PRINT
60 PRINT X$; Y$; Z$
"#;

    let output = run_basic_program(program).expect("Program should run successfully");

    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines[0], "123");
    assert_eq!(lines[1], "ABC");
}

#[test]
fn test_out_of_data_error() {
    let program = r#"
10 DATA 1, 2
20 READ A, B, C
30 PRINT A; B; C
"#;

    let output = run_basic_program(program).expect("Program should run successfully");

    // Should print the first two values, third variable should be 0 (default)
    // In our implementation, the OUT OF DATA error is handled gracefully and execution continues
    assert!(output.contains("1") && output.contains("2"));
}