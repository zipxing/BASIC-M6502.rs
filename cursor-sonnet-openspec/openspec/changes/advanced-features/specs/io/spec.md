# io Specification Delta

## ADDED Requirements

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

