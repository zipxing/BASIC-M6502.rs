# Functions Specification

## ADDED Requirements

### Requirement: 数学函数 - 基础
系统 SHALL 实现基础数学函数 SGN, INT, ABS。

#### Scenario: SGN 符号函数
- **WHEN** SGN(10)
- **THEN** 返回 1
- **WHEN** SGN(-10)
- **THEN** 返回 -1
- **WHEN** SGN(0)
- **THEN** 返回 0

#### Scenario: INT 取整函数
- **WHEN** INT(3.7)
- **THEN** 返回 3
- **WHEN** INT(-3.7)
- **THEN** 返回 -4（向下取整）

#### Scenario: ABS 绝对值
- **WHEN** ABS(-42)
- **THEN** 返回 42
- **WHEN** ABS(42)
- **THEN** 返回 42

### Requirement: 数学函数 - 平方根和乘方
系统 SHALL 实现 SQR 和 EXP 函数。

#### Scenario: SQR 平方根
- **WHEN** SQR(16)
- **THEN** 返回 4
- **WHEN** SQR(2)
- **THEN** 返回约 1.414

#### Scenario: SQR 负数错误
- **WHEN** SQR(-1)
- **THEN** 返回 IllegalQuantity 错误

#### Scenario: EXP 指数函数
- **WHEN** EXP(0)
- **THEN** 返回 1
- **WHEN** EXP(1)
- **THEN** 返回约 2.718

### Requirement: 数学函数 - 对数
系统 SHALL 实现 LOG 自然对数。

#### Scenario: LOG 函数
- **WHEN** LOG(2.718)
- **THEN** 返回约 1

#### Scenario: LOG 非正数错误
- **WHEN** LOG(0) 或 LOG(-1)
- **THEN** 返回 IllegalQuantity 错误

### Requirement: 三角函数
系统 SHALL 实现 SIN, COS, TAN 三角函数（参数为弧度）。

#### Scenario: SIN 正弦
- **WHEN** SIN(0)
- **THEN** 返回 0
- **WHEN** SIN(π/2)
- **THEN** 返回约 1

#### Scenario: COS 余弦
- **WHEN** COS(0)
- **THEN** 返回 1
- **WHEN** COS(π)
- **THEN** 返回约 -1

#### Scenario: TAN 正切
- **WHEN** TAN(0)
- **THEN** 返回 0
- **WHEN** TAN(π/4)
- **THEN** 返回约 1

### Requirement: 反三角函数
系统 SHALL 实现 ATN 反正切函数。

#### Scenario: ATN 反正切
- **WHEN** ATN(1)
- **THEN** 返回约 π/4
- **WHEN** ATN(0)
- **THEN** 返回 0

### Requirement: 随机数函数
系统 SHALL 实现 RND 随机数生成器。

#### Scenario: RND 生成随机数
- **WHEN** RND(1)
- **THEN** 返回 [0, 1) 范围的随机数

#### Scenario: RND 重复上一个
- **WHEN** RND(0)
- **THEN** 返回上一个随机数

#### Scenario: RND 负数重置种子
- **WHEN** RND(-1)
- **THEN** 重置随机数种子

### Requirement: 字符串函数 - 长度
系统 SHALL 实现 LEN 字符串长度函数。

#### Scenario: LEN 普通字符串
- **WHEN** LEN("HELLO")
- **THEN** 返回 5

#### Scenario: LEN 空字符串
- **WHEN** LEN("")
- **THEN** 返回 0

### Requirement: 字符串函数 - 子串
系统 SHALL 实现 LEFT$, RIGHT$, MID$ 子串函数。

#### Scenario: LEFT$ 左子串
- **WHEN** LEFT$("HELLO", 3)
- **THEN** 返回 "HEL"

#### Scenario: RIGHT$ 右子串
- **WHEN** RIGHT$("HELLO", 2)
- **THEN** 返回 "LO"

#### Scenario: MID$ 中间子串
- **WHEN** MID$("HELLO", 2, 3)
- **THEN** 返回 "ELL"（从位置 2 开始，长度 3）

#### Scenario: MID$ 省略长度
- **WHEN** MID$("HELLO", 3)
- **THEN** 返回 "LLO"（从位置 3 到结尾）

### Requirement: 字符串函数 - 转换
系统 SHALL 实现 STR$, VAL, ASC, CHR$ 转换函数。

#### Scenario: STR$ 数字转字符串
- **WHEN** STR$(123)
- **THEN** 返回 " 123"（前导空格）

#### Scenario: VAL 字符串转数字
- **WHEN** VAL("123")
- **THEN** 返回 123
- **WHEN** VAL("12.5")
- **THEN** 返回 12.5

#### Scenario: VAL 非数字
- **WHEN** VAL("ABC")
- **THEN** 返回 0

#### Scenario: ASC 字符码
- **WHEN** ASC("A")
- **THEN** 返回 65

#### Scenario: ASC 空字符串
- **WHEN** ASC("")
- **THEN** 返回错误

#### Scenario: CHR$ 码转字符
- **WHEN** CHR$(65)
- **THEN** 返回 "A"

#### Scenario: CHR$ 范围检查
- **WHEN** CHR$(256)
- **THEN** 返回错误

### Requirement: 系统函数
系统 SHALL 实现 FRE, POS, PEEK 系统函数。

#### Scenario: FRE 剩余内存
- **WHEN** FRE(0)
- **THEN** 返回模拟的可用内存量

#### Scenario: POS 光标位置
- **WHEN** POS(0)
- **THEN** 返回当前打印位置（0-列宽）

#### Scenario: PEEK 读内存
- **WHEN** PEEK(1024)
- **THEN** 返回模拟内存地址的值

### Requirement: 用户函数
系统 SHALL 实现 USR 函数（可选）。

#### Scenario: USR 调用
- **WHEN** USR(X)
- **THEN** 调用用户定义的机器语言例程（模拟）

