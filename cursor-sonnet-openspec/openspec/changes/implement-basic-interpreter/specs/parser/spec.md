# Parser Specification

## ADDED Requirements

### Requirement: Expression Parsing
系统 SHALL 解析表达式，包括变量、常量、函数调用和运算符，并正确处理运算符优先级。

#### Scenario: 简单算术表达式
- **WHEN** 输入 tokens 对应 "2 + 3"
- **THEN** 解析为加法表达式节点

#### Scenario: 运算符优先级
- **WHEN** 输入 "2 + 3 * 4"
- **THEN** 解析为 2 + (3 * 4)，乘法优先

#### Scenario: 括号改变优先级
- **WHEN** 输入 "(2 + 3) * 4"
- **THEN** 解析为 (2 + 3) * 4，加法先计算

#### Scenario: 乘方优先级
- **WHEN** 输入 "2 ^ 3 * 4"
- **THEN** 解析为 (2 ^ 3) * 4，乘方最高优先级

### Requirement: 函数调用解析
系统 SHALL 解析函数调用，包括单参数和多参数函数。

#### Scenario: 单参数函数
- **WHEN** 输入 "SIN(X)"
- **THEN** 解析为函数调用，函数名 SIN，参数 X

#### Scenario: 嵌套函数调用
- **WHEN** 输入 "SQR(ABS(X))"
- **THEN** 解析为嵌套的函数调用

#### Scenario: 多参数函数
- **WHEN** 输入 "LEFT$(A$, 5)"
- **THEN** 解析为两个参数的函数调用

### Requirement: 数组访问解析
系统 SHALL 解析数组元素访问，支持多维数组。

#### Scenario: 一维数组
- **WHEN** 输入 "A(5)"
- **THEN** 解析为数组访问，变量 A，索引 5

#### Scenario: 二维数组
- **WHEN** 输入 "B(3, 4)"
- **THEN** 解析为二维数组访问

#### Scenario: 表达式作为索引
- **WHEN** 输入 "A(I + 1)"
- **THEN** 索引部分解析为表达式

### Requirement: LET 语句解析
系统 SHALL 解析赋值语句，支持简单变量和数组元素赋值。

#### Scenario: 简单赋值
- **WHEN** 输入 "LET A = 10"
- **THEN** 解析为赋值语句，变量 A，值 10

#### Scenario: LET 可选
- **WHEN** 输入 "A = 10"（省略 LET）
- **THEN** 同样解析为赋值语句

#### Scenario: 表达式赋值
- **WHEN** 输入 "A = B + C * 2"
- **THEN** 右侧解析为表达式

### Requirement: PRINT 语句解析
系统 SHALL 解析 PRINT 语句，支持多个表达式、分隔符（逗号、分号）。

#### Scenario: 打印单个值
- **WHEN** 输入 "PRINT 42"
- **THEN** 解析为 PRINT 语句，参数列表包含一个表达式

#### Scenario: 打印多个值（逗号分隔）
- **WHEN** 输入 "PRINT A, B, C"
- **THEN** 逗号表示列分隔

#### Scenario: 打印多个值（分号分隔）
- **WHEN** 输入 "PRINT A; B; C"
- **THEN** 分号表示紧密连接

#### Scenario: TAB 和 SPC 函数
- **WHEN** 输入 "PRINT TAB(10); A"
- **THEN** 解析 TAB 函数调用

### Requirement: 流程控制语句解析
系统 SHALL 解析 GOTO, IF...THEN, GOSUB, RETURN 等流程控制语句。

#### Scenario: GOTO 语句
- **WHEN** 输入 "GOTO 100"
- **THEN** 解析为 GOTO，目标行号 100

#### Scenario: IF...THEN 语句
- **WHEN** 输入 "IF A > 10 THEN 200"
- **THEN** 解析为条件语句，条件表达式，目标行号

#### Scenario: IF...THEN 执行语句
- **WHEN** 输入 "IF A > 10 THEN PRINT A"
- **THEN** THEN 后跟语句而非行号

#### Scenario: GOSUB 和 RETURN
- **WHEN** 输入 "GOSUB 500" 和 "RETURN"
- **THEN** 分别解析为子程序调用和返回

### Requirement: 循环语句解析
系统 SHALL 解析 FOR...NEXT 循环语句，支持 STEP。

#### Scenario: 基本 FOR 循环
- **WHEN** 输入 "FOR I = 1 TO 10"
- **THEN** 解析为循环，变量 I，起始 1，结束 10

#### Scenario: 带 STEP 的循环
- **WHEN** 输入 "FOR I = 10 TO 1 STEP -1"
- **THEN** 步长为 -1

#### Scenario: NEXT 语句
- **WHEN** 输入 "NEXT I"
- **THEN** 解析为 NEXT，变量 I

#### Scenario: NEXT 省略变量
- **WHEN** 输入 "NEXT"
- **THEN** 解析为 NEXT，默认最内层循环变量

### Requirement: INPUT 语句解析
系统 SHALL 解析 INPUT 语句，支持提示符和多个变量。

