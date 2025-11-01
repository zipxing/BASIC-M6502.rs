# statements Specification Delta

## ADDED Requirements

### Requirement: 高级语句 - GET
系统 SHALL 实现 GET 语句，用于单字符输入（不等待回车）。

#### Scenario: GET 基本使用
- **WHEN** 执行 `GET A$`
- **THEN** 读取单个字符并赋值给 A$，不等待回车

#### Scenario: GET 数字变量
- **WHEN** 执行 `GET A`
- **THEN** 读取单个字符的 ASCII 码并赋值给 A

#### Scenario: GET 无输入
- **WHEN** 执行 GET 时没有可用输入
- **THEN** 返回空字符串或 0（取决于变量类型）

### Requirement: 高级语句 - NULL
系统 SHALL 实现 NULL 语句作为空语句（占位符）。

#### Scenario: NULL 语句执行
- **WHEN** 执行 `NULL`
- **THEN** 不执行任何操作，继续执行下一条语句

#### Scenario: NULL 在单行多语句中
- **WHEN** 执行 `PRINT "A": NULL: PRINT "B"`
- **THEN** 输出 "A" 和 "B"，NULL 语句被忽略

#### Scenario: NULL 不改变程序状态
- **WHEN** 执行 `X = 10: NULL: PRINT X`
- **THEN** 输出 10，变量值不变

### Requirement: 用户自定义函数 - DEF FN 语句
系统 SHALL 支持 DEF FN 语句定义用户自定义函数。

#### Scenario: DEF FN 定义单行函数
- **WHEN** 执行 `DEF FN SQ(X) = X * X`
- **THEN** 函数 SQ 被定义，接受一个参数 X，返回 X 的平方

#### Scenario: DEF FN 多参数函数
- **WHEN** 执行 `DEF FN ADD(A, B) = A + B`
- **THEN** 函数 ADD 被定义，接受两个参数

#### Scenario: DEF FN 字符串函数
- **WHEN** 执行 `DEF FN UPPER$(S$) = CHR$(ASC(S$) - 32)`
- **THEN** 字符串函数被定义（如果支持）

#### Scenario: DEF FN 函数覆盖
- **WHEN** 先定义 `DEF FN TEST(X) = X`，再定义 `DEF FN TEST(X) = X * 2`
- **THEN** 后定义覆盖前定义

