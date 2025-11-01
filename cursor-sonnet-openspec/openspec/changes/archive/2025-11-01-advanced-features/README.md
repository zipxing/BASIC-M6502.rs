# Advanced Features Documentation

本文档介绍 BASIC-M6502.rs 解释器的高级功能。

## 目录

1. [系统函数](#系统函数)
2. [用户自定义函数](#用户自定义函数)
3. [高级语句](#高级语句)

---

## 系统函数

### POS(x) - 光标位置

`POS(x)` 函数返回当前打印光标所在的列位置（1-based，从 1 开始计数）。

**语法：**
```basic
POS(x)
```

**参数：**
- `x` - 任意数值（参数值被忽略，仅用于语法兼容性）

**返回值：**
- 返回当前列位置（1-based 整数）

**示例：**
```basic
10 PRINT "HELLO"; POS(0)    REM 打印 "HELLO" 后显示当前位置
20 PRINT TAB(10); POS(0)     REM 使用 TAB 后检查位置
30 PRINT SPC(5); POS(0)      REM 使用 SPC 后检查位置
```

**注意事项：**
- POS 函数在 PRINT 语句执行时更新
- 换行后列位置重置为 1
- TAB 和 SPC 会影响列位置
- POS 的参数值被忽略，但必须提供（BASIC 6502 语法要求）

---

## 用户自定义函数

### DEF FN - 定义函数

`DEF FN` 语句用于定义单行用户自定义函数。

**语法：**
```basic
DEF FN name(param) = expression
```

**参数：**
- `name` - 函数名（标识符）
- `param` - 参数名（单个参数）
- `expression` - 函数体表达式

**示例：**
```basic
10 DEF FN SQUARE(X) = X * X
20 DEF FN DOUBLE(Y) = Y + Y
30 DEF FN ADDG(X) = X + GVAL    REM 可以使用全局变量
```

**限制：**
- 仅支持单行函数定义
- 仅支持单个参数
- 函数名不能与内置函数冲突

### FN - 调用用户函数

`FN name(arg)` 用于调用用户定义函数。

**语法：**
```basic
FN name(arg)
```

**参数：**
- `name` - 函数名（DEF FN 定义的名称）
- `arg` - 参数值（表达式）

**示例：**
```basic
10 DEF FN SQUARE(X) = X * X
20 PRINT FN SQUARE(5)              REM 输出 25
30 PRINT FN SQUARE(3) + FN DOUBLE(2)  REM 函数组合使用
40 PRINT FN SQUARE(FN DOUBLE(2))   REM 嵌套调用，输出 16
```

**作用域规则：**
- 函数参数只在函数体内有效
- 函数可以使用全局变量
- 函数调用不会修改全局变量（即使参数名与全局变量同名）

**示例：作用域演示**
```basic
10 X = 100              REM 全局变量 X
20 DEF FN TEST(X) = X * 2
30 PRINT FN TEST(5)     REM 输出 10，使用参数值 5
40 PRINT X              REM 输出 100，全局变量未被修改
```

---

## 高级语句

### GET - 单字符输入

`GET` 语句从输入缓冲区读取单个字符，不等待回车键。

**语法：**
```basic
GET variable
```

**参数：**
- `variable` - 变量名（简单变量，不支持数组）

**行为：**
- 字符串变量：存储读取的字符（单个字符字符串）
- 数值变量：存储字符的 ASCII 码值
- 无输入时：字符串变量为空字符串，数值变量为 0

**示例：**
```basic
10 GET CH$              REM 读取字符到字符串变量
20 PRINT "CHAR: ["; CH$; "]"
30 GET CH               REM 读取字符的 ASCII 码
40 PRINT "ASCII: "; CH
```

**注意事项：**
- GET 不等待回车键，立即读取可用字符
- 如果没有可用输入，返回空值或 0
- 在标准终端环境中，GET 的行为可能受终端缓冲影响

### NULL - 空语句

`NULL` 语句是一个空操作语句，不执行任何操作。

**语法：**
```basic
NULL
```

**用途：**
- 作为占位符
- 在条件语句中作为空分支
- 在程序调试时临时禁用语句

**示例：**
```basic
10 NULL                REM 空语句
20 PRINT "HELLO": NULL: PRINT "WORLD"  REM 单行多语句
30 IF X > 10 THEN NULL ELSE PRINT "X <= 10"
```

**注意事项：**
- NULL 语句不改变程序状态
- NULL 语句不影响变量值
- NULL 语句不产生输出

---

## 完整示例程序

```basic
10 REM 高级功能演示程序
20 
30 REM POS 函数示例
40 PRINT "POS TEST:";
50 PRINT " COL="; POS(0);
60 PRINT "A"; " COL="; POS(0)
70 
80 REM 用户自定义函数
90 DEF FN SQUARE(X) = X * X
100 DEF FN DOUBLE(Y) = Y + Y
110 PRINT "FN SQUARE(5)="; FN SQUARE(5)
120 PRINT "FN DOUBLE(7)="; FN DOUBLE(7)
130 PRINT "FN SQUARE(3)+FN DOUBLE(2)="; FN SQUARE(3) + FN DOUBLE(2)
140 
150 REM 函数中使用全局变量
160 GVAL = 10
170 DEF FN ADDG(X) = X + GVAL
180 PRINT "FN ADDG(5)="; FN ADDG(5)
190 
200 REM NULL 语句
210 NULL
220 PRINT "AFTER NULL"
230 NULL: NULL: PRINT "MULTIPLE NULL OK"
240 
250 REM GET 语句（需要交互）
260 REM GET CH$
270 REM PRINT "GET CHAR: ["; CH$; "]"
280 
290 END
```

---

## 错误处理

### 常见错误

1. **未定义函数**
   ```
   ?Undefined function: FN UNKNOWN
   ```
   解决方法：在使用前先定义函数

2. **函数参数数量错误**
   ```
   ?FN SQUARE requires 1 argument
   ```
   解决方法：确保调用时提供正确数量的参数

3. **GET 数组元素错误**
   ```
   ?SyntaxError: GET does not support array elements
   ```
   解决方法：GET 只能用于简单变量

---

## 兼容性说明

这些高级功能遵循 Microsoft BASIC 6502 的语义：
- POS 函数参数被忽略（仅用于语法兼容）
- 用户函数仅支持单参数
- GET 在标准终端中可能受缓冲影响
- NULL 语句不会影响程序执行

---

## 相关文档

- [BASIC 语句参考](specs/statements/spec.md)
- [函数参考](specs/functions/spec.md)
- [I/O 操作参考](specs/io/spec.md)

