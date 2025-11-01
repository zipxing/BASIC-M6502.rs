# io Specification

## Purpose
TBD - created by archiving change implement-basic-interpreter. Update Purpose after archive.
## Requirements
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
系统 SHALL 支持用户中断输入和执行，并保存执行状态以便恢复。

#### Scenario: Ctrl+C 中断程序执行
- **WHEN** 程序运行时按 Ctrl+C
- **THEN** 中断执行，返回直接模式

#### Scenario: Ctrl+C 中断输入等待
- **WHEN** 等待 INPUT 输入时按 Ctrl+C
- **THEN** 中断输入，返回直接模式

#### Scenario: 中断消息显示
- **WHEN** Ctrl+C 中断
- **THEN** 显示 "?BREAK IN line XXX"（显示当前行号）

#### Scenario: 保存中断状态
- **WHEN** Ctrl+C 中断发生
- **THEN** 保存当前执行行号和语句位置

#### Scenario: CONT 从中断点恢复
- **WHEN** Ctrl+C 中断后执行 CONT
- **THEN** 从中断点继续执行程序

#### Scenario: 中断后程序不变
- **WHEN** Ctrl+C 中断后
- **THEN** 程序和变量状态保持不变，可用 LIST 查看

### Requirement: 单字符输入 - GET
系统 SHALL 实现 GET 语句，用于单字符输入（不等待回车）。

#### Scenario: GET 字符串变量
- **WHEN** 执行 `GET A$`
- **THEN** 读取单个字符并赋值给字符串变量 A$，不等待回车键

#### Scenario: GET 数字变量
- **WHEN** 执行 `GET A`
- **THEN** 读取单个字符的 ASCII 码值并赋值给数字变量 A

#### Scenario: GET 无可用输入
- **WHEN** 执行 GET 时输入缓冲区为空
- **THEN** 字符串变量赋值为空字符串，数字变量赋值为 0

#### Scenario: GET 在循环中
- **WHEN** 在循环中多次执行 GET
- **THEN** 每次读取一个字符，直到输入缓冲区为空

#### Scenario: GET 与 INPUT 的区别
- **WHEN** 使用 GET 而不是 INPUT
- **THEN** 不需要按回车键，立即读取可用字符

### Requirement: GET 输入缓冲区
系统 SHALL 维护输入缓冲区以支持 GET 语句。

#### Scenario: GET 读取缓冲区
- **WHEN** 用户输入多个字符后执行 GET
- **THEN** GET 读取缓冲区中的第一个字符

#### Scenario: GET 缓冲区清空
- **WHEN** 执行 GET 读取所有可用字符后
- **THEN** 后续 GET 等待新输入或返回空值

### Requirement: GET 错误处理
系统 SHALL 处理 GET 语句的错误情况。

#### Scenario: GET 数组元素错误
- **WHEN** 执行 `GET A(1)`
- **THEN** 返回 SyntaxError（GET 不支持数组元素）

#### Scenario: GET EOF 处理
- **WHEN** 执行 GET 时遇到 EOF
- **THEN** 返回空字符串或 0，或返回错误（取决于实现）

