# operators Specification

## Purpose
TBD - created by archiving change implement-basic-interpreter. Update Purpose after archive.
## Requirements
### Requirement: 算术运算符
系统 SHALL 实现加减乘除和乘方运算符。

#### Scenario: 加法
- **WHEN** 计算 5 + 3
- **THEN** 返回 8

#### Scenario: 减法
- **WHEN** 计算 10 - 7
- **THEN** 返回 3

#### Scenario: 乘法
- **WHEN** 计算 4 * 5
- **THEN** 返回 20

#### Scenario: 除法
- **WHEN** 计算 15 / 3
- **THEN** 返回 5

#### Scenario: 浮点除法
- **WHEN** 计算 10 / 4
- **THEN** 返回 2.5

#### Scenario: 除以零
- **WHEN** 计算 5 / 0
- **THEN** 返回 DivisionByZero 错误

#### Scenario: 乘方
- **WHEN** 计算 2 ^ 3
- **THEN** 返回 8

#### Scenario: 负数乘方
- **WHEN** 计算 (-2) ^ 3
- **THEN** 返回 -8

### Requirement: 一元运算符
系统 SHALL 实现一元负号和正号。

#### Scenario: 一元负号
- **WHEN** 计算 -5
- **THEN** 返回 -5

#### Scenario: 双重负号
- **WHEN** 计算 --5
- **THEN** 返回 5

#### Scenario: 表达式中的负号
- **WHEN** 计算 3 * -2
- **THEN** 返回 -6

### Requirement: 关系运算符
系统 SHALL 实现所有关系运算符，返回 -1（真）或 0（假）。

#### Scenario: 等于
- **WHEN** 计算 5 = 5
- **THEN** 返回 -1（真）
- **WHEN** 计算 5 = 4
- **THEN** 返回 0（假）

#### Scenario: 不等于
- **WHEN** 计算 5 <> 4
- **THEN** 返回 -1（真）

#### Scenario: 小于
- **WHEN** 计算 3 < 5
- **THEN** 返回 -1（真）

#### Scenario: 大于
- **WHEN** 计算 7 > 3
- **THEN** 返回 -1（真）

#### Scenario: 小于等于
- **WHEN** 计算 5 <= 5
- **THEN** 返回 -1（真）

#### Scenario: 大于等于
- **WHEN** 计算 10 >= 5
- **THEN** 返回 -1（真）

### Requirement: 逻辑运算符
系统 SHALL 实现 AND, OR, NOT 逻辑运算符。

#### Scenario: AND 运算
- **WHEN** 计算 -1 AND -1
- **THEN** 返回 -1
- **WHEN** 计算 -1 AND 0
- **THEN** 返回 0

#### Scenario: OR 运算
- **WHEN** 计算 -1 OR 0
- **THEN** 返回 -1
- **WHEN** 计算 0 OR 0
- **THEN** 返回 0

#### Scenario: NOT 运算
- **WHEN** 计算 NOT 0
- **THEN** 返回 -1
- **WHEN** 计算 NOT -1
- **THEN** 返回 0

#### Scenario: 按位逻辑运算
- **WHEN** 计算 5 AND 3（二进制）
- **THEN** 返回 1（按位与）

### Requirement: 字符串运算符
系统 SHALL 实现字符串连接和比较。

#### Scenario: 字符串连接
- **WHEN** 计算 "HELLO" + " " + "WORLD"
- **THEN** 返回 "HELLO WORLD"

#### Scenario: 字符串相等
- **WHEN** 计算 "ABC" = "ABC"
- **THEN** 返回 -1（真）

#### Scenario: 字符串比较（字典序）
- **WHEN** 计算 "ABC" < "XYZ"
- **THEN** 返回 -1（真）

#### Scenario: 大小写敏感
- **WHEN** 计算 "abc" = "ABC"
- **THEN** 返回 0（假）

### Requirement: 运算符优先级
系统 SHALL 正确处理运算符优先级。

#### Scenario: 乘法优先于加法
- **WHEN** 计算 2 + 3 * 4
- **THEN** 返回 14（不是 20）

#### Scenario: 乘方优先于乘法
- **WHEN** 计算 2 * 3 ^ 2
- **THEN** 返回 18（不是 36）

#### Scenario: 括号优先级最高
- **WHEN** 计算 (2 + 3) * 4
- **THEN** 返回 20

#### Scenario: 关系运算符优先级
- **WHEN** 计算 3 + 2 > 4
- **THEN** 先计算 3+2，再比较 5>4，返回 -1

#### Scenario: 逻辑运算符优先级
- **WHEN** 计算 A > 5 AND B < 10
- **THEN** 先计算关系运算，再计算 AND

### Requirement: 类型转换
系统 SHALL 在需要时进行隐式类型转换。

#### Scenario: 整数和浮点混合运算
- **WHEN** 计算 5 + 2.5
- **THEN** 返回 7.5

#### Scenario: 除法总是返回浮点
- **WHEN** 计算 5 / 2
- **THEN** 返回 2.5（不是 2）

### Requirement: 运算符结合性
系统 SHALL 正确处理运算符的结合性。

#### Scenario: 左结合（加法）
- **WHEN** 计算 10 - 3 - 2
- **THEN** 返回 5（从左到右：(10-3)-2）

#### Scenario: 右结合（乘方）
- **WHEN** 计算 2 ^ 3 ^ 2
- **THEN** 返回 512（从右到左：2^(3^2) = 2^9）

### Requirement: 错误处理
系统 SHALL 对运算错误给出清晰提示。

#### Scenario: 类型不匹配
- **WHEN** 计算 "ABC" + 123
- **THEN** 返回 TypeMismatch 错误

#### Scenario: 溢出检测
- **WHEN** 计算超大数值
- **THEN** 返回 Overflow 错误

