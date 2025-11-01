# functions Specification Delta

## ADDED Requirements

### Requirement: 系统函数 - POS
系统 SHALL 实现 POS(x) 函数，用于获取当前打印光标位置。

#### Scenario: POS 基本使用
- **WHEN** 调用 POS(0)
- **THEN** 返回当前打印列位置（1-based，从 1 开始）

#### Scenario: POS 参数忽略
- **WHEN** 调用 POS(任意值)
- **THEN** 返回相同的当前光标位置（参数被忽略）

#### Scenario: POS 初始位置
- **WHEN** 程序开始时调用 POS(0)
- **THEN** 返回 1（第一列）

#### Scenario: POS 打印后位置
- **WHEN** 执行 `PRINT "HELLO"; POS(0)`
- **THEN** POS(0) 返回 6（假设 "HELLO" 占 5 列，加上当前列）

#### Scenario: POS 换行后重置
- **WHEN** 执行 `PRINT "HELLO": POS(0)`
- **THEN** POS(0) 返回 1（换行后重置到第一列）

#### Scenario: POS 受 TAB 影响
- **WHEN** 执行 `PRINT TAB(10); POS(0)`
- **THEN** POS(0) 返回 11（TAB 移动到第 10 列后，当前位置为 11）

### Requirement: 用户自定义函数 - DEF FN
系统 SHALL 支持 DEF FN 语句定义单行函数。

#### Scenario: DEF FN 定义函数
- **WHEN** 执行 `DEF FN SQ(X) = X * X`
- **THEN** 函数 SQ 被定义，可以后续调用

#### Scenario: DEF FN 函数名冲突
- **WHEN** 重复定义同名函数
- **THEN** 后定义覆盖前定义

### Requirement: 用户自定义函数 - FN 调用
系统 SHALL 支持 FN name(arg) 语法调用用户定义函数。

#### Scenario: FN 调用函数
- **WHEN** 定义了 `DEF FN SQ(X) = X * X` 后调用 FN SQ(5)
- **THEN** 返回 25

#### Scenario: FN 调用未定义函数
- **WHEN** 调用未定义的函数 FN UNDEF(1)
- **THEN** 返回 UndefinedFunction 错误

#### Scenario: FN 参数绑定
- **WHEN** 定义了 `DEF FN ADD(A, B) = A + B` 后调用 FN ADD(3, 4)
- **THEN** 返回 7，参数正确绑定

### Requirement: 用户自定义函数 - 作用域
用户定义函数 SHALL 正确管理参数作用域，不影响全局变量。

#### Scenario: 函数参数作用域
- **WHEN** 定义了 `DEF FN TEST(X) = X * 2`，全局变量 X=10，调用 FN TEST(5)
- **THEN** 返回 10，全局变量 X 仍为 10

