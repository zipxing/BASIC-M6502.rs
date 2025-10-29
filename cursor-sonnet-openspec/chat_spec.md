# 填写项目细节和技术栈
_Exported on 2025/10/29 at GMT+8 10:29:08 from Cursor (1.7.54)_

---

**User**

Please read openspec/project.md and help me fill it out
    with details about my project, tech stack, and conventions 中文对话

---

**Cursor**

我来帮你填写项目文档。让我先读取相关文件了解你的项目。

好的，我看到你的项目已经有一些基本信息了。让我读取一下 m6502.asm 文件的开头部分来更好地了解项目细节，然后帮你完善文档。

好的，我已经了解了你的项目！这是一个很有意思的项目 - 分析经典的 Microsoft BASIC 6502 汇编代码并用 Rust 重新实现。现在我来帮你完善 `project.md` 文档：

```markdown
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

```

完成了！我已经帮你大幅完善了 `openspec/project.md` 文档。主要添加和补充的内容包括：

## 主要改进：

### 1. **项目背景** (Purpose)
- 说明了这是 Microsoft BASIC 6502 8K 版本 1.1
- 强调了历史意义和项目目标

### 2. **技术栈详细信息** (Tech Stack)
- 明确了 Rust stable 版本
- 列出了交互模式和批处理模式的具体技术选择
- 提到了数值计算和字符处理的注意事项

### 3. **代码风格规范** (Code Style)
- 添加了 rustfmt 规范
- 强调了注释和命名规范

### 4. **架构模式** (Architecture Patterns)
- 详细描述了解释器的三阶段模式
- 列出了六个核心模块的设计
- 明确了错误处理和状态管理方法

### 5. **领域上下文** (Domain Context) - 全新添加
这是最重要的补充部分：
- **BASIC 6502 特性清单**：列出了所有需要实现的功能
- **配置开关说明**：解释了原汇编代码中的各种编译开关
- **实现优先级**：给出了清晰的七个阶段的实现顺序

### 6. **约束条件** (Constraints)
- 添加了兼容性、可读性、模块大小等具体约束

### 7. **外部依赖** (Dependencies)
- 列出了最小化的依赖集合

这份文档现在可以作为你和 AI 助手协作的重要参考，确保项目按照统一的标准和优先级进行开发。你觉得还需要补充或修改什么内容吗？

---

**User**

我想先根据原始汇编代码的功能，生成一份实施计划，请使用OpenSpec 创建一个提案。

---

**Cursor**

好的！我来帮你根据原始汇编代码创建一个完整的 OpenSpec 提案。首先让我检查一下现有的状态，然后分析汇编代码的主要功能模块。

好的，目前没有任何 specs 和 changes。现在让我分析汇编代码的主要功能模块，然后创建提案。

很好！让我再看一下命令关键字的列表，然后创建详细的实施提案。

太好了！我现在对整个 BASIC 解释器的结构有了清晰的了解。让我创建一个完整的 OpenSpec 提案。

现在让我创建详细的提案文档。我会根据汇编代码分析，创建一个完整的实施计划。

```markdown
# Change Proposal: Implement BASIC Interpreter

## Why

需要将经典的 Microsoft BASIC 6502 解释器从汇编语言翻译为现代 Rust 实现。原始的 m6502.asm 代码大约 6800 行，包含了完整的 BASIC 解释器功能。通过 Rust 重新实现，可以：

- 提供更好的可维护性和可读性
- 利用 Rust 的类型安全和内存安全特性
- 创建一个可在现代平台运行的经典 BASIC 环境
- 保留原有的 BASIC 6502 语义和行为

## What Changes

这是一个全新的项目实施，将从零开始构建一个完整的 BASIC 解释器。主要组件包括：

### 核心组件
- **词法分析器 (Tokenizer)**: 将源代码转换为 token 流
- **语法解析器 (Parser)**: 解析 token 并构建抽象语法树或中间表示
- **运行时环境 (Runtime)**: 管理程序执行状态和流程控制
- **变量系统 (Variables)**: 处理整型、浮点和字符串变量，支持数组
- **内存管理 (Memory)**: 字符串空间管理、栈管理

### BASIC 语言功能

**语句 (27个)**:
- 流程控制: END, FOR, NEXT, IF, THEN, GOTO, GOSUB, RETURN, ON, RUN, STOP, CONT
- 数据处理: LET, DATA, READ, RESTORE, DIM
- I/O: INPUT, PRINT, LIST
- 系统: NEW, CLEAR, REM
- 文件: LOAD, SAVE (阶段2实现)
- 高级: DEF, POKE, WAIT, NULL, GET

**函数 (22个)**:
- 数学: SGN, INT, ABS, SQR, RND, LOG, EXP, SIN, COS, TAN, ATN
- 字符串: LEN, STR$, VAL, ASC, CHR$, LEFT$, RIGHT$, MID$
- 系统: FRE, POS, PEEK, USR

**运算符**:
- 算术: +, -, *, /, ^ (乘方)
- 逻辑: AND, OR, NOT
- 关系: =, <>, <, >, <=, >=

### 实施阶段

**阶段 1: 核心基础设施** (2-3周)
- 词法分析器和 token 定义
- 基础解析器框架
- 运行时环境搭建
- 简单变量管理

**阶段 2: 基础语句** (2-3周)
- 赋值语句 (LET)
- 输出语句 (PRINT)
- 流程控制 (GOTO, GOSUB, RETURN)
- 条件语句 (IF...THEN)
- 循环语句 (FOR...NEXT)

**阶段 3: 数值运算** (2周)
- 算术运算符实现
- 浮点数系统
- 基本数学函数

**阶段 4: 字符串和数组** (2周)
- 字符串变量和运算
- 字符串函数
- 数组支持 (DIM)

**阶段 5: 交互功能** (1-2周)
- INPUT 语句
- 行编辑功能 (rustyline)
- 错误处理和消息

**阶段 6: 高级特性** (2周)
- 用户自定义函数 (DEF FN)
- DATA/READ/RESTORE
- 其他辅助功能

**阶段 7: 文件和系统** (可选)
- LOAD/SAVE 功能
- 系统接口

## Impact

### 新增 Capabilities (10个)
1. **tokenizer** - 词法分析
2. **parser** - 语法分析
3. **runtime** - 运行时环境
4. **variables** - 变量管理
5. **statements** - 语句执行
6. **functions** - 内置函数
7. **operators** - 运算符
8. **io** - 输入输出
9. **float-arithmetic** - 浮点运算
10. **memory** - 内存管理

### 代码结构
预计创建以下 Rust 模块：
- `src/main.rs` - 入口点
- `src/tokenizer.rs` - 词法分析
- `src/parser.rs` - 语法分析
- `src/runtime.rs` - 运行时
- `src/variables.rs` - 变量系统
- `src/statements.rs` - 语句执行
- `src/functions.rs` - 内置函数
- `src/operators.rs` - 运算符
- `src/io.rs` - I/O 系统
- `src/float.rs` - 浮点运算
- `src/memory.rs` - 内存管理
- `src/error.rs` - 错误处理

### 测试策略
- 每个模块对应单元测试
- 每个 BASIC 关键字对应测试用例
- 集成测试覆盖完整的 BASIC 程序

### 文档
- 代码注释对应原汇编代码位置
- 各模块 API 文档
- 用户使用示例

## Dependencies
- Rust stable 工具链
- rustyline (或类似的行编辑库)
- 标准库 (无其他重度依赖)

## Timeline
- 预计总时长: 10-14 周
- 可分阶段交付，每个阶段都能运行和测试

## Success Criteria
- 所有核心 BASIC 语句正确执行
- 所有内置函数正确计算
- 能运行经典的 BASIC 程序 (如游戏、计算程序)
- 通过完整的测试套件
- 代码清晰可维护

```

```markdown
# Implementation Tasks

## 1. 项目初始化
- [ ] 1.1 创建 Cargo 项目结构
- [ ] 1.2 配置 Cargo.toml 依赖
- [ ] 1.3 设置基础错误类型和结果类型
- [ ] 1.4 创建项目 README
- [ ] 1.5 设置测试框架

## 2. 词法分析器 (Tokenizer)
- [ ] 2.1 定义 Token 枚举类型
- [ ] 2.2 定义所有保留字 (27个语句关键字)
- [ ] 2.3 定义所有函数名 (22个内置函数)
- [ ] 2.4 定义运算符 token
- [ ] 2.5 实现数字常量识别 (整数和浮点数)
- [ ] 2.6 实现标识符识别 (变量名、行号)
- [ ] 2.7 实现字符串常量识别
- [ ] 2.8 处理空格和注释
- [ ] 2.9 实现行号解析
- [ ] 2.10 编写词法分析器单元测试

## 3. 语法解析器 (Parser)
- [ ] 3.1 定义抽象语法树 (AST) 数据结构
- [ ] 3.2 实现表达式解析 (递归下降或优先级攀升)
- [ ] 3.3 实现语句解析器基础框架
- [ ] 3.4 解析简单语句 (LET, PRINT, REM, END)
- [ ] 3.5 解析流程控制语句 (GOTO, IF)
- [ ] 3.6 解析循环语句 (FOR...NEXT)
- [ ] 3.7 解析子程序调用 (GOSUB, RETURN)
- [ ] 3.8 解析数组和函数调用
- [ ] 3.9 实现错误恢复机制
- [ ] 3.10 编写解析器单元测试

## 4. 运行时环境 (Runtime)
- [ ] 4.1 定义程序状态结构
- [ ] 4.2 实现程序行存储和索引
- [ ] 4.3 实现程序计数器和行号跳转
- [ ] 4.4 实现 GOSUB 栈管理
- [ ] 4.5 实现 FOR 循环栈管理
- [ ] 4.6 实现直接模式 vs 程序模式
- [ ] 4.7 实现 NEW 命令 (清空程序)
- [ ] 4.8 实现 LIST 命令 (列出程序)
- [ ] 4.9 实现 RUN 命令
- [ ] 4.10 实现 STOP, END, CONT 命令
- [ ] 4.11 编写运行时单元测试

## 5. 变量系统 (Variables)
- [ ] 5.1 定义变量值类型 (整数、浮点、字符串)
- [ ] 5.2 实现变量名解析 (A-Z, A0-Z9, A-Z$)
- [ ] 5.3 实现简单变量存储 (HashMap)
- [ ] 5.4 实现数组变量存储
- [ ] 5.5 实现 DIM 语句
- [ ] 5.6 实现数组边界检查
- [ ] 5.7 实现多维数组支持
- [ ] 5.8 实现变量类型检查
- [ ] 5.9 实现 CLEAR 命令 (清空变量)
- [ ] 5.10 编写变量系统单元测试

## 6. 基础语句实现
- [ ] 6.1 LET 语句 (赋值)
- [ ] 6.2 PRINT 语句 (基础输出)
- [ ] 6.3 PRINT 使用分隔符 (逗号、分号)
- [ ] 6.4 PRINT TAB() 和 SPC() 函数
- [ ] 6.5 REM 语句 (注释)
- [ ] 6.6 GOTO 语句
- [ ] 6.7 IF...THEN 语句
- [ ] 6.8 GOSUB 和 RETURN 语句
- [ ] 6.9 ON...GOTO 和 ON...GOSUB 语句
- [ ] 6.10 FOR...NEXT 语句
- [ ] 6.11 FOR...STEP 支持
- [ ] 6.12 测试所有基础语句

## 7. 运算符实现
- [ ] 7.1 算术运算符 (+, -, *, /)
- [ ] 7.2 乘方运算符 (^)
- [ ] 7.3 一元负号 (-)
- [ ] 7.4 关系运算符 (=, <>, <, >, <=, >=)
- [ ] 7.5 逻辑运算符 (AND, OR, NOT)
- [ ] 7.6 字符串连接 (+)
- [ ] 7.7 运算符优先级处理
- [ ] 7.8 类型转换和检查
- [ ] 7.9 测试所有运算符

## 8. 浮点运算 (Float Arithmetic)
- [ ] 8.1 选择浮点数表示 (f64 或自定义)
- [ ] 8.2 实现加法和减法
- [ ] 8.3 实现乘法和除法
- [ ] 8.4 实现乘方运算
- [ ] 8.5 实现数值比较
- [ ] 8.6 实现整数和浮点转换
- [ ] 8.7 实现浮点数输入解析
- [ ] 8.8 实现浮点数输出格式化
- [ ] 8.9 处理溢出和下溢
- [ ] 8.10 测试浮点运算精度

## 9. 数学函数
- [ ] 9.1 SGN(x) - 符号函数
- [ ] 9.2 INT(x) - 取整函数
- [ ] 9.3 ABS(x) - 绝对值
- [ ] 9.4 SQR(x) - 平方根
- [ ] 9.5 EXP(x) - 指数函数
- [ ] 9.6 LOG(x) - 自然对数
- [ ] 9.7 SIN(x), COS(x), TAN(x) - 三角函数
- [ ] 9.8 ATN(x) - 反正切
- [ ] 9.9 RND(x) - 随机数
- [ ] 9.10 测试所有数学函数

## 10. 字符串功能
- [ ] 10.1 字符串变量存储
- [ ] 10.2 字符串连接操作
- [ ] 10.3 LEN(s$) - 字符串长度
- [ ] 10.4 LEFT$(s$, n) - 左子串
- [ ] 10.5 RIGHT$(s$, n) - 右子串
- [ ] 10.6 MID$(s$, n[, m]) - 中间子串
- [ ] 10.7 STR$(x) - 数字转字符串
- [ ] 10.8 VAL(s$) - 字符串转数字
- [ ] 10.9 ASC(s$) - 字符 ASCII 码
- [ ] 10.10 CHR$(x) - ASCII 码转字符
- [ ] 10.11 字符串比较
- [ ] 10.12 测试所有字符串函数

## 11. 输入输出 (I/O)
- [ ] 11.1 实现基础 INPUT 语句
- [ ] 11.2 INPUT 提示符支持
- [ ] 11.3 INPUT 多个变量
- [ ] 11.4 INPUT 错误处理和重试
- [ ] 11.5 实现 DATA 语句
- [ ] 11.6 实现 READ 语句
- [ ] 11.7 实现 RESTORE 语句
- [ ] 11.8 集成 rustyline 行编辑
- [ ] 11.9 实现命令历史
- [ ] 11.10 测试 I/O 功能

## 12. 系统函数
- [ ] 12.1 FRE(x) - 剩余内存
- [ ] 12.2 POS(x) - 光标位置
- [ ] 12.3 PEEK(addr) - 读内存 (模拟)
- [ ] 12.4 POKE addr, value - 写内存 (模拟)
- [ ] 12.5 WAIT 语句
- [ ] 12.6 测试系统函数

## 13. 用户自定义函数
- [ ] 13.1 DEF FN 语句解析
- [ ] 13.2 单行函数定义
- [ ] 13.3 函数调用 FN name(arg)
- [ ] 13.4 函数作用域管理
- [ ] 13.5 测试用户定义函数

## 14. 错误处理
- [ ] 14.1 定义所有错误类型
- [ ] 14.2 实现错误消息 (中文/英文)
- [ ] 14.3 语法错误报告 (行号、位置)
- [ ] 14.4 运行时错误处理
- [ ] 14.5 STOP 和 END 的区别
- [ ] 14.6 CONT 命令恢复执行
- [ ] 14.7 测试错误处理

## 15. 交互模式
- [ ] 15.1 REPL 主循环
- [ ] 15.2 直接模式命令执行
- [ ] 15.3 程序编辑 (输入行号)
- [ ] 15.4 行删除 (输入空行)
- [ ] 15.5 LIST 命令变体 (LIST 10-50)
- [ ] 15.6 启动横幅和提示符
- [ ] 15.7 退出命令
- [ ] 15.8 测试交互模式

## 16. 高级功能 (可选)
- [ ] 16.1 LOAD 命令 (从文件加载程序)
- [ ] 16.2 SAVE 命令 (保存程序到文件)
- [ ] 16.3 GET 语句 (单字符输入)
- [ ] 16.4 NULL 语句
- [ ] 16.5 CMD 和 SYS 语句
- [ ] 16.6 文件格式定义
- [ ] 16.7 测试文件操作

## 17. 集成测试
- [ ] 17.1 简单程序测试 (Hello World)
- [ ] 17.2 循环测试 (求和、阶乘)
- [ ] 17.3 数组测试 (排序、搜索)
- [ ] 17.4 字符串处理测试
- [ ] 17.5 子程序测试
- [ ] 17.6 递归函数测试
- [ ] 17.7 经典游戏测试 (猜数字、星际迷航)
- [ ] 17.8 数学计算测试
- [ ] 17.9 综合测试程序
- [ ] 17.10 性能基准测试

## 18. 文档和完善
- [ ] 18.1 完善代码注释
- [ ] 18.2 编写用户手册
- [ ] 18.3 编写开发者文档
- [ ] 18.4 添加使用示例
- [ ] 18.5 优化错误消息
- [ ] 18.6 代码重构和清理
- [ ] 18.7 性能分析和优化
- [ ] 18.8 最终测试

## 19. 部署和发布
- [ ] 19.1 配置 CI/CD
- [ ] 19.2 创建发布版本
- [ ] 19.3 编写 CHANGELOG
- [ ] 19.4 准备示例程序
- [ ] 19.5 发布第一个版本

```

现在创建设计文档：

```markdown
# Technical Design Document

## Context

重新实现 Microsoft BASIC 6502 解释器是一个复杂的系统工程项目。原始的汇编代码约 6800 行，包含了完整的词法分析、语法解析、运行时环境、内存管理和各种 BASIC 语言特性。

**约束条件**:
- 保持与原 BASIC 6502 的语义兼容性
- 使用 Rust 语言的最佳实践
- 最小化外部依赖
- 单文件模块，每个模块 300-500 行
- 优先考虑代码清晰度而非性能

**利益相关者**:
- 开发者（单人项目，但代码需要可维护）
- 用户（希望运行经典 BASIC 程序）

## Goals / Non-Goals

### Goals
- 实现完整的 BASIC 6502 核心功能
- 代码清晰、模块化、易于理解
- 完整的单元测试覆盖
- 能运行经典的 BASIC 程序
- 提供良好的错误消息

### Non-Goals
- 性能优化到极致（合理即可）
- 向后兼容所有历史平台的特定功能
- 实现 BASIC 扩展或现代特性
- 图形界面（仅命令行）
- 多线程支持

## Decisions

### 1. 架构模式：经典三阶段解释器

**决策**: 采用 Tokenizer → Parser → Runtime 的三阶段架构

**理由**:
- 职责清晰分离，易于理解和维护
- 符合编译器设计的标准模式
- 便于独立测试各个阶段
- 与原汇编代码的结构相对应

**替代方案**:
- 直接解释执行（边读边执行）：不利于错误检查和跳转
- 编译为字节码：增加复杂度，对 BASIC 来说收益不大

### 2. AST vs 中间表示

**决策**: 使用轻量级的中间表示，而非完整的 AST

**理由**:
- BASIC 语言结构简单，不需要复杂的 AST
- 行号驱动的执行模式更适合线性表示
- 节省内存，符合原 6502 的精神
- 更接近原汇编实现的方式

**实现**:
```rust
// 每行程序存储为：
struct ProgramLine {
    line_number: u16,
    tokens: Vec<Token>,  // 预处理的 token 序列
}

