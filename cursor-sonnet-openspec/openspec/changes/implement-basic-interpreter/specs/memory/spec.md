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

