# I/O Specification

## ADDED Requirements

### Requirement: INPUT 语句
系统 SHALL 实现 INPUT 语句，从用户读取输入。

#### Scenario: 基本输入
- **WHEN** 执行 "INPUT A"
- **THEN** 显示 "? " 提示符，等待用户输入

#### Scenario: 带提示符的输入
- **WHEN** 执行 `INPUT "ENTER VALUE"; A`
- **THEN** 显示 "ENTER VALUE? "

#### Scenario: 输入多个变量
- **WHEN** 执行 "INPUT A, B, C"
- **THEN** 提示用户输入三个值（逗号分隔）

#### Scenario: 输入类型检查
- **WHEN** INPUT A（数值变量），用户输入 "ABC"
- **THEN** 显示 "?REDO FROM START"，重新输入

#### Scenario: 字符串输入
- **WHEN** INPUT A$，用户输入 HELLO
- **THEN** A$ = "HELLO"

#### Scenario: 字符串带引号
- **WHEN** INPUT A$，用户输入 "HELLO, WORLD"
- **THEN** A$ = "HELLO, WORLD"（保留逗号）

### Requirement: PRINT 输出格式
系统 SHALL 实现 PRINT 的各种输出格式。

#### Scenario: 数值输出格式
- **WHEN** PRINT 正数
- **THEN** 前后各有一个空格
- **WHEN** PRINT 负数
- **THEN** 前有空格，负号紧跟数字

#### Scenario: 科学计数法
- **WHEN** PRINT 极大或极小数值
- **THEN** 使用科学计数法（如 1.5E+10）

#### Scenario: 列对齐（逗号）
- **WHEN** PRINT A, B, C
- **THEN** 每个值占 14 个字符宽度

#### Scenario: 自动换行
- **WHEN** 打印超过行宽（72 字符）
- **THEN** 自动换行

### Requirement: TAB 和 SPC 函数
系统 SHALL 实现 TAB 和 SPC 格式化函数。

#### Scenario: TAB 跳转到指定列
- **WHEN** PRINT TAB(10); "HELLO"
- **THEN** 从第 10 列开始输出

#### Scenario: TAB 小于当前位置
- **WHEN** 当前列 20，执行 TAB(10)
- **THEN** 换行后跳转到第 10 列

#### Scenario: SPC 输出空格
- **WHEN** PRINT SPC(5); "HELLO"
- **THEN** 输出 5 个空格后输出 HELLO

### Requirement: DATA/READ 机制
系统 SHALL 实现 DATA 语句和 READ 语句的数据交换。

#### Scenario: DATA 存储
- **WHEN** 程序包含 "10 DATA 1, 2, 3"
- **THEN** 数据被存储供 READ 使用

#### Scenario: READ 顺序读取
- **WHEN** 执行 "READ A, B, C"
- **THEN** 依次从 DATA 读取 1, 2, 3

#### Scenario: 多行 DATA
- **WHEN** 多行 DATA 语句
- **THEN** 数据连续存储

#### Scenario: 混合数据类型
- **WHEN** DATA 包含数值和字符串
- **THEN** READ 根据变量类型正确读取

#### Scenario: OUT OF DATA 错误
- **WHEN** READ 超过可用数据
- **THEN** 返回 OutOfData 错误

### Requirement: RESTORE 数据指针
系统 SHALL 实现 RESTORE 重置数据指针。

#### Scenario: RESTORE 重置到开头
- **WHEN** READ 几次后执行 RESTORE
- **THEN** 数据指针回到第一个 DATA

#### Scenario: RESTORE 到指定行
- **WHEN** 执行 "RESTORE 100"
- **THEN** 数据指针移到行 100 的 DATA

### Requirement: GET 语句（可选）
系统 SHALL 实现 GET 语句，读取单个字符。

#### Scenario: GET 读取字符
- **WHEN** 执行 "GET A$"
- **THEN** 等待用户按键，读取单个字符

#### Scenario: GET 不回显
- **WHEN** GET 读取字符
- **THEN** 字符不显示在屏幕上

#### Scenario: GET 不等待回车
- **WHEN** GET 读取
- **THEN** 按键立即返回，无需回车

### Requirement: 行编辑功能
系统 SHALL 在交互模式提供行编辑功能。

#### Scenario: 光标移动
- **WHEN** 用户输入时
- **THEN** 支持左右箭头移动光标

#### Scenario: 删除字符
- **WHEN** 用户按 Backspace
- **THEN** 删除前一个字符

#### Scenario: 命令历史
- **WHEN** 用户按上下箭头
- **THEN** 浏览历史命令

#### Scenario: Home/End 键
- **WHEN** 用户按 Home/End
- **THEN** 跳转到行首/行尾

### Requirement: 文件 I/O（可选）
系统 SHALL 实现 LOAD 和 SAVE 命令。

#### Scenario: SAVE 保存程序
- **WHEN** 执行 `SAVE "PROGRAM.BAS"`
- **THEN** 程序保存到文件

#### Scenario: LOAD 加载程序
- **WHEN** 执行 `LOAD "PROGRAM.BAS"`
- **THEN** 从文件加载程序（清空当前程序）

#### Scenario: 文件不存在
- **WHEN** LOAD 不存在的文件
- **THEN** 返回 FileNotFound 错误

### Requirement: 错误输出
系统 SHALL 将错误消息输出到标准错误流。

#### Scenario: 运行时错误显示
- **WHEN** 发生运行时错误
- **THEN** 显示 "?ERROR_NAME IN line"

#### Scenario: 语法错误显示
- **WHEN** 发生语法错误
- **THEN** 显示 "?SYNTAX ERROR"

### Requirement: 输入中断
系统 SHALL 支持用户中断输入和执行。

#### Scenario: Ctrl+C 中断
- **WHEN** 程序运行或等待输入时按 Ctrl+C
- **THEN** 中断执行，返回直接模式

#### Scenario: 中断消息
- **WHEN** Ctrl+C 中断
- **THEN** 显示 "?BREAK IN line"