// 程序存储在 BTreeMap 中，按行号排序
type Program = BTreeMap<u16, ProgramLine>;
```

### 3. 变量存储策略

**决策**: 使用 HashMap 存储变量，区分数值和字符串

**理由**:
- Rust 的 HashMap 性能优秀
- 支持任意变量名（不限于 26 个字母）
- 类型安全（编译时保证）
- 易于实现和调试

**数据结构**:
```rust
enum Value {
    Integer(i32),
    Float(f64),
    String(String),
    Array(Vec<Value>),  // 多维数组展平存储
}

struct Variables {
    simple: HashMap<String, Value>,
    arrays: HashMap<String, Array>,
}

struct Array {
    dimensions: Vec<usize>,
    data: Vec<Value>,
}
```

**替代方案**:
- 单字母变量用数组（A-Z）：不够灵活，不支持 A1, B2 等
- 全局符号表：不利于作用域管理（DEF FN）

### 4. 浮点数实现

**决策**: 使用 Rust 标准的 f64

**理由**:
- 简单直接，无需重新实现浮点运算
- 精度更高（原 6502 使用 5 字节浮点）
- 与 Rust 生态系统兼容
- 标准数学函数直接可用

**注意事项**:
- 可能与原 BASIC 的精度有细微差异
- 需要在测试中注明这一点
- 输出格式需要模拟原 BASIC 的风格

**替代方案**:
- 实现 5 字节 MS-BASIC 浮点格式：过于复杂，收益低
- 使用 Decimal 类型：不适合科学计算

### 5. 字符串管理

**决策**: 使用 Rust String，不做特殊的字符串池

**理由**:
- Rust 的 String 已经很高效
- 自动内存管理，避免内存泄漏
- 不需要模拟 6502 的字符串空间管理

**实现**:
- 字符串变量直接存储 String
- 字符串操作返回新的 String（写时复制）
- 临时字符串由 Rust 自动回收

### 6. 流程控制实现

**决策**: 使用栈来管理 GOSUB 和 FOR 循环

**理由**:
- 符合原汇编实现的方式
- 支持嵌套调用和循环
- 易于实现 RETURN 和 NEXT 的匹配检查

**数据结构**:
```rust
enum CallFrame {
    Gosub { return_line: u16, return_pos: usize },
    For { 
        var_name: String,
        end_value: f64,
        step: f64,
        loop_start: (u16, usize),
    },
}

struct Runtime {
    call_stack: Vec<CallFrame>,
    // ...
}
```

### 7. 错误处理策略

**决策**: 使用 Result 类型，自定义错误枚举

**理由**:
- 符合 Rust 的惯用法
- 强制错误处理
- 易于传播和转换错误

**实现**:
```rust
#[derive(Debug)]
enum BasicError {
    SyntaxError(String),
    TypeError(String),
    RuntimeError(String),
    UndefinedLine(u16),
    UndefinedVariable(String),
    DivisionByZero,
    OutOfMemory,
    // ... 对应原 BASIC 的各种错误
}

