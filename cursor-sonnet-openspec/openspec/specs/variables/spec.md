# variables Specification

## Purpose
TBD - created by archiving change implement-basic-interpreter. Update Purpose after archive.
## Requirements
### Requirement: 变量类型支持
系统 SHALL 支持三种变量类型：数值（整数和浮点）、字符串和数组。

#### Scenario: 数值变量
- **WHEN** 赋值 "A = 42"
- **THEN** A 存储为数值类型

#### Scenario: 字符串变量
- **WHEN** 赋值 `A$ = "HELLO"`
- **THEN** A$ 存储为字符串类型

#### Scenario: 类型区分
- **WHEN** 同时存在 A 和 A$
- **THEN** 两者是不同的变量

### Requirement: 变量命名规则
系统 SHALL 支持单字母变量（A-Z）和字母数字组合（如 A1, B2），字符串变量以 $ 结尾。

#### Scenario: 单字母变量名
- **WHEN** 使用 A, B, ..., Z
- **THEN** 所有单字母都是有效变量名

#### Scenario: 字母数字组合
- **WHEN** 使用 A0, A1, ..., Z9
- **THEN** 所有组合都是有效变量名

#### Scenario: 字符串变量后缀
- **WHEN** 使用 A$, NAME$
- **THEN** $ 后缀标识字符串类型

#### Scenario: 大小写不敏感
- **WHEN** 使用 a, A, 或 A
- **THEN** 视为同一个变量

### Requirement: 变量初始值
系统 SHALL 对未赋值的变量提供默认值：数值为 0，字符串为空字符串。

#### Scenario: 未初始化数值变量
- **WHEN** 读取未赋值的变量 X
- **THEN** 返回 0

#### Scenario: 未初始化字符串变量
- **WHEN** 读取未赋值的 X$
- **THEN** 返回 ""

### Requirement: 变量赋值
系统 SHALL 支持简单变量的赋值和读取。

#### Scenario: 数值赋值
- **WHEN** 执行 "A = 100"
- **THEN** A 的值为 100

#### Scenario: 字符串赋值
- **WHEN** 执行 `B$ = "TEST"`
- **THEN** B$ 的值为 "TEST"

#### Scenario: 变量间赋值
- **WHEN** 执行 "A = 10" 然后 "B = A"
- **THEN** B 的值为 10

#### Scenario: 表达式赋值
- **WHEN** 执行 "C = A + B * 2"
- **THEN** C 的值为表达式计算结果

### Requirement: 数组声明
系统 SHALL 支持 DIM 语句声明数组，指定维度和大小。

#### Scenario: 一维数组声明
- **WHEN** 执行 "DIM A(10)"
- **THEN** 创建大小为 11 的数组（索引 0-10）

#### Scenario: 二维数组声明
- **WHEN** 执行 "DIM B(5, 10)"
- **THEN** 创建 6x11 的二维数组

#### Scenario: 三维数组
- **WHEN** 执行 "DIM C(2, 3, 4)"
- **THEN** 创建三维数组

#### Scenario: 字符串数组
- **WHEN** 执行 "DIM A$(10)"
- **THEN** 创建字符串数组

### Requirement: 数组元素访问
系统 SHALL 支持数组元素的读写，进行边界检查。

#### Scenario: 数组元素赋值
- **WHEN** 执行 "A(5) = 100"
- **THEN** 数组 A 的第 5 个元素为 100

#### Scenario: 数组元素读取
- **WHEN** 执行 "PRINT A(5)"
- **THEN** 输出 100

#### Scenario: 多维数组访问
- **WHEN** 执行 "B(2, 3) = 50"
- **THEN** 二维数组元素被赋值

#### Scenario: 数组下标越界
- **WHEN** DIM A(10)，访问 A(11)
- **THEN** 返回 RuntimeError::SubscriptOutOfRange

#### Scenario: 负数索引
- **WHEN** 访问 A(-1)
- **THEN** 返回错误

### Requirement: 隐式数组声明
系统 SHALL 支持不经 DIM 直接使用数组（默认大小 10）。

#### Scenario: 未声明数组自动创建
- **WHEN** 未 DIM，直接使用 A(5)
- **THEN** 自动创建 DIM A(10)

#### Scenario: 隐式数组大小限制
- **WHEN** 未 DIM，访问 A(11)
- **THEN** 返回错误

### Requirement: 数组重新声明
系统 SHALL 防止对已声明的数组再次 DIM。

#### Scenario: 重复 DIM 错误
- **WHEN** 执行 DIM A(10)，再执行 DIM A(20)
- **THEN** 返回 RuntimeError::RedimensionedArray

### Requirement: 变量清空
系统 SHALL 实现 CLEAR 命令，清空所有变量和数组。

#### Scenario: CLEAR 清空简单变量
- **WHEN** 有变量赋值后执行 CLEAR
- **THEN** 所有变量值被清空（数值为 0，字符串为空）

#### Scenario: CLEAR 清空数组
- **WHEN** 有数组声明后执行 CLEAR
- **THEN** 数组被释放，需重新 DIM

#### Scenario: CLEAR 后变量重用
- **WHEN** CLEAR 后使用变量
- **THEN** 变量恢复默认值

### Requirement: 类型检查
系统 SHALL 在赋值和运算时检查类型匹配。

#### Scenario: 数值变量赋字符串
- **WHEN** 执行 "A = \"HELLO\""（A 是数值变量）
- **THEN** 返回 RuntimeError::TypeMismatch

#### Scenario: 字符串变量赋数值
- **WHEN** 执行 "A$ = 123"
- **THEN** 返回 RuntimeError::TypeMismatch

#### Scenario: 数组类型一致性
- **WHEN** DIM A(10)，执行 A(5) = "TEXT"
- **THEN** 返回类型错误

### Requirement: 作用域管理
系统 SHALL 为用户定义函数（DEF FN）提供局部变量作用域。

#### Scenario: 函数参数作为局部变量
- **WHEN** DEF FNA(X) = X * X，调用 FNA(5)
- **THEN** X 在函数内是局部变量

#### Scenario: 函数外访问函数变量
- **WHEN** 函数定义 DEF FNA(X)，在外部访问 X
- **THEN** X 是全局变量，与函数参数无关

### Requirement: 变量存储优化
系统 SHALL 使用高效的数据结构存储变量（如 HashMap）。

#### Scenario: 快速变量查找
- **WHEN** 查找变量
- **THEN** 时间复杂度为 O(1)

#### Scenario: 内存效率
- **WHEN** 大量变量和数组
- **THEN** 内存使用合理

### Requirement: 调试支持
系统 SHALL 提供查看所有变量的功能（可选的 VAR 或 VARS 命令）。

#### Scenario: 列出所有变量
- **WHEN** 执行 VARS（可选功能）
- **THEN** 显示所有已赋值的变量及其值

