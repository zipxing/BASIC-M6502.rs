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