#### Scenario: 基本 INPUT
- **WHEN** 输入 "INPUT A"
- **THEN** 解析为 INPUT 语句，变量列表包含 A

#### Scenario: 带提示符的 INPUT
- **WHEN** 输入 `INPUT "ENTER VALUE"; A`
- **THEN** 提示符为 "ENTER VALUE"

#### Scenario: 输入多个变量
- **WHEN** 输入 "INPUT A, B, C"
- **THEN** 变量列表包含 A, B, C

### Requirement: DIM 语句解析
系统 SHALL 解析 DIM 语句，声明数组维度。

#### Scenario: 一维数组声明
- **WHEN** 输入 "DIM A(10)"
- **THEN** 解析为 DIM，数组 A，大小 10

#### Scenario: 多维数组声明
- **WHEN** 输入 "DIM B(5, 10)"
- **THEN** 解析为 DIM，数组 B，维度 5x10

#### Scenario: 多个数组声明
- **WHEN** 输入 "DIM A(10), B(20)"
- **THEN** 一条语句声明多个数组

### Requirement: DATA/READ/RESTORE 解析
系统 SHALL 解析 DATA, READ, RESTORE 语句。

#### Scenario: DATA 语句
- **WHEN** 输入 "DATA 1, 2, 3, 4"
- **THEN** 解析为 DATA，数据列表 [1, 2, 3, 4]

#### Scenario: 混合类型 DATA
- **WHEN** 输入 `DATA 10, "HELLO", 3.14`
- **THEN** 支持数字和字符串混合

#### Scenario: READ 语句
- **WHEN** 输入 "READ A, B, C"
- **THEN** 解析为 READ，变量列表

#### Scenario: RESTORE 语句
- **WHEN** 输入 "RESTORE" 或 "RESTORE 100"
- **THEN** 重置或跳转到指定行

### Requirement: DEF FN 解析
系统 SHALL 解析用户自定义函数。

#### Scenario: 单参数函数定义
- **WHEN** 输入 "DEF FNA(X) = X * X"
- **THEN** 解析为函数定义，名称 FNA，参数 X，表达式 X * X

#### Scenario: 函数调用
- **WHEN** 输入 "Y = FNA(5)"
- **THEN** 解析为函数调用

### Requirement: ON...GOTO 解析
系统 SHALL 解析 ON...GOTO 和 ON...GOSUB 语句。

#### Scenario: ON GOTO
- **WHEN** 输入 "ON X GOTO 100, 200, 300"
- **THEN** 根据 X 的值跳转到不同行号

#### Scenario: ON GOSUB
- **WHEN** 输入 "ON X GOSUB 100, 200, 300"
- **THEN** 根据 X 的值调用不同子程序

### Requirement: 其他语句解析
系统 SHALL 解析 END, STOP, REM, CLEAR, NEW, LIST 等语句。

#### Scenario: END 语句
- **WHEN** 输入 "END"
- **THEN** 解析为程序结束

#### Scenario: REM 语句
- **WHEN** 输入 "REM THIS IS A COMMENT"
- **THEN** 解析为注释，内容被保留但不执行

#### Scenario: CLEAR 语句
- **WHEN** 输入 "CLEAR"
- **THEN** 解析为清空变量

### Requirement: 语句分隔符
系统 SHALL 支持冒号分隔一行中的多条语句，包括复杂的控制流语句。

#### Scenario: 单行多语句
- **WHEN** 输入 "10 A=1: B=2: PRINT A+B"
- **THEN** 解析为三条语句

#### Scenario: 单行 FOR 循环
- **WHEN** 输入 "10 FOR I=1 TO 10: PRINT I: NEXT I"
- **THEN** 解析为完整的 FOR...NEXT 循环结构

#### Scenario: 单行嵌套 FOR 循环
- **WHEN** 输入 "10 FOR I=1 TO 3: FOR J=1 TO 3: PRINT I*J: NEXT J: NEXT I"
- **THEN** 正确解析嵌套循环

#### Scenario: 单行 IF...THEN 语句
- **WHEN** 输入 "10 IF A>10 THEN A=0: PRINT A"
- **THEN** THEN 后的多条语句被正确解析

#### Scenario: 单行 GOSUB 和计算
- **WHEN** 输入 "10 A=5: GOSUB 100: PRINT A"
- **THEN** 三条语句按顺序执行

#### Scenario: 复杂单行语句
- **WHEN** 输入 "10 INPUT A: IF A>0 THEN B=A*2: PRINT B: GOTO 20"
- **THEN** 所有语句正确解析和执行

### Requirement: 错误处理
系统 SHALL 对语法错误给出清晰的错误位置和消息。

#### Scenario: 缺少表达式
- **WHEN** 输入 "LET A ="（缺少右值）
- **THEN** 返回 ParseError::ExpectedExpression

#### Scenario: 括号不匹配
- **WHEN** 输入 "PRINT (A + B"
- **THEN** 返回 ParseError::UnmatchedParenthesis

#### Scenario: 无效语句
- **WHEN** 输入 "XYZ 123"（XYZ 不是关键字）
- **THEN** 返回 ParseError::InvalidStatement