type Result<T> = std::result::Result<T, BasicError>;
```

### 8. 交互模式：选择 rustyline

**决策**: 使用 rustyline crate 实现 REPL

**理由**:
- 提供现代的行编辑功能（光标移动、删除）
- 命令历史记录
- 跨平台支持
- 轻量级，单一依赖

**替代方案**:
- 标准输入（std::io::stdin）：功能太基础，用户体验差
- readline 系统库：需要 FFI，跨平台问题

### 9. 测试策略

**决策**: 单元测试 + 集成测试 + 黄金测试

**单元测试**:
- 每个模块独立测试
- 覆盖边界情况

**集成测试**:
- 完整的 BASIC 程序测试
- 放在 `tests/` 目录

**黄金测试** (Golden Tests):
- 运行 BASIC 程序，比较输出
- 测试文件: `tests/basic_programs/*.bas`
- 期望输出: `tests/expected/*.txt`

**实现**:
```rust
#[test]
fn test_program_output() {
    let program = load_program("tests/basic_programs/hello.bas");
    let output = run_program(program);
    let expected = read_file("tests/expected/hello.txt");
    assert_eq!(output, expected);
}
```

### 10. 模块组织

**决策**: 按功能模块划分，每个模块 300-500 行

**结构**:
```
src/
├── main.rs           # 入口点，REPL
├── error.rs          # 错误定义 (~100 行)
├── token.rs          # Token 定义 (~150 行)
├── tokenizer.rs      # 词法分析 (~400 行)
├── parser.rs         # 语法分析 (~500 行)
├── runtime.rs        # 运行时环境 (~500 行)
├── variables.rs      # 变量管理 (~300 行)
├── statements/       # 语句实现
│   ├── mod.rs
│   ├── control.rs    # GOTO, IF, FOR, etc.
│   ├── io.rs         # PRINT, INPUT, etc.
│   └── data.rs       # LET, DIM, DATA, etc.
├── functions/        # 函数实现
│   ├── mod.rs
│   ├── math.rs       # 数学函数
│   ├── string.rs     # 字符串函数
│   └── system.rs     # 系统函数
├── operators.rs      # 运算符 (~300 行)
├── float.rs          # 浮点运算辅助 (~200 行)
└── memory.rs         # 内存管理辅助 (~200 行)
```

## Risks / Trade-offs

### 风险 1: 语义差异

**风险**: Rust 实现可能与原 6502 汇编有细微差异

**缓解**:
- 仔细对照原汇编代码注释
- 编写大量测试用例
- 在文档中注明已知差异

### 风险 2: 浮点精度

**风险**: f64 与原 5 字节浮点格式精度不同

**缓解**:
- 测试时使用容差比较
- 文档说明精度差异
- 必要时提供兼容模式

### 风险 3: 性能

**风险**: 解释执行可能比原汇编慢（或快很多）

**缓解**:
- 性能不是主要目标，合理即可
- 必要时进行 profiling 和优化
- 避免过早优化

### 风险 4: 范围蔓延

**风险**: 功能范围过大，难以完成

**缓解**:
- 严格按阶段实施
- 每个阶段都能运行和测试
- 可选功能放在后期

### Trade-off 1: 简洁 vs 性能

**选择**: 简洁优先

**理由**: 代码可维护性更重要，现代硬件性能足够

### Trade-off 2: 兼容性 vs 现代化

**选择**: 语义兼容，接口现代化

**理由**: 
- 保持 BASIC 程序可运行（兼容性）
- 使用现代行编辑、UTF-8（现代化）

## Migration Plan

N/A - 这是全新项目

## Implementation Order

1. **第 1 周**: 项目初始化、错误类型、Token 定义
2. **第 2-3 周**: 词法分析器、基础解析器
3. **第 4-5 周**: 运行时环境、基础语句 (LET, PRINT, GOTO)
4. **第 6-7 周**: 流程控制 (IF, FOR, GOSUB)、运算符
5. **第 8 周**: 浮点运算、数学函数
6. **第 9 周**: 字符串功能、数组
7. **第 10 周**: I/O 功能 (INPUT, DATA/READ)
8. **第 11 周**: 交互模式、用户定义函数
9. **第 12 周**: 集成测试、错误处理完善
10. **第 13-14 周**: 可选功能、文档、发布

## Open Questions

1. **是否支持行编辑中删除程序行？**
   - 答：支持，输入行号 + 空行即可删除

2. **错误消息使用中文还是英文？**
   - 答：优先中文，可配置

3. **是否实现 LOAD/SAVE？**
   - 答：基础版本不实现，作为可选功能

4. **是否支持多平台（Windows, Linux, macOS）？**
   - 答：是，Rust 天然跨平台

5. **如何处理无限循环？**
   - 答：提供 Ctrl+C 中断支持

## References

- Microsoft BASIC 6502 源代码: `m6502.asm`
- OpenSpec project.md: 项目约定和架构模式
- Rust 文档: https://doc.rust-lang.org/
- rustyline: https://docs.rs/rustyline/

```

现在我创建各个功能模块的规格说明。先创建核心模块的 specs：

```markdown
# Tokenizer Specification

## ADDED Requirements

### Requirement: Token Type Definition
系统 SHALL 定义所有 BASIC 语言的 token 类型，包括关键字、标识符、字面量、运算符和分隔符。

#### Scenario: 识别保留字
- **WHEN** 输入字符串 "PRINT"
- **THEN** 返回 Token::Print

#### Scenario: 识别变量名
- **WHEN** 输入字符串 "A1"
- **THEN** 返回 Token::Identifier("A1")

#### Scenario: 识别数字
- **WHEN** 输入字符串 "123.45"
- **THEN** 返回 Token::Number(123.45)

### Requirement: 保留字识别
系统 SHALL 识别所有 27 个 BASIC 语句关键字，包括 END, FOR, NEXT, DATA, INPUT, DIM, READ, LET, GOTO, RUN, IF, RESTORE, GOSUB, RETURN, REM, STOP, ON, NULL, WAIT, LOAD, SAVE, DEF, POKE, PRINT, CONT, LIST, CLEAR, GET, NEW。

#### Scenario: 大小写不敏感
- **WHEN** 输入 "print", "PRINT", 或 "Print"
- **THEN** 均识别为 Print 关键字

#### Scenario: 关键字优先于标识符
- **WHEN** 输入 "FOR"
- **THEN** 返回 Token::For 而非 Identifier

### Requirement: 函数名识别
系统 SHALL 识别所有 22 个内置函数名，包括 SGN, INT, ABS, USR, FRE, POS, SQR, RND, LOG, EXP, COS, SIN, TAN, ATN, PEEK, LEN, STR$, VAL, ASC, CHR$, LEFT$, RIGHT$, MID$。

#### Scenario: 字符串函数带美元符号
- **WHEN** 输入 "LEFT$"
- **THEN** 返回 Token::Function("LEFT$")

### Requirement: 数字常量解析
系统 SHALL 解析整数和浮点数常量，支持科学计数法。

#### Scenario: 整数
- **WHEN** 输入 "42"
- **THEN** 返回 Token::Number(42.0)

#### Scenario: 浮点数
- **WHEN** 输入 "3.14159"
- **THEN** 返回 Token::Number(3.14159)

#### Scenario: 科学计数法
- **WHEN** 输入 "1.5E-10"
- **THEN** 返回 Token::Number(1.5e-10)

#### Scenario: 负数
- **WHEN** 输入 "-123"
- **THEN** 返回 Token::Minus 和 Token::Number(123)

### Requirement: 字符串常量解析
系统 SHALL 解析双引号括起的字符串常量，支持空字符串。

#### Scenario: 普通字符串
- **WHEN** 输入 `"HELLO WORLD"`
- **THEN** 返回 Token::String("HELLO WORLD")

#### Scenario: 空字符串
- **WHEN** 输入 `""`
- **THEN** 返回 Token::String("")

#### Scenario: 包含空格的字符串
- **WHEN** 输入 `"  SPACES  "`
- **THEN** 返回 Token::String("  SPACES  ")

### Requirement: 变量名识别
系统 SHALL 识别变量名，支持单字母、字母+数字组合，字符串变量以 $ 结尾。

#### Scenario: 单字母变量
- **WHEN** 输入 "A"
- **THEN** 返回 Token::Identifier("A")

#### Scenario: 字母数字组合
- **WHEN** 输入 "A1", "X9", "Z0"
- **THEN** 分别返回对应的 Identifier

#### Scenario: 字符串变量
- **WHEN** 输入 "A$", "NAME$"
- **THEN** 返回 Token::Identifier("A$"), Token::Identifier("NAME$")

#### Scenario: 数组变量
- **WHEN** 输入 "A(5)"
- **THEN** 返回 Token::Identifier("A"), Token::LeftParen, Token::Number(5), Token::RightParen

### Requirement: 运算符识别
系统 SHALL 识别所有运算符，包括算术运算符 (+, -, *, /, ^)、关系运算符 (=, <>, <, >, <=, >=) 和逻辑运算符 (AND, OR, NOT)。

#### Scenario: 算术运算符
- **WHEN** 输入 "+", "-", "*", "/", "^"
- **THEN** 分别返回对应的运算符 token

#### Scenario: 关系运算符
- **WHEN** 输入 "<=", ">=", "<>"
- **THEN** 返回 Token::LessEqual, Token::GreaterEqual, Token::NotEqual

#### Scenario: 逻辑运算符
- **WHEN** 输入 "AND", "OR", "NOT"
- **THEN** 返回对应的逻辑运算符 token

### Requirement: 行号处理
系统 SHALL 识别行号（1-65535 范围的整数），位于行首时作为行号处理。

#### Scenario: 有效行号
- **WHEN** 输入 "10 PRINT"
- **THEN** 首个 token 为 Token::LineNumber(10)

#### Scenario: 行号范围
- **WHEN** 输入行号 1, 100, 65535
- **THEN** 均作为有效行号识别

### Requirement: 空格和分隔符处理
系统 SHALL 正确处理空格、制表符作为分隔符，逗号和分号作为语句分隔符。

#### Scenario: 空格分隔
- **WHEN** 输入 "PRINT A"
- **THEN** 返回 Token::Print, Token::Identifier("A")

#### Scenario: 多个空格
- **WHEN** 输入 "PRINT   A"
- **THEN** 多余空格被忽略

#### Scenario: 逗号分隔（PRINT 语句）
- **WHEN** 输入 "PRINT A,B"
- **THEN** 返回 Token::Print, Token::Identifier("A"), Token::Comma, Token::Identifier("B")

#### Scenario: 分号（行内分隔）
- **WHEN** 输入 "A=1: B=2"
- **THEN** 冒号作为语句分隔符

### Requirement: 注释处理
系统 SHALL 识别 REM 语句，REM 之后到行尾的所有内容作为注释忽略。

#### Scenario: REM 注释
- **WHEN** 输入 "10 REM THIS IS A COMMENT"
- **THEN** REM 之后的文本被忽略

#### Scenario: 单引号注释（可选）
- **WHEN** 输入 "10 PRINT A ' COMMENT"
- **THEN** 单引号之后的内容被忽略

### Requirement: 错误处理
系统 SHALL 对非法字符和格式错误给出清晰的错误消息。

#### Scenario: 非法字符
- **WHEN** 输入包含非 ASCII 字符（除字符串内）
- **THEN** 返回 TokenError::IllegalCharacter

#### Scenario: 未闭合字符串
- **WHEN** 输入 `"HELLO` 缺少闭合引号
- **THEN** 返回 TokenError::UnterminatedString

#### Scenario: 数字格式错误
- **WHEN** 输入 "1.2.3"
- **THEN** 返回 TokenError::InvalidNumber

```

```markdown
# Parser Specification

## ADDED Requirements

### Requirement: Expression Parsing
系统 SHALL 解析表达式，包括变量、常量、函数调用和运算符，并正确处理运算符优先级。

#### Scenario: 简单算术表达式
- **WHEN** 输入 tokens 对应 "2 + 3"
- **THEN** 解析为加法表达式节点

#### Scenario: 运算符优先级
- **WHEN** 输入 "2 + 3 * 4"
- **THEN** 解析为 2 + (3 * 4)，乘法优先

#### Scenario: 括号改变优先级
- **WHEN** 输入 "(2 + 3) * 4"
- **THEN** 解析为 (2 + 3) * 4，加法先计算

#### Scenario: 乘方优先级
- **WHEN** 输入 "2 ^ 3 * 4"
- **THEN** 解析为 (2 ^ 3) * 4，乘方最高优先级

### Requirement: 函数调用解析
系统 SHALL 解析函数调用，包括单参数和多参数函数。

#### Scenario: 单参数函数
- **WHEN** 输入 "SIN(X)"
- **THEN** 解析为函数调用，函数名 SIN，参数 X

#### Scenario: 嵌套函数调用
- **WHEN** 输入 "SQR(ABS(X))"
- **THEN** 解析为嵌套的函数调用

#### Scenario: 多参数函数
- **WHEN** 输入 "LEFT$(A$, 5)"
- **THEN** 解析为两个参数的函数调用

### Requirement: 数组访问解析
系统 SHALL 解析数组元素访问，支持多维数组。

#### Scenario: 一维数组
- **WHEN** 输入 "A(5)"
- **THEN** 解析为数组访问，变量 A，索引 5

#### Scenario: 二维数组
- **WHEN** 输入 "B(3, 4)"
- **THEN** 解析为二维数组访问

#### Scenario: 表达式作为索引
- **WHEN** 输入 "A(I + 1)"
- **THEN** 索引部分解析为表达式

### Requirement: LET 语句解析
系统 SHALL 解析赋值语句，支持简单变量和数组元素赋值。

#### Scenario: 简单赋值
- **WHEN** 输入 "LET A = 10"
- **THEN** 解析为赋值语句，变量 A，值 10

#### Scenario: LET 可选
- **WHEN** 输入 "A = 10"（省略 LET）
- **THEN** 同样解析为赋值语句

#### Scenario: 表达式赋值
- **WHEN** 输入 "A = B + C * 2"
- **THEN** 右侧解析为表达式

### Requirement: PRINT 语句解析
系统 SHALL 解析 PRINT 语句，支持多个表达式、分隔符（逗号、分号）。

#### Scenario: 打印单个值
- **WHEN** 输入 "PRINT 42"
- **THEN** 解析为 PRINT 语句，参数列表包含一个表达式

#### Scenario: 打印多个值（逗号分隔）
- **WHEN** 输入 "PRINT A, B, C"
- **THEN** 逗号表示列分隔

#### Scenario: 打印多个值（分号分隔）
- **WHEN** 输入 "PRINT A; B; C"
- **THEN** 分号表示紧密连接

#### Scenario: TAB 和 SPC 函数
- **WHEN** 输入 "PRINT TAB(10); A"
- **THEN** 解析 TAB 函数调用

### Requirement: 流程控制语句解析
系统 SHALL 解析 GOTO, IF...THEN, GOSUB, RETURN 等流程控制语句。

#### Scenario: GOTO 语句
- **WHEN** 输入 "GOTO 100"
- **THEN** 解析为 GOTO，目标行号 100

#### Scenario: IF...THEN 语句
- **WHEN** 输入 "IF A > 10 THEN 200"
- **THEN** 解析为条件语句，条件表达式，目标行号

#### Scenario: IF...THEN 执行语句
- **WHEN** 输入 "IF A > 10 THEN PRINT A"
- **THEN** THEN 后跟语句而非行号

#### Scenario: GOSUB 和 RETURN
- **WHEN** 输入 "GOSUB 500" 和 "RETURN"
- **THEN** 分别解析为子程序调用和返回

### Requirement: 循环语句解析
系统 SHALL 解析 FOR...NEXT 循环语句，支持 STEP。

#### Scenario: 基本 FOR 循环
- **WHEN** 输入 "FOR I = 1 TO 10"
- **THEN** 解析为循环，变量 I，起始 1，结束 10

#### Scenario: 带 STEP 的循环
- **WHEN** 输入 "FOR I = 10 TO 1 STEP -1"
- **THEN** 步长为 -1

#### Scenario: NEXT 语句
- **WHEN** 输入 "NEXT I"
- **THEN** 解析为 NEXT，变量 I

#### Scenario: NEXT 省略变量
- **WHEN** 输入 "NEXT"
- **THEN** 解析为 NEXT，默认最内层循环变量

### Requirement: INPUT 语句解析
系统 SHALL 解析 INPUT 语句，支持提示符和多个变量。

#### Scenario: 基本 INPUT
- **WHEN** 输入 "INPUT A"
- **THEN** 解析为 INPUT 语句，变量列表包含 A

#### Scenario: 带提示符的 INPUT
- **WHEN** 输入 `INPUT "ENTER VALUE"; A`
- **THEN** 提示符为 "ENTER VALUE"

#### Scenario: 输入多个变量
- **WHEN** 输入 "INPUT A, B, C"
- **THEN** 变量列表包含 A, B, C

### Requirement: DIM 语句解析
系统 SHALL 解析 DIM 语句，声明数组维度。

#### Scenario: 一维数组声明
- **WHEN** 输入 "DIM A(10)"
- **THEN** 解析为 DIM，数组 A，大小 10

#### Scenario: 多维数组声明
- **WHEN** 输入 "DIM B(5, 10)"
- **THEN** 解析为 DIM，数组 B，维度 5x10

#### Scenario: 多个数组声明
- **WHEN** 输入 "DIM A(10), B(20)"
- **THEN** 一条语句声明多个数组

### Requirement: DATA/READ/RESTORE 解析
系统 SHALL 解析 DATA, READ, RESTORE 语句。

#### Scenario: DATA 语句
- **WHEN** 输入 "DATA 1, 2, 3, 4"
- **THEN** 解析为 DATA，数据列表 [1, 2, 3, 4]

#### Scenario: 混合类型 DATA
- **WHEN** 输入 `DATA 10, "HELLO", 3.14`
- **THEN** 支持数字和字符串混合

#### Scenario: READ 语句
- **WHEN** 输入 "READ A, B, C"
- **THEN** 解析为 READ，变量列表

#### Scenario: RESTORE 语句
- **WHEN** 输入 "RESTORE" 或 "RESTORE 100"
- **THEN** 重置或跳转到指定行

### Requirement: DEF FN 解析
系统 SHALL 解析用户自定义函数。

#### Scenario: 单参数函数定义
- **WHEN** 输入 "DEF FNA(X) = X * X"
- **THEN** 解析为函数定义，名称 FNA，参数 X，表达式 X * X

#### Scenario: 函数调用
- **WHEN** 输入 "Y = FNA(5)"
- **THEN** 解析为函数调用

### Requirement: ON...GOTO 解析
系统 SHALL 解析 ON...GOTO 和 ON...GOSUB 语句。

#### Scenario: ON GOTO
- **WHEN** 输入 "ON X GOTO 100, 200, 300"
- **THEN** 根据 X 的值跳转到不同行号

#### Scenario: ON GOSUB
- **WHEN** 输入 "ON X GOSUB 100, 200, 300"
- **THEN** 根据 X 的值调用不同子程序

### Requirement: 其他语句解析
系统 SHALL 解析 END, STOP, REM, CLEAR, NEW, LIST 等语句。

#### Scenario: END 语句
- **WHEN** 输入 "END"
- **THEN** 解析为程序结束

#### Scenario: REM 语句
- **WHEN** 输入 "REM THIS IS A COMMENT"
- **THEN** 解析为注释，内容被保留但不执行

#### Scenario: CLEAR 语句
- **WHEN** 输入 "CLEAR"
- **THEN** 解析为清空变量

### Requirement: 语句分隔符
系统 SHALL 支持冒号分隔一行中的多条语句。

#### Scenario: 单行多语句
- **WHEN** 输入 "10 A=1: B=2: PRINT A+B"
- **THEN** 解析为三条语句

### Requirement: 错误处理
系统 SHALL 对语法错误给出清晰的错误位置和消息。

#### Scenario: 缺少表达式
- **WHEN** 输入 "LET A ="（缺少右值）
- **THEN** 返回 ParseError::ExpectedExpression

#### Scenario: 括号不匹配
- **WHEN** 输入 "PRINT (A + B"
- **THEN** 返回 ParseError::UnmatchedParenthesis

#### Scenario: 无效语句
- **WHEN** 输入 "XYZ 123"（XYZ 不是关键字）
- **THEN** 返回 ParseError::InvalidStatement

```

继续创建其他核心模块的规格说明：

```markdown
# Runtime Specification

## ADDED Requirements

### Requirement: 程序存储和管理
系统 SHALL 存储 BASIC 程序行，按行号排序，支持插入、删除和查找。

#### Scenario: 添加程序行
- **WHEN** 输入 "10 PRINT HELLO"
- **THEN** 行 10 被添加到程序中

#### Scenario: 替换现有行
- **WHEN** 已有行 10，再输入 "10 PRINT WORLD"
- **THEN** 行 10 被新内容替换

#### Scenario: 删除程序行
- **WHEN** 输入 "10"（仅行号，无内容）
- **THEN** 行 10 从程序中删除

#### Scenario: 行号排序
- **WHEN** 输入顺序为 30, 10, 20
- **THEN** 程序按 10, 20, 30 顺序存储

### Requirement: 程序执行
系统 SHALL 从指定行号开始顺序执行程序，直到遇到 END, STOP 或程序结尾。

#### Scenario: 从第一行开始执行
- **WHEN** 执行 RUN 命令
- **THEN** 从最小行号开始执行

#### Scenario: 从指定行开始
- **WHEN** 执行 "RUN 100"
- **THEN** 从行 100 开始执行

#### Scenario: 顺序执行
- **WHEN** 程序有行 10, 20, 30
- **THEN** 依次执行 10, 20, 30

#### Scenario: END 停止执行
- **WHEN** 遇到 END 语句
- **THEN** 程序正常结束

### Requirement: 行号跳转
系统 SHALL 支持 GOTO 和 GOSUB 的行号跳转。

#### Scenario: GOTO 跳转
- **WHEN** 执行 "GOTO 100"
- **THEN** 下一条执行的是行 100

#### Scenario: 跳转到不存在的行
- **WHEN** GOTO 到不存在的行号
- **THEN** 返回 RuntimeError::UndefinedLine

#### Scenario: 向前跳转
- **WHEN** 从行 100 GOTO 50
- **THEN** 成功向前跳转

### Requirement: 子程序调用栈
系统 SHALL 维护 GOSUB 调用栈，支持嵌套子程序调用。

#### Scenario: GOSUB 调用
- **WHEN** 在行 10 执行 "GOSUB 100"
- **THEN** 跳转到 100，返回地址入栈

#### Scenario: RETURN 返回
- **WHEN** 执行 RETURN
- **THEN** 返回到上一个 GOSUB 的下一条语句

#### Scenario: 嵌套 GOSUB
- **WHEN** 执行嵌套的 GOSUB
- **THEN** 栈正确维护，RETURN 按正确顺序返回

#### Scenario: RETURN 无对应 GOSUB
- **WHEN** 没有 GOSUB 就执行 RETURN
- **THEN** 返回 RuntimeError::ReturnWithoutGosub

#### Scenario: GOSUB 栈深度限制
- **WHEN** GOSUB 嵌套超过限制（如 100 层）
- **THEN** 返回 RuntimeError::StackOverflow

### Requirement: FOR 循环栈
系统 SHALL 维护 FOR 循环栈，管理循环变量、终值和步长。

#### Scenario: FOR 循环执行
- **WHEN** 执行 "FOR I = 1 TO 10"
- **THEN** 循环信息入栈，I 初始化为 1

#### Scenario: NEXT 执行
- **WHEN** 执行 "NEXT I"
- **THEN** I 增加步长，检查是否继续循环

#### Scenario: 循环结束
- **WHEN** I 超过终值
- **THEN** 退出循环，继续执行 NEXT 后的语句

#### Scenario: 嵌套 FOR 循环
- **WHEN** 执行嵌套 FOR 循环
- **THEN** 内外循环正确管理

#### Scenario: NEXT 变量不匹配
- **WHEN** FOR I ... NEXT J
- **THEN** 返回 RuntimeError::NextWithoutFor

### Requirement: 直接模式执行
系统 SHALL 支持直接执行语句（无行号），不保存到程序中。

#### Scenario: 直接执行 PRINT
- **WHEN** 输入 "PRINT 2+3"（无行号）
- **THEN** 立即执行并输出 5

#### Scenario: 直接模式中的变量
- **WHEN** 直接执行 "A=5" 然后 "PRINT A"
- **THEN** 变量在直接模式和程序模式间共享

#### Scenario: 直接模式不能使用 GOTO
- **WHEN** 直接模式执行 "GOTO 100"
- **THEN** 返回错误（无上下文）

### Requirement: NEW 命令
系统 SHALL 实现 NEW 命令，清空程序和变量。

#### Scenario: NEW 清空程序
- **WHEN** 程序有多行，执行 NEW
- **THEN** 程序被完全清空

#### Scenario: NEW 清空变量
- **WHEN** 有变量值，执行 NEW
- **THEN** 所有变量被清空

#### Scenario: NEW 重置运行状态
- **WHEN** 程序在运行中断后执行 NEW
- **THEN** 运行状态完全重置

### Requirement: LIST 命令
系统 SHALL 实现 LIST 命令，显示程序内容。

#### Scenario: LIST 全部程序
- **WHEN** 执行 LIST
- **THEN** 显示所有程序行

#### Scenario: LIST 单行
- **WHEN** 执行 "LIST 10"
- **THEN** 仅显示行 10

#### Scenario: LIST 范围
- **WHEN** 执行 "LIST 10-50"
- **THEN** 显示行 10 到 50

#### Scenario: 空程序 LIST
- **WHEN** 程序为空时执行 LIST
- **THEN** 无输出或显示 "EMPTY"

### Requirement: STOP 和 CONT
系统 SHALL 实现 STOP 命令暂停执行，CONT 命令继续执行。

#### Scenario: STOP 暂停
- **WHEN** 执行 STOP
- **THEN** 程序暂停，显示 "BREAK IN line"

#### Scenario: CONT 继续
- **WHEN** STOP 后执行 CONT
- **THEN** 从 STOP 的下一条语句继续

#### Scenario: 未暂停时 CONT
- **WHEN** 没有 STOP 就执行 CONT
- **THEN** 返回 RuntimeError::CantContinue

#### Scenario: 程序修改后 CONT
- **WHEN** STOP 后修改程序再 CONT
- **THEN** 返回错误（无法继续）

### Requirement: RUN 命令
系统 SHALL 实现 RUN 命令，清空变量后执行程序。

#### Scenario: RUN 清空变量
- **WHEN** 有变量值时执行 RUN
- **THEN** 变量被清空

#### Scenario: RUN 保留程序
- **WHEN** 执行 RUN
- **THEN** 程序内容不变

#### Scenario: RUN 指定行号
- **WHEN** 执行 "RUN 100"
- **THEN** 从行 100 开始执行

### Requirement: 错误处理和恢复
系统 SHALL 捕获运行时错误，显示错误消息和行号，返回直接模式。

#### Scenario: 显示错误消息
- **WHEN** 发生运行时错误
- **THEN** 显示错误类型和行号

#### Scenario: 返回直接模式
- **WHEN** 错误发生后
- **THEN** 返回直接模式，等待用户输入

#### Scenario: 错误不破坏状态
- **WHEN** 错误发生
- **THEN** 程序和变量状态保持（可用 LIST 查看）

```

继续创建变量系统和其他模块的规格：

```markdown
# Variables Specification

## ADDED Requirements

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

```

继续创建剩余模块的规格。由于文件较多，我会批量创建：

```markdown
# Statements Specification

## ADDED Requirements

### Requirement: LET 语句
系统 SHALL 实现 LET 赋值语句，LET 关键字可省略。

#### Scenario: 显式 LET
- **WHEN** 执行 "LET A = 10"
- **THEN** A 被赋值为 10

#### Scenario: 隐式 LET
- **WHEN** 执行 "A = 10"
- **THEN** A 被赋值为 10

#### Scenario: 表达式赋值
- **WHEN** 执行 "A = 2 + 3 * 4"
- **THEN** A 的值为 14

### Requirement: PRINT 语句
系统 SHALL 实现 PRINT 语句，支持多种输出格式。

#### Scenario: 打印数值
- **WHEN** 执行 "PRINT 42"
- **THEN** 输出 "42" 并换行

#### Scenario: 打印字符串
- **WHEN** 执行 `PRINT "HELLO"`
- **THEN** 输出 "HELLO" 并换行

#### Scenario: 打印变量
- **WHEN** A=10，执行 "PRINT A"
- **THEN** 输出 "10"

#### Scenario: 逗号分隔（列对齐）
- **WHEN** 执行 "PRINT A, B, C"
- **THEN** 输出值以列对齐（14 列宽度）

#### Scenario: 分号分隔（紧密连接）
- **WHEN** 执行 "PRINT A; B; C"
- **THEN** 值紧密连接输出

#### Scenario: 行尾分号（不换行）
- **WHEN** 执行 "PRINT A;"
- **THEN** 输出 A 的值但不换行

#### Scenario: 空 PRINT
- **WHEN** 执行 "PRINT"
- **THEN** 仅输出换行

### Requirement: IF...THEN 语句
系统 SHALL 实现条件语句，支持 THEN 后跟行号或语句。

#### Scenario: 条件为真跳转行号
- **WHEN** A>10，执行 "IF A>10 THEN 100"
- **THEN** 跳转到行 100

#### Scenario: 条件为假继续
- **WHEN** A<=10，执行 "IF A>10 THEN 100"
- **THEN** 继续执行下一条语句

#### Scenario: THEN 后跟语句
- **WHEN** 执行 "IF A>10 THEN PRINT A"
- **THEN** 条件为真时执行 PRINT

#### Scenario: 关系运算符
- **WHEN** 支持 =, <>, <, >, <=, >=
- **THEN** 所有关系运算符正确工作

### Requirement: GOTO 语句
系统 SHALL 实现无条件跳转。

#### Scenario: 跳转到指定行
- **WHEN** 执行 "GOTO 100"
- **THEN** 下一条执行行 100

#### Scenario: 行号不存在
- **WHEN** GOTO 到不存在的行号
- **THEN** 返回 UndefinedLine 错误

### Requirement: GOSUB 和 RETURN 语句
系统 SHALL 实现子程序调用和返回。

#### Scenario: 子程序调用
- **WHEN** 执行 "GOSUB 500"
- **THEN** 跳转到 500，返回地址入栈

#### Scenario: 子程序返回
- **WHEN** 执行 "RETURN"
- **THEN** 返回到 GOSUB 的下一条语句

#### Scenario: 嵌套子程序
- **WHEN** 子程序内再调用 GOSUB
- **THEN** 返回顺序正确

### Requirement: FOR...NEXT 循环
系统 SHALL 实现 FOR 循环，支持正负步长。

#### Scenario: 正向循环
- **WHEN** 执行 "FOR I=1 TO 10: PRINT I: NEXT I"
- **THEN** 输出 1 到 10

#### Scenario: 步长为 2
- **WHEN** 执行 "FOR I=0 TO 10 STEP 2"
- **THEN** I 取值 0, 2, 4, 6, 8, 10

#### Scenario: 负步长
- **WHEN** 执行 "FOR I=10 TO 1 STEP -1"
- **THEN** I 从 10 递减到 1

#### Scenario: 嵌套循环
- **WHEN** 嵌套 FOR 循环
- **THEN** 内外循环正确执行

### Requirement: ON...GOTO 和 ON...GOSUB
系统 SHALL 实现基于表达式值的多路跳转。

#### Scenario: ON GOTO
- **WHEN** X=2，执行 "ON X GOTO 100,200,300"
- **THEN** 跳转到 200

#### Scenario: ON GOSUB
- **WHEN** X=1，执行 "ON X GOSUB 100,200,300"
- **THEN** 调用行 100 子程序

#### Scenario: 值超出范围
- **WHEN** X=5，但只有 3 个目标
- **THEN** 继续执行下一条语句（不跳转）

### Requirement: DATA, READ, RESTORE 语句
系统 SHALL 实现数据读取机制。

#### Scenario: READ 从 DATA 读取
- **WHEN** "10 DATA 1,2,3" 和 "20 READ A"
- **THEN** A 的值为 1

#### Scenario: 多次 READ
- **WHEN** 连续 READ A, B, C
- **THEN** 依次读取 DATA 中的值

#### Scenario: RESTORE 重置
- **WHEN** 执行 RESTORE
- **THEN** DATA 指针重置到开头

#### Scenario: READ 超出 DATA
- **WHEN** READ 的次数超过 DATA 的数量
- **THEN** 返回 OutOfData 错误

### Requirement: DIM 语句
系统 SHALL 实现数组声明。

#### Scenario: 一维数组
- **WHEN** 执行 "DIM A(100)"
- **THEN** 创建 101 个元素的数组

#### Scenario: 多维数组
- **WHEN** 执行 "DIM B(10,20)"
- **THEN** 创建 11x21 的二维数组

### Requirement: DEF FN 语句
系统 SHALL 实现用户自定义函数。

#### Scenario: 函数定义
- **WHEN** 执行 "DEF FNA(X) = X * X"
- **THEN** 定义平方函数

#### Scenario: 函数调用
- **WHEN** 执行 "Y = FNA(5)"
- **THEN** Y 的值为 25

### Requirement: REM 语句
系统 SHALL 实现注释，REM 后的内容被忽略。

#### Scenario: 注释行
- **WHEN** 执行 "REM THIS IS A COMMENT"
- **THEN** 无任何操作

### Requirement: END 和 STOP 语句
系统 SHALL 实现程序结束和暂停。

#### Scenario: END 结束程序
- **WHEN** 执行 "END"
- **THEN** 程序正常结束

#### Scenario: STOP 暂停程序
- **WHEN** 执行 "STOP"
- **THEN** 程序暂停，可用 CONT 继续

### Requirement: POKE 和 WAIT 语句
系统 SHALL 实现内存操作语句（模拟）。

#### Scenario: POKE 写入
- **WHEN** 执行 "POKE 1024, 65"
- **THEN** 模拟写入内存地址

#### Scenario: WAIT 等待
- **WHEN** 执行 "WAIT 1024, 1"
- **THEN** 模拟等待条件

```

```markdown
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

```

继续创建最后几个模块的规格：

```markdown
# Operators Specification

## ADDED Requirements

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

```

```markdown
# I/O Specification

## ADDED Requirements

### Requirement: INPUT 语句
系统 SHALL 实现 INPUT 语句，从用户读取输入。

#### Scenario: 基本输入
- **WHEN** 执行 "INPUT A"
- **THEN** 显示 "? " 提示符，等待用户输入

#### Scenario: 带提示符的输入
- **WHEN** 执行 `INPUT "ENTER VALUE"; A`
- **THEN** 显示 "ENTER VALUE? "

#### Scenario: 输入多个变量
- **WHEN** 执行 "INPUT A, B, C"
- **THEN** 提示用户输入三个值（逗号分隔）

#### Scenario: 输入类型检查
- **WHEN** INPUT A（数值变量），用户输入 "ABC"
- **THEN** 显示 "?REDO FROM START"，重新输入

#### Scenario: 字符串输入
- **WHEN** INPUT A$，用户输入 HELLO
- **THEN** A$ = "HELLO"

#### Scenario: 字符串带引号
- **WHEN** INPUT A$，用户输入 "HELLO, WORLD"
- **THEN** A$ = "HELLO, WORLD"（保留逗号）

### Requirement: PRINT 输出格式
系统 SHALL 实现 PRINT 的各种输出格式。

#### Scenario: 数值输出格式
- **WHEN** PRINT 正数
- **THEN** 前后各有一个空格
- **WHEN** PRINT 负数
- **THEN** 前有空格，负号紧跟数字

#### Scenario: 科学计数法
- **WHEN** PRINT 极大或极小数值
- **THEN** 使用科学计数法（如 1.5E+10）

#### Scenario: 列对齐（逗号）
- **WHEN** PRINT A, B, C
- **THEN** 每个值占 14 个字符宽度

#### Scenario: 自动换行
- **WHEN** 打印超过行宽（72 字符）
- **THEN** 自动换行

### Requirement: TAB 和 SPC 函数
系统 SHALL 实现 TAB 和 SPC 格式化函数。

#### Scenario: TAB 跳转到指定列
- **WHEN** PRINT TAB(10); "HELLO"
- **THEN** 从第 10 列开始输出

#### Scenario: TAB 小于当前位置
- **WHEN** 当前列 20，执行 TAB(10)
- **THEN** 换行后跳转到第 10 列

#### Scenario: SPC 输出空格
- **WHEN** PRINT SPC(5); "HELLO"
- **THEN** 输出 5 个空格后输出 HELLO

### Requirement: DATA/READ 机制
系统 SHALL 实现 DATA 语句和 READ 语句的数据交换。

#### Scenario: DATA 存储
- **WHEN** 程序包含 "10 DATA 1, 2, 3"
- **THEN** 数据被存储供 READ 使用

#### Scenario: READ 顺序读取
- **WHEN** 执行 "READ A, B, C"
- **THEN** 依次从 DATA 读取 1, 2, 3

#### Scenario: 多行 DATA
- **WHEN** 多行 DATA 语句
- **THEN** 数据连续存储

#### Scenario: 混合数据类型
- **WHEN** DATA 包含数值和字符串
- **THEN** READ 根据变量类型正确读取

#### Scenario: OUT OF DATA 错误
- **WHEN** READ 超过可用数据
- **THEN** 返回 OutOfData 错误

### Requirement: RESTORE 数据指针
系统 SHALL 实现 RESTORE 重置数据指针。

#### Scenario: RESTORE 重置到开头
- **WHEN** READ 几次后执行 RESTORE
- **THEN** 数据指针回到第一个 DATA

#### Scenario: RESTORE 到指定行
- **WHEN** 执行 "RESTORE 100"
- **THEN** 数据指针移到行 100 的 DATA

### Requirement: GET 语句（可选）
系统 SHALL 实现 GET 语句，读取单个字符。

#### Scenario: GET 读取字符
- **WHEN** 执行 "GET A$"
- **THEN** 等待用户按键，读取单个字符

#### Scenario: GET 不回显
- **WHEN** GET 读取字符
- **THEN** 字符不显示在屏幕上

#### Scenario: GET 不等待回车
- **WHEN** GET 读取
- **THEN** 按键立即返回，无需回车

### Requirement: 行编辑功能
系统 SHALL 在交互模式提供行编辑功能。

#### Scenario: 光标移动
- **WHEN** 用户输入时
- **THEN** 支持左右箭头移动光标

#### Scenario: 删除字符
- **WHEN** 用户按 Backspace
- **THEN** 删除前一个字符

#### Scenario: 命令历史
- **WHEN** 用户按上下箭头
- **THEN** 浏览历史命令

#### Scenario: Home/End 键
- **WHEN** 用户按 Home/End
- **THEN** 跳转到行首/行尾

### Requirement: 文件 I/O（可选）
系统 SHALL 实现 LOAD 和 SAVE 命令。

#### Scenario: SAVE 保存程序
- **WHEN** 执行 `SAVE "PROGRAM.BAS"`
- **THEN** 程序保存到文件

#### Scenario: LOAD 加载程序
- **WHEN** 执行 `LOAD "PROGRAM.BAS"`
- **THEN** 从文件加载程序（清空当前程序）

#### Scenario: 文件不存在
- **WHEN** LOAD 不存在的文件
- **THEN** 返回 FileNotFound 错误

### Requirement: 错误输出
系统 SHALL 将错误消息输出到标准错误流。

#### Scenario: 运行时错误显示
- **WHEN** 发生运行时错误
- **THEN** 显示 "?ERROR_NAME IN line"

#### Scenario: 语法错误显示
- **WHEN** 发生语法错误
- **THEN** 显示 "?SYNTAX ERROR"

### Requirement: 输入中断
系统 SHALL 支持用户中断输入和执行。

#### Scenario: Ctrl+C 中断
- **WHEN** 程序运行或等待输入时按 Ctrl+C
- **THEN** 中断执行，返回直接模式

#### Scenario: 中断消息
- **WHEN** Ctrl+C 中断
- **THEN** 显示 "?BREAK IN line"

```

```markdown
# Float Arithmetic Specification

## ADDED Requirements

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

```

```markdown
# Memory Management Specification

## ADDED Requirements

### Requirement: 程序存储
系统 SHALL 高效存储 BASIC 程序行，支持快速查找和修改。

#### Scenario: 行号索引
- **WHEN** 存储程序行
- **THEN** 使用 BTreeMap 按行号排序存储

#### Scenario: 查找性能
- **WHEN** GOTO 跳转
- **THEN** 查找时间复杂度为 O(log n)

#### Scenario: 插入和删除
- **WHEN** 添加或删除行
- **THEN** 自动维护排序

### Requirement: 变量存储
系统 SHALL 使用 HashMap 存储变量，提供 O(1) 访问时间。

#### Scenario: 变量查找
- **WHEN** 访问变量
- **THEN** 平均 O(1) 时间复杂度

#### Scenario: 变量数量
- **WHEN** 使用大量变量
- **THEN** 支持数百个变量无性能问题

### Requirement: 数组内存管理
系统 SHALL 为数组分配连续内存（逻辑上），支持多维索引。

#### Scenario: 一维数组
- **WHEN** DIM A(100)
- **THEN** 分配 101 个元素空间

#### Scenario: 多维数组展平
- **WHEN** DIM B(5, 10)
- **THEN** 分配 6*11=66 个元素，使用行优先索引

#### Scenario: 数组大小限制
- **WHEN** DIM 极大数组
- **THEN** 检查内存限制，返回 OutOfMemory 错误

### Requirement: 字符串内存管理
系统 SHALL 使用 Rust String 类型，自动管理字符串内存。

#### Scenario: 字符串创建
- **WHEN** 赋值字符串
- **THEN** Rust 自动分配内存

#### Scenario: 字符串释放
- **WHEN** 变量超出作用域或 CLEAR
- **THEN** Rust 自动释放内存

#### Scenario: 字符串拼接
- **WHEN** "A" + "B" + "C"
- **THEN** 创建新字符串，原字符串可被回收

### Requirement: 栈内存管理
系统 SHALL 为 GOSUB 和 FOR 循环维护栈结构。

#### Scenario: GOSUB 栈
- **WHEN** GOSUB 调用
- **THEN** 返回地址入栈

#### Scenario: FOR 循环栈
- **WHEN** FOR 循环
- **THEN** 循环信息入栈

#### Scenario: 栈深度限制
- **WHEN** 栈深度超过限制
- **THEN** 返回 StackOverflow 错误

### Requirement: 内存使用查询
系统 SHALL 实现 FRE 函数，报告可用内存。

#### Scenario: FRE(0) 返回值
- **WHEN** 调用 FRE(0)
- **THEN** 返回模拟的可用内存值

#### Scenario: 内存使用增长
- **WHEN** 创建大量变量和数组
- **THEN** FRE 返回值减少

### Requirement: 内存清理
系统 SHALL 在 NEW, CLEAR, RUN 时正确清理内存。

#### Scenario: NEW 清理
- **WHEN** 执行 NEW
- **THEN** 程序、变量、数组全部清空

#### Scenario: CLEAR 清理
- **WHEN** 执行 CLEAR
- **THEN** 变量和数组清空，程序保留

#### Scenario: RUN 清理
- **WHEN** 执行 RUN
- **THEN** 变量清空，程序保留

### Requirement: DATA 存储
系统 SHALL 存储 DATA 语句的数据，提供顺序访问。

#### Scenario: DATA 预处理
- **WHEN** 程序加载
- **THEN** 收集所有 DATA 到列表

#### Scenario: READ 指针
- **WHEN** READ 读取
- **THEN** 维护当前读取位置

#### Scenario: RESTORE 重置
- **WHEN** RESTORE 执行
- **THEN** 指针重置到开头

### Requirement: 程序文本存储
系统 SHALL 保留原始程序文本（用于 LIST）。

#### Scenario: 保存原文
- **WHEN** 输入程序行
- **THEN** 保存原始文本（或 token 可重构文本）

#### Scenario: LIST 输出
- **WHEN** 执行 LIST
- **THEN** 输出可读的程序文本

### Requirement: 内存效率
系统 SHALL 在合理范围内优化内存使用。

#### Scenario: 避免重复字符串
- **WHEN** 多个变量引用同一字符串
- **THEN** Rust String 自动优化（写时复制）

#### Scenario: 延迟分配
- **WHEN** 声明但未使用的数组
- **THEN** 仅在首次访问时分配

### Requirement: 内存限制配置
系统 SHALL 允许配置内存限制（模拟 6502 环境）。

#### Scenario: 最大程序大小
- **WHEN** 程序行数或大小超限
- **THEN** 返回 OutOfMemory 错误

#### Scenario: 最大变量数
- **WHEN** 变量数量超限
- **THEN** 返回 OutOfMemory 错误

### Requirement: 安全的内存访问
系统 SHALL 保证内存访问安全，无越界或悬空指针。

#### Scenario: 数组边界检查
- **WHEN** 访问数组元素
- **THEN** 总是检查边界

#### Scenario: Rust 安全保证
- **WHEN** 所有内存操作
- **THEN** 利用 Rust 的所有权和借用检查

### Requirement: 内存泄漏防护
系统 SHALL 利用 Rust 的 RAII 防止内存泄漏。

#### Scenario: 自动释放
- **WHEN** 变量或数组不再使用
- **THEN** Rust 自动回收内存

#### Scenario: 异常安全
- **WHEN** 错误发生
- **THEN** 已分配内存仍正确释放

```

完美！现在让我验证这个提案是否符合 OpenSpec 规范：

太好了！提案验证通过。现在让我查看一下创建的完整内容：

让我再查看一下所有 specs 的概览：

完美！我已经成功为你创建了一个完整的 OpenSpec 提案。让我给你做个总结：

## 📋 创建的提案概览

### 📁 提案结构
```
openspec/changes/implement-basic-interpreter/
├── proposal.md       # 项目总体提案（为什么做、做什么、影响）
├── tasks.md          # 详细的 165 个实施任务清单
├── design.md         # 技术设计决策文档
└── specs/            # 10 个功能模块的规格说明
    ├── tokenizer/
    ├── parser/
    ├── runtime/
    ├── variables/
    ├── statements/
    ├── functions/
    ├── operators/
    ├── io/
    ├── float-arithmetic/
    └── memory/
```

### 🎯 项目规划要点

**实施阶段**（10-14 周）：
1. ✅ 核心基础设施（词法分析、解析器）
2. ✅ 基础语句（LET, PRINT, GOTO, IF, FOR）
3. ✅ 数值运算（运算符、浮点、数学函数）
4. ✅ 字符串和数组
5. ✅ 交互功能（INPUT、行编辑）
6. ✅ 高级特性（DEF FN、DATA/READ）
7. ✅ 文件和系统（可选）

**功能覆盖**：
- 27 个 BASIC 语句
- 22 个内置函数
- 完整的运算符支持
- 数组和字符串处理
- 交互式 REPL

**技术决策**：
- 使用 Rust f64 表示浮点数
- HashMap 存储变量
- BTreeMap 存储程序行
- rustyline 提供行编辑
- 完整的单元测试和集成测试

### ✅ 验证结果

提案已通过 OpenSpec 严格验证：
```bash
openspec validate implement-basic-interpreter --strict
# ✓ Change 'implement-basic-interpreter' is valid
```

### 📊 任务统计

- **总任务数**: 165 个
- **已完成**: 0 个（刚创建提案）
- **待完成**: 165 个

### 🚀 下一步建议

1. **审查提案**：仔细阅读 `proposal.md` 和 `design.md`，确认技术方案
2. **调整计划**：如需修改，可以编辑 tasks.md 和各个 spec.md
3. **开始实施**：按照 tasks.md 的顺序开始实施
4. **使用命令**：
   ```bash
   # 查看提案详情
   openspec show implement-basic-interpreter
   
   # 再次验证
   openspec validate implement-basic-interpreter --strict
   
   # 查看所有任务
   cat openspec/changes/implement-basic-interpreter/tasks.md
   ```

所有文档都用中文编写，包含详细的场景测试用例。每个功能模块都有清晰的需求定义和验收标准。你觉得这个提案如何？需要调整什么内容吗？

```plaintext
openspec/changes/implement-basic-interpreter/
├── proposal.md       # 项目总体提案（为什么做、做什么、影响）
├── tasks.md          # 详细的 165 个实施任务清单
├── design.md         # 技术设计决策文档
└── specs/            # 10 个功能模块的规格说明
    ├── tokenizer/
    ├── parser/
    ├── runtime/
    ├── variables/
    ├── statements/
    ├── functions/
    ├── operators/
    ├── io/
    ├── float-arithmetic/
    └── memory/
```

```shellscript
openspec validate implement-basic-interpreter --strict
# ✓ Change 'implement-basic-interpreter' is valid
```

```shellscript
   # 查看提案详情
   openspec show implement-basic-interpreter
   
   # 再次验证
   openspec validate implement-basic-interpreter --strict
   
   # 查看所有任务
   cat openspec/changes/implement-basic-interpreter/tasks.md
```

---

**User**

tasks.md中，关于ctrlc的处理，在哪个阶段？关于ctrlc的处理，我的要求写在了design.md中

---

**Cursor**

让我查看一下 `tasks.md` 中关于 Ctrl+C 处理的任务位置和内容：

我发现 `tasks.md` 中**缺少 Ctrl+C 中断处理的明确任务**。目前只在第14章"错误处理"中提到了 STOP 和 CONT 命令（第 14.5 和 14.6 项），但没有专门的 Ctrl+C 处理任务。

根据你在 `design.md` 中的要求："提供 Ctrl+C 中断支持, cont之后要求可以在断点继续执行"，我建议在 tasks.md 中补充这个任务。让我为你更新：

```markdown
# Implementation Tasks

## 1. 项目初始化
- [ ] 1.1 创建 Cargo 项目结构
- [ ] 1.2 配置 Cargo.toml 依赖
- [ ] 1.3 设置基础错误类型和结果类型
- [ ] 1.4 创建项目 README
- [ ] 1.5 设置测试框架

## 2. 词法分析器 (Tokenizer)
- [ ] 2.1 定义 Token 枚举类型
- [ ] 2.2 定义所有保留字 (27个语句关键字)
- [ ] 2.3 定义所有函数名 (22个内置函数)
- [ ] 2.4 定义运算符 token
- [ ] 2.5 实现数字常量识别 (整数和浮点数)
- [ ] 2.6 实现标识符识别 (变量名、行号)
- [ ] 2.7 实现字符串常量识别
- [ ] 2.8 处理空格和注释
- [ ] 2.9 实现行号解析
- [ ] 2.10 编写词法分析器单元测试

## 3. 语法解析器 (Parser)
- [ ] 3.1 定义抽象语法树 (AST) 数据结构
- [ ] 3.2 实现表达式解析 (递归下降或优先级攀升)
- [ ] 3.3 实现语句解析器基础框架
- [ ] 3.4 解析简单语句 (LET, PRINT, REM, END)
- [ ] 3.5 解析流程控制语句 (GOTO, IF)
- [ ] 3.6 解析循环语句 (FOR...NEXT)
- [ ] 3.7 解析子程序调用 (GOSUB, RETURN)
- [ ] 3.8 解析数组和函数调用
- [ ] 3.9 实现错误恢复机制
- [ ] 3.10 编写解析器单元测试

## 4. 运行时环境 (Runtime)
- [ ] 4.1 定义程序状态结构
- [ ] 4.2 实现程序行存储和索引
- [ ] 4.3 实现程序计数器和行号跳转
- [ ] 4.4 实现 GOSUB 栈管理
- [ ] 4.5 实现 FOR 循环栈管理
- [ ] 4.6 实现直接模式 vs 程序模式
- [ ] 4.7 实现 NEW 命令 (清空程序)
- [ ] 4.8 实现 LIST 命令 (列出程序)
- [ ] 4.9 实现 RUN 命令
- [ ] 4.10 实现 STOP, END, CONT 命令
- [ ] 4.11 编写运行时单元测试

## 5. 变量系统 (Variables)
- [ ] 5.1 定义变量值类型 (整数、浮点、字符串)
- [ ] 5.2 实现变量名解析 (A-Z, A0-Z9, A-Z$)
- [ ] 5.3 实现简单变量存储 (HashMap)
- [ ] 5.4 实现数组变量存储
- [ ] 5.5 实现 DIM 语句
- [ ] 5.6 实现数组边界检查
- [ ] 5.7 实现多维数组支持
- [ ] 5.8 实现变量类型检查
- [ ] 5.9 实现 CLEAR 命令 (清空变量)
- [ ] 5.10 编写变量系统单元测试

## 6. 基础语句实现
- [ ] 6.1 LET 语句 (赋值)
- [ ] 6.2 PRINT 语句 (基础输出)
- [ ] 6.3 PRINT 使用分隔符 (逗号、分号)
- [ ] 6.4 PRINT TAB() 和 SPC() 函数
- [ ] 6.5 REM 语句 (注释)
- [ ] 6.6 GOTO 语句
- [ ] 6.7 IF...THEN 语句
- [ ] 6.8 GOSUB 和 RETURN 语句
- [ ] 6.9 ON...GOTO 和 ON...GOSUB 语句
- [ ] 6.10 FOR...NEXT 语句
- [ ] 6.11 FOR...STEP 支持
- [ ] 6.12 测试所有基础语句

## 7. 运算符实现
- [ ] 7.1 算术运算符 (+, -, *, /)
- [ ] 7.2 乘方运算符 (^)
- [ ] 7.3 一元负号 (-)
- [ ] 7.4 关系运算符 (=, <>, <, >, <=, >=)
- [ ] 7.5 逻辑运算符 (AND, OR, NOT)
- [ ] 7.6 字符串连接 (+)
- [ ] 7.7 运算符优先级处理
- [ ] 7.8 类型转换和检查
- [ ] 7.9 测试所有运算符

## 8. 浮点运算 (Float Arithmetic)
- [ ] 8.1 选择浮点数表示 (f64 或自定义)
- [ ] 8.2 实现加法和减法
- [ ] 8.3 实现乘法和除法
- [ ] 8.4 实现乘方运算
- [ ] 8.5 实现数值比较
- [ ] 8.6 实现整数和浮点转换
- [ ] 8.7 实现浮点数输入解析
- [ ] 8.8 实现浮点数输出格式化
- [ ] 8.9 处理溢出和下溢
- [ ] 8.10 测试浮点运算精度

## 9. 数学函数
- [ ] 9.1 SGN(x) - 符号函数
- [ ] 9.2 INT(x) - 取整函数
- [ ] 9.3 ABS(x) - 绝对值
- [ ] 9.4 SQR(x) - 平方根
- [ ] 9.5 EXP(x) - 指数函数
- [ ] 9.6 LOG(x) - 自然对数
- [ ] 9.7 SIN(x), COS(x), TAN(x) - 三角函数
- [ ] 9.8 ATN(x) - 反正切
- [ ] 9.9 RND(x) - 随机数
- [ ] 9.10 测试所有数学函数

## 10. 字符串功能
- [ ] 10.1 字符串变量存储
- [ ] 10.2 字符串连接操作
- [ ] 10.3 LEN(s$) - 字符串长度
- [ ] 10.4 LEFT$(s$, n) - 左子串
- [ ] 10.5 RIGHT$(s$, n) - 右子串
- [ ] 10.6 MID$(s$, n[, m]) - 中间子串
- [ ] 10.7 STR$(x) - 数字转字符串
- [ ] 10.8 VAL(s$) - 字符串转数字
- [ ] 10.9 ASC(s$) - 字符 ASCII 码
- [ ] 10.10 CHR$(x) - ASCII 码转字符
- [ ] 10.11 字符串比较
- [ ] 10.12 测试所有字符串函数

## 11. 输入输出 (I/O)
- [ ] 11.1 实现基础 INPUT 语句
- [ ] 11.2 INPUT 提示符支持
- [ ] 11.3 INPUT 多个变量
- [ ] 11.4 INPUT 错误处理和重试
- [ ] 11.5 实现 DATA 语句
- [ ] 11.6 实现 READ 语句
- [ ] 11.7 实现 RESTORE 语句
- [ ] 11.8 集成 rustyline 行编辑
- [ ] 11.9 实现命令历史
- [ ] 11.10 测试 I/O 功能

## 12. 系统函数
- [ ] 12.1 FRE(x) - 剩余内存
- [ ] 12.2 POS(x) - 光标位置
- [ ] 12.3 PEEK(addr) - 读内存 (模拟)
- [ ] 12.4 POKE addr, value - 写内存 (模拟)
- [ ] 12.5 WAIT 语句
- [ ] 12.6 测试系统函数

## 13. 用户自定义函数
- [ ] 13.1 DEF FN 语句解析
- [ ] 13.2 单行函数定义
- [ ] 13.3 函数调用 FN name(arg)
- [ ] 13.4 函数作用域管理
- [ ] 13.5 测试用户定义函数

## 14. 错误处理
- [ ] 14.1 定义所有错误类型
- [ ] 14.2 实现错误消息 (中文/英文)
- [ ] 14.3 语法错误报告 (行号、位置)
- [ ] 14.4 运行时错误处理
- [ ] 14.5 STOP 和 END 的区别
- [ ] 14.6 CONT 命令恢复执行
- [ ] 14.7 实现 Ctrl+C 中断处理（捕获信号）
- [ ] 14.8 Ctrl+C 中断后保存执行状态（行号、位置）
- [ ] 14.9 确保 CONT 可以从 Ctrl+C 中断点继续执行
- [ ] 14.10 测试 Ctrl+C 中断和恢复流程
- [ ] 14.11 测试其他错误处理场景

## 15. 交互模式
- [ ] 15.1 REPL 主循环
- [ ] 15.2 直接模式命令执行
- [ ] 15.3 程序编辑 (输入行号)
- [ ] 15.4 行删除 (输入空行)
- [ ] 15.5 LIST 命令变体 (LIST 10-50)
- [ ] 15.6 启动横幅和提示符
- [ ] 15.7 退出命令
- [ ] 15.8 测试交互模式

## 16. 高级功能 (可选)
- [ ] 16.1 LOAD 命令 (从文件加载程序)
- [ ] 16.2 SAVE 命令 (保存程序到文件)
- [ ] 16.3 GET 语句 (单字符输入)
- [ ] 16.4 NULL 语句
- [ ] 16.5 CMD 和 SYS 语句
- [ ] 16.6 文件格式定义
- [ ] 16.7 测试文件操作

## 17. 集成测试
- [ ] 17.1 简单程序测试 (Hello World)
- [ ] 17.2 循环测试 (求和、阶乘)
- [ ] 17.3 数组测试 (排序、搜索)
- [ ] 17.4 字符串处理测试
- [ ] 17.5 子程序测试
- [ ] 17.6 递归函数测试
- [ ] 17.7 经典游戏测试 (猜数字、星际迷航)
- [ ] 17.8 数学计算测试
- [ ] 17.9 综合测试程序
- [ ] 17.10 性能基准测试

## 18. 文档和完善
- [ ] 18.1 完善代码注释
- [ ] 18.2 编写用户手册
- [ ] 18.3 编写开发者文档
- [ ] 18.4 添加使用示例
- [ ] 18.5 优化错误消息
- [ ] 18.6 代码重构和清理
- [ ] 18.7 性能分析和优化
- [ ] 18.8 最终测试

## 19. 部署和发布
- [ ] 19.1 配置 CI/CD
- [ ] 19.2 创建发布版本
- [ ] 19.3 编写 CHANGELOG
- [ ] 19.4 准备示例程序
- [ ] 19.5 发布第一个版本


```

完成了！我已经在 **第14章"错误处理"**中添加了关于 Ctrl+C 中断处理的详细任务：

- **14.7** - 实现 Ctrl+C 中断处理（捕获信号）
- **14.8** - Ctrl+C 中断后保存执行状态（行号、位置）
- **14.9** - 确保 CONT 可以从 Ctrl+C 中断点继续执行
- **14.10** - 测试 Ctrl+C 中断和恢复流程

这样就符合你在 `design.md` 中的要求了："提供 Ctrl+C 中断支持, cont之后要求可以在断点继续执行"。

这些任务位于**第14章（错误处理）**，与 STOP/CONT 命令（14.5 和 14.6）在一起，因为它们在技术实现上是相关的——都需要保存执行状态并支持恢复。

现在任务总数从 165 个增加到 169 个。需要我同时更新 `io/spec.md` 文件，添加 Ctrl+C 中断的需求规格吗？

---

**User**

好的

---

**Cursor**

好的，让我更新 `io/spec.md` 文件，添加 Ctrl+C 中断处理的详细规格：

```markdown
# I/O Specification

## ADDED Requirements

### Requirement: INPUT 语句
系统 SHALL 实现 INPUT 语句，从用户读取输入。

#### Scenario: 基本输入
- **WHEN** 执行 "INPUT A"
- **THEN** 显示 "? " 提示符，等待用户输入

#### Scenario: 带提示符的输入
- **WHEN** 执行 `INPUT "ENTER VALUE"; A`
- **THEN** 显示 "ENTER VALUE? "

#### Scenario: 输入多个变量
- **WHEN** 执行 "INPUT A, B, C"
- **THEN** 提示用户输入三个值（逗号分隔）

#### Scenario: 输入类型检查
- **WHEN** INPUT A（数值变量），用户输入 "ABC"
- **THEN** 显示 "?REDO FROM START"，重新输入

#### Scenario: 字符串输入
- **WHEN** INPUT A$，用户输入 HELLO
- **THEN** A$ = "HELLO"

#### Scenario: 字符串带引号
- **WHEN** INPUT A$，用户输入 "HELLO, WORLD"
- **THEN** A$ = "HELLO, WORLD"（保留逗号）

### Requirement: PRINT 输出格式
系统 SHALL 实现 PRINT 的各种输出格式。

#### Scenario: 数值输出格式
- **WHEN** PRINT 正数
- **THEN** 前后各有一个空格
- **WHEN** PRINT 负数
- **THEN** 前有空格，负号紧跟数字

#### Scenario: 科学计数法
- **WHEN** PRINT 极大或极小数值
- **THEN** 使用科学计数法（如 1.5E+10）

#### Scenario: 列对齐（逗号）
- **WHEN** PRINT A, B, C
- **THEN** 每个值占 14 个字符宽度

#### Scenario: 自动换行
- **WHEN** 打印超过行宽（72 字符）
- **THEN** 自动换行

### Requirement: TAB 和 SPC 函数
系统 SHALL 实现 TAB 和 SPC 格式化函数。

#### Scenario: TAB 跳转到指定列
- **WHEN** PRINT TAB(10); "HELLO"
- **THEN** 从第 10 列开始输出

#### Scenario: TAB 小于当前位置
- **WHEN** 当前列 20，执行 TAB(10)
- **THEN** 换行后跳转到第 10 列

#### Scenario: SPC 输出空格
- **WHEN** PRINT SPC(5); "HELLO"
- **THEN** 输出 5 个空格后输出 HELLO

### Requirement: DATA/READ 机制
系统 SHALL 实现 DATA 语句和 READ 语句的数据交换。

#### Scenario: DATA 存储
- **WHEN** 程序包含 "10 DATA 1, 2, 3"
- **THEN** 数据被存储供 READ 使用

#### Scenario: READ 顺序读取
- **WHEN** 执行 "READ A, B, C"
- **THEN** 依次从 DATA 读取 1, 2, 3

#### Scenario: 多行 DATA
- **WHEN** 多行 DATA 语句
- **THEN** 数据连续存储

#### Scenario: 混合数据类型
- **WHEN** DATA 包含数值和字符串
- **THEN** READ 根据变量类型正确读取

#### Scenario: OUT OF DATA 错误
- **WHEN** READ 超过可用数据
- **THEN** 返回 OutOfData 错误

### Requirement: RESTORE 数据指针
系统 SHALL 实现 RESTORE 重置数据指针。

#### Scenario: RESTORE 重置到开头
- **WHEN** READ 几次后执行 RESTORE
- **THEN** 数据指针回到第一个 DATA

#### Scenario: RESTORE 到指定行
- **WHEN** 执行 "RESTORE 100"
- **THEN** 数据指针移到行 100 的 DATA

### Requirement: GET 语句（可选）
系统 SHALL 实现 GET 语句，读取单个字符。

#### Scenario: GET 读取字符
- **WHEN** 执行 "GET A$"
- **THEN** 等待用户按键，读取单个字符

#### Scenario: GET 不回显
- **WHEN** GET 读取字符
- **THEN** 字符不显示在屏幕上

#### Scenario: GET 不等待回车
- **WHEN** GET 读取
- **THEN** 按键立即返回，无需回车

### Requirement: 行编辑功能
系统 SHALL 在交互模式提供行编辑功能。

#### Scenario: 光标移动
- **WHEN** 用户输入时
- **THEN** 支持左右箭头移动光标

#### Scenario: 删除字符
- **WHEN** 用户按 Backspace
- **THEN** 删除前一个字符

#### Scenario: 命令历史
- **WHEN** 用户按上下箭头
- **THEN** 浏览历史命令

#### Scenario: Home/End 键
- **WHEN** 用户按 Home/End
- **THEN** 跳转到行首/行尾

### Requirement: 文件 I/O（可选）
系统 SHALL 实现 LOAD 和 SAVE 命令。

#### Scenario: SAVE 保存程序
- **WHEN** 执行 `SAVE "PROGRAM.BAS"`
- **THEN** 程序保存到文件

#### Scenario: LOAD 加载程序
- **WHEN** 执行 `LOAD "PROGRAM.BAS"`
- **THEN** 从文件加载程序（清空当前程序）

#### Scenario: 文件不存在
- **WHEN** LOAD 不存在的文件
- **THEN** 返回 FileNotFound 错误

### Requirement: 错误输出
系统 SHALL 将错误消息输出到标准错误流。

#### Scenario: 运行时错误显示
- **WHEN** 发生运行时错误
- **THEN** 显示 "?ERROR_NAME IN line"

#### Scenario: 语法错误显示
- **WHEN** 发生语法错误
- **THEN** 显示 "?SYNTAX ERROR"

### Requirement: 输入中断
系统 SHALL 支持用户中断输入和执行，并保存执行状态以便恢复。

#### Scenario: Ctrl+C 中断程序执行
- **WHEN** 程序运行时按 Ctrl+C
- **THEN** 中断执行，返回直接模式

#### Scenario: Ctrl+C 中断输入等待
- **WHEN** 等待 INPUT 输入时按 Ctrl+C
- **THEN** 中断输入，返回直接模式

#### Scenario: 中断消息显示
- **WHEN** Ctrl+C 中断
- **THEN** 显示 "?BREAK IN line XXX"（显示当前行号）

#### Scenario: 保存中断状态
- **WHEN** Ctrl+C 中断发生
- **THEN** 保存当前执行行号和语句位置

#### Scenario: CONT 从中断点恢复
- **WHEN** Ctrl+C 中断后执行 CONT
- **THEN** 从中断点继续执行程序

#### Scenario: 中断后程序不变
- **WHEN** Ctrl+C 中断后
- **THEN** 程序和变量状态保持不变，可用 LIST 查看


```

现在让我也更新 `runtime/spec.md`，补充 Ctrl+C 中断后 CONT 恢复的场景：

```markdown
# Runtime Specification

## ADDED Requirements

### Requirement: 程序存储和管理
系统 SHALL 存储 BASIC 程序行，按行号排序，支持插入、删除和查找。

#### Scenario: 添加程序行
- **WHEN** 输入 "10 PRINT HELLO"
- **THEN** 行 10 被添加到程序中

#### Scenario: 替换现有行
- **WHEN** 已有行 10，再输入 "10 PRINT WORLD"
- **THEN** 行 10 被新内容替换

#### Scenario: 删除程序行
- **WHEN** 输入 "10"（仅行号，无内容）
- **THEN** 行 10 从程序中删除

#### Scenario: 行号排序
- **WHEN** 输入顺序为 30, 10, 20
- **THEN** 程序按 10, 20, 30 顺序存储

### Requirement: 程序执行
系统 SHALL 从指定行号开始顺序执行程序，直到遇到 END, STOP 或程序结尾。

#### Scenario: 从第一行开始执行
- **WHEN** 执行 RUN 命令
- **THEN** 从最小行号开始执行

#### Scenario: 从指定行开始
- **WHEN** 执行 "RUN 100"
- **THEN** 从行 100 开始执行

#### Scenario: 顺序执行
- **WHEN** 程序有行 10, 20, 30
- **THEN** 依次执行 10, 20, 30

#### Scenario: END 停止执行
- **WHEN** 遇到 END 语句
- **THEN** 程序正常结束

### Requirement: 行号跳转
系统 SHALL 支持 GOTO 和 GOSUB 的行号跳转。

#### Scenario: GOTO 跳转
- **WHEN** 执行 "GOTO 100"
- **THEN** 下一条执行的是行 100

#### Scenario: 跳转到不存在的行
- **WHEN** GOTO 到不存在的行号
- **THEN** 返回 RuntimeError::UndefinedLine

#### Scenario: 向前跳转
- **WHEN** 从行 100 GOTO 50
- **THEN** 成功向前跳转

### Requirement: 子程序调用栈
系统 SHALL 维护 GOSUB 调用栈，支持嵌套子程序调用。

#### Scenario: GOSUB 调用
- **WHEN** 在行 10 执行 "GOSUB 100"
- **THEN** 跳转到 100，返回地址入栈

#### Scenario: RETURN 返回
- **WHEN** 执行 RETURN
- **THEN** 返回到上一个 GOSUB 的下一条语句

#### Scenario: 嵌套 GOSUB
- **WHEN** 执行嵌套的 GOSUB
- **THEN** 栈正确维护，RETURN 按正确顺序返回

#### Scenario: RETURN 无对应 GOSUB
- **WHEN** 没有 GOSUB 就执行 RETURN
- **THEN** 返回 RuntimeError::ReturnWithoutGosub

#### Scenario: GOSUB 栈深度限制
- **WHEN** GOSUB 嵌套超过限制（如 100 层）
- **THEN** 返回 RuntimeError::StackOverflow

### Requirement: FOR 循环栈
系统 SHALL 维护 FOR 循环栈，管理循环变量、终值和步长。

#### Scenario: FOR 循环执行
- **WHEN** 执行 "FOR I = 1 TO 10"
- **THEN** 循环信息入栈，I 初始化为 1

#### Scenario: NEXT 执行
- **WHEN** 执行 "NEXT I"
- **THEN** I 增加步长，检查是否继续循环

#### Scenario: 循环结束
- **WHEN** I 超过终值
- **THEN** 退出循环，继续执行 NEXT 后的语句

#### Scenario: 嵌套 FOR 循环
- **WHEN** 执行嵌套 FOR 循环
- **THEN** 内外循环正确管理

#### Scenario: NEXT 变量不匹配
- **WHEN** FOR I ... NEXT J
- **THEN** 返回 RuntimeError::NextWithoutFor

### Requirement: 直接模式执行
系统 SHALL 支持直接执行语句（无行号），不保存到程序中。

#### Scenario: 直接执行 PRINT
- **WHEN** 输入 "PRINT 2+3"（无行号）
- **THEN** 立即执行并输出 5

#### Scenario: 直接模式中的变量
- **WHEN** 直接执行 "A=5" 然后 "PRINT A"
- **THEN** 变量在直接模式和程序模式间共享

#### Scenario: 直接模式不能使用 GOTO
- **WHEN** 直接模式执行 "GOTO 100"
- **THEN** 返回错误（无上下文）

### Requirement: NEW 命令
系统 SHALL 实现 NEW 命令，清空程序和变量。

#### Scenario: NEW 清空程序
- **WHEN** 程序有多行，执行 NEW
- **THEN** 程序被完全清空

#### Scenario: NEW 清空变量
- **WHEN** 有变量值，执行 NEW
- **THEN** 所有变量被清空

#### Scenario: NEW 重置运行状态
- **WHEN** 程序在运行中断后执行 NEW
- **THEN** 运行状态完全重置

### Requirement: LIST 命令
系统 SHALL 实现 LIST 命令，显示程序内容。

#### Scenario: LIST 全部程序
- **WHEN** 执行 LIST
- **THEN** 显示所有程序行

#### Scenario: LIST 单行
- **WHEN** 执行 "LIST 10"
- **THEN** 仅显示行 10

#### Scenario: LIST 范围
- **WHEN** 执行 "LIST 10-50"
- **THEN** 显示行 10 到 50

#### Scenario: 空程序 LIST
- **WHEN** 程序为空时执行 LIST
- **THEN** 无输出或显示 "EMPTY"

### Requirement: STOP 和 CONT
系统 SHALL 实现 STOP 命令暂停执行，CONT 命令继续执行，同时支持 Ctrl+C 中断后的恢复。

#### Scenario: STOP 暂停
- **WHEN** 执行 STOP
- **THEN** 程序暂停，显示 "BREAK IN line"

#### Scenario: CONT 从 STOP 继续
- **WHEN** STOP 后执行 CONT
- **THEN** 从 STOP 的下一条语句继续

#### Scenario: Ctrl+C 中断暂停
- **WHEN** 程序运行时按 Ctrl+C
- **THEN** 程序中断，显示 "?BREAK IN line XXX"

#### Scenario: CONT 从 Ctrl+C 中断点继续
- **WHEN** Ctrl+C 中断后执行 CONT
- **THEN** 从中断点（当前语句）继续执行

#### Scenario: 未暂停时 CONT
- **WHEN** 没有 STOP 或中断就执行 CONT
- **THEN** 返回 RuntimeError::CantContinue

#### Scenario: 程序修改后 CONT
- **WHEN** STOP/中断后修改程序再 CONT
- **THEN** 返回错误（无法继续）

#### Scenario: 中断状态保存
- **WHEN** STOP 或 Ctrl+C 中断
- **THEN** 保存当前行号、语句位置和所有变量状态

### Requirement: RUN 命令
系统 SHALL 实现 RUN 命令，清空变量后执行程序。

#### Scenario: RUN 清空变量
- **WHEN** 有变量值时执行 RUN
- **THEN** 变量被清空

#### Scenario: RUN 保留程序
- **WHEN** 执行 RUN
- **THEN** 程序内容不变

#### Scenario: RUN 指定行号
- **WHEN** 执行 "RUN 100"
- **THEN** 从行 100 开始执行

### Requirement: 错误处理和恢复
系统 SHALL 捕获运行时错误，显示错误消息和行号，返回直接模式。

#### Scenario: 显示错误消息
- **WHEN** 发生运行时错误
- **THEN** 显示错误类型和行号

#### Scenario: 返回直接模式
- **WHEN** 错误发生后
- **THEN** 返回直接模式，等待用户输入

#### Scenario: 错误不破坏状态
- **WHEN** 错误发生
- **THEN** 程序和变量状态保持（可用 LIST 查看）


```

完美！现在让我验证一下更新后的提案是否仍然符合规范：

完成了！✅ 我已经全面更新了 Ctrl+C 中断处理的规格和任务，验证通过。

## 📝 更新总结

### 1️⃣ **tasks.md** - 第14章（错误处理）
新增了 4 个关于 Ctrl+C 的任务：
- **14.7** - 实现 Ctrl+C 中断处理（捕获信号）
- **14.8** - Ctrl+C 中断后保存执行状态（行号、位置）
- **14.9** - 确保 CONT 可以从 Ctrl+C 中断点继续执行
- **14.10** - 测试 Ctrl+C 中断和恢复流程

### 2️⃣ **specs/io/spec.md** - 输入中断需求
扩展了"输入中断"需求，新增了 5 个场景：
- ✅ Ctrl+C 中断程序执行
- ✅ Ctrl+C 中断输入等待
- ✅ 中断消息显示（显示行号）
- ✅ **保存中断状态**
- ✅ **CONT 从中断点恢复** ← 你的核心要求
- ✅ 中断后程序不变

### 3️⃣ **specs/runtime/spec.md** - STOP 和 CONT 需求
扩展了 STOP/CONT 需求，新增了 3 个场景：
- ✅ Ctrl+C 中断暂停
- ✅ **CONT 从 Ctrl+C 中断点继续执行** ← 明确从当前语句继续
- ✅ 中断状态保存（行号、位置、变量）

## 🎯 关键点

现在规格中明确要求：
1. **Ctrl+C 中断时**：保存当前执行行号和语句位置
2. **CONT 命令**：可以从 Ctrl+C 中断点继续执行（不是下一条，是当前语句）
3. **状态保持**：变量和程序状态完全保留

这完全符合你在 `design.md` 中的要求："提供 Ctrl+C 中断支持, cont之后要求可以在断点继续执行"。

所有更新已通过 OpenSpec 严格验证 ✓

---

**User**

另外，还有一个需要注意的点，我也希望补充到规格里，就是冒号分割的，在一行之内的语句，包括在一行的FOR语句，需要支持！

---

**Cursor**

好的！这是一个很重要的需求。让我先检查一下现有规格中关于冒号分隔语句的描述：

好的！我看到现有规格中有提到冒号分隔，但不够明确和完整。让我补充更详细的场景，特别是 FOR 循环在一行内的情况：

```markdown
# Parser Specification

## ADDED Requirements

### Requirement: Expression Parsing
系统 SHALL 解析表达式，包括变量、常量、函数调用和运算符，并正确处理运算符优先级。

#### Scenario: 简单算术表达式
- **WHEN** 输入 tokens 对应 "2 + 3"
- **THEN** 解析为加法表达式节点

#### Scenario: 运算符优先级
- **WHEN** 输入 "2 + 3 * 4"
- **THEN** 解析为 2 + (3 * 4)，乘法优先

#### Scenario: 括号改变优先级
- **WHEN** 输入 "(2 + 3) * 4"
- **THEN** 解析为 (2 + 3) * 4，加法先计算

#### Scenario: 乘方优先级
- **WHEN** 输入 "2 ^ 3 * 4"
- **THEN** 解析为 (2 ^ 3) * 4，乘方最高优先级

### Requirement: 函数调用解析
系统 SHALL 解析函数调用，包括单参数和多参数函数。

#### Scenario: 单参数函数
- **WHEN** 输入 "SIN(X)"
- **THEN** 解析为函数调用，函数名 SIN，参数 X

#### Scenario: 嵌套函数调用
- **WHEN** 输入 "SQR(ABS(X))"
- **THEN** 解析为嵌套的函数调用

#### Scenario: 多参数函数
- **WHEN** 输入 "LEFT$(A$, 5)"
- **THEN** 解析为两个参数的函数调用

### Requirement: 数组访问解析
系统 SHALL 解析数组元素访问，支持多维数组。

#### Scenario: 一维数组
- **WHEN** 输入 "A(5)"
- **THEN** 解析为数组访问，变量 A，索引 5

#### Scenario: 二维数组
- **WHEN** 输入 "B(3, 4)"
- **THEN** 解析为二维数组访问

#### Scenario: 表达式作为索引
- **WHEN** 输入 "A(I + 1)"
- **THEN** 索引部分解析为表达式

### Requirement: LET 语句解析
系统 SHALL 解析赋值语句，支持简单变量和数组元素赋值。

#### Scenario: 简单赋值
- **WHEN** 输入 "LET A = 10"
- **THEN** 解析为赋值语句，变量 A，值 10

#### Scenario: LET 可选
- **WHEN** 输入 "A = 10"（省略 LET）
- **THEN** 同样解析为赋值语句

#### Scenario: 表达式赋值
- **WHEN** 输入 "A = B + C * 2"
- **THEN** 右侧解析为表达式

### Requirement: PRINT 语句解析
系统 SHALL 解析 PRINT 语句，支持多个表达式、分隔符（逗号、分号）。

#### Scenario: 打印单个值
- **WHEN** 输入 "PRINT 42"
- **THEN** 解析为 PRINT 语句，参数列表包含一个表达式

#### Scenario: 打印多个值（逗号分隔）
- **WHEN** 输入 "PRINT A, B, C"
- **THEN** 逗号表示列分隔

#### Scenario: 打印多个值（分号分隔）
- **WHEN** 输入 "PRINT A; B; C"
- **THEN** 分号表示紧密连接

#### Scenario: TAB 和 SPC 函数
- **WHEN** 输入 "PRINT TAB(10); A"
- **THEN** 解析 TAB 函数调用

### Requirement: 流程控制语句解析
系统 SHALL 解析 GOTO, IF...THEN, GOSUB, RETURN 等流程控制语句。

#### Scenario: GOTO 语句
- **WHEN** 输入 "GOTO 100"
- **THEN** 解析为 GOTO，目标行号 100

#### Scenario: IF...THEN 语句
- **WHEN** 输入 "IF A > 10 THEN 200"
- **THEN** 解析为条件语句，条件表达式，目标行号

#### Scenario: IF...THEN 执行语句
- **WHEN** 输入 "IF A > 10 THEN PRINT A"
- **THEN** THEN 后跟语句而非行号

#### Scenario: GOSUB 和 RETURN
- **WHEN** 输入 "GOSUB 500" 和 "RETURN"
- **THEN** 分别解析为子程序调用和返回

### Requirement: 循环语句解析
系统 SHALL 解析 FOR...NEXT 循环语句，支持 STEP。

#### Scenario: 基本 FOR 循环
- **WHEN** 输入 "FOR I = 1 TO 10"
- **THEN** 解析为循环，变量 I，起始 1，结束 10

#### Scenario: 带 STEP 的循环
- **WHEN** 输入 "FOR I = 10 TO 1 STEP -1"
- **THEN** 步长为 -1

#### Scenario: NEXT 语句
- **WHEN** 输入 "NEXT I"
- **THEN** 解析为 NEXT，变量 I

#### Scenario: NEXT 省略变量
- **WHEN** 输入 "NEXT"
- **THEN** 解析为 NEXT，默认最内层循环变量

### Requirement: INPUT 语句解析
系统 SHALL 解析 INPUT 语句，支持提示符和多个变量。

#### Scenario: 基本 INPUT
- **WHEN** 输入 "INPUT A"
- **THEN** 解析为 INPUT 语句，变量列表包含 A

#### Scenario: 带提示符的 INPUT
- **WHEN** 输入 `INPUT "ENTER VALUE"; A`
- **THEN** 提示符为 "ENTER VALUE"

#### Scenario: 输入多个变量
- **WHEN** 输入 "INPUT A, B, C"
- **THEN** 变量列表包含 A, B, C

### Requirement: DIM 语句解析
系统 SHALL 解析 DIM 语句，声明数组维度。

#### Scenario: 一维数组声明
- **WHEN** 输入 "DIM A(10)"
- **THEN** 解析为 DIM，数组 A，大小 10

#### Scenario: 多维数组声明
- **WHEN** 输入 "DIM B(5, 10)"
- **THEN** 解析为 DIM，数组 B，维度 5x10

#### Scenario: 多个数组声明
- **WHEN** 输入 "DIM A(10), B(20)"
- **THEN** 一条语句声明多个数组

### Requirement: DATA/READ/RESTORE 解析
系统 SHALL 解析 DATA, READ, RESTORE 语句。

#### Scenario: DATA 语句
- **WHEN** 输入 "DATA 1, 2, 3, 4"
- **THEN** 解析为 DATA，数据列表 [1, 2, 3, 4]

#### Scenario: 混合类型 DATA
- **WHEN** 输入 `DATA 10, "HELLO", 3.14`
- **THEN** 支持数字和字符串混合

#### Scenario: READ 语句
- **WHEN** 输入 "READ A, B, C"
- **THEN** 解析为 READ，变量列表

#### Scenario: RESTORE 语句
- **WHEN** 输入 "RESTORE" 或 "RESTORE 100"
- **THEN** 重置或跳转到指定行

### Requirement: DEF FN 解析
系统 SHALL 解析用户自定义函数。

#### Scenario: 单参数函数定义
- **WHEN** 输入 "DEF FNA(X) = X * X"
- **THEN** 解析为函数定义，名称 FNA，参数 X，表达式 X * X

#### Scenario: 函数调用
- **WHEN** 输入 "Y = FNA(5)"
- **THEN** 解析为函数调用

### Requirement: ON...GOTO 解析
系统 SHALL 解析 ON...GOTO 和 ON...GOSUB 语句。

#### Scenario: ON GOTO
- **WHEN** 输入 "ON X GOTO 100, 200, 300"
- **THEN** 根据 X 的值跳转到不同行号

#### Scenario: ON GOSUB
- **WHEN** 输入 "ON X GOSUB 100, 200, 300"
- **THEN** 根据 X 的值调用不同子程序

### Requirement: 其他语句解析
系统 SHALL 解析 END, STOP, REM, CLEAR, NEW, LIST 等语句。

#### Scenario: END 语句
- **WHEN** 输入 "END"
- **THEN** 解析为程序结束

#### Scenario: REM 语句
- **WHEN** 输入 "REM THIS IS A COMMENT"
- **THEN** 解析为注释，内容被保留但不执行

#### Scenario: CLEAR 语句
- **WHEN** 输入 "CLEAR"
- **THEN** 解析为清空变量

### Requirement: 语句分隔符
系统 SHALL 支持冒号分隔一行中的多条语句，包括复杂的控制流语句。

#### Scenario: 单行多语句
- **WHEN** 输入 "10 A=1: B=2: PRINT A+B"
- **THEN** 解析为三条语句

#### Scenario: 单行 FOR 循环
- **WHEN** 输入 "10 FOR I=1 TO 10: PRINT I: NEXT I"
- **THEN** 解析为完整的 FOR...NEXT 循环结构

#### Scenario: 单行嵌套 FOR 循环
- **WHEN** 输入 "10 FOR I=1 TO 3: FOR J=1 TO 3: PRINT I*J: NEXT J: NEXT I"
- **THEN** 正确解析嵌套循环

#### Scenario: 单行 IF...THEN 语句
- **WHEN** 输入 "10 IF A>10 THEN A=0: PRINT A"
- **THEN** THEN 后的多条语句被正确解析

#### Scenario: 单行 GOSUB 和计算
- **WHEN** 输入 "10 A=5: GOSUB 100: PRINT A"
- **THEN** 三条语句按顺序执行

#### Scenario: 复杂单行语句
- **WHEN** 输入 "10 INPUT A: IF A>0 THEN B=A*2: PRINT B: GOTO 20"
- **THEN** 所有语句正确解析和执行

### Requirement: 错误处理
系统 SHALL 对语法错误给出清晰的错误位置和消息。

#### Scenario: 缺少表达式
- **WHEN** 输入 "LET A ="（缺少右值）
- **THEN** 返回 ParseError::ExpectedExpression

#### Scenario: 括号不匹配
- **WHEN** 输入 "PRINT (A + B"
- **THEN** 返回 ParseError::UnmatchedParenthesis

#### Scenario: 无效语句
- **WHEN** 输入 "XYZ 123"（XYZ 不是关键字）
- **THEN** 返回 ParseError::InvalidStatement


```

现在让我也在 tokenizer 规格中补充冒号作为语句分隔符的说明：

```markdown
# Tokenizer Specification

## ADDED Requirements

### Requirement: Token Type Definition
系统 SHALL 定义所有 BASIC 语言的 token 类型，包括关键字、标识符、字面量、运算符和分隔符。

#### Scenario: 识别保留字
- **WHEN** 输入字符串 "PRINT"
- **THEN** 返回 Token::Print

#### Scenario: 识别变量名
- **WHEN** 输入字符串 "A1"
- **THEN** 返回 Token::Identifier("A1")

#### Scenario: 识别数字
- **WHEN** 输入字符串 "123.45"
- **THEN** 返回 Token::Number(123.45)

### Requirement: 保留字识别
系统 SHALL 识别所有 27 个 BASIC 语句关键字，包括 END, FOR, NEXT, DATA, INPUT, DIM, READ, LET, GOTO, RUN, IF, RESTORE, GOSUB, RETURN, REM, STOP, ON, NULL, WAIT, LOAD, SAVE, DEF, POKE, PRINT, CONT, LIST, CLEAR, GET, NEW。

#### Scenario: 大小写不敏感
- **WHEN** 输入 "print", "PRINT", 或 "Print"
- **THEN** 均识别为 Print 关键字

#### Scenario: 关键字优先于标识符
- **WHEN** 输入 "FOR"
- **THEN** 返回 Token::For 而非 Identifier

### Requirement: 函数名识别
系统 SHALL 识别所有 22 个内置函数名，包括 SGN, INT, ABS, USR, FRE, POS, SQR, RND, LOG, EXP, COS, SIN, TAN, ATN, PEEK, LEN, STR$, VAL, ASC, CHR$, LEFT$, RIGHT$, MID$。

#### Scenario: 字符串函数带美元符号
- **WHEN** 输入 "LEFT$"
- **THEN** 返回 Token::Function("LEFT$")

### Requirement: 数字常量解析
系统 SHALL 解析整数和浮点数常量，支持科学计数法。

#### Scenario: 整数
- **WHEN** 输入 "42"
- **THEN** 返回 Token::Number(42.0)

#### Scenario: 浮点数
- **WHEN** 输入 "3.14159"
- **THEN** 返回 Token::Number(3.14159)

#### Scenario: 科学计数法
- **WHEN** 输入 "1.5E-10"
- **THEN** 返回 Token::Number(1.5e-10)

#### Scenario: 负数
- **WHEN** 输入 "-123"
- **THEN** 返回 Token::Minus 和 Token::Number(123)

### Requirement: 字符串常量解析
系统 SHALL 解析双引号括起的字符串常量，支持空字符串。

#### Scenario: 普通字符串
- **WHEN** 输入 `"HELLO WORLD"`
- **THEN** 返回 Token::String("HELLO WORLD")

#### Scenario: 空字符串
- **WHEN** 输入 `""`
- **THEN** 返回 Token::String("")

#### Scenario: 包含空格的字符串
- **WHEN** 输入 `"  SPACES  "`
- **THEN** 返回 Token::String("  SPACES  ")

### Requirement: 变量名识别
系统 SHALL 识别变量名，支持单字母、字母+数字组合，字符串变量以 $ 结尾。

#### Scenario: 单字母变量
- **WHEN** 输入 "A"
- **THEN** 返回 Token::Identifier("A")

#### Scenario: 字母数字组合
- **WHEN** 输入 "A1", "X9", "Z0"
- **THEN** 分别返回对应的 Identifier

#### Scenario: 字符串变量
- **WHEN** 输入 "A$", "NAME$"
- **THEN** 返回 Token::Identifier("A$"), Token::Identifier("NAME$")

#### Scenario: 数组变量
- **WHEN** 输入 "A(5)"
- **THEN** 返回 Token::Identifier("A"), Token::LeftParen, Token::Number(5), Token::RightParen

### Requirement: 运算符识别
系统 SHALL 识别所有运算符，包括算术运算符 (+, -, *, /, ^)、关系运算符 (=, <>, <, >, <=, >=) 和逻辑运算符 (AND, OR, NOT)。

#### Scenario: 算术运算符
- **WHEN** 输入 "+", "-", "*", "/", "^"
- **THEN** 分别返回对应的运算符 token

#### Scenario: 关系运算符
- **WHEN** 输入 "<=", ">=", "<>"
- **THEN** 返回 Token::LessEqual, Token::GreaterEqual, Token::NotEqual

#### Scenario: 逻辑运算符
- **WHEN** 输入 "AND", "OR", "NOT"
- **THEN** 返回对应的逻辑运算符 token

### Requirement: 行号处理
系统 SHALL 识别行号（1-65535 范围的整数），位于行首时作为行号处理。

#### Scenario: 有效行号
- **WHEN** 输入 "10 PRINT"
- **THEN** 首个 token 为 Token::LineNumber(10)

#### Scenario: 行号范围
- **WHEN** 输入行号 1, 100, 65535
- **THEN** 均作为有效行号识别

### Requirement: 空格和分隔符处理
系统 SHALL 正确处理空格、制表符作为分隔符，逗号和分号作为输出分隔符，冒号作为语句分隔符。

#### Scenario: 空格分隔
- **WHEN** 输入 "PRINT A"
- **THEN** 返回 Token::Print, Token::Identifier("A")

#### Scenario: 多个空格
- **WHEN** 输入 "PRINT   A"
- **THEN** 多余空格被忽略

#### Scenario: 逗号分隔（PRINT 语句）
- **WHEN** 输入 "PRINT A,B"
- **THEN** 返回 Token::Print, Token::Identifier("A"), Token::Comma, Token::Identifier("B")

#### Scenario: 分号（PRINT 紧密连接）
- **WHEN** 输入 "PRINT A;B"
- **THEN** 返回 Token::Print, Token::Identifier("A"), Token::Semicolon, Token::Identifier("B")

#### Scenario: 冒号（语句分隔符）
- **WHEN** 输入 "A=1: B=2"
- **THEN** 返回 Token::Identifier("A"), Token::Equal, Token::Number(1), Token::Colon, Token::Identifier("B"), Token::Equal, Token::Number(2)

#### Scenario: 复杂语句分隔
- **WHEN** 输入 "FOR I=1 TO 10: PRINT I: NEXT I"
- **THEN** 冒号正确分隔三个语句

### Requirement: 注释处理
系统 SHALL 识别 REM 语句，REM 之后到行尾的所有内容作为注释忽略。

#### Scenario: REM 注释
- **WHEN** 输入 "10 REM THIS IS A COMMENT"
- **THEN** REM 之后的文本被忽略

#### Scenario: 单引号注释（可选）
- **WHEN** 输入 "10 PRINT A ' COMMENT"
- **THEN** 单引号之后的内容被忽略

### Requirement: 错误处理
系统 SHALL 对非法字符和格式错误给出清晰的错误消息。

#### Scenario: 非法字符
- **WHEN** 输入包含非 ASCII 字符（除字符串内）
- **THEN** 返回 TokenError::IllegalCharacter

#### Scenario: 未闭合字符串
- **WHEN** 输入 `"HELLO` 缺少闭合引号
- **THEN** 返回 TokenError::UnterminatedString

#### Scenario: 数字格式错误
- **WHEN** 输入 "1.2.3"
- **THEN** 返回 TokenError::InvalidNumber


```

现在让我检查并补充 `tasks.md` 中关于单行多语句解析的任务：

```markdown
# Implementation Tasks

## 1. 项目初始化
- [ ] 1.1 创建 Cargo 项目结构
- [ ] 1.2 配置 Cargo.toml 依赖
- [ ] 1.3 设置基础错误类型和结果类型
- [ ] 1.4 创建项目 README
- [ ] 1.5 设置测试框架

## 2. 词法分析器 (Tokenizer)
- [ ] 2.1 定义 Token 枚举类型
- [ ] 2.2 定义所有保留字 (27个语句关键字)
- [ ] 2.3 定义所有函数名 (22个内置函数)
- [ ] 2.4 定义运算符 token
- [ ] 2.5 实现数字常量识别 (整数和浮点数)
- [ ] 2.6 实现标识符识别 (变量名、行号)
- [ ] 2.7 实现字符串常量识别
- [ ] 2.8 处理空格和注释
- [ ] 2.9 实现行号解析
- [ ] 2.10 编写词法分析器单元测试

## 3. 语法解析器 (Parser)
- [ ] 3.1 定义抽象语法树 (AST) 数据结构
- [ ] 3.2 实现表达式解析 (递归下降或优先级攀升)
- [ ] 3.3 实现语句解析器基础框架
- [ ] 3.4 解析简单语句 (LET, PRINT, REM, END)
- [ ] 3.5 解析流程控制语句 (GOTO, IF)
- [ ] 3.6 解析循环语句 (FOR...NEXT)
- [ ] 3.7 解析子程序调用 (GOSUB, RETURN)
- [ ] 3.8 解析数组和函数调用
- [ ] 3.9 实现冒号分隔的单行多语句解析
- [ ] 3.10 实现单行 FOR 循环（FOR...NEXT 在一行内）
- [ ] 3.11 实现单行嵌套 FOR 循环支持
- [ ] 3.12 实现错误恢复机制
- [ ] 3.13 编写解析器单元测试（包括单行多语句）

## 4. 运行时环境 (Runtime)
- [ ] 4.1 定义程序状态结构
- [ ] 4.2 实现程序行存储和索引
- [ ] 4.3 实现程序计数器和行号跳转
- [ ] 4.4 实现 GOSUB 栈管理
- [ ] 4.5 实现 FOR 循环栈管理
- [ ] 4.6 实现直接模式 vs 程序模式
- [ ] 4.7 实现 NEW 命令 (清空程序)
- [ ] 4.8 实现 LIST 命令 (列出程序)
- [ ] 4.9 实现 RUN 命令
- [ ] 4.10 实现 STOP, END, CONT 命令
- [ ] 4.11 编写运行时单元测试

## 5. 变量系统 (Variables)
- [ ] 5.1 定义变量值类型 (整数、浮点、字符串)
- [ ] 5.2 实现变量名解析 (A-Z, A0-Z9, A-Z$)
- [ ] 5.3 实现简单变量存储 (HashMap)
- [ ] 5.4 实现数组变量存储
- [ ] 5.5 实现 DIM 语句
- [ ] 5.6 实现数组边界检查
- [ ] 5.7 实现多维数组支持
- [ ] 5.8 实现变量类型检查
- [ ] 5.9 实现 CLEAR 命令 (清空变量)
- [ ] 5.10 编写变量系统单元测试

## 6. 基础语句实现
- [ ] 6.1 LET 语句 (赋值)
- [ ] 6.2 PRINT 语句 (基础输出)
- [ ] 6.3 PRINT 使用分隔符 (逗号、分号)
- [ ] 6.4 PRINT TAB() 和 SPC() 函数
- [ ] 6.5 REM 语句 (注释)
- [ ] 6.6 GOTO 语句
- [ ] 6.7 IF...THEN 语句
- [ ] 6.8 GOSUB 和 RETURN 语句
- [ ] 6.9 ON...GOTO 和 ON...GOSUB 语句
- [ ] 6.10 FOR...NEXT 语句
- [ ] 6.11 FOR...STEP 支持
- [ ] 6.12 测试所有基础语句

## 7. 运算符实现
- [ ] 7.1 算术运算符 (+, -, *, /)
- [ ] 7.2 乘方运算符 (^)
- [ ] 7.3 一元负号 (-)
- [ ] 7.4 关系运算符 (=, <>, <, >, <=, >=)
- [ ] 7.5 逻辑运算符 (AND, OR, NOT)
- [ ] 7.6 字符串连接 (+)
- [ ] 7.7 运算符优先级处理
- [ ] 7.8 类型转换和检查
- [ ] 7.9 测试所有运算符

## 8. 浮点运算 (Float Arithmetic)
- [ ] 8.1 选择浮点数表示 (f64 或自定义)
- [ ] 8.2 实现加法和减法
- [ ] 8.3 实现乘法和除法
- [ ] 8.4 实现乘方运算
- [ ] 8.5 实现数值比较
- [ ] 8.6 实现整数和浮点转换
- [ ] 8.7 实现浮点数输入解析
- [ ] 8.8 实现浮点数输出格式化
- [ ] 8.9 处理溢出和下溢
- [ ] 8.10 测试浮点运算精度

## 9. 数学函数
- [ ] 9.1 SGN(x) - 符号函数
- [ ] 9.2 INT(x) - 取整函数
- [ ] 9.3 ABS(x) - 绝对值
- [ ] 9.4 SQR(x) - 平方根
- [ ] 9.5 EXP(x) - 指数函数
- [ ] 9.6 LOG(x) - 自然对数
- [ ] 9.7 SIN(x), COS(x), TAN(x) - 三角函数
- [ ] 9.8 ATN(x) - 反正切
- [ ] 9.9 RND(x) - 随机数
- [ ] 9.10 测试所有数学函数

## 10. 字符串功能
- [ ] 10.1 字符串变量存储
- [ ] 10.2 字符串连接操作
- [ ] 10.3 LEN(s$) - 字符串长度
- [ ] 10.4 LEFT$(s$, n) - 左子串
- [ ] 10.5 RIGHT$(s$, n) - 右子串
- [ ] 10.6 MID$(s$, n[, m]) - 中间子串
- [ ] 10.7 STR$(x) - 数字转字符串
- [ ] 10.8 VAL(s$) - 字符串转数字
- [ ] 10.9 ASC(s$) - 字符 ASCII 码
- [ ] 10.10 CHR$(x) - ASCII 码转字符
- [ ] 10.11 字符串比较
- [ ] 10.12 测试所有字符串函数

## 11. 输入输出 (I/O)
- [ ] 11.1 实现基础 INPUT 语句
- [ ] 11.2 INPUT 提示符支持
- [ ] 11.3 INPUT 多个变量
- [ ] 11.4 INPUT 错误处理和重试
- [ ] 11.5 实现 DATA 语句
- [ ] 11.6 实现 READ 语句
- [ ] 11.7 实现 RESTORE 语句
- [ ] 11.8 集成 rustyline 行编辑
- [ ] 11.9 实现命令历史
- [ ] 11.10 测试 I/O 功能

## 12. 系统函数
- [ ] 12.1 FRE(x) - 剩余内存
- [ ] 12.2 POS(x) - 光标位置
- [ ] 12.3 PEEK(addr) - 读内存 (模拟)
- [ ] 12.4 POKE addr, value - 写内存 (模拟)
- [ ] 12.5 WAIT 语句
- [ ] 12.6 测试系统函数

## 13. 用户自定义函数
- [ ] 13.1 DEF FN 语句解析
- [ ] 13.2 单行函数定义
- [ ] 13.3 函数调用 FN name(arg)
- [ ] 13.4 函数作用域管理
- [ ] 13.5 测试用户定义函数

## 14. 错误处理
- [ ] 14.1 定义所有错误类型
- [ ] 14.2 实现错误消息 (中文/英文)
- [ ] 14.3 语法错误报告 (行号、位置)
- [ ] 14.4 运行时错误处理
- [ ] 14.5 STOP 和 END 的区别
- [ ] 14.6 CONT 命令恢复执行
- [ ] 14.7 实现 Ctrl+C 中断处理（捕获信号）
- [ ] 14.8 Ctrl+C 中断后保存执行状态（行号、位置）
- [ ] 14.9 确保 CONT 可以从 Ctrl+C 中断点继续执行
- [ ] 14.10 测试 Ctrl+C 中断和恢复流程
- [ ] 14.11 测试其他错误处理场景

## 15. 交互模式
- [ ] 15.1 REPL 主循环
- [ ] 15.2 直接模式命令执行
- [ ] 15.3 程序编辑 (输入行号)
- [ ] 15.4 行删除 (输入空行)
- [ ] 15.5 LIST 命令变体 (LIST 10-50)
- [ ] 15.6 启动横幅和提示符
- [ ] 15.7 退出命令
- [ ] 15.8 测试交互模式

## 16. 高级功能 (可选)
- [ ] 16.1 LOAD 命令 (从文件加载程序)
- [ ] 16.2 SAVE 命令 (保存程序到文件)
- [ ] 16.3 GET 语句 (单字符输入)
- [ ] 16.4 NULL 语句
- [ ] 16.5 CMD 和 SYS 语句
- [ ] 16.6 文件格式定义
- [ ] 16.7 测试文件操作

## 17. 集成测试
- [ ] 17.1 简单程序测试 (Hello World)
- [ ] 17.2 循环测试 (求和、阶乘)
- [ ] 17.3 数组测试 (排序、搜索)
- [ ] 17.4 字符串处理测试
- [ ] 17.5 子程序测试
- [ ] 17.6 递归函数测试
- [ ] 17.7 经典游戏测试 (猜数字、星际迷航)
- [ ] 17.8 数学计算测试
- [ ] 17.9 综合测试程序
- [ ] 17.10 性能基准测试

## 18. 文档和完善
- [ ] 18.1 完善代码注释
- [ ] 18.2 编写用户手册
- [ ] 18.3 编写开发者文档
- [ ] 18.4 添加使用示例
- [ ] 18.5 优化错误消息
- [ ] 18.6 代码重构和清理
- [ ] 18.7 性能分析和优化
- [ ] 18.8 最终测试

## 19. 部署和发布
- [ ] 19.1 配置 CI/CD
- [ ] 19.2 创建发布版本
- [ ] 19.3 编写 CHANGELOG
- [ ] 19.4 准备示例程序
- [ ] 19.5 发布第一个版本


```

```markdown
# Implementation Tasks

## 1. 项目初始化
- [ ] 1.1 创建 Cargo 项目结构
- [ ] 1.2 配置 Cargo.toml 依赖
- [ ] 1.3 设置基础错误类型和结果类型
- [ ] 1.4 创建项目 README
- [ ] 1.5 设置测试框架

## 2. 词法分析器 (Tokenizer)
- [ ] 2.1 定义 Token 枚举类型
- [ ] 2.2 定义所有保留字 (27个语句关键字)
- [ ] 2.3 定义所有函数名 (22个内置函数)
- [ ] 2.4 定义运算符 token
- [ ] 2.5 定义分隔符 token (冒号、逗号、分号)
- [ ] 2.6 实现数字常量识别 (整数和浮点数)
- [ ] 2.7 实现标识符识别 (变量名、行号)
- [ ] 2.8 实现字符串常量识别
- [ ] 2.9 处理空格和注释
- [ ] 2.10 实现行号解析
- [ ] 2.11 实现冒号作为语句分隔符的识别
- [ ] 2.12 编写词法分析器单元测试（包括分隔符）

## 3. 语法解析器 (Parser)
- [ ] 3.1 定义抽象语法树 (AST) 数据结构
- [ ] 3.2 实现表达式解析 (递归下降或优先级攀升)
- [ ] 3.3 实现语句解析器基础框架
- [ ] 3.4 解析简单语句 (LET, PRINT, REM, END)
- [ ] 3.5 解析流程控制语句 (GOTO, IF)
- [ ] 3.6 解析循环语句 (FOR...NEXT)
- [ ] 3.7 解析子程序调用 (GOSUB, RETURN)
- [ ] 3.8 解析数组和函数调用
- [ ] 3.9 实现冒号分隔的单行多语句解析
- [ ] 3.10 实现单行 FOR 循环（FOR...NEXT 在一行内）
- [ ] 3.11 实现单行嵌套 FOR 循环支持
- [ ] 3.12 实现错误恢复机制
- [ ] 3.13 编写解析器单元测试（包括单行多语句）

## 4. 运行时环境 (Runtime)
- [ ] 4.1 定义程序状态结构
- [ ] 4.2 实现程序行存储和索引
- [ ] 4.3 实现程序计数器和行号跳转
- [ ] 4.4 实现 GOSUB 栈管理
- [ ] 4.5 实现 FOR 循环栈管理
- [ ] 4.6 实现直接模式 vs 程序模式
- [ ] 4.7 实现 NEW 命令 (清空程序)
- [ ] 4.8 实现 LIST 命令 (列出程序)
- [ ] 4.9 实现 RUN 命令
- [ ] 4.10 实现 STOP, END, CONT 命令
- [ ] 4.11 编写运行时单元测试

## 5. 变量系统 (Variables)
- [ ] 5.1 定义变量值类型 (整数、浮点、字符串)
- [ ] 5.2 实现变量名解析 (A-Z, A0-Z9, A-Z$)
- [ ] 5.3 实现简单变量存储 (HashMap)
- [ ] 5.4 实现数组变量存储
- [ ] 5.5 实现 DIM 语句
- [ ] 5.6 实现数组边界检查
- [ ] 5.7 实现多维数组支持
- [ ] 5.8 实现变量类型检查
- [ ] 5.9 实现 CLEAR 命令 (清空变量)
- [ ] 5.10 编写变量系统单元测试

## 6. 基础语句实现
- [ ] 6.1 LET 语句 (赋值)
- [ ] 6.2 PRINT 语句 (基础输出)
- [ ] 6.3 PRINT 使用分隔符 (逗号、分号)
- [ ] 6.4 PRINT TAB() 和 SPC() 函数
- [ ] 6.5 REM 语句 (注释)
- [ ] 6.6 GOTO 语句
- [ ] 6.7 IF...THEN 语句
- [ ] 6.8 GOSUB 和 RETURN 语句
- [ ] 6.9 ON...GOTO 和 ON...GOSUB 语句
- [ ] 6.10 FOR...NEXT 语句
- [ ] 6.11 FOR...STEP 支持
- [ ] 6.12 测试所有基础语句

## 7. 运算符实现
- [ ] 7.1 算术运算符 (+, -, *, /)
- [ ] 7.2 乘方运算符 (^)
- [ ] 7.3 一元负号 (-)
- [ ] 7.4 关系运算符 (=, <>, <, >, <=, >=)
- [ ] 7.5 逻辑运算符 (AND, OR, NOT)
- [ ] 7.6 字符串连接 (+)
- [ ] 7.7 运算符优先级处理
- [ ] 7.8 类型转换和检查
- [ ] 7.9 测试所有运算符

## 8. 浮点运算 (Float Arithmetic)
- [ ] 8.1 选择浮点数表示 (f64 或自定义)
- [ ] 8.2 实现加法和减法
- [ ] 8.3 实现乘法和除法
- [ ] 8.4 实现乘方运算
- [ ] 8.5 实现数值比较
- [ ] 8.6 实现整数和浮点转换
- [ ] 8.7 实现浮点数输入解析
- [ ] 8.8 实现浮点数输出格式化
- [ ] 8.9 处理溢出和下溢
- [ ] 8.10 测试浮点运算精度

## 9. 数学函数
- [ ] 9.1 SGN(x) - 符号函数
- [ ] 9.2 INT(x) - 取整函数
- [ ] 9.3 ABS(x) - 绝对值
- [ ] 9.4 SQR(x) - 平方根
- [ ] 9.5 EXP(x) - 指数函数
- [ ] 9.6 LOG(x) - 自然对数
- [ ] 9.7 SIN(x), COS(x), TAN(x) - 三角函数
- [ ] 9.8 ATN(x) - 反正切
- [ ] 9.9 RND(x) - 随机数
- [ ] 9.10 测试所有数学函数

## 10. 字符串功能
- [ ] 10.1 字符串变量存储
- [ ] 10.2 字符串连接操作
- [ ] 10.3 LEN(s$) - 字符串长度
- [ ] 10.4 LEFT$(s$, n) - 左子串
- [ ] 10.5 RIGHT$(s$, n) - 右子串
- [ ] 10.6 MID$(s$, n[, m]) - 中间子串
- [ ] 10.7 STR$(x) - 数字转字符串
- [ ] 10.8 VAL(s$) - 字符串转数字
- [ ] 10.9 ASC(s$) - 字符 ASCII 码
- [ ] 10.10 CHR$(x) - ASCII 码转字符
- [ ] 10.11 字符串比较
- [ ] 10.12 测试所有字符串函数

## 11. 输入输出 (I/O)
- [ ] 11.1 实现基础 INPUT 语句
- [ ] 11.2 INPUT 提示符支持
- [ ] 11.3 INPUT 多个变量
- [ ] 11.4 INPUT 错误处理和重试
- [ ] 11.5 实现 DATA 语句
- [ ] 11.6 实现 READ 语句
- [ ] 11.7 实现 RESTORE 语句
- [ ] 11.8 集成 rustyline 行编辑
- [ ] 11.9 实现命令历史
- [ ] 11.10 测试 I/O 功能

## 12. 系统函数
- [ ] 12.1 FRE(x) - 剩余内存
- [ ] 12.2 POS(x) - 光标位置
- [ ] 12.3 PEEK(addr) - 读内存 (模拟)
- [ ] 12.4 POKE addr, value - 写内存 (模拟)
- [ ] 12.5 WAIT 语句
- [ ] 12.6 测试系统函数

## 13. 用户自定义函数
- [ ] 13.1 DEF FN 语句解析
- [ ] 13.2 单行函数定义
- [ ] 13.3 函数调用 FN name(arg)
- [ ] 13.4 函数作用域管理
- [ ] 13.5 测试用户定义函数

## 14. 错误处理
- [ ] 14.1 定义所有错误类型
- [ ] 14.2 实现错误消息 (中文/英文)
- [ ] 14.3 语法错误报告 (行号、位置)
- [ ] 14.4 运行时错误处理
- [ ] 14.5 STOP 和 END 的区别
- [ ] 14.6 CONT 命令恢复执行
- [ ] 14.7 实现 Ctrl+C 中断处理（捕获信号）
- [ ] 14.8 Ctrl+C 中断后保存执行状态（行号、位置）
- [ ] 14.9 确保 CONT 可以从 Ctrl+C 中断点继续执行
- [ ] 14.10 测试 Ctrl+C 中断和恢复流程
- [ ] 14.11 测试其他错误处理场景

## 15. 交互模式
- [ ] 15.1 REPL 主循环
- [ ] 15.2 直接模式命令执行
- [ ] 15.3 程序编辑 (输入行号)
- [ ] 15.4 行删除 (输入空行)
- [ ] 15.5 LIST 命令变体 (LIST 10-50)
- [ ] 15.6 启动横幅和提示符
- [ ] 15.7 退出命令
- [ ] 15.8 测试交互模式

## 16. 高级功能 (可选)
- [ ] 16.1 LOAD 命令 (从文件加载程序)
- [ ] 16.2 SAVE 命令 (保存程序到文件)
- [ ] 16.3 GET 语句 (单字符输入)
- [ ] 16.4 NULL 语句
- [ ] 16.5 CMD 和 SYS 语句
- [ ] 16.6 文件格式定义
- [ ] 16.7 测试文件操作

## 17. 集成测试
- [ ] 17.1 简单程序测试 (Hello World)
- [ ] 17.2 循环测试 (求和、阶乘)
- [ ] 17.3 数组测试 (排序、搜索)
- [ ] 17.4 字符串处理测试
- [ ] 17.5 子程序测试
- [ ] 17.6 递归函数测试
- [ ] 17.7 经典游戏测试 (猜数字、星际迷航)
- [ ] 17.8 数学计算测试
- [ ] 17.9 综合测试程序
- [ ] 17.10 性能基准测试

## 18. 文档和完善
- [ ] 18.1 完善代码注释
- [ ] 18.2 编写用户手册
- [ ] 18.3 编写开发者文档
- [ ] 18.4 添加使用示例
- [ ] 18.5 优化错误消息
- [ ] 18.6 代码重构和清理
- [ ] 18.7 性能分析和优化
- [ ] 18.8 最终测试

## 19. 部署和发布
- [ ] 19.1 配置 CI/CD
- [ ] 19.2 创建发布版本
- [ ] 19.3 编写 CHANGELOG
- [ ] 19.4 准备示例程序
- [ ] 19.5 发布第一个版本


```

现在让我在 `design.md` 中补充关于单行多语句的设计说明：

```markdown
# Technical Design Document

## Context

重新实现 Microsoft BASIC 6502 解释器是一个复杂的系统工程项目。原始的汇编代码约 6800 行，包含了完整的词法分析、语法解析、运行时环境、内存管理和各种 BASIC 语言特性。

**约束条件**:
- 保持与原 BASIC 6502 的语义兼容性
- 使用 Rust 语言的最佳实践
- 最小化外部依赖
- 单文件模块，每个模块 300-500 行
- 优先考虑代码清晰度而非性能

**利益相关者**:
- 开发者（单人项目，但代码需要可维护）
- 用户（希望运行经典 BASIC 程序）

## Goals / Non-Goals

### Goals
- 实现完整的 BASIC 6502 核心功能
- 代码清晰、模块化、易于理解
- 完整的单元测试覆盖
- 能运行经典的 BASIC 程序
- 提供良好的错误消息

### Non-Goals
- 性能优化到极致（合理即可）
- 向后兼容所有历史平台的特定功能
- 实现 BASIC 扩展或现代特性
- 图形界面（仅命令行）
- 多线程支持

## Decisions

### 1. 架构模式：经典三阶段解释器

**决策**: 采用 Tokenizer → Parser → Runtime 的三阶段架构

**理由**:
- 职责清晰分离，易于理解和维护
- 符合编译器设计的标准模式
- 便于独立测试各个阶段
- 与原汇编代码的结构相对应

**替代方案**:
- 直接解释执行（边读边执行）：不利于错误检查和跳转
- 编译为字节码：增加复杂度，对 BASIC 来说收益不大

### 2. AST vs 中间表示

**决策**: 使用轻量级的中间表示，而非完整的 AST

**理由**:
- BASIC 语言结构简单，不需要复杂的 AST
- 行号驱动的执行模式更适合线性表示
- 节省内存，符合原 6502 的精神
- 更接近原汇编实现的方式

**实现**:
```rust
// 每行程序存储为：
struct ProgramLine {
    line_number: u16,
    tokens: Vec<Token>,  // 预处理的 token 序列
}

// 程序存储在 BTreeMap 中，按行号排序
type Program = BTreeMap<u16, ProgramLine>;
```

### 3. 变量存储策略

**决策**: 使用 HashMap 存储变量，区分数值和字符串

**理由**:
- Rust 的 HashMap 性能优秀
- 支持任意变量名（不限于 26 个字母）
- 类型安全（编译时保证）
- 易于实现和调试

**数据结构**:
```rust
enum Value {
    Integer(i32),
    Float(f64),
    String(String),
    Array(Vec<Value>),  // 多维数组展平存储
}

struct Variables {
    simple: HashMap<String, Value>,
    arrays: HashMap<String, Array>,
}

struct Array {
    dimensions: Vec<usize>,
    data: Vec<Value>,
}
```

**替代方案**:
- 单字母变量用数组（A-Z）：不够灵活，不支持 A1, B2 等
- 全局符号表：不利于作用域管理（DEF FN）

### 4. 浮点数实现

**决策**: 使用 Rust 标准的 f64

**理由**:
- 简单直接，无需重新实现浮点运算
- 精度更高（原 6502 使用 5 字节浮点）
- 与 Rust 生态系统兼容
- 标准数学函数直接可用

**注意事项**:
- 可能与原 BASIC 的精度有细微差异
- 需要在测试中注明这一点
- 输出格式需要模拟原 BASIC 的风格

**替代方案**:
- 实现 5 字节 MS-BASIC 浮点格式：过于复杂，收益低
- 使用 Decimal 类型：不适合科学计算

### 5. 字符串管理

**决策**: 使用 Rust String，不做特殊的字符串池

**理由**:
- Rust 的 String 已经很高效
- 自动内存管理，避免内存泄漏
- 不需要模拟 6502 的字符串空间管理

**实现**:
- 字符串变量直接存储 String
- 字符串操作返回新的 String（写时复制）
- 临时字符串由 Rust 自动回收

### 6. 流程控制实现

**决策**: 使用栈来管理 GOSUB 和 FOR 循环

**理由**:
- 符合原汇编实现的方式
- 支持嵌套调用和循环
- 易于实现 RETURN 和 NEXT 的匹配检查

**数据结构**:
```rust
enum CallFrame {
    Gosub { return_line: u16, return_pos: usize },
    For { 
        var_name: String,
        end_value: f64,
        step: f64,
        loop_start: (u16, usize),
    },
}

struct Runtime {
    call_stack: Vec<CallFrame>,
    // ...
}
```

### 7. 错误处理策略

**决策**: 使用 Result 类型，自定义错误枚举

**理由**:
- 符合 Rust 的惯用法
- 强制错误处理
- 易于传播和转换错误

**实现**:
```rust
#[derive(Debug)]
enum BasicError {
    SyntaxError(String),
    TypeError(String),
    RuntimeError(String),
    UndefinedLine(u16),
    UndefinedVariable(String),
    DivisionByZero,
    OutOfMemory,
    // ... 对应原 BASIC 的各种错误
}

type Result<T> = std::result::Result<T, BasicError>;
```

### 8. 交互模式：选择 rustyline

**决策**: 使用 rustyline crate 实现 REPL

**理由**:
- 提供现代的行编辑功能（光标移动、删除）
- 命令历史记录
- 跨平台支持
- 轻量级，单一依赖

**替代方案**:
- 标准输入（std::io::stdin）：功能太基础，用户体验差
- readline 系统库：需要 FFI，跨平台问题

### 9. 测试策略

**决策**: 单元测试 + 集成测试 + 黄金测试

**单元测试**:
- 每个模块独立测试
- 覆盖边界情况

**集成测试**:
- 完整的 BASIC 程序测试
- 放在 `tests/` 目录

**黄金测试** (Golden Tests):
- 运行 BASIC 程序，比较输出
- 测试文件: `tests/basic_programs/*.bas`
- 期望输出: `tests/expected/*.txt`

**实现**:
```rust
#[test]
fn test_program_output() {
    let program = load_program("tests/basic_programs/hello.bas");
    let output = run_program(program);
    let expected = read_file("tests/expected/hello.txt");
    assert_eq!(output, expected);
}
```

### 10. 模块组织

**决策**: 按功能模块划分，每个模块 300-500 行

**结构**:
```
src/
├── main.rs           # 入口点，REPL
├── error.rs          # 错误定义 (~100 行)
├── token.rs          # Token 定义 (~150 行)
├── tokenizer.rs      # 词法分析 (~400 行)
├── parser.rs         # 语法分析 (~500 行)
├── runtime.rs        # 运行时环境 (~500 行)
├── variables.rs      # 变量管理 (~300 行)
├── statements/       # 语句实现
│   ├── mod.rs
│   ├── control.rs    # GOTO, IF, FOR, etc.
│   ├── io.rs         # PRINT, INPUT, etc.
│   └── data.rs       # LET, DIM, DATA, etc.
├── functions/        # 函数实现
│   ├── mod.rs
│   ├── math.rs       # 数学函数
│   ├── string.rs     # 字符串函数
│   └── system.rs     # 系统函数
├── operators.rs      # 运算符 (~300 行)
├── float.rs          # 浮点运算辅助 (~200 行)
└── memory.rs         # 内存管理辅助 (~200 行)
```

### 11. 单行多语句解析

**决策**: 支持冒号分隔的单行多语句，包括完整的 FOR...NEXT 循环

**理由**:
- 这是原 BASIC 6502 的核心特性
- 极大提高代码紧凑性和可读性
- 符合用户对经典 BASIC 的使用习惯
- 允许在一行内完成简单循环

**实现策略**:
```rust
// Tokenizer: 识别冒号为语句分隔符
Token::Colon  // 语句分隔符

// Parser: 按冒号分割一行为多条语句
fn parse_line(tokens: &[Token]) -> Vec<Statement> {
    let mut statements = vec![];
    let parts = split_by_colon(tokens);
    for part in parts {
        statements.push(parse_statement(part));
    }
    statements
}

// 特殊处理 FOR 循环
// FOR I=1 TO 10: PRINT I: NEXT I
// 需要正确匹配 FOR 和 NEXT，即使在同一行
```

**关键场景**:
- 简单多语句：`A=1: B=2: PRINT A+B`
- 单行 FOR 循环：`FOR I=1 TO 10: PRINT I: NEXT I`
- 嵌套 FOR 循环：`FOR I=1 TO 3: FOR J=1 TO 3: PRINT I*J: NEXT J: NEXT I`
- 条件后多语句：`IF A>10 THEN A=0: PRINT A: GOTO 20`

**挑战**:
- 正确匹配 FOR 和 NEXT（需要栈来追踪）
- IF...THEN 后的多条语句处理
- 保持行号追踪和错误报告的准确性

**替代方案**:
- 禁止单行多语句：违背 BASIC 传统，用户体验差
- 仅支持简单语句，禁止 FOR 循环：限制过多

## Risks / Trade-offs

### 风险 1: 语义差异

**风险**: Rust 实现可能与原 6502 汇编有细微差异

**缓解**:
- 仔细对照原汇编代码注释
- 编写大量测试用例
- 在文档中注明已知差异

### 风险 2: 浮点精度

**风险**: f64 与原 5 字节浮点格式精度不同

**缓解**:
- 测试时使用容差比较
- 文档说明精度差异
- 必要时提供兼容模式

### 风险 3: 性能

**风险**: 解释执行可能比原汇编慢（或快很多）

**缓解**:
- 性能不是主要目标，合理即可
- 必要时进行 profiling 和优化
- 避免过早优化

### 风险 4: 范围蔓延

**风险**: 功能范围过大，难以完成

**缓解**:
- 严格按阶段实施
- 每个阶段都能运行和测试
- 可选功能放在后期

### Trade-off 1: 简洁 vs 性能

**选择**: 简洁优先

**理由**: 代码可维护性更重要，现代硬件性能足够

### Trade-off 2: 兼容性 vs 现代化

**选择**: 语义兼容，接口现代化

**理由**: 
- 保持 BASIC 程序可运行（兼容性）
- 使用现代行编辑、UTF-8（现代化）

## Migration Plan

N/A - 这是全新项目

## Implementation Order

1. **第 1 周**: 项目初始化、错误类型、Token 定义
2. **第 2-3 周**: 词法分析器、基础解析器
3. **第 4-5 周**: 运行时环境、基础语句 (LET, PRINT, GOTO)
4. **第 6-7 周**: 流程控制 (IF, FOR, GOSUB)、运算符
5. **第 8 周**: 浮点运算、数学函数
6. **第 9 周**: 字符串功能、数组
7. **第 10 周**: I/O 功能 (INPUT, DATA/READ)
8. **第 11 周**: 交互模式、用户定义函数
9. **第 12 周**: 集成测试、错误处理完善
10. **第 13-14 周**: 可选功能、文档、发布

## Open Questions

1. **是否支持行编辑中删除程序行？**
   - 答：支持，输入行号 + 空行即可删除

2. **错误消息使用中文还是英文？**
   - 答：优先中文，可配置

3. **是否实现 LOAD/SAVE？**
   - 答：基础版本不实现，作为可选功能下一个阶段实现

4. **是否支持多平台（Windows, Linux, macOS）？**
   - 答：是，Rust 天然跨平台

5. **如何处理无限循环？**
   - 答：提供 Ctrl+C 中断支持, cont之后要求可以在断点继续执行

## References

- Microsoft BASIC 6502 源代码: `m6502.asm`
- OpenSpec project.md: 项目约定和架构模式
- Rust 文档: https://doc.rust-lang.org/
- rustyline: https://docs.rs/rustyline/


```

完美！现在让我验证一下所有更新：

完美！✅ 所有更新已完成并通过验证。我已经全面补充了关于**冒号分隔的单行多语句**支持，包括**单行 FOR 循环**的规格和任务。

## 📝 更新总结

### 1️⃣ **tokenizer/spec.md** - 分隔符处理
✅ 更新了"空格和分隔符处理"需求：
- 明确区分：逗号和分号是输出分隔符，冒号是语句分隔符
- 新增场景：冒号作为语句分隔符
- 新增场景：复杂语句分隔（`FOR I=1 TO 10: PRINT I: NEXT I`）

### 2️⃣ **parser/spec.md** - 语句分隔符
✅ 扩展了"语句分隔符"需求，新增 6 个场景：
- ✅ 单行多语句：`A=1: B=2: PRINT A+B`
- ✅ **单行 FOR 循环**：`FOR I=1 TO 10: PRINT I: NEXT I`
- ✅ **单行嵌套 FOR 循环**：`FOR I=1 TO 3: FOR J=1 TO 3: PRINT I*J: NEXT J: NEXT I`
- ✅ 单行 IF...THEN 语句：`IF A>10 THEN A=0: PRINT A`
- ✅ 单行 GOSUB 和计算：`A=5: GOSUB 100: PRINT A`
- ✅ 复杂单行语句：`INPUT A: IF A>0 THEN B=A*2: PRINT B: GOTO 20`

### 3️⃣ **tasks.md** - 实施任务
✅ 在词法分析器（第2章）新增 2 个任务：
- **2.5** - 定义分隔符 token (冒号、逗号、分号)
- **2.11** - 实现冒号作为语句分隔符的识别

✅ 在语法解析器（第3章）新增 3 个任务：
- **3.9** - 实现冒号分隔的单行多语句解析
- **3.10** - **实现单行 FOR 循环（FOR...NEXT 在一行内）**
- **3.11** - **实现单行嵌套 FOR 循环支持**

### 4️⃣ **design.md** - 设计决策
✅ 新增**第11个设计决策**："单行多语句解析"
- 说明了为什么需要这个特性（BASIC 核心特性）
- 提供了实现策略（Tokenizer + Parser）
- 列出了关键场景和挑战
- 特别强调了 FOR...NEXT 匹配的复杂性

## 🎯 关键点

现在规格中明确要求支持：
1. **冒号分隔的多条语句**在一行内
2. **完整的 FOR...NEXT 循环**可以写在一行
3. **嵌套的 FOR 循环**也可以在一行内
4. **IF...THEN 后的多条语句**支持

这完全符合你的要求："冒号分割的，在一行之内的语句，包括在一行的FOR语句，需要支持！"

任务总数从 169 个增加到 **173 个**。所有更新已通过 OpenSpec 严格验证 ✓

