# Change Proposal: Advanced Features

## Why

BASIC 解释器的核心功能已经实现，现在需要添加高级功能以完善语言特性，包括系统函数、用户自定义函数以及一些高级语句。这些功能将提高解释器的完整性和实用性。

## What Changes

### 系统函数
- **FRE(x)** - 查询剩余内存
- **POS(x)** - 获取光标位置
- **PEEK(addr)** - 读取内存地址（模拟）
- **POKE addr, value** - 写入内存地址（模拟）
- **WAIT** - 等待语句

### 用户自定义函数
- **DEF FN** - 定义单行函数
- **FN name(arg)** - 调用用户定义函数
- 函数作用域管理

### 高级语句
- **GET** - 单字符输入
- **NULL** - 空语句
- **CMD** 和 **SYS** - 系统命令（可选）

## Impact

- **Affected specs**: 
  - `functions` - 添加系统函数和用户自定义函数
  - `statements` - 添加高级语句
  - `memory` - 添加内存模拟功能（PEEK/POKE）
  - `io` - 添加 GET 语句

- **Affected code**: 
  - `src/executor.rs` - 添加函数和语句执行逻辑
  - `src/parser.rs` - 添加语句解析
  - `src/tokenizer.rs` - 添加关键字识别
  - `src/variables.rs` - 可能需要添加函数存储

