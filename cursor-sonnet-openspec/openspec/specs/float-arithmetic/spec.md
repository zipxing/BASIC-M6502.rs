# float-arithmetic Specification

## Purpose
TBD - created by archiving change implement-basic-interpreter. Update Purpose after archive.
## Requirements
### Requirement: 浮点数表示
系统 SHALL 使用 IEEE 754 双精度浮点数（f64）表示数值。

#### Scenario: 正常浮点数
- **WHEN** 存储 3.14159
- **THEN** 保持完整精度

#### Scenario: 极小数值
- **WHEN** 存储 1.5E-200
- **THEN** 正确表示

#### Scenario: 极大数值
- **WHEN** 存储 9.9E+99
- **THEN** 正确表示

### Requirement: 浮点加法
系统 SHALL 实现精确的浮点加法。

#### Scenario: 正数相加
- **WHEN** 计算 1.5 + 2.3
- **THEN** 返回 3.8

#### Scenario: 正负数相加
- **WHEN** 计算 10.5 + (-3.2)
- **THEN** 返回 7.3

#### Scenario: 极小数相加
- **WHEN** 计算 1E-10 + 2E-10
- **THEN** 返回 3E-10

### Requirement: 浮点减法
系统 SHALL 实现精确的浮点减法。

#### Scenario: 正数相减
- **WHEN** 计算 5.7 - 2.3
- **THEN** 返回 3.4

#### Scenario: 精度损失处理
- **WHEN** 计算接近值相减
- **THEN** 结果保持有效精度

### Requirement: 浮点乘法
系统 SHALL 实现精确的浮点乘法。

#### Scenario: 正常乘法
- **WHEN** 计算 2.5 * 4.0
- **THEN** 返回 10.0

#### Scenario: 零乘法
- **WHEN** 计算 0.0 * 任意数
- **THEN** 返回 0.0

#### Scenario: 负数乘法
- **WHEN** 计算 -2.5 * 3.0
- **THEN** 返回 -7.5

### Requirement: 浮点除法
系统 SHALL 实现精确的浮点除法，处理除以零。

#### Scenario: 正常除法
- **WHEN** 计算 10.0 / 4.0
- **THEN** 返回 2.5

#### Scenario: 除以零
- **WHEN** 计算 5.0 / 0.0
- **THEN** 返回 DivisionByZero 错误

#### Scenario: 零除以数
- **WHEN** 计算 0.0 / 5.0
- **THEN** 返回 0.0

### Requirement: 浮点比较
系统 SHALL 实现浮点数比较，处理精度问题。

#### Scenario: 精确相等
- **WHEN** 比较 3.0 = 3.0
- **THEN** 返回真

#### Scenario: 浮点误差容忍
- **WHEN** 比较 0.1 + 0.2 = 0.3
- **THEN** 考虑浮点精度（可能需要容差）

#### Scenario: 大小比较
- **WHEN** 比较 3.14 < 3.15
- **THEN** 返回真

### Requirement: 数值转换
系统 SHALL 实现整数和浮点数之间的转换。

#### Scenario: 整数转浮点
- **WHEN** 42（整数）参与浮点运算
- **THEN** 自动转换为 42.0

#### Scenario: 浮点转整数（INT 函数）
- **WHEN** INT(3.7)
- **THEN** 返回 3（向下取整）

#### Scenario: 四舍五入（非标准，可选）
- **WHEN** 实现 ROUND 函数
- **THEN** 标准四舍五入

### Requirement: 特殊值处理
系统 SHALL 处理特殊浮点值（无穷大、NaN）。

#### Scenario: 溢出产生无穷大
- **WHEN** 计算极大数值相乘
- **THEN** 返回 Overflow 错误或 Infinity

#### Scenario: 下溢产生零
- **WHEN** 计算极小数值相除
- **THEN** 返回 0 或 Underflow 错误

#### Scenario: 非法运算产生 NaN
- **WHEN** 计算 0.0 / 0.0 或 SQR(-1)
- **THEN** 返回适当错误（不返回 NaN 给用户）

### Requirement: 科学计数法输出
系统 SHALL 在适当时使用科学计数法显示浮点数。

#### Scenario: 大数显示
- **WHEN** PRINT 1234567890
- **THEN** 显示 "1.23456789E+09"

#### Scenario: 小数显示
- **WHEN** PRINT 0.000000123
- **THEN** 显示 "1.23E-07"

#### Scenario: 正常范围显示
- **WHEN** PRINT 123.456
- **THEN** 显示 "123.456"（不用科学计数法）

### Requirement: 精度限制
系统 SHALL 明确浮点数的有效精度范围。

#### Scenario: 有效数字
- **WHEN** 存储和计算浮点数
- **THEN** 保持约 15-16 位有效数字

#### Scenario: 精度丢失警告
- **WHEN** 运算导致精度损失
- **THEN** 结果仍然合理（不报错）

### Requirement: 浮点常量解析
系统 SHALL 解析各种格式的浮点常量。

#### Scenario: 普通小数
- **WHEN** 输入 "3.14"
- **THEN** 解析为 3.14

#### Scenario: 科学计数法
- **WHEN** 输入 "1.5E10" 或 "1.5E+10"
- **THEN** 解析为 15000000000.0

#### Scenario: 负指数
- **WHEN** 输入 "2E-5"
- **THEN** 解析为 0.00002

#### Scenario: 省略小数点
- **WHEN** 输入 "5E3"
- **THEN** 解析为 5000.0

