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

