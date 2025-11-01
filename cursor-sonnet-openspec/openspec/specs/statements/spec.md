# statements Specification

## Purpose
TBD - created by archiving change implement-basic-interpreter. Update Purpose after archive.
## Requirements
### Requirement: LET 语句
系统 SHALL 实现 LET 赋值语句，LET 关键字可省略。

#### Scenario: 显式 LET
- **WHEN** 执行 "LET A = 10"
- **THEN** A 被赋值为 10

#### Scenario: 隐式 LET
- **WHEN** 执行 "A = 10"
- **THEN** A 被赋值为 10

#### Scenario: 表达式赋值
- **WHEN** 执行 "A = 2 + 3 * 4"
- **THEN** A 的值为 14

### Requirement: PRINT 语句
系统 SHALL 实现 PRINT 语句，支持多种输出格式。

#### Scenario: 打印数值
- **WHEN** 执行 "PRINT 42"
- **THEN** 输出 "42" 并换行

#### Scenario: 打印字符串
- **WHEN** 执行 `PRINT "HELLO"`
- **THEN** 输出 "HELLO" 并换行

#### Scenario: 打印变量
- **WHEN** A=10，执行 "PRINT A"
- **THEN** 输出 "10"

#### Scenario: 逗号分隔（列对齐）
- **WHEN** 执行 "PRINT A, B, C"
- **THEN** 输出值以列对齐（14 列宽度）

#### Scenario: 分号分隔（紧密连接）
- **WHEN** 执行 "PRINT A; B; C"
- **THEN** 值紧密连接输出

#### Scenario: 行尾分号（不换行）
- **WHEN** 执行 "PRINT A;"
- **THEN** 输出 A 的值但不换行

#### Scenario: 空 PRINT
- **WHEN** 执行 "PRINT"
- **THEN** 仅输出换行

### Requirement: IF...THEN 语句
系统 SHALL 实现条件语句，支持 THEN 后跟行号或语句。

#### Scenario: 条件为真跳转行号
- **WHEN** A>10，执行 "IF A>10 THEN 100"
- **THEN** 跳转到行 100

#### Scenario: 条件为假继续
- **WHEN** A<=10，执行 "IF A>10 THEN 100"
- **THEN** 继续执行下一条语句

#### Scenario: THEN 后跟语句
- **WHEN** 执行 "IF A>10 THEN PRINT A"
- **THEN** 条件为真时执行 PRINT

#### Scenario: 关系运算符
- **WHEN** 支持 =, <>, <, >, <=, >=
- **THEN** 所有关系运算符正确工作

### Requirement: GOTO 语句
系统 SHALL 实现无条件跳转。

#### Scenario: 跳转到指定行
- **WHEN** 执行 "GOTO 100"
- **THEN** 下一条执行行 100

#### Scenario: 行号不存在
- **WHEN** GOTO 到不存在的行号
- **THEN** 返回 UndefinedLine 错误

### Requirement: GOSUB 和 RETURN 语句
系统 SHALL 实现子程序调用和返回。

#### Scenario: 子程序调用
- **WHEN** 执行 "GOSUB 500"
- **THEN** 跳转到 500，返回地址入栈

#### Scenario: 子程序返回
- **WHEN** 执行 "RETURN"
- **THEN** 返回到 GOSUB 的下一条语句

#### Scenario: 嵌套子程序
- **WHEN** 子程序内再调用 GOSUB
- **THEN** 返回顺序正确

### Requirement: FOR...NEXT 循环
系统 SHALL 实现 FOR 循环，支持正负步长。

#### Scenario: 正向循环
- **WHEN** 执行 "FOR I=1 TO 10: PRINT I: NEXT I"
- **THEN** 输出 1 到 10

#### Scenario: 步长为 2
- **WHEN** 执行 "FOR I=0 TO 10 STEP 2"
- **THEN** I 取值 0, 2, 4, 6, 8, 10

#### Scenario: 负步长
- **WHEN** 执行 "FOR I=10 TO 1 STEP -1"
- **THEN** I 从 10 递减到 1

#### Scenario: 嵌套循环
- **WHEN** 嵌套 FOR 循环
- **THEN** 内外循环正确执行

### Requirement: ON...GOTO 和 ON...GOSUB
系统 SHALL 实现基于表达式值的多路跳转。

#### Scenario: ON GOTO
- **WHEN** X=2，执行 "ON X GOTO 100,200,300"
- **THEN** 跳转到 200

#### Scenario: ON GOSUB
- **WHEN** X=1，执行 "ON X GOSUB 100,200,300"
- **THEN** 调用行 100 子程序

#### Scenario: 值超出范围
- **WHEN** X=5，但只有 3 个目标
- **THEN** 继续执行下一条语句（不跳转）

### Requirement: DATA, READ, RESTORE 语句
系统 SHALL 实现数据读取机制。

#### Scenario: READ 从 DATA 读取
- **WHEN** "10 DATA 1,2,3" 和 "20 READ A"
- **THEN** A 的值为 1

#### Scenario: 多次 READ
- **WHEN** 连续 READ A, B, C
- **THEN** 依次读取 DATA 中的值

#### Scenario: RESTORE 重置
- **WHEN** 执行 RESTORE
- **THEN** DATA 指针重置到开头

#### Scenario: READ 超出 DATA
- **WHEN** READ 的次数超过 DATA 的数量
- **THEN** 返回 OutOfData 错误

### Requirement: DIM 语句
系统 SHALL 实现数组声明。

#### Scenario: 一维数组
- **WHEN** 执行 "DIM A(100)"
- **THEN** 创建 101 个元素的数组

#### Scenario: 多维数组
- **WHEN** 执行 "DIM B(10,20)"
- **THEN** 创建 11x21 的二维数组

### Requirement: DEF FN 语句
系统 SHALL 实现用户自定义函数。

#### Scenario: 函数定义
- **WHEN** 执行 "DEF FNA(X) = X * X"
- **THEN** 定义平方函数

#### Scenario: 函数调用
- **WHEN** 执行 "Y = FNA(5)"
- **THEN** Y 的值为 25

### Requirement: REM 语句
系统 SHALL 实现注释，REM 后的内容被忽略。

#### Scenario: 注释行
- **WHEN** 执行 "REM THIS IS A COMMENT"
- **THEN** 无任何操作

### Requirement: END 和 STOP 语句
系统 SHALL 实现程序结束和暂停。

#### Scenario: END 结束程序
- **WHEN** 执行 "END"
- **THEN** 程序正常结束

#### Scenario: STOP 暂停程序
- **WHEN** 执行 "STOP"
- **THEN** 程序暂停，可用 CONT 继续

### Requirement: POKE 和 WAIT 语句
系统 SHALL 实现内存操作语句（模拟）。

#### Scenario: POKE 写入
- **WHEN** 执行 "POKE 1024, 65"
- **THEN** 模拟写入内存地址

#### Scenario: WAIT 等待
- **WHEN** 执行 "WAIT 1024, 1"
- **THEN** 模拟等待条件

