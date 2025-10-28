# Project Context

## Purpose
对Microsoft BASIC 6502 (m6502.asm) 汇编源代码的深入分析,然后制作现代的Rust语言实现。

项目背景：
- 原始代码是 Microsoft BASIC 6502 8K 版本 1.1
- 这是经典的 8 位计算机时代的 BASIC 解释器
- 支持多个历史平台：Apple, Commodore, OSI, MOS Tech/KIM 等
- 目标是保持原有功能语义的同时，使用现代 Rust 实现一个高质量、可维护的解释器

## Tech Stack
- **核心语言**: Rust (stable)
- **交互模式**: rustyline 或类似的行编辑库，提供命令历史、补全等现代功能
- **批处理模式**: 使用 Rust 标准库的文件 I/O 功能
- **数值计算**: 可能需要支持浮点运算库（根据原 BASIC 的数学包功能）
- **字符处理**: UTF-8 字符串处理，同时兼容原有的 ASCII/字符集

## Project Conventions

### Code Style
- 遵循标准 Rust 格式规范 (rustfmt)
- 用尽可能简洁的单文件模块来实现
- 尽量减少外部 crate 依赖，优先使用标准库
- 代码注释要清晰，特别是与原汇编代码对应的部分
- 变量和函数命名采用描述性名称，便于理解

### Architecture Patterns
- **解释器模式**: 词法分析 → 语法分析 → 执行
- **模块化设计**:
  - 词法分析器 (Tokenizer)
  - 语法解析器 (Parser)
  - 运行时环境 (Runtime)
  - 变量管理 (Variables)
  - 内存管理 (Memory)
  - I/O 子系统 (Input/Output)
- **错误处理**: 使用 Result 类型，提供清晰的错误信息
- **状态管理**: 使用结构体封装解释器状态
- **扩展性**: 预留接口以便后续添加扩展功能

### Testing Strategy
- 针对每个功能（BASIC 关键字）编写严格的单元测试
- 集成测试：准备一系列 BASIC 程序测试用例
- 交互模式需要设计人工测试用例，开发者配合进行测试
- 设计一个综合的 BASIC 源文件，尽可能覆盖已有功能
- 对照原 6502 实现的行为，确保语义一致性
- 性能测试：虽然不是主要目标，但要确保合理的执行效率

### Git Workflow
- 暂时不需要复杂的 git 流程，开发者会人工处理
- 提交信息要清晰，说明实现了哪个功能或修复了什么问题

## Domain Context

### Microsoft BASIC 6502 特性
- **数据类型**: 支持整型和浮点数（根据配置）
- **数组支持**: 一维和多维数组
- **字符串处理**: 基本的字符串操作
- **控制流**: IF/THEN/ELSE, FOR/NEXT, WHILE/WEND, GOTO/GOSUB
- **I/O 功能**: INPUT, PRINT, READ/DATA
- **文件操作**: LOAD, SAVE (根据平台)
- **数学函数**: 三角函数、对数、随机数等
- **其他**: DIM, LET, REM, END 等基础语句

### 原汇编代码的配置开关
- REALIO: 目标平台选择
- INTPRC: 整型数组支持
- ADDPRC: 额外精度支持
- LNGERR: 长错误信息
- TIME: 时钟功能
- EXTIO: 扩展 I/O
- DISKO: 存储和加载功能
- ROMSW: ROM 模式开关

### 实现优先级
1. 核心语句：LET, PRINT, INPUT, IF/THEN, GOTO, GOSUB/RETURN
2. 循环结构：FOR/NEXT
3. 数组：DIM
4. 数学运算：算术运算符和基本函数
5. 字符串操作
6. 文件 I/O
7. 高级功能和平台特定功能

## Important Constraints
- 把任务分解为简单明确的小任务，便于测试和开发
- 保持与原 BASIC 6502 语义的兼容性
- 代码要清晰可读，便于后续维护和扩展
- 单个模块不要过大，建议 300-500 行以内
- 优先实现核心功能，再逐步添加扩展功能
- 性能不是第一优先级，但要保持合理的执行效率

## External Dependencies
预期的最小依赖集合：
- **rustyline**: 交互式命令行编辑 (可选，仅用于 REPL 模式)
- **标准库**: std::io, std::collections, std::fs 等
- 其他依赖根据实际需要添加，但要保持最小化原则
