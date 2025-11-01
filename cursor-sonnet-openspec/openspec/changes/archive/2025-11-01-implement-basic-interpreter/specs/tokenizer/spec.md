# Tokenizer Specification

## ADDED Requirements

### Requirement: Token Type Definition
系统 SHALL 定义所有 BASIC 语言的 token 类型，包括关键字、标识符、字面量、运算符和分隔符。

#### Scenario: 识别保留字
- **WHEN** 输入字符串 "PRINT"
- **THEN** 返回 Token::Print

#### Scenario: 识别变量名
- **WHEN** 输入字符串 "A1"
- **THEN** 返回 Token::Identifier("A1")

#### Scenario: 识别数字
- **WHEN** 输入字符串 "123.45"
- **THEN** 返回 Token::Number(123.45)

### Requirement: 保留字识别
系统 SHALL 识别所有 27 个 BASIC 语句关键字，包括 END, FOR, NEXT, DATA, INPUT, DIM, READ, LET, GOTO, RUN, IF, RESTORE, GOSUB, RETURN, REM, STOP, ON, NULL, WAIT, LOAD, SAVE, DEF, POKE, PRINT, CONT, LIST, CLEAR, GET, NEW。

#### Scenario: 大小写不敏感
- **WHEN** 输入 "print", "PRINT", 或 "Print"
- **THEN** 均识别为 Print 关键字

#### Scenario: 关键字优先于标识符
- **WHEN** 输入 "FOR"
- **THEN** 返回 Token::For 而非 Identifier

### Requirement: 函数名识别
系统 SHALL 识别所有 22 个内置函数名，包括 SGN, INT, ABS, USR, FRE, POS, SQR, RND, LOG, EXP, COS, SIN, TAN, ATN, PEEK, LEN, STR$, VAL, ASC, CHR$, LEFT$, RIGHT$, MID$。

#### Scenario: 字符串函数带美元符号
- **WHEN** 输入 "LEFT$"
- **THEN** 返回 Token::Function("LEFT$")

### Requirement: 数字常量解析
系统 SHALL 解析整数和浮点数常量，支持科学计数法。

#### Scenario: 整数
- **WHEN** 输入 "42"
- **THEN** 返回 Token::Number(42.0)

#### Scenario: 浮点数
- **WHEN** 输入 "3.14159"
- **THEN** 返回 Token::Number(3.14159)

#### Scenario: 科学计数法
- **WHEN** 输入 "1.5E-10"
- **THEN** 返回 Token::Number(1.5e-10)

#### Scenario: 负数
- **WHEN** 输入 "-123"
- **THEN** 返回 Token::Minus 和 Token::Number(123)

### Requirement: 字符串常量解析
系统 SHALL 解析双引号括起的字符串常量，支持空字符串。

#### Scenario: 普通字符串
- **WHEN** 输入 `"HELLO WORLD"`
- **THEN** 返回 Token::String("HELLO WORLD")

#### Scenario: 空字符串
- **WHEN** 输入 `""`
- **THEN** 返回 Token::String("")

#### Scenario: 包含空格的字符串
- **WHEN** 输入 `"  SPACES  "`
- **THEN** 返回 Token::String("  SPACES  ")

### Requirement: 变量名识别
系统 SHALL 识别变量名，支持单字母、字母+数字组合，字符串变量以 $ 结尾。

#### Scenario: 单字母变量
- **WHEN** 输入 "A"
- **THEN** 返回 Token::Identifier("A")

#### Scenario: 字母数字组合
- **WHEN** 输入 "A1", "X9", "Z0"
- **THEN** 分别返回对应的 Identifier

#### Scenario: 字符串变量
- **WHEN** 输入 "A$", "NAME$"
- **THEN** 返回 Token::Identifier("A$"), Token::Identifier("NAME$")

#### Scenario: 数组变量
- **WHEN** 输入 "A(5)"
- **THEN** 返回 Token::Identifier("A"), Token::LeftParen, Token::Number(5), Token::RightParen

### Requirement: 运算符识别
系统 SHALL 识别所有运算符，包括算术运算符 (+, -, *, /, ^)、关系运算符 (=, <>, <, >, <=, >=) 和逻辑运算符 (AND, OR, NOT)。

#### Scenario: 算术运算符
- **WHEN** 输入 "+", "-", "*", "/", "^"
- **THEN** 分别返回对应的运算符 token

#### Scenario: 关系运算符
- **WHEN** 输入 "<=", ">=", "<>"
- **THEN** 返回 Token::LessEqual, Token::GreaterEqual, Token::NotEqual

#### Scenario: 逻辑运算符
- **WHEN** 输入 "AND", "OR", "NOT"
- **THEN** 返回对应的逻辑运算符 token

### Requirement: 行号处理
系统 SHALL 识别行号（1-65535 范围的整数），位于行首时作为行号处理。

#### Scenario: 有效行号
- **WHEN** 输入 "10 PRINT"
- **THEN** 首个 token 为 Token::LineNumber(10)

#### Scenario: 行号范围
- **WHEN** 输入行号 1, 100, 65535
- **THEN** 均作为有效行号识别

### Requirement: 空格和分隔符处理
系统 SHALL 正确处理空格、制表符作为分隔符，逗号和分号作为输出分隔符，冒号作为语句分隔符。

#### Scenario: 空格分隔
- **WHEN** 输入 "PRINT A"
- **THEN** 返回 Token::Print, Token::Identifier("A")

#### Scenario: 多个空格
- **WHEN** 输入 "PRINT   A"
- **THEN** 多余空格被忽略

#### Scenario: 逗号分隔（PRINT 语句）
- **WHEN** 输入 "PRINT A,B"
- **THEN** 返回 Token::Print, Token::Identifier("A"), Token::Comma, Token::Identifier("B")

#### Scenario: 分号（PRINT 紧密连接）
- **WHEN** 输入 "PRINT A;B"
- **THEN** 返回 Token::Print, Token::Identifier("A"), Token::Semicolon, Token::Identifier("B")

#### Scenario: 冒号（语句分隔符）
- **WHEN** 输入 "A=1: B=2"
- **THEN** 返回 Token::Identifier("A"), Token::Equal, Token::Number(1), Token::Colon, Token::Identifier("B"), Token::Equal, Token::Number(2)

#### Scenario: 复杂语句分隔
- **WHEN** 输入 "FOR I=1 TO 10: PRINT I: NEXT I"
- **THEN** 冒号正确分隔三个语句

### Requirement: 注释处理
系统 SHALL 识别 REM 语句，REM 之后到行尾的所有内容作为注释忽略。

#### Scenario: REM 注释
- **WHEN** 输入 "10 REM THIS IS A COMMENT"
- **THEN** REM 之后的文本被忽略

#### Scenario: 单引号注释（可选）
- **WHEN** 输入 "10 PRINT A ' COMMENT"
- **THEN** 单引号之后的内容被忽略

### Requirement: 错误处理
系统 SHALL 对非法字符和格式错误给出清晰的错误消息。

#### Scenario: 非法字符
- **WHEN** 输入包含非 ASCII 字符（除字符串内）
- **THEN** 返回 TokenError::IllegalCharacter

#### Scenario: 未闭合字符串
- **WHEN** 输入 `"HELLO` 缺少闭合引号
- **THEN** 返回 TokenError::UnterminatedString

#### Scenario: 数字格式错误
- **WHEN** 输入 "1.2.3"
- **THEN** 返回 TokenError::InvalidNumber

