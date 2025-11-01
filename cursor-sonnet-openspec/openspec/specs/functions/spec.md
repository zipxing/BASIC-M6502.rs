# functions Specification

## Purpose
TBD - created by archiving change implement-basic-interpreter. Update Purpose after archive.
## Requirements
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

