# Change Proposal: Advanced Features

## Why

BASIC 解释器的核心功能已经实现并经过综合测试验证。现在需要添加高级功能以完善语言特性，包括系统函数、用户自定义函数以及一些高级语句。这些功能将提高解释器的完整性和实用性，使其更接近经典 Microsoft BASIC 6502 的特性。

## What Changes

### 系统函数 (System Functions)
- **POS(x)** - 获取当前打印光标位置

### 用户自定义函数 (User-Defined Functions)
- **DEF FN** - 定义单行函数
- **FN name(arg)** - 调用用户定义函数
- 函数作用域和参数绑定管理

### 高级语句 (Advanced Statements)
- **GET** - 单字符输入（不等待回车）
- **NULL** - 空语句（占位符）

## 不实现的功能

以下功能不在本次变更范围内：
- **FRE(x)** - 查询剩余内存大小
- **PEEK(addr)** - 读取内存地址的值
- **POKE addr, value** - 写入值到内存地址
- **WAIT** - 等待内存地址满足特定条件
- **CMD** - 系统命令
- **SYS** - 调用机器语言程序

## Impact

- **Affected specs**: 
  - `functions` - 添加系统函数 POS 和用户自定义函数支持
  - `statements` - 添加高级语句（GET, NULL）和 DEF FN
  - `io` - 添加 GET 单字符输入

- **Affected code**: 
  - `src/executor.rs` - 添加函数和语句执行逻辑，跟踪打印列位置
  - `src/parser.rs` - 添加语句和函数定义解析
  - `src/tokenizer.rs` - 添加关键字识别（POS, GET, NULL, DEF, FN）
  - `src/variables.rs` - 可能需要添加函数存储和管理

