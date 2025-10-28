# Runtime Specification

## ADDED Requirements

### Requirement: 程序存储和管理
系统 SHALL 存储 BASIC 程序行，按行号排序，支持插入、删除和查找。

#### Scenario: 添加程序行
- **WHEN** 输入 "10 PRINT HELLO"
- **THEN** 行 10 被添加到程序中

#### Scenario: 替换现有行
- **WHEN** 已有行 10，再输入 "10 PRINT WORLD"
- **THEN** 行 10 被新内容替换

#### Scenario: 删除程序行
- **WHEN** 输入 "10"（仅行号，无内容）
- **THEN** 行 10 从程序中删除

#### Scenario: 行号排序
- **WHEN** 输入顺序为 30, 10, 20
- **THEN** 程序按 10, 20, 30 顺序存储

### Requirement: 程序执行
系统 SHALL 从指定行号开始顺序执行程序，直到遇到 END, STOP 或程序结尾。

#### Scenario: 从第一行开始执行
- **WHEN** 执行 RUN 命令
- **THEN** 从最小行号开始执行

#### Scenario: 从指定行开始
- **WHEN** 执行 "RUN 100"
- **THEN** 从行 100 开始执行

#### Scenario: 顺序执行
- **WHEN** 程序有行 10, 20, 30
- **THEN** 依次执行 10, 20, 30

#### Scenario: END 停止执行
- **WHEN** 遇到 END 语句
- **THEN** 程序正常结束

### Requirement: 行号跳转
系统 SHALL 支持 GOTO 和 GOSUB 的行号跳转。

#### Scenario: GOTO 跳转
- **WHEN** 执行 "GOTO 100"
- **THEN** 下一条执行的是行 100

#### Scenario: 跳转到不存在的行
- **WHEN** GOTO 到不存在的行号
- **THEN** 返回 RuntimeError::UndefinedLine

#### Scenario: 向前跳转
- **WHEN** 从行 100 GOTO 50
- **THEN** 成功向前跳转

### Requirement: 子程序调用栈
系统 SHALL 维护 GOSUB 调用栈，支持嵌套子程序调用。

#### Scenario: GOSUB 调用
- **WHEN** 在行 10 执行 "GOSUB 100"
- **THEN** 跳转到 100，返回地址入栈

#### Scenario: RETURN 返回
- **WHEN** 执行 RETURN
- **THEN** 返回到上一个 GOSUB 的下一条语句

#### Scenario: 嵌套 GOSUB
- **WHEN** 执行嵌套的 GOSUB
- **THEN** 栈正确维护，RETURN 按正确顺序返回

#### Scenario: RETURN 无对应 GOSUB
- **WHEN** 没有 GOSUB 就执行 RETURN
- **THEN** 返回 RuntimeError::ReturnWithoutGosub

#### Scenario: GOSUB 栈深度限制
- **WHEN** GOSUB 嵌套超过限制（如 100 层）
- **THEN** 返回 RuntimeError::StackOverflow

### Requirement: FOR 循环栈
系统 SHALL 维护 FOR 循环栈，管理循环变量、终值和步长。

#### Scenario: FOR 循环执行
- **WHEN** 执行 "FOR I = 1 TO 10"
- **THEN** 循环信息入栈，I 初始化为 1

#### Scenario: NEXT 执行
- **WHEN** 执行 "NEXT I"
- **THEN** I 增加步长，检查是否继续循环

#### Scenario: 循环结束
- **WHEN** I 超过终值
- **THEN** 退出循环，继续执行 NEXT 后的语句

#### Scenario: 嵌套 FOR 循环
- **WHEN** 执行嵌套 FOR 循环
- **THEN** 内外循环正确管理

#### Scenario: NEXT 变量不匹配
- **WHEN** FOR I ... NEXT J
- **THEN** 返回 RuntimeError::NextWithoutFor

### Requirement: 直接模式执行
系统 SHALL 支持直接执行语句（无行号），不保存到程序中。

#### Scenario: 直接执行 PRINT
- **WHEN** 输入 "PRINT 2+3"（无行号）
- **THEN** 立即执行并输出 5

#### Scenario: 直接模式中的变量
- **WHEN** 直接执行 "A=5" 然后 "PRINT A"
- **THEN** 变量在直接模式和程序模式间共享

#### Scenario: 直接模式不能使用 GOTO
- **WHEN** 直接模式执行 "GOTO 100"
- **THEN** 返回错误（无上下文）

### Requirement: NEW 命令
系统 SHALL 实现 NEW 命令，清空程序和变量。

#### Scenario: NEW 清空程序
- **WHEN** 程序有多行，执行 NEW
- **THEN** 程序被完全清空

#### Scenario: NEW 清空变量
- **WHEN** 有变量值，执行 NEW
- **THEN** 所有变量被清空

#### Scenario: NEW 重置运行状态
- **WHEN** 程序在运行中断后执行 NEW
- **THEN** 运行状态完全重置

### Requirement: LIST 命令
系统 SHALL 实现 LIST 命令，显示程序内容。

#### Scenario: LIST 全部程序
- **WHEN** 执行 LIST
- **THEN** 显示所有程序行

#### Scenario: LIST 单行
- **WHEN** 执行 "LIST 10"
- **THEN** 仅显示行 10

#### Scenario: LIST 范围
- **WHEN** 执行 "LIST 10-50"
- **THEN** 显示行 10 到 50

#### Scenario: 空程序 LIST
- **WHEN** 程序为空时执行 LIST
- **THEN** 无输出或显示 "EMPTY"

### Requirement: STOP 和 CONT
系统 SHALL 实现 STOP 命令暂停执行，CONT 命令继续执行。

#### Scenario: STOP 暂停
- **WHEN** 执行 STOP
- **THEN** 程序暂停，显示 "BREAK IN line"

#### Scenario: CONT 继续
- **WHEN** STOP 后执行 CONT
- **THEN** 从 STOP 的下一条语句继续

#### Scenario: 未暂停时 CONT
- **WHEN** 没有 STOP 就执行 CONT
- **THEN** 返回 RuntimeError::CantContinue

#### Scenario: 程序修改后 CONT
- **WHEN** STOP 后修改程序再 CONT
- **THEN** 返回错误（无法继续）

### Requirement: RUN 命令
系统 SHALL 实现 RUN 命令，清空变量后执行程序。

#### Scenario: RUN 清空变量
- **WHEN** 有变量值时执行 RUN
- **THEN** 变量被清空

#### Scenario: RUN 保留程序
- **WHEN** 执行 RUN
- **THEN** 程序内容不变

#### Scenario: RUN 指定行号
- **WHEN** 执行 "RUN 100"
- **THEN** 从行 100 开始执行

### Requirement: 错误处理和恢复
系统 SHALL 捕获运行时错误，显示错误消息和行号，返回直接模式。

#### Scenario: 显示错误消息
- **WHEN** 发生运行时错误
- **THEN** 显示错误类型和行号

#### Scenario: 返回直接模式
- **WHEN** 错误发生后
- **THEN** 返回直接模式，等待用户输入

#### Scenario: 错误不破坏状态
- **WHEN** 错误发生
- **THEN** 程序和变量状态保持（可用 LIST 查看）

