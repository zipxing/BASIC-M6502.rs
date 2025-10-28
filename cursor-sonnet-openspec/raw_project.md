# Project Context

## Purpose
对Microsoft BASIC 6502 (m6502.asm) 汇编源代码的深入分析,然后制作现代的Rust语言实现。

## Tech Stack
- Rust 
- 交互模式可采用rustyline 或类似的行编辑库，提供命令历史、补全等现代功能
- 批处理模式使用 Rust 标准库

## Project Conventions

### Code Style
- 遵循标准 Rust 格式规范 (rustfmt)
- 用尽可能简洁的单文件模块来实现
- 尽量减少外部 crate 依赖，优先使用标准库

### Architecture Patterns

### Testing Strategy
- 针对每个功能（BASIC 关键字）编写严格的单元测试
- 交互模式需要设计人工测试用例，开发者配合进行测试
- 设计一个综合的 BASIC 源文件，尽可能覆盖已有功能

### Git Workflow
- 暂时不需要复杂的 git 流程，开发者会人工处理

## Domain Context

## Important Constraints
- 把任务分解为简单明确的小任务，便于测试和开发

## External Dependencies
