# 交互模式下打印问题
_Exported on 2025/10/29 at GMT+8 16:01:54 from Cursor (1.7.54)_

---

**User**

我现在进行人工测试，发现在交互模式下，我输入print 111回车，并没有打印出来111

---

**Cursor**

我来帮你调查这个问题。让我先看看当前的代码实现，特别是交互模式和PRINT语句的处理逻辑。

我找到问题了！在交互模式下，`PRINT` 语句的输出被保存到了内部缓冲区（用于测试），但没有真正打印到终端。

让我修改 `executor.rs` 中的输出逻辑，使其同时输出到终端和缓冲区：

```rust
/// 执行引擎
///
/// 求值表达式并执行语句

use crate::ast::*;
use crate::error::{BasicError, Result};
use crate::runtime::Runtime;
use crate::variables::{Value, Variables};

/// 输入回调函数类型
pub type InputCallback = Box<dyn FnMut(&str) -> Option<String>>;

/// 执行引擎
pub struct Executor {
    runtime: Runtime,
    variables: Variables,
    /// 输出缓冲区（用于测试和捕获输出）
    output_buffer: Vec<String>,
    /// 当前打印列位置
    print_column: usize,
    /// DATA 数据存储
    data_values: Vec<DataValue>,
    /// DATA 数据指针（当前读取位置）
    data_pointer: usize,
    /// 输入回调函数（用于测试）
    input_callback: Option<InputCallback>,
}

/// DATA 值类型
#[derive(Debug, Clone, PartialEq)]
pub enum DataValue {
    Number(f64),
    String(String),
}

impl Executor {
    /// 创建新的执行器
    pub fn new() -> Self {
        Executor {
            runtime: Runtime::new(),
            variables: Variables::new(),
            output_buffer: Vec::new(),
            print_column: 0,
            data_values: Vec::new(),
            data_pointer: 0,
            input_callback: None,
        }
    }
    
    /// 设置输入回调函数（用于测试）
    pub fn set_input_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&str) -> Option<String> + 'static,
    {
        self.input_callback = Some(Box::new(callback));
    }
    
    /// 添加 DATA 值
    pub fn add_data_value(&mut self, value: DataValue) {
        self.data_values.push(value);
    }
    
    /// 重置 DATA 指针
    pub fn restore_data(&mut self) {
        self.data_pointer = 0;
    }
    
    /// 读取下一个 DATA 值
    fn read_data_value(&mut self) -> Result<DataValue> {
        if self.data_pointer >= self.data_values.len() {
            return Err(BasicError::OutOfData);
        }
        
        let value = self.data_values[self.data_pointer].clone();
        self.data_pointer += 1;
        Ok(value)
    }
    
    /// 获取输出内容（用于测试）
    pub fn get_output(&self) -> String {
        self.output_buffer.join("")
    }
    
    /// 清空输出缓冲区
    pub fn clear_output(&mut self) {
        self.output_buffer.clear();
        self.print_column = 0;
    }
    
    /// 输出文本（添加到缓冲区并打印到终端）
    fn output(&mut self, text: &str) {
        // 打印到终端
        print!("{}", text);
        use std::io::Write;
        std::io::stdout().flush().ok();
        
        // 同时添加到缓冲区（用于测试）
        self.output_buffer.push(text.to_string());
        
        // 更新列位置
        for ch in text.chars() {
            if ch == '\n' {
                self.print_column = 0;
            } else {
                self.print_column += 1;
            }
        }
    }
    
    /// 输出换行
    fn output_newline(&mut self) {
        self.output("\n");
    }

    /// 获取运行时引用
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    /// 获取变量存储引用
    pub fn variables(&self) -> &Variables {
        &self.variables
    }

    /// 获取运行时可变引用
    pub fn runtime_mut(&mut self) -> &mut Runtime {
        &mut self.runtime
    }

    /// 获取变量存储可变引用
    pub fn variables_mut(&mut self) -> &mut Variables {
        &mut self.variables
    }

    /// 求值表达式
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            
            Expr::String(s) => Ok(Value::String(s.clone())),
            
            Expr::Variable(name) => {
                Ok(self.variables.get(name))
            }
            
            Expr::ArrayAccess { name, indices } => {
                // 求值所有索引
                let idx_values: Result<Vec<usize>> = indices.iter()
                    .map(|idx_expr| {
                        self.eval_expr(idx_expr)?
                            .as_number()
                            .and_then(|n| {
                                if n < 0.0 {
                                    Err(BasicError::SubscriptOutOfRange(
                                        "Negative array index".to_string()
                                    ))
                                } else {
                                    Ok(n as usize)
                                }
                            })
                    })
                    .collect();
                
                let indices_usize = idx_values?;
                self.variables.get_array_element(name, &indices_usize)
            }
            
            Expr::FunctionCall { name, args } => {
                self.eval_function_call(name, args)
            }
            
            Expr::BinaryOp { left, op, right } => {
                self.eval_binary_op(left, *op, right)
            }
            
            Expr::UnaryOp { op, operand } => {
                self.eval_unary_op(*op, operand)
            }
        }
    }

    /// 求值二元运算
    fn eval_binary_op(&mut self, left: &Expr, op: BinaryOperator, right: &Expr) -> Result<Value> {
        use BinaryOperator::*;

        let left_val = self.eval_expr(left)?;
        let right_val = self.eval_expr(right)?;

        match op {
            // 算术运算符
            Add => {
                if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    Ok(Value::Number(l + r))
                } else if left_val.is_string() && right_val.is_string() {
                    // 字符串连接
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    Ok(Value::String(format!("{}{}", l, r)))
                } else {
                    Err(BasicError::TypeMismatch(
                        "Cannot add incompatible types".to_string()
                    ))
                }
            }
            
            Subtract => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l - r))
            }
            
            Multiply => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l * r))
            }
            
            Divide => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                if r == 0.0 {
                    return Err(BasicError::DivisionByZero);
                }
                Ok(Value::Number(l / r))
            }
            
            Power => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l.powf(r)))
            }
            
            // 关系运算符（BASIC 中 true = -1, false = 0）
            Equal => {
                let result = if left_val == right_val { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            NotEqual => {
                let result = if left_val != right_val { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            Less => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l < r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l < r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            Greater => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l > r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l > r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            LessEqual => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l <= r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l <= r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            GreaterEqual => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l >= r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l >= r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            // 逻辑运算符（按位）
            And => {
                let l = left_val.as_number()? as i32;
                let r = right_val.as_number()? as i32;
                Ok(Value::Number((l & r) as f64))
            }
            
            Or => {
                let l = left_val.as_number()? as i32;
                let r = right_val.as_number()? as i32;
                Ok(Value::Number((l | r) as f64))
            }
        }
    }

    /// 求值一元运算
    fn eval_unary_op(&mut self, op: UnaryOperator, operand: &Expr) -> Result<Value> {
        let val = self.eval_expr(operand)?;
        
        match op {
            UnaryOperator::Minus => {
                let n = val.as_number()?;
                Ok(Value::Number(-n))
            }
            UnaryOperator::Not => {
                let n = val.as_number()? as i32;
                Ok(Value::Number((!n) as f64))
            }
        }
    }

    /// 求值函数调用（内置函数）
    fn eval_function_call(&mut self, name: &str, args: &[Expr]) -> Result<Value> {
        match name.to_uppercase().as_str() {
            // 数学函数
            "SGN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SGN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                let result = if n > 0.0 { 1.0 } else if n < 0.0 { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            "INT" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("INT requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.floor()))
            }
            
            "ABS" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ABS requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.abs()))
            }
            
            "SQR" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SQR requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n < 0.0 {
                    return Err(BasicError::IllegalQuantity("SQR of negative number".to_string()));
                }
                Ok(Value::Number(n.sqrt()))
            }
            
            "SIN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SIN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.sin()))
            }
            
            "COS" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("COS requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.cos()))
            }
            
            "TAN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("TAN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.tan()))
            }
            
            "ATN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ATN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.atan()))
            }
            
            "LOG" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("LOG requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n <= 0.0 {
                    return Err(BasicError::IllegalQuantity("LOG of non-positive number".to_string()));
                }
                Ok(Value::Number(n.ln()))
            }
            
            "EXP" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("EXP requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.exp()))
            }
            
            // 字符串函数
            "LEN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("LEN requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                Ok(Value::Number(s.len() as f64))
            }
            
            "ASC" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ASC requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                if s.is_empty() {
                    return Err(BasicError::IllegalQuantity("ASC of empty string".to_string()));
                }
                Ok(Value::Number(s.chars().next().unwrap() as u8 as f64))
            }
            
            "CHR$" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("CHR$ requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n < 0.0 || n > 255.0 {
                    return Err(BasicError::IllegalQuantity("CHR$ argument out of range".to_string()));
                }
                let ch = n as u8 as char;
                Ok(Value::String(ch.to_string()))
            }
            
            "STR$" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("STR$ requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                // BASIC 的 STR$ 在正数前加空格
                let s = if n >= 0.0 {
                    format!(" {}", n)
                } else {
                    n.to_string()
                };
                Ok(Value::String(s))
            }
            
            "VAL" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("VAL requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = s.trim().parse::<f64>().unwrap_or(0.0);
                Ok(Value::Number(n))
            }
            
            "LEFT$" => {
                if args.len() != 2 {
                    return Err(BasicError::SyntaxError("LEFT$ requires 2 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = self.eval_expr(&args[1])?.as_number()? as usize;
                let result: String = s.chars().take(n).collect();
                Ok(Value::String(result))
            }
            
            "RIGHT$" => {
                if args.len() != 2 {
                    return Err(BasicError::SyntaxError("RIGHT$ requires 2 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = self.eval_expr(&args[1])?.as_number()? as usize;
                let len = s.chars().count();
                let skip = if n > len { 0 } else { len - n };
                let result: String = s.chars().skip(skip).collect();
                Ok(Value::String(result))
            }
            
            "MID$" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err(BasicError::SyntaxError("MID$ requires 2 or 3 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let start = self.eval_expr(&args[1])?.as_number()? as usize;
                
                // BASIC 的 MID$ 是 1-based
                let start = if start > 0 { start - 1 } else { 0 };
                
                let chars: Vec<char> = s.chars().collect();
                
                if args.len() == 3 {
                    let len = self.eval_expr(&args[2])?.as_number()? as usize;
                    let result: String = chars.iter().skip(start).take(len).collect();
                    Ok(Value::String(result))
                } else {
                    let result: String = chars.iter().skip(start).collect();
                    Ok(Value::String(result))
                }
            }
            
            _ => Err(BasicError::SyntaxError(
                format!("Unknown function: {}", name)
            )),
        }
    }

    /// 执行语句
    pub fn execute_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Let { target, value } => {
                let val = self.eval_expr(value)?;
                
                match target {
                    AssignTarget::Variable(name) => {
                        self.variables.set(name, val)?;
                    }
                    AssignTarget::ArrayElement { name, indices } => {
                        let idx_values: Result<Vec<usize>> = indices.iter()
                            .map(|idx_expr| {
                                self.eval_expr(idx_expr)?
                                    .as_number()
                                    .map(|n| n as usize)
                            })
                            .collect();
                        
                        let indices_usize = idx_values?;
                        self.variables.set_array_element(name, &indices_usize, val)?;
                    }
                }
                
                Ok(())
            }
            
            Statement::End => {
                self.runtime.end_execution();
                Ok(())
            }
            
            Statement::Stop => {
                self.runtime.pause_execution();
                Ok(())
            }
            
            Statement::New => {
                self.runtime.clear_program();
                self.variables.clear();
                Ok(())
            }
            
            Statement::Clear => {
                self.variables.clear();
                Ok(())
            }
            
            Statement::Dim { arrays } => {
                for arr_dim in arrays {
                    let dimensions: Result<Vec<usize>> = arr_dim.dimensions.iter()
                        .map(|dim_expr| {
                            self.eval_expr(dim_expr)?
                                .as_number()
                                .map(|n| n as usize)
                        })
                        .collect();
                    
                    let dims = dimensions?;
                    self.variables.dim_array(&arr_dim.name, dims)?;
                }
                Ok(())
            }
            
            Statement::Print { items } => {
                self.execute_print(items)?;
                Ok(())
            }
            
            Statement::Goto { line_number } => {
                let line_val = self.eval_expr(line_number)?;
                let line = line_val.as_number()? as u16;
                self.runtime.set_execution_position(line, 0)?;
                Ok(())
            }
            
            Statement::If { condition, then_part } => {
                let cond_val = self.eval_expr(condition)?;
                let cond_num = cond_val.as_number()?;
                
                // BASIC 中，任何非零值都是真
                if cond_num != 0.0 {
                    match then_part.as_ref() {
                        ThenPart::LineNumber(line) => {
                            self.runtime.set_execution_position(*line as u16, 0)?;
                        }
                        ThenPart::Statement(stmt) => {
                            self.execute_statement(stmt)?;
                        }
                        ThenPart::Statements(stmts) => {
                            for stmt in stmts {
                                self.execute_statement(stmt)?;
                            }
                        }
                    }
                }
                Ok(())
            }
            
            Statement::Gosub { line_number } => {
                // 保存返回地址（当前行号和语句索引）
                let return_line = self.runtime.get_current_line().unwrap_or(0);
                let return_stmt = 0; // 简化：返回到下一行的第一条语句
                
                // 入栈
                self.runtime.push_gosub(return_line, return_stmt)?;
                
                // 跳转到子程序
                let line_val = self.eval_expr(line_number)?;
                let line = line_val.as_number()? as u16;
                self.runtime.set_execution_position(line, 0)?;
                
                Ok(())
            }
            
            Statement::Return => {
                // 从栈中弹出返回地址
                let (return_line, return_stmt) = self.runtime.pop_gosub()?;
                
                // 跳转回返回地址
                self.runtime.set_execution_position(return_line, return_stmt)?;
                
                Ok(())
            }
            
            Statement::Input { prompt, variables } => {
                // 提取变量名
                let var_names: Vec<String> = variables.iter()
                    .map(|target| match target {
                        AssignTarget::Variable(name) => name.clone(),
                        AssignTarget::ArrayElement { .. } => {
                            // INPUT 不支持数组元素
                            String::new()
                        }
                    })
                    .collect();
                
                self.execute_input(prompt.as_deref(), &var_names)?;
                Ok(())
            }
            
            Statement::Data { values: _ } => {
                // DATA 语句在程序加载时处理，执行时跳过
                // 数据已经存储在 data_values 中
                Ok(())
            }
            
            Statement::Read { variables } => {
                for target in variables {
                    let var_name = match target {
                        AssignTarget::Variable(name) => name.as_str(),
                        AssignTarget::ArrayElement { .. } => {
                            return Err(BasicError::SyntaxError(
                                "READ does not support array elements".to_string()
                            ));
                        }
                    };
                    
                    let data_val = self.read_data_value()?;
                    
                    // 根据变量名判断类型
                    if var_name.ends_with('$') {
                        // 字符串变量
                        let str_val = match data_val {
                            DataValue::String(s) => s,
                            DataValue::Number(n) => n.to_string(),
                        };
                        self.variables.set(var_name, Value::String(str_val))?;
                    } else {
                        // 数值变量
                        let num_val = match data_val {
                            DataValue::Number(n) => n,
                            DataValue::String(s) => {
                                s.trim().parse::<f64>().unwrap_or(0.0)
                            }
                        };
                        self.variables.set(var_name, Value::Number(num_val))?;
                    }
                }
                Ok(())
            }
            
            Statement::Restore { line_number } => {
                if line_number.is_some() {
                    // RESTORE 到指定行（暂不支持，需要跟踪每行的 DATA 位置）
                    return Err(BasicError::SyntaxError(
                        "RESTORE to specific line not yet implemented".to_string()
                    ));
                }
                self.restore_data();
                Ok(())
            }
            
            Statement::For { var, start, end, step } => {
                // 计算起始值、结束值和步长
                let start_val = self.eval_expr(&start)?;
                let end_val = self.eval_expr(&end)?;
                let step_val = if let Some(ref s) = step {
                    self.eval_expr(s)?
                } else {
                    Value::Number(1.0)
                };
                
                // 提取数值
                let start_num = start_val.as_number()?;
                let end_num = end_val.as_number()?;
                let step_num = step_val.as_number()?;
                
                // 检查步长
                if step_num == 0.0 {
                    return Err(BasicError::IllegalQuantity(
                        "FOR loop step cannot be zero".to_string()
                    ));
                }
                
                // 设置循环变量初值
                self.variables.set(var, Value::Number(start_num))?;
                
                // 获取当前位置
                let loop_line = self.runtime.get_current_line()
                    .ok_or_else(|| BasicError::SyntaxError("FOR without line number".to_string()))?;
                let loop_stmt = self.runtime.get_current_stmt_index();
                
                // 将循环信息压入栈
                self.runtime.push_for_loop(
                    var.clone(),
                    end_num,
                    step_num,
                    loop_line,
                    loop_stmt,
                )?;
                
                Ok(())
            }
            
            Statement::Next { var } => {
                // 弹出 FOR 循环信息
                let (loop_var, end_val, step_val, loop_line, loop_stmt) = 
                    self.runtime.pop_for_loop(var.clone())?;
                
                // 获取当前循环变量的值
                let current_val = self.variables.get(&loop_var).as_number()?;
                
                // 递增/递减
                let new_val = current_val + step_val;
                
                // 检查是否继续循环
                let should_continue = if step_val > 0.0 {
                    new_val <= end_val
                } else {
                    new_val >= end_val
                };
                
                if should_continue {
                    // 更新循环变量
                    self.variables.set(&loop_var, Value::Number(new_val))?;
                    
                    // 重新压入栈（继续循环）
                    self.runtime.push_for_loop(
                        loop_var.clone(),
                        end_val,
                        step_val,
                        loop_line,
                        loop_stmt,
                    )?;
                    
                    // 跳转回 FOR 语句的下一条语句
                    self.runtime.set_execution_position(loop_line, loop_stmt + 1)?;
                }
                // 否则继续执行下一条语句（循环结束）
                
                Ok(())
            }
            
            Statement::On { expr, targets, is_gosub } => {
                // 计算表达式的值
                let index_val = self.eval_expr(&expr)?;
                let index = index_val.as_number()? as i32;
                
                // 索引从 1 开始
                if index < 1 || index as usize > targets.len() {
                    // 超出范围，继续执行下一条语句
                    return Ok(());
                }
                
                // 获取目标行号
                let target_line = targets[(index - 1) as usize];
                
                if *is_gosub {
                    // ON...GOSUB：保存返回地址并跳转
                    let return_line = self.runtime.get_current_line()
                        .ok_or_else(|| BasicError::SyntaxError("GOSUB without line number".to_string()))?;
                    let return_stmt = self.runtime.get_current_stmt_index();
                    
                    self.runtime.push_gosub(return_line, return_stmt)?;
                    self.runtime.set_execution_position(target_line, 0)?;
                } else {
                    // ON...GOTO：直接跳转
                    self.runtime.set_execution_position(target_line, 0)?;
                }
                
                Ok(())
            }
            
            Statement::Load { filename } => {
                self.execute_load(filename)?;
                Ok(())
            }
            
            Statement::Save { filename } => {
                self.execute_save(filename)?;
                Ok(())
            }
            
            _ => {
                // 其他语句暂未实现
                Err(BasicError::SyntaxError(
                    "Statement not yet implemented".to_string()
                ))
            }
        }
    }
    
    /// 执行 INPUT 语句
    fn execute_input(&mut self, prompt: Option<&str>, variables: &[String]) -> Result<()> {
        // 显示提示符
        if let Some(p) = prompt {
            self.output(p);
            self.output("? ");
        } else {
            self.output("? ");
        }
        
        // 读取输入
        let input_line = if let Some(ref mut callback) = self.input_callback {
            let prompt_text = prompt.unwrap_or("");
            callback(prompt_text).ok_or_else(|| {
                BasicError::SyntaxError("No input provided".to_string())
            })?
        } else {
            // 在实际 REPL 中，这里会从 stdin 读取
            return Err(BasicError::SyntaxError(
                "No input callback set".to_string()
            ));
        };
        
        // 解析输入值（考虑引号内的逗号）
        let values = Self::parse_input_values(&input_line);
        
        if values.len() != variables.len() {
            self.output("?EXTRA IGNORED\n");
        }
        
        // 赋值给变量
        for (i, var_name) in variables.iter().enumerate() {
            if i >= values.len() {
                break;
            }
            
            let input_val = &values[i];
            
            if var_name.ends_with('$') {
                // 字符串变量
                let str_val = if input_val.starts_with('"') && input_val.ends_with('"') {
                    // 去掉引号
                    input_val[1..input_val.len()-1].to_string()
                } else {
                    input_val.clone()
                };
                self.variables.set(var_name, Value::String(str_val))?;
            } else {
                // 数值变量
                match input_val.parse::<f64>() {
                    Ok(num) => {
                        self.variables.set(var_name, Value::Number(num))?;
                    }
                    Err(_) => {
                        self.output("?REDO FROM START\n");
                        return Err(BasicError::TypeMismatch(
                            "Invalid number input".to_string()
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// 解析输入值，处理带引号的字符串
    fn parse_input_values(input: &str) -> Vec<String> {
        let mut values = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        
        for ch in input.chars() {
            match ch {
                '"' => {
                    in_quotes = !in_quotes;
                    current.push(ch);
                }
                ',' if !in_quotes => {
                    values.push(current.trim().to_string());
                    current.clear();
                }
                _ => {
                    current.push(ch);
                }
            }
        }
        
        if !current.is_empty() || input.ends_with(',') {
            values.push(current.trim().to_string());
        }
        
        values
    }
    
    /// 执行 SAVE 命令 - 保存程序到文件
    fn execute_save(&self, filename: &str) -> Result<()> {
        use std::fs::File;
        use std::io::Write;
        
        let program = self.runtime.clone_program();
        if program.is_empty() {
            return Err(BasicError::SyntaxError("No program to save".to_string()));
        }
        
        let mut file = File::create(filename).map_err(|e| {
            BasicError::SyntaxError(format!("Failed to create file: {}", e))
        })?;
        
        for (_, line) in program.iter() {
            let line_text = Self::serialize_program_line(line);
            writeln!(file, "{}", line_text).map_err(|e| {
                BasicError::SyntaxError(format!("Failed to write to file: {}", e))
            })?;
        }
        
        Ok(())
    }
    
    /// 将程序行序列化为文本
    fn serialize_program_line(line: &ProgramLine) -> String {
        let mut result = format!("{}", line.line_number);
        
        for (i, stmt) in line.statements.iter().enumerate() {
            if i > 0 {
                result.push_str(":");
            }
            result.push(' ');
            result.push_str(&Self::serialize_statement(stmt));
        }
        
        result
    }
    
    /// 将语句序列化为文本
    fn serialize_statement(stmt: &Statement) -> String {
        match stmt {
            Statement::Let { target, value } => {
                format!("{} = {}", Self::serialize_assign_target(target), Self::serialize_expr(value))
            }
            Statement::Print { items } => {
                let mut result = "PRINT ".to_string();
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        result.push_str(&Self::serialize_print_item(item));
                    } else {
                        result.push_str(&Self::serialize_print_item(item));
                    }
                }
                result
            }
            Statement::If { condition, then_part } => {
                format!("IF {} THEN {}", Self::serialize_expr(condition), Self::serialize_then_part(then_part))
            }
            Statement::Goto { line_number } => {
                format!("GOTO {}", Self::serialize_expr(line_number))
            }
            Statement::Gosub { line_number } => {
                format!("GOSUB {}", Self::serialize_expr(line_number))
            }
            Statement::Return => "RETURN".to_string(),
            Statement::For { var, start, end, step } => {
                let mut result = format!("FOR {} = {} TO {}", var, Self::serialize_expr(start), Self::serialize_expr(end));
                if let Some(step_expr) = step {
                    result.push_str(&format!(" STEP {}", Self::serialize_expr(step_expr)));
                }
                result
            }
            Statement::Next { var } => {
                if let Some(v) = var {
                    format!("NEXT {}", v)
                } else {
                    "NEXT".to_string()
                }
            }
            Statement::On { expr, targets, is_gosub } => {
                let keyword = if *is_gosub { "GOSUB" } else { "GOTO" };
                let target_str = targets.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("ON {} {} {}", Self::serialize_expr(expr), keyword, target_str)
            }
            Statement::Input { prompt, variables } => {
                let mut result = "INPUT ".to_string();
                if let Some(p) = prompt {
                    result.push_str(&format!("\"{}\" ", p));
                }
                let var_str = variables.iter()
                    .map(|v| Self::serialize_assign_target(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                result.push_str(&var_str);
                result
            }
            Statement::Dim { arrays } => {
                let arr_str = arrays.iter()
                    .map(|arr| {
                        let dims = arr.dimensions.iter()
                            .map(|d| Self::serialize_expr(d))
                            .collect::<Vec<_>>()
                            .join(",");
                        format!("{}({})", arr.name, dims)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("DIM {}", arr_str)
            }
            Statement::Data { values } => {
                let val_str = values.iter()
                    .map(|v| match v {
                        crate::ast::DataValue::Number(n) => n.to_string(),
                        crate::ast::DataValue::String(s) => format!("\"{}\"", s),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("DATA {}", val_str)
            }
            Statement::Read { variables } => {
                let var_str = variables.iter()
                    .map(|v| Self::serialize_assign_target(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("READ {}", var_str)
            }
            Statement::Restore { line_number } => {
                if let Some(ln) = line_number {
                    format!("RESTORE {}", ln)
                } else {
                    "RESTORE".to_string()
                }
            }
            Statement::Rem => "REM".to_string(),
            Statement::End => "END".to_string(),
            Statement::Stop => "STOP".to_string(),
            Statement::New => "NEW".to_string(),
            Statement::Clear => "CLEAR".to_string(),
            _ => "REM UNSUPPORTED STATEMENT".to_string(),
        }
    }
    
    /// 将表达式序列化为文本
    fn serialize_expr(expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => n.to_string(),
            Expr::String(s) => format!("\"{}\"", s),
            Expr::Variable(name) => name.clone(),
            Expr::ArrayAccess { name, indices } => {
                let idx_str = indices.iter()
                    .map(|i| Self::serialize_expr(i))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, idx_str)
            }
            Expr::FunctionCall { name, args } => {
                let arg_str = args.iter()
                    .map(|a| Self::serialize_expr(a))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, arg_str)
            }
            Expr::BinaryOp { left, op, right } => {
                let op_str = match op {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Subtract => "-",
                    BinaryOperator::Multiply => "*",
                    BinaryOperator::Divide => "/",
                    BinaryOperator::Power => "^",
                    BinaryOperator::Equal => "=",
                    BinaryOperator::NotEqual => "<>",
                    BinaryOperator::Less => "<",
                    BinaryOperator::Greater => ">",
                    BinaryOperator::LessEqual => "<=",
                    BinaryOperator::GreaterEqual => ">=",
                    BinaryOperator::And => " AND ",
                    BinaryOperator::Or => " OR ",
                };
                format!("({} {} {})", Self::serialize_expr(left), op_str, Self::serialize_expr(right))
            }
            Expr::UnaryOp { op, operand } => {
                let op_str = match op {
                    UnaryOperator::Minus => "-",
                    UnaryOperator::Not => "NOT ",
                };
                format!("{}{}", op_str, Self::serialize_expr(operand))
            }
        }
    }
    
    /// 将赋值目标序列化为文本
    fn serialize_assign_target(target: &AssignTarget) -> String {
        match target {
            AssignTarget::Variable(name) => name.clone(),
            AssignTarget::ArrayElement { name, indices } => {
                let idx_str = indices.iter()
                    .map(|i| Self::serialize_expr(i))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, idx_str)
            }
        }
    }
    
    /// 将THEN部分序列化为文本
    fn serialize_then_part(then_part: &ThenPart) -> String {
        match then_part {
            ThenPart::LineNumber(ln) => ln.to_string(),
            ThenPart::Statement(stmt) => Self::serialize_statement(stmt),
            ThenPart::Statements(stmts) => {
                stmts.iter()
                    .map(|s| Self::serialize_statement(s))
                    .collect::<Vec<_>>()
                    .join(":")
            }
        }
    }
    
    /// 将PRINT项序列化为文本
    fn serialize_print_item(item: &PrintItem) -> String {
        match item {
            PrintItem::Expr(expr) => Self::serialize_expr(expr),
            PrintItem::Tab(expr) => format!("TAB({})", Self::serialize_expr(expr)),
            PrintItem::Spc(expr) => format!("SPC({})", Self::serialize_expr(expr)),
            PrintItem::Comma => ",".to_string(),
            PrintItem::Semicolon => ";".to_string(),
        }
    }
    
    /// 执行 LOAD 命令 - 从文件加载程序
    fn execute_load(&mut self, filename: &str) -> Result<()> {
        use std::fs;
        use crate::tokenizer::Tokenizer;
        use crate::parser::Parser;
        
        // 读取文件内容
        let content = fs::read_to_string(filename).map_err(|e| {
            BasicError::SyntaxError(format!("Failed to read file: {}", e))
        })?;
        
        // 清空当前程序
        self.runtime.clear_program();
        self.variables.clear();
        
        // 逐行解析并添加到程序
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            // 使用tokenizer和parser解析每一行
            let mut tokenizer = Tokenizer::new(line);
            let tokens = tokenizer.tokenize_line()?;
            
            let mut parser = Parser::new(tokens);
            if let Some(program_line) = parser.parse_line()? {
                if program_line.line_number > 0 {
                    self.runtime.add_line(program_line);
                }
            }
        }
        
        Ok(())
    }
    
    /// 执行 PRINT 语句
    fn execute_print(&mut self, items: &[PrintItem]) -> Result<()> {
        if items.is_empty() {
            // 空 PRINT，仅输出换行
            self.output_newline();
            return Ok(());
        }
        
        for item in items.iter() {
            match item {
                PrintItem::Expr(expr) => {
                    let val = self.eval_expr(expr)?;
                    self.print_value(&val)?;
                }
                PrintItem::Tab(expr) => {
                    let target_col = self.eval_expr(expr)?.as_number()? as usize;
                    if target_col > self.print_column {
                        let spaces = target_col - self.print_column;
                        self.output(&" ".repeat(spaces));
                    } else if target_col < self.print_column {
                        // TAB 到更小的列，换行后跳转
                        self.output_newline();
                        self.output(&" ".repeat(target_col));
                    }
                }
                PrintItem::Spc(expr) => {
                    let spaces = self.eval_expr(expr)?.as_number()? as usize;
                    self.output(&" ".repeat(spaces));
                }
                PrintItem::Comma => {
                    // 逗号：对齐到下一个 14 列边界
                    let next_col = ((self.print_column / 14) + 1) * 14;
                    let spaces_needed = next_col - self.print_column;
                    self.output(&" ".repeat(spaces_needed));
                }
                PrintItem::Semicolon => {
                    // 分号：不添加空格（紧密连接）
                }
            }
        }
        
        // 检查最后一项是否是分隔符
        if let Some(last) = items.last() {
            if !matches!(last, PrintItem::Comma | PrintItem::Semicolon) {
                // 如果最后不是分隔符，输出换行
                self.output_newline();
            }
        } else {
            self.output_newline();
        }
        
        Ok(())
    }
    
    /// 打印值（根据 BASIC 格式）
    fn print_value(&mut self, val: &Value) -> Result<()> {
        match val {
            Value::Number(n) => {
                // BASIC 数值格式：正数前后各有空格，负数前有空格
                let formatted = if *n >= 0.0 {
                    format!(" {} ", n)
                } else {
                    format!(" {}", n)
                };
                self.output(&formatted);
            }
            Value::String(s) => {
                // 普通字符串，直接输出
                self.output(s);
            }
        }
        Ok(())
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Requirement: 算术运算符 - 加法
    #[test]
    fn test_addition() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Add,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    // Requirement: 算术运算符 - 减法
    #[test]
    fn test_subtraction() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(10.0),
            BinaryOperator::Subtract,
            Expr::Number(7.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    // Requirement: 算术运算符 - 乘法
    #[test]
    fn test_multiplication() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(4.0),
            BinaryOperator::Multiply,
            Expr::Number(5.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(20.0));
    }

    // Requirement: 算术运算符 - 除法
    #[test]
    fn test_division() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(15.0),
            BinaryOperator::Divide,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    // Requirement: 算术运算符 - 浮点除法
    #[test]
    fn test_float_division() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(10.0),
            BinaryOperator::Divide,
            Expr::Number(4.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(2.5));
    }

    // Requirement: 算术运算符 - 除以零
    #[test]
    fn test_division_by_zero() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Divide,
            Expr::Number(0.0)
        );
        let result = exec.eval_expr(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BasicError::DivisionByZero));
    }

    // Requirement: 算术运算符 - 乘方
    #[test]
    fn test_power() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(2.0),
            BinaryOperator::Power,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    // Requirement: 一元运算符 - 一元负号
    #[test]
    fn test_unary_minus() {
        let mut exec = Executor::new();
        let expr = Expr::unary(UnaryOperator::Minus, Expr::Number(5.0));
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-5.0));
    }

    // Requirement: 关系运算符 - 等于
    #[test]
    fn test_equal() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Equal,
            Expr::Number(5.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-1.0)); // BASIC true = -1
    }

    // Requirement: 关系运算符 - 不等于
    #[test]
    fn test_not_equal() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::NotEqual,
            Expr::Number(4.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-1.0));
    }

    // Requirement: 字符串运算符 - 字符串连接
    #[test]
    fn test_string_concatenation() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::String("HELLO".to_string()),
            BinaryOperator::Add,
            Expr::String(" WORLD".to_string())
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::String("HELLO WORLD".to_string()));
    }

    // Test: 变量读取
    #[test]
    fn test_variable_read() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(42.0)).unwrap();
        
        let expr = Expr::Variable("A".to_string());
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    // Test: LET 语句执行
    #[test]
    fn test_let_statement() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Let {
            target: AssignTarget::Variable("X".to_string()),
            value: Expr::Number(100.0),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.variables.get("X"), Value::Number(100.0));
    }

    // Test: DIM 语句执行
    #[test]
    fn test_dim_statement() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Dim {
            arrays: vec![
                ArrayDim {
                    name: "A".to_string(),
                    dimensions: vec![Expr::Number(10.0)],
                }
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert!(exec.variables.has_array("A"));
    }

    // Test: 数学函数
    #[test]
    fn test_math_functions() {
        let mut exec = Executor::new();
        
        // ABS
        let expr = Expr::FunctionCall {
            name: "ABS".to_string(),
            args: vec![Expr::Number(-42.0)],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(42.0));
        
        // INT
        let expr = Expr::FunctionCall {
            name: "INT".to_string(),
            args: vec![Expr::Number(3.7)],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    // Test: 字符串函数
    #[test]
    fn test_string_functions() {
        let mut exec = Executor::new();
        
        // LEN
        let expr = Expr::FunctionCall {
            name: "LEN".to_string(),
            args: vec![Expr::String("HELLO".to_string())],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));
        
        // LEFT$
        let expr = Expr::FunctionCall {
            name: "LEFT$".to_string(),
            args: vec![
                Expr::String("HELLO".to_string()),
                Expr::Number(3.0),
            ],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::String("HEL".to_string()));
    }

    // Test: 复杂表达式
    #[test]
    fn test_complex_expression() {
        let mut exec = Executor::new();
        
        // 2 + 3 * 4 = 14
        let expr = Expr::binary(
            Expr::Number(2.0),
            BinaryOperator::Add,
            Expr::binary(
                Expr::Number(3.0),
                BinaryOperator::Multiply,
                Expr::Number(4.0)
            )
        );
        
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(14.0));
    }

    // Requirement: PRINT 语句 - 打印数值
    #[test]
    fn test_print_number() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(42.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 42 \n");
    }

    // Requirement: PRINT 语句 - 打印字符串
    #[test]
    fn test_print_string() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("HELLO".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "HELLO\n");
    }

    // Requirement: PRINT 语句 - 打印变量
    #[test]
    fn test_print_variable() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(10.0)).unwrap();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Variable("A".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 10 \n");
    }

    // Requirement: PRINT 语句 - 分号分隔（紧密连接）
    #[test]
    fn test_print_semicolon() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(1.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::Number(2.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::Number(3.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 1  2  3 \n");
    }

    // Requirement: PRINT 语句 - 行尾分号（不换行）
    #[test]
    fn test_print_no_newline() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(42.0)),
                PrintItem::Semicolon,
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 42 ");
    }

    // Requirement: PRINT 语句 - 空 PRINT
    #[test]
    fn test_print_empty() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "\n");
    }

    // Requirement: PRINT 语句 - 逗号分隔（列对齐）
    #[test]
    fn test_print_comma_alignment() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(1.0)),
                PrintItem::Comma,
                PrintItem::Expr(Expr::Number(2.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // 第一个数 " 1 " 占 3 列，逗号应该对齐到第 14 列
        assert!(output.starts_with(" 1 "));
        assert!(output.contains(" 2 "));
    }

    // Requirement: GOTO 语句
    #[test]
    fn test_goto_statement() {
        let mut exec = Executor::new();
        
        // 添加程序行
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("A".to_string()),
                value: Expr::Number(1.0),
            }]
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("B".to_string()),
                value: Expr::Number(99.0),
            }]
        });
        
        let stmt = Statement::Goto {
            line_number: Expr::Number(100.0),
        };
        exec.execute_statement(&stmt).unwrap();
        
        // 验证跳转成功（下一行应该是 100）
        assert_eq!(exec.runtime().get_current_line(), Some(100));
    }

    // Requirement: IF...THEN 语句 - 条件为真
    #[test]
    fn test_if_then_true() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(15.0)).unwrap();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行来设置初始状态
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::LineNumber(100)),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.runtime().get_current_line(), Some(100));
    }

    // Requirement: IF...THEN 语句 - 条件为假
    #[test]
    fn test_if_then_false() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(5.0)).unwrap();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![],
        });
        
        let current_line = exec.runtime().get_current_line();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::LineNumber(100)),
        };
        
        exec.execute_statement(&stmt).unwrap();
        // 条件为假，不应该跳转
        assert_eq!(exec.runtime().get_current_line(), current_line);
    }

    // Requirement: IF...THEN 语句 - THEN 后跟语句
    #[test]
    fn test_if_then_statement() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(15.0)).unwrap();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::Statement(
                Statement::Print {
                    items: vec![
                        PrintItem::Expr(Expr::String("TRUE".to_string())),
                    ],
                }
            )),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "TRUE\n");
    }

    // Test: TAB 函数
    #[test]
    fn test_tab_function() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("A".to_string())),
                PrintItem::Semicolon,
                PrintItem::Tab(Expr::Number(10.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::String("B".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // A 在列 0，TAB(10) 跳到列 10，然后是 B
        assert!(output.starts_with("A"));
        assert!(output.contains("B"));
    }

    // Test: SPC 函数
    #[test]
    fn test_spc_function() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("A".to_string())),
                PrintItem::Semicolon,
                PrintItem::Spc(Expr::Number(5.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::String("B".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // A + 5个空格 + B
        assert_eq!(output, "A     B\n");
    }

    // Requirement: GOSUB 和 RETURN 语句 - 子程序调用
    #[test]
    fn test_gosub_statement() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 500,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        let stmt = Statement::Gosub {
            line_number: Expr::Number(500.0),
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证跳转到子程序
        assert_eq!(exec.runtime().get_current_line(), Some(500));
        // 验证调用栈深度
        assert_eq!(exec.runtime().stack_depth(), 1);
    }

    // Requirement: GOSUB 和 RETURN 语句 - 子程序返回
    #[test]
    fn test_return_statement() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 500,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行并设置调用栈
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        exec.runtime_mut().push_gosub(20, 0).unwrap();
        exec.runtime_mut().set_execution_position(500, 0).unwrap();
        
        let stmt = Statement::Return;
        exec.execute_statement(&stmt).unwrap();
        
        // 验证返回到调用点
        assert_eq!(exec.runtime().get_current_line(), Some(20));
        // 验证调用栈已弹出
        assert_eq!(exec.runtime().stack_depth(), 0);
    }

    // Requirement: GOSUB 和 RETURN 语句 - 嵌套子程序
    #[test]
    fn test_nested_gosub() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 200,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // 第一次 GOSUB
        exec.execute_statement(&Statement::Gosub {
            line_number: Expr::Number(100.0),
        }).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 1);
        
        // 第二次 GOSUB（嵌套）
        exec.execute_statement(&Statement::Gosub {
            line_number: Expr::Number(200.0),
        }).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 2);
        assert_eq!(exec.runtime().get_current_line(), Some(200));
        
        // 第一次 RETURN
        exec.execute_statement(&Statement::Return).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 1);
        assert_eq!(exec.runtime().get_current_line(), Some(100));
        
        // 第二次 RETURN
        exec.execute_statement(&Statement::Return).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 0);
        assert_eq!(exec.runtime().get_current_line(), Some(10));
    }

    // Requirement: INPUT 语句 - 基本输入
    #[test]
    fn test_input_basic() {
        let mut exec = Executor::new();
        
        // 设置输入回调
        exec.set_input_callback(|_| Some("42".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证输出提示符
        assert!(exec.get_output().contains("? "));
        
        // 验证变量赋值
        assert_eq!(exec.variables.get("A"), Value::Number(42.0));
    }

    // Requirement: INPUT 语句 - 带提示符的输入
    #[test]
    fn test_input_with_prompt() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("100".to_string()));
        
        let stmt = Statement::Input {
            prompt: Some("ENTER VALUE".to_string()),
            variables: vec![AssignTarget::Variable("X".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证提示符
        assert!(exec.get_output().contains("ENTER VALUE? "));
        assert_eq!(exec.variables.get("X"), Value::Number(100.0));
    }

    // Requirement: INPUT 语句 - 输入多个变量
    #[test]
    fn test_input_multiple_variables() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("10, 20, 30".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string()),
                AssignTarget::Variable("C".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(10.0));
        assert_eq!(exec.variables.get("B"), Value::Number(20.0));
        assert_eq!(exec.variables.get("C"), Value::Number(30.0));
    }

    // Requirement: INPUT 语句 - 字符串输入
    #[test]
    fn test_input_string() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("HELLO".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A$".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A$"), Value::String("HELLO".to_string()));
    }

    // Requirement: INPUT 语句 - 字符串带引号
    #[test]
    fn test_input_string_with_quotes() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("\"HELLO, WORLD\"".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A$".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A$"), Value::String("HELLO, WORLD".to_string()));
    }

    // Requirement: DATA/READ 机制 - DATA 存储和 READ 读取
    #[test]
    fn test_data_read() {
        let mut exec = Executor::new();
        
        // 添加 DATA 值
        exec.add_data_value(DataValue::Number(1.0));
        exec.add_data_value(DataValue::Number(2.0));
        exec.add_data_value(DataValue::Number(3.0));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string()),
                AssignTarget::Variable("C".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(1.0));
        assert_eq!(exec.variables.get("B"), Value::Number(2.0));
        assert_eq!(exec.variables.get("C"), Value::Number(3.0));
    }

    // Requirement: DATA/READ 机制 - 混合数据类型
    #[test]
    fn test_data_read_mixed_types() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(42.0));
        exec.add_data_value(DataValue::String("HELLO".to_string()));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B$".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(42.0));
        assert_eq!(exec.variables.get("B$"), Value::String("HELLO".to_string()));
    }

    // Requirement: DATA/READ 机制 - OUT OF DATA 错误
    #[test]
    fn test_out_of_data_error() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(1.0));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string())
            ],
        };
        
        let result = exec.execute_statement(&stmt);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BasicError::OutOfData));
    }

    // Requirement: RESTORE 数据指针 - RESTORE 重置到开头
    #[test]
    fn test_restore() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(1.0));
        exec.add_data_value(DataValue::Number(2.0));
        
        // 第一次 READ
        exec.execute_statement(&Statement::Read {
            variables: vec![AssignTarget::Variable("A".to_string())],
        }).unwrap();
        assert_eq!(exec.variables.get("A"), Value::Number(1.0));
        
        // RESTORE
        exec.execute_statement(&Statement::Restore {
            line_number: None,
        }).unwrap();
        
        // 第二次 READ（应该重新从头开始）
        exec.execute_statement(&Statement::Read {
            variables: vec![AssignTarget::Variable("B".to_string())],
        }).unwrap();
        assert_eq!(exec.variables.get("B"), Value::Number(1.0));
    }

    // Requirement: FOR...NEXT 循环 - 正向循环
    #[test]
    fn test_for_next_basic() {
        let mut exec = Executor::new();
        
        // 添加测试程序：FOR I=1 TO 3: PRINT I: NEXT I
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(1.0),
                    end: Expr::Number(3.0),
                    step: None,
                },
            ],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Next { var: Some("I".to_string()) }],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // 第一次循环：I=1
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(1.0),
            end: Expr::Number(3.0),
            step: None,
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
        
        // NEXT：I=2
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=3
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
        
        // NEXT：循环结束 (I递增到4但不再循环)
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        // 循环已结束，变量值应该为循环后的值 4
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
    }

    // Requirement: FOR...NEXT 循环 - 步长为 2
    #[test]
    fn test_for_next_step() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(0.0),
                    end: Expr::Number(4.0),
                    step: Some(Expr::Number(2.0)),
                },
            ],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // FOR I=0 TO 4 STEP 2
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(0.0),
            end: Expr::Number(4.0),
            step: Some(Expr::Number(2.0)),
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(0.0));
        
        // NEXT：I=2
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=4
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(4.0));
        
        // NEXT：循环结束
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(4.0));
    }

    // Requirement: FOR...NEXT 循环 - 负步长
    #[test]
    fn test_for_next_negative_step() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(3.0),
                    end: Expr::Number(1.0),
                    step: Some(Expr::Number(-1.0)),
                },
            ],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // FOR I=3 TO 1 STEP -1
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(3.0),
            end: Expr::Number(1.0),
            step: Some(Expr::Number(-1.0)),
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
        
        // NEXT：I=2
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=1
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
        
        // NEXT：循环结束
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
    }

    // Requirement: ON...GOTO - 基于表达式的跳转
    #[test]
    fn test_on_goto() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 200,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // ON 2 GOTO 100,200,300
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(2.0),
            targets: vec![100, 200, 300],
            is_gosub: false,
        }).unwrap();
        
        // 应该跳转到 200
        assert_eq!(exec.runtime().get_current_line(), Some(200));
    }

    // Requirement: ON...GOSUB - 基于表达式的子程序调用
    #[test]
    fn test_on_gosub() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // ON 1 GOSUB 100,200
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(1.0),
            targets: vec![100, 200],
            is_gosub: true,
        }).unwrap();
        
        // 应该跳转到 100
        assert_eq!(exec.runtime().get_current_line(), Some(100));
        // 栈深度应该为 1
        assert_eq!(exec.runtime().stack_depth(), 1);
    }

    // Requirement: ON...GOTO - 值超出范围
    #[test]
    fn test_on_goto_out_of_range() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        let current_line = exec.runtime().get_current_line();
        
        // ON 5 GOTO 100,200  (5 超出范围)
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(5.0),
            targets: vec![100, 200],
            is_gosub: false,
        }).unwrap();
        
        // 应该继续在当前行
        assert_eq!(exec.runtime().get_current_line(), current_line);
    }
    
    #[test]
    fn test_save_and_load() {
        use std::fs;
        
        let mut exec = Executor::new();
        
        // 添加一些程序行
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Print {
                items: vec![PrintItem::Expr(Expr::String("HELLO".to_string()))],
            }],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("A".to_string()),
                value: Expr::Number(42.0),
            }],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 30,
            statements: vec![Statement::End],
        });
        
        // 保存程序到文件
        let filename = "test_program.bas";
        exec.execute_statement(&Statement::Save {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证文件存在
        assert!(fs::metadata(filename).is_ok());
        
        // 清空程序
        exec.runtime_mut().clear_program();
        assert_eq!(exec.runtime().line_count(), 0);
        
        // 加载程序
        exec.execute_statement(&Statement::Load {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证程序已加载
        assert_eq!(exec.runtime().line_count(), 3);
        assert!(exec.runtime().get_line(10).is_some());
        assert!(exec.runtime().get_line(20).is_some());
        assert!(exec.runtime().get_line(30).is_some());
        
        // 清理测试文件
        fs::remove_file(filename).ok();
    }
    
    #[test]
    fn test_save_empty_program() {
        let mut exec = Executor::new();
        
        // 尝试保存空程序应该失败
        let result = exec.execute_statement(&Statement::Save {
            filename: "empty.bas".to_string(),
        });
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_load_nonexistent_file() {
        let mut exec = Executor::new();
        
        // 尝试加载不存在的文件应该失败
        let result = exec.execute_statement(&Statement::Load {
            filename: "nonexistent.bas".to_string(),
        });
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_save_complex_program() {
        use std::fs;
        
        let mut exec = Executor::new();
        
        // 创建一个更复杂的程序
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(1.0),
                    end: Expr::Number(10.0),
                    step: Some(Expr::Number(1.0)),
                },
            ],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![
                Statement::Print {
                    items: vec![PrintItem::Expr(Expr::Variable("I".to_string()))],
                },
            ],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 30,
            statements: vec![Statement::Next { var: Some("I".to_string()) }],
        });
        
        // 保存并重新加载
        let filename = "test_complex.bas";
        exec.execute_statement(&Statement::Save {
            filename: filename.to_string(),
        }).unwrap();
        
        exec.runtime_mut().clear_program();
        
        exec.execute_statement(&Statement::Load {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证程序结构
        assert_eq!(exec.runtime().line_count(), 3);
        
        // 清理
        fs::remove_file(filename).ok();
    }
}


```

现在让我编译并测试这个修复：

编译成功！现在让我创建一个测试来验证修复：

太好了！现在 `PRINT 111` 能正常输出了。让我再测试几个场景以确保修复完整：

完美！让我再测试一下程序模式（带行号）：

非常好！问题已经解决了。让我总结一下：

## 问题原因

在交互模式下，`PRINT` 语句的输出只被保存到内部缓冲区（`output_buffer`），而没有真正打印到终端。这个缓冲区最初是为单元测试设计的。

## 解决方案

我修改了 `src/executor.rs` 中的 `output()` 方法，使其同时：
1. 打印到终端（使用 `print!()` 和 `stdout().flush()`）
2. 保存到缓冲区（保持测试兼容性）

## 验证结果

现在交互模式下的 `PRINT` 语句工作正常：

1. **直接模式**：`print 111` → 输出 ` 111 `
2. **表达式**：`print 1+2+3` → 输出 ` 6 `
3. **变量**：`print a` → 输出变量值
4. **字符串**：`print "HELLO"` → 输出 `HELLO`
5. **程序模式**：带行号的程序中的 PRINT 也能正常工作

注意：BASIC 的数值格式会在正数前后各添加一个空格，这是标准的 BASIC 行为。

---

**User**

list打印语句不完整，执行是正确的，请修复：10 print 123
20 for i=1 to 3
30 print i
40 next
list
10 PRINT 1
20 FOR
30 PRINT 1
40 NEXT
run
 123
 1
 2
 3

---

**Cursor**

我看到问题了！`LIST` 命令使用的 `format_statement` 函数只是简化版本，没有正确显示完整的语句内容。让我查看并修复它：

问题找到了！`main.rs` 中的 `format_statement` 函数是简化版本，只显示语句类型而不是完整内容。而 `executor.rs` 中已经有完整的序列化函数用于 SAVE 命令。

让我将 `executor.rs` 中的序列化函数改为公开的，然后在 `main.rs` 中使用它们：

```rust
/// 执行引擎
///
/// 求值表达式并执行语句

use crate::ast::*;
use crate::error::{BasicError, Result};
use crate::runtime::Runtime;
use crate::variables::{Value, Variables};

/// 输入回调函数类型
pub type InputCallback = Box<dyn FnMut(&str) -> Option<String>>;

/// 执行引擎
pub struct Executor {
    runtime: Runtime,
    variables: Variables,
    /// 输出缓冲区（用于测试和捕获输出）
    output_buffer: Vec<String>,
    /// 当前打印列位置
    print_column: usize,
    /// DATA 数据存储
    data_values: Vec<DataValue>,
    /// DATA 数据指针（当前读取位置）
    data_pointer: usize,
    /// 输入回调函数（用于测试）
    input_callback: Option<InputCallback>,
}

/// DATA 值类型
#[derive(Debug, Clone, PartialEq)]
pub enum DataValue {
    Number(f64),
    String(String),
}

impl Executor {
    /// 创建新的执行器
    pub fn new() -> Self {
        Executor {
            runtime: Runtime::new(),
            variables: Variables::new(),
            output_buffer: Vec::new(),
            print_column: 0,
            data_values: Vec::new(),
            data_pointer: 0,
            input_callback: None,
        }
    }
    
    /// 设置输入回调函数（用于测试）
    pub fn set_input_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&str) -> Option<String> + 'static,
    {
        self.input_callback = Some(Box::new(callback));
    }
    
    /// 添加 DATA 值
    pub fn add_data_value(&mut self, value: DataValue) {
        self.data_values.push(value);
    }
    
    /// 重置 DATA 指针
    pub fn restore_data(&mut self) {
        self.data_pointer = 0;
    }
    
    /// 读取下一个 DATA 值
    fn read_data_value(&mut self) -> Result<DataValue> {
        if self.data_pointer >= self.data_values.len() {
            return Err(BasicError::OutOfData);
        }
        
        let value = self.data_values[self.data_pointer].clone();
        self.data_pointer += 1;
        Ok(value)
    }
    
    /// 获取输出内容（用于测试）
    pub fn get_output(&self) -> String {
        self.output_buffer.join("")
    }
    
    /// 清空输出缓冲区
    pub fn clear_output(&mut self) {
        self.output_buffer.clear();
        self.print_column = 0;
    }
    
    /// 输出文本（添加到缓冲区并打印到终端）
    fn output(&mut self, text: &str) {
        // 打印到终端
        print!("{}", text);
        use std::io::Write;
        std::io::stdout().flush().ok();
        
        // 同时添加到缓冲区（用于测试）
        self.output_buffer.push(text.to_string());
        
        // 更新列位置
        for ch in text.chars() {
            if ch == '\n' {
                self.print_column = 0;
            } else {
                self.print_column += 1;
            }
        }
    }
    
    /// 输出换行
    fn output_newline(&mut self) {
        self.output("\n");
    }

    /// 获取运行时引用
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    /// 获取变量存储引用
    pub fn variables(&self) -> &Variables {
        &self.variables
    }

    /// 获取运行时可变引用
    pub fn runtime_mut(&mut self) -> &mut Runtime {
        &mut self.runtime
    }

    /// 获取变量存储可变引用
    pub fn variables_mut(&mut self) -> &mut Variables {
        &mut self.variables
    }

    /// 求值表达式
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            
            Expr::String(s) => Ok(Value::String(s.clone())),
            
            Expr::Variable(name) => {
                Ok(self.variables.get(name))
            }
            
            Expr::ArrayAccess { name, indices } => {
                // 求值所有索引
                let idx_values: Result<Vec<usize>> = indices.iter()
                    .map(|idx_expr| {
                        self.eval_expr(idx_expr)?
                            .as_number()
                            .and_then(|n| {
                                if n < 0.0 {
                                    Err(BasicError::SubscriptOutOfRange(
                                        "Negative array index".to_string()
                                    ))
                                } else {
                                    Ok(n as usize)
                                }
                            })
                    })
                    .collect();
                
                let indices_usize = idx_values?;
                self.variables.get_array_element(name, &indices_usize)
            }
            
            Expr::FunctionCall { name, args } => {
                self.eval_function_call(name, args)
            }
            
            Expr::BinaryOp { left, op, right } => {
                self.eval_binary_op(left, *op, right)
            }
            
            Expr::UnaryOp { op, operand } => {
                self.eval_unary_op(*op, operand)
            }
        }
    }

    /// 求值二元运算
    fn eval_binary_op(&mut self, left: &Expr, op: BinaryOperator, right: &Expr) -> Result<Value> {
        use BinaryOperator::*;

        let left_val = self.eval_expr(left)?;
        let right_val = self.eval_expr(right)?;

        match op {
            // 算术运算符
            Add => {
                if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    Ok(Value::Number(l + r))
                } else if left_val.is_string() && right_val.is_string() {
                    // 字符串连接
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    Ok(Value::String(format!("{}{}", l, r)))
                } else {
                    Err(BasicError::TypeMismatch(
                        "Cannot add incompatible types".to_string()
                    ))
                }
            }
            
            Subtract => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l - r))
            }
            
            Multiply => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l * r))
            }
            
            Divide => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                if r == 0.0 {
                    return Err(BasicError::DivisionByZero);
                }
                Ok(Value::Number(l / r))
            }
            
            Power => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l.powf(r)))
            }
            
            // 关系运算符（BASIC 中 true = -1, false = 0）
            Equal => {
                let result = if left_val == right_val { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            NotEqual => {
                let result = if left_val != right_val { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            Less => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l < r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l < r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            Greater => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l > r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l > r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            LessEqual => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l <= r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l <= r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            GreaterEqual => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l >= r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l >= r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            // 逻辑运算符（按位）
            And => {
                let l = left_val.as_number()? as i32;
                let r = right_val.as_number()? as i32;
                Ok(Value::Number((l & r) as f64))
            }
            
            Or => {
                let l = left_val.as_number()? as i32;
                let r = right_val.as_number()? as i32;
                Ok(Value::Number((l | r) as f64))
            }
        }
    }

    /// 求值一元运算
    fn eval_unary_op(&mut self, op: UnaryOperator, operand: &Expr) -> Result<Value> {
        let val = self.eval_expr(operand)?;
        
        match op {
            UnaryOperator::Minus => {
                let n = val.as_number()?;
                Ok(Value::Number(-n))
            }
            UnaryOperator::Not => {
                let n = val.as_number()? as i32;
                Ok(Value::Number((!n) as f64))
            }
        }
    }

    /// 求值函数调用（内置函数）
    fn eval_function_call(&mut self, name: &str, args: &[Expr]) -> Result<Value> {
        match name.to_uppercase().as_str() {
            // 数学函数
            "SGN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SGN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                let result = if n > 0.0 { 1.0 } else if n < 0.0 { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            "INT" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("INT requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.floor()))
            }
            
            "ABS" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ABS requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.abs()))
            }
            
            "SQR" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SQR requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n < 0.0 {
                    return Err(BasicError::IllegalQuantity("SQR of negative number".to_string()));
                }
                Ok(Value::Number(n.sqrt()))
            }
            
            "SIN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SIN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.sin()))
            }
            
            "COS" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("COS requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.cos()))
            }
            
            "TAN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("TAN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.tan()))
            }
            
            "ATN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ATN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.atan()))
            }
            
            "LOG" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("LOG requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n <= 0.0 {
                    return Err(BasicError::IllegalQuantity("LOG of non-positive number".to_string()));
                }
                Ok(Value::Number(n.ln()))
            }
            
            "EXP" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("EXP requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.exp()))
            }
            
            // 字符串函数
            "LEN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("LEN requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                Ok(Value::Number(s.len() as f64))
            }
            
            "ASC" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ASC requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                if s.is_empty() {
                    return Err(BasicError::IllegalQuantity("ASC of empty string".to_string()));
                }
                Ok(Value::Number(s.chars().next().unwrap() as u8 as f64))
            }
            
            "CHR$" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("CHR$ requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n < 0.0 || n > 255.0 {
                    return Err(BasicError::IllegalQuantity("CHR$ argument out of range".to_string()));
                }
                let ch = n as u8 as char;
                Ok(Value::String(ch.to_string()))
            }
            
            "STR$" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("STR$ requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                // BASIC 的 STR$ 在正数前加空格
                let s = if n >= 0.0 {
                    format!(" {}", n)
                } else {
                    n.to_string()
                };
                Ok(Value::String(s))
            }
            
            "VAL" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("VAL requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = s.trim().parse::<f64>().unwrap_or(0.0);
                Ok(Value::Number(n))
            }
            
            "LEFT$" => {
                if args.len() != 2 {
                    return Err(BasicError::SyntaxError("LEFT$ requires 2 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = self.eval_expr(&args[1])?.as_number()? as usize;
                let result: String = s.chars().take(n).collect();
                Ok(Value::String(result))
            }
            
            "RIGHT$" => {
                if args.len() != 2 {
                    return Err(BasicError::SyntaxError("RIGHT$ requires 2 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = self.eval_expr(&args[1])?.as_number()? as usize;
                let len = s.chars().count();
                let skip = if n > len { 0 } else { len - n };
                let result: String = s.chars().skip(skip).collect();
                Ok(Value::String(result))
            }
            
            "MID$" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err(BasicError::SyntaxError("MID$ requires 2 or 3 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let start = self.eval_expr(&args[1])?.as_number()? as usize;
                
                // BASIC 的 MID$ 是 1-based
                let start = if start > 0 { start - 1 } else { 0 };
                
                let chars: Vec<char> = s.chars().collect();
                
                if args.len() == 3 {
                    let len = self.eval_expr(&args[2])?.as_number()? as usize;
                    let result: String = chars.iter().skip(start).take(len).collect();
                    Ok(Value::String(result))
                } else {
                    let result: String = chars.iter().skip(start).collect();
                    Ok(Value::String(result))
                }
            }
            
            _ => Err(BasicError::SyntaxError(
                format!("Unknown function: {}", name)
            )),
        }
    }

    /// 执行语句
    pub fn execute_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Let { target, value } => {
                let val = self.eval_expr(value)?;
                
                match target {
                    AssignTarget::Variable(name) => {
                        self.variables.set(name, val)?;
                    }
                    AssignTarget::ArrayElement { name, indices } => {
                        let idx_values: Result<Vec<usize>> = indices.iter()
                            .map(|idx_expr| {
                                self.eval_expr(idx_expr)?
                                    .as_number()
                                    .map(|n| n as usize)
                            })
                            .collect();
                        
                        let indices_usize = idx_values?;
                        self.variables.set_array_element(name, &indices_usize, val)?;
                    }
                }
                
                Ok(())
            }
            
            Statement::End => {
                self.runtime.end_execution();
                Ok(())
            }
            
            Statement::Stop => {
                self.runtime.pause_execution();
                Ok(())
            }
            
            Statement::New => {
                self.runtime.clear_program();
                self.variables.clear();
                Ok(())
            }
            
            Statement::Clear => {
                self.variables.clear();
                Ok(())
            }
            
            Statement::Dim { arrays } => {
                for arr_dim in arrays {
                    let dimensions: Result<Vec<usize>> = arr_dim.dimensions.iter()
                        .map(|dim_expr| {
                            self.eval_expr(dim_expr)?
                                .as_number()
                                .map(|n| n as usize)
                        })
                        .collect();
                    
                    let dims = dimensions?;
                    self.variables.dim_array(&arr_dim.name, dims)?;
                }
                Ok(())
            }
            
            Statement::Print { items } => {
                self.execute_print(items)?;
                Ok(())
            }
            
            Statement::Goto { line_number } => {
                let line_val = self.eval_expr(line_number)?;
                let line = line_val.as_number()? as u16;
                self.runtime.set_execution_position(line, 0)?;
                Ok(())
            }
            
            Statement::If { condition, then_part } => {
                let cond_val = self.eval_expr(condition)?;
                let cond_num = cond_val.as_number()?;
                
                // BASIC 中，任何非零值都是真
                if cond_num != 0.0 {
                    match then_part.as_ref() {
                        ThenPart::LineNumber(line) => {
                            self.runtime.set_execution_position(*line as u16, 0)?;
                        }
                        ThenPart::Statement(stmt) => {
                            self.execute_statement(stmt)?;
                        }
                        ThenPart::Statements(stmts) => {
                            for stmt in stmts {
                                self.execute_statement(stmt)?;
                            }
                        }
                    }
                }
                Ok(())
            }
            
            Statement::Gosub { line_number } => {
                // 保存返回地址（当前行号和语句索引）
                let return_line = self.runtime.get_current_line().unwrap_or(0);
                let return_stmt = 0; // 简化：返回到下一行的第一条语句
                
                // 入栈
                self.runtime.push_gosub(return_line, return_stmt)?;
                
                // 跳转到子程序
                let line_val = self.eval_expr(line_number)?;
                let line = line_val.as_number()? as u16;
                self.runtime.set_execution_position(line, 0)?;
                
                Ok(())
            }
            
            Statement::Return => {
                // 从栈中弹出返回地址
                let (return_line, return_stmt) = self.runtime.pop_gosub()?;
                
                // 跳转回返回地址
                self.runtime.set_execution_position(return_line, return_stmt)?;
                
                Ok(())
            }
            
            Statement::Input { prompt, variables } => {
                // 提取变量名
                let var_names: Vec<String> = variables.iter()
                    .map(|target| match target {
                        AssignTarget::Variable(name) => name.clone(),
                        AssignTarget::ArrayElement { .. } => {
                            // INPUT 不支持数组元素
                            String::new()
                        }
                    })
                    .collect();
                
                self.execute_input(prompt.as_deref(), &var_names)?;
                Ok(())
            }
            
            Statement::Data { values: _ } => {
                // DATA 语句在程序加载时处理，执行时跳过
                // 数据已经存储在 data_values 中
                Ok(())
            }
            
            Statement::Read { variables } => {
                for target in variables {
                    let var_name = match target {
                        AssignTarget::Variable(name) => name.as_str(),
                        AssignTarget::ArrayElement { .. } => {
                            return Err(BasicError::SyntaxError(
                                "READ does not support array elements".to_string()
                            ));
                        }
                    };
                    
                    let data_val = self.read_data_value()?;
                    
                    // 根据变量名判断类型
                    if var_name.ends_with('$') {
                        // 字符串变量
                        let str_val = match data_val {
                            DataValue::String(s) => s,
                            DataValue::Number(n) => n.to_string(),
                        };
                        self.variables.set(var_name, Value::String(str_val))?;
                    } else {
                        // 数值变量
                        let num_val = match data_val {
                            DataValue::Number(n) => n,
                            DataValue::String(s) => {
                                s.trim().parse::<f64>().unwrap_or(0.0)
                            }
                        };
                        self.variables.set(var_name, Value::Number(num_val))?;
                    }
                }
                Ok(())
            }
            
            Statement::Restore { line_number } => {
                if line_number.is_some() {
                    // RESTORE 到指定行（暂不支持，需要跟踪每行的 DATA 位置）
                    return Err(BasicError::SyntaxError(
                        "RESTORE to specific line not yet implemented".to_string()
                    ));
                }
                self.restore_data();
                Ok(())
            }
            
            Statement::For { var, start, end, step } => {
                // 计算起始值、结束值和步长
                let start_val = self.eval_expr(&start)?;
                let end_val = self.eval_expr(&end)?;
                let step_val = if let Some(ref s) = step {
                    self.eval_expr(s)?
                } else {
                    Value::Number(1.0)
                };
                
                // 提取数值
                let start_num = start_val.as_number()?;
                let end_num = end_val.as_number()?;
                let step_num = step_val.as_number()?;
                
                // 检查步长
                if step_num == 0.0 {
                    return Err(BasicError::IllegalQuantity(
                        "FOR loop step cannot be zero".to_string()
                    ));
                }
                
                // 设置循环变量初值
                self.variables.set(var, Value::Number(start_num))?;
                
                // 获取当前位置
                let loop_line = self.runtime.get_current_line()
                    .ok_or_else(|| BasicError::SyntaxError("FOR without line number".to_string()))?;
                let loop_stmt = self.runtime.get_current_stmt_index();
                
                // 将循环信息压入栈
                self.runtime.push_for_loop(
                    var.clone(),
                    end_num,
                    step_num,
                    loop_line,
                    loop_stmt,
                )?;
                
                Ok(())
            }
            
            Statement::Next { var } => {
                // 弹出 FOR 循环信息
                let (loop_var, end_val, step_val, loop_line, loop_stmt) = 
                    self.runtime.pop_for_loop(var.clone())?;
                
                // 获取当前循环变量的值
                let current_val = self.variables.get(&loop_var).as_number()?;
                
                // 递增/递减
                let new_val = current_val + step_val;
                
                // 检查是否继续循环
                let should_continue = if step_val > 0.0 {
                    new_val <= end_val
                } else {
                    new_val >= end_val
                };
                
                if should_continue {
                    // 更新循环变量
                    self.variables.set(&loop_var, Value::Number(new_val))?;
                    
                    // 重新压入栈（继续循环）
                    self.runtime.push_for_loop(
                        loop_var.clone(),
                        end_val,
                        step_val,
                        loop_line,
                        loop_stmt,
                    )?;
                    
                    // 跳转回 FOR 语句的下一条语句
                    self.runtime.set_execution_position(loop_line, loop_stmt + 1)?;
                }
                // 否则继续执行下一条语句（循环结束）
                
                Ok(())
            }
            
            Statement::On { expr, targets, is_gosub } => {
                // 计算表达式的值
                let index_val = self.eval_expr(&expr)?;
                let index = index_val.as_number()? as i32;
                
                // 索引从 1 开始
                if index < 1 || index as usize > targets.len() {
                    // 超出范围，继续执行下一条语句
                    return Ok(());
                }
                
                // 获取目标行号
                let target_line = targets[(index - 1) as usize];
                
                if *is_gosub {
                    // ON...GOSUB：保存返回地址并跳转
                    let return_line = self.runtime.get_current_line()
                        .ok_or_else(|| BasicError::SyntaxError("GOSUB without line number".to_string()))?;
                    let return_stmt = self.runtime.get_current_stmt_index();
                    
                    self.runtime.push_gosub(return_line, return_stmt)?;
                    self.runtime.set_execution_position(target_line, 0)?;
                } else {
                    // ON...GOTO：直接跳转
                    self.runtime.set_execution_position(target_line, 0)?;
                }
                
                Ok(())
            }
            
            Statement::Load { filename } => {
                self.execute_load(filename)?;
                Ok(())
            }
            
            Statement::Save { filename } => {
                self.execute_save(filename)?;
                Ok(())
            }
            
            _ => {
                // 其他语句暂未实现
                Err(BasicError::SyntaxError(
                    "Statement not yet implemented".to_string()
                ))
            }
        }
    }
    
    /// 执行 INPUT 语句
    fn execute_input(&mut self, prompt: Option<&str>, variables: &[String]) -> Result<()> {
        // 显示提示符
        if let Some(p) = prompt {
            self.output(p);
            self.output("? ");
        } else {
            self.output("? ");
        }
        
        // 读取输入
        let input_line = if let Some(ref mut callback) = self.input_callback {
            let prompt_text = prompt.unwrap_or("");
            callback(prompt_text).ok_or_else(|| {
                BasicError::SyntaxError("No input provided".to_string())
            })?
        } else {
            // 在实际 REPL 中，这里会从 stdin 读取
            return Err(BasicError::SyntaxError(
                "No input callback set".to_string()
            ));
        };
        
        // 解析输入值（考虑引号内的逗号）
        let values = Self::parse_input_values(&input_line);
        
        if values.len() != variables.len() {
            self.output("?EXTRA IGNORED\n");
        }
        
        // 赋值给变量
        for (i, var_name) in variables.iter().enumerate() {
            if i >= values.len() {
                break;
            }
            
            let input_val = &values[i];
            
            if var_name.ends_with('$') {
                // 字符串变量
                let str_val = if input_val.starts_with('"') && input_val.ends_with('"') {
                    // 去掉引号
                    input_val[1..input_val.len()-1].to_string()
                } else {
                    input_val.clone()
                };
                self.variables.set(var_name, Value::String(str_val))?;
            } else {
                // 数值变量
                match input_val.parse::<f64>() {
                    Ok(num) => {
                        self.variables.set(var_name, Value::Number(num))?;
                    }
                    Err(_) => {
                        self.output("?REDO FROM START\n");
                        return Err(BasicError::TypeMismatch(
                            "Invalid number input".to_string()
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// 解析输入值，处理带引号的字符串
    fn parse_input_values(input: &str) -> Vec<String> {
        let mut values = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        
        for ch in input.chars() {
            match ch {
                '"' => {
                    in_quotes = !in_quotes;
                    current.push(ch);
                }
                ',' if !in_quotes => {
                    values.push(current.trim().to_string());
                    current.clear();
                }
                _ => {
                    current.push(ch);
                }
            }
        }
        
        if !current.is_empty() || input.ends_with(',') {
            values.push(current.trim().to_string());
        }
        
        values
    }
    
    /// 执行 SAVE 命令 - 保存程序到文件
    fn execute_save(&self, filename: &str) -> Result<()> {
        use std::fs::File;
        use std::io::Write;
        
        let program = self.runtime.clone_program();
        if program.is_empty() {
            return Err(BasicError::SyntaxError("No program to save".to_string()));
        }
        
        let mut file = File::create(filename).map_err(|e| {
            BasicError::SyntaxError(format!("Failed to create file: {}", e))
        })?;
        
        for (_, line) in program.iter() {
            let line_text = Self::serialize_program_line(line);
            writeln!(file, "{}", line_text).map_err(|e| {
                BasicError::SyntaxError(format!("Failed to write to file: {}", e))
            })?;
        }
        
        Ok(())
    }
    
    /// 将程序行序列化为文本
    pub fn serialize_program_line(line: &ProgramLine) -> String {
        let mut result = format!("{}", line.line_number);
        
        for (i, stmt) in line.statements.iter().enumerate() {
            if i > 0 {
                result.push_str(":");
            }
            result.push(' ');
            result.push_str(&Self::serialize_statement(stmt));
        }
        
        result
    }
    
    /// 将语句序列化为文本
    fn serialize_statement(stmt: &Statement) -> String {
        match stmt {
            Statement::Let { target, value } => {
                format!("{} = {}", Self::serialize_assign_target(target), Self::serialize_expr(value))
            }
            Statement::Print { items } => {
                let mut result = "PRINT ".to_string();
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        result.push_str(&Self::serialize_print_item(item));
                    } else {
                        result.push_str(&Self::serialize_print_item(item));
                    }
                }
                result
            }
            Statement::If { condition, then_part } => {
                format!("IF {} THEN {}", Self::serialize_expr(condition), Self::serialize_then_part(then_part))
            }
            Statement::Goto { line_number } => {
                format!("GOTO {}", Self::serialize_expr(line_number))
            }
            Statement::Gosub { line_number } => {
                format!("GOSUB {}", Self::serialize_expr(line_number))
            }
            Statement::Return => "RETURN".to_string(),
            Statement::For { var, start, end, step } => {
                let mut result = format!("FOR {} = {} TO {}", var, Self::serialize_expr(start), Self::serialize_expr(end));
                if let Some(step_expr) = step {
                    result.push_str(&format!(" STEP {}", Self::serialize_expr(step_expr)));
                }
                result
            }
            Statement::Next { var } => {
                if let Some(v) = var {
                    format!("NEXT {}", v)
                } else {
                    "NEXT".to_string()
                }
            }
            Statement::On { expr, targets, is_gosub } => {
                let keyword = if *is_gosub { "GOSUB" } else { "GOTO" };
                let target_str = targets.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("ON {} {} {}", Self::serialize_expr(expr), keyword, target_str)
            }
            Statement::Input { prompt, variables } => {
                let mut result = "INPUT ".to_string();
                if let Some(p) = prompt {
                    result.push_str(&format!("\"{}\" ", p));
                }
                let var_str = variables.iter()
                    .map(|v| Self::serialize_assign_target(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                result.push_str(&var_str);
                result
            }
            Statement::Dim { arrays } => {
                let arr_str = arrays.iter()
                    .map(|arr| {
                        let dims = arr.dimensions.iter()
                            .map(|d| Self::serialize_expr(d))
                            .collect::<Vec<_>>()
                            .join(",");
                        format!("{}({})", arr.name, dims)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("DIM {}", arr_str)
            }
            Statement::Data { values } => {
                let val_str = values.iter()
                    .map(|v| match v {
                        crate::ast::DataValue::Number(n) => n.to_string(),
                        crate::ast::DataValue::String(s) => format!("\"{}\"", s),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("DATA {}", val_str)
            }
            Statement::Read { variables } => {
                let var_str = variables.iter()
                    .map(|v| Self::serialize_assign_target(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("READ {}", var_str)
            }
            Statement::Restore { line_number } => {
                if let Some(ln) = line_number {
                    format!("RESTORE {}", ln)
                } else {
                    "RESTORE".to_string()
                }
            }
            Statement::Rem => "REM".to_string(),
            Statement::End => "END".to_string(),
            Statement::Stop => "STOP".to_string(),
            Statement::New => "NEW".to_string(),
            Statement::Clear => "CLEAR".to_string(),
            _ => "REM UNSUPPORTED STATEMENT".to_string(),
        }
    }
    
    /// 将表达式序列化为文本
    fn serialize_expr(expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => n.to_string(),
            Expr::String(s) => format!("\"{}\"", s),
            Expr::Variable(name) => name.clone(),
            Expr::ArrayAccess { name, indices } => {
                let idx_str = indices.iter()
                    .map(|i| Self::serialize_expr(i))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, idx_str)
            }
            Expr::FunctionCall { name, args } => {
                let arg_str = args.iter()
                    .map(|a| Self::serialize_expr(a))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, arg_str)
            }
            Expr::BinaryOp { left, op, right } => {
                let op_str = match op {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Subtract => "-",
                    BinaryOperator::Multiply => "*",
                    BinaryOperator::Divide => "/",
                    BinaryOperator::Power => "^",
                    BinaryOperator::Equal => "=",
                    BinaryOperator::NotEqual => "<>",
                    BinaryOperator::Less => "<",
                    BinaryOperator::Greater => ">",
                    BinaryOperator::LessEqual => "<=",
                    BinaryOperator::GreaterEqual => ">=",
                    BinaryOperator::And => " AND ",
                    BinaryOperator::Or => " OR ",
                };
                format!("({} {} {})", Self::serialize_expr(left), op_str, Self::serialize_expr(right))
            }
            Expr::UnaryOp { op, operand } => {
                let op_str = match op {
                    UnaryOperator::Minus => "-",
                    UnaryOperator::Not => "NOT ",
                };
                format!("{}{}", op_str, Self::serialize_expr(operand))
            }
        }
    }
    
    /// 将赋值目标序列化为文本
    fn serialize_assign_target(target: &AssignTarget) -> String {
        match target {
            AssignTarget::Variable(name) => name.clone(),
            AssignTarget::ArrayElement { name, indices } => {
                let idx_str = indices.iter()
                    .map(|i| Self::serialize_expr(i))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, idx_str)
            }
        }
    }
    
    /// 将THEN部分序列化为文本
    fn serialize_then_part(then_part: &ThenPart) -> String {
        match then_part {
            ThenPart::LineNumber(ln) => ln.to_string(),
            ThenPart::Statement(stmt) => Self::serialize_statement(stmt),
            ThenPart::Statements(stmts) => {
                stmts.iter()
                    .map(|s| Self::serialize_statement(s))
                    .collect::<Vec<_>>()
                    .join(":")
            }
        }
    }
    
    /// 将PRINT项序列化为文本
    fn serialize_print_item(item: &PrintItem) -> String {
        match item {
            PrintItem::Expr(expr) => Self::serialize_expr(expr),
            PrintItem::Tab(expr) => format!("TAB({})", Self::serialize_expr(expr)),
            PrintItem::Spc(expr) => format!("SPC({})", Self::serialize_expr(expr)),
            PrintItem::Comma => ",".to_string(),
            PrintItem::Semicolon => ";".to_string(),
        }
    }
    
    /// 执行 LOAD 命令 - 从文件加载程序
    fn execute_load(&mut self, filename: &str) -> Result<()> {
        use std::fs;
        use crate::tokenizer::Tokenizer;
        use crate::parser::Parser;
        
        // 读取文件内容
        let content = fs::read_to_string(filename).map_err(|e| {
            BasicError::SyntaxError(format!("Failed to read file: {}", e))
        })?;
        
        // 清空当前程序
        self.runtime.clear_program();
        self.variables.clear();
        
        // 逐行解析并添加到程序
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            // 使用tokenizer和parser解析每一行
            let mut tokenizer = Tokenizer::new(line);
            let tokens = tokenizer.tokenize_line()?;
            
            let mut parser = Parser::new(tokens);
            if let Some(program_line) = parser.parse_line()? {
                if program_line.line_number > 0 {
                    self.runtime.add_line(program_line);
                }
            }
        }
        
        Ok(())
    }
    
    /// 执行 PRINT 语句
    fn execute_print(&mut self, items: &[PrintItem]) -> Result<()> {
        if items.is_empty() {
            // 空 PRINT，仅输出换行
            self.output_newline();
            return Ok(());
        }
        
        for item in items.iter() {
            match item {
                PrintItem::Expr(expr) => {
                    let val = self.eval_expr(expr)?;
                    self.print_value(&val)?;
                }
                PrintItem::Tab(expr) => {
                    let target_col = self.eval_expr(expr)?.as_number()? as usize;
                    if target_col > self.print_column {
                        let spaces = target_col - self.print_column;
                        self.output(&" ".repeat(spaces));
                    } else if target_col < self.print_column {
                        // TAB 到更小的列，换行后跳转
                        self.output_newline();
                        self.output(&" ".repeat(target_col));
                    }
                }
                PrintItem::Spc(expr) => {
                    let spaces = self.eval_expr(expr)?.as_number()? as usize;
                    self.output(&" ".repeat(spaces));
                }
                PrintItem::Comma => {
                    // 逗号：对齐到下一个 14 列边界
                    let next_col = ((self.print_column / 14) + 1) * 14;
                    let spaces_needed = next_col - self.print_column;
                    self.output(&" ".repeat(spaces_needed));
                }
                PrintItem::Semicolon => {
                    // 分号：不添加空格（紧密连接）
                }
            }
        }
        
        // 检查最后一项是否是分隔符
        if let Some(last) = items.last() {
            if !matches!(last, PrintItem::Comma | PrintItem::Semicolon) {
                // 如果最后不是分隔符，输出换行
                self.output_newline();
            }
        } else {
            self.output_newline();
        }
        
        Ok(())
    }
    
    /// 打印值（根据 BASIC 格式）
    fn print_value(&mut self, val: &Value) -> Result<()> {
        match val {
            Value::Number(n) => {
                // BASIC 数值格式：正数前后各有空格，负数前有空格
                let formatted = if *n >= 0.0 {
                    format!(" {} ", n)
                } else {
                    format!(" {}", n)
                };
                self.output(&formatted);
            }
            Value::String(s) => {
                // 普通字符串，直接输出
                self.output(s);
            }
        }
        Ok(())
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Requirement: 算术运算符 - 加法
    #[test]
    fn test_addition() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Add,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    // Requirement: 算术运算符 - 减法
    #[test]
    fn test_subtraction() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(10.0),
            BinaryOperator::Subtract,
            Expr::Number(7.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    // Requirement: 算术运算符 - 乘法
    #[test]
    fn test_multiplication() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(4.0),
            BinaryOperator::Multiply,
            Expr::Number(5.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(20.0));
    }

    // Requirement: 算术运算符 - 除法
    #[test]
    fn test_division() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(15.0),
            BinaryOperator::Divide,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    // Requirement: 算术运算符 - 浮点除法
    #[test]
    fn test_float_division() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(10.0),
            BinaryOperator::Divide,
            Expr::Number(4.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(2.5));
    }

    // Requirement: 算术运算符 - 除以零
    #[test]
    fn test_division_by_zero() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Divide,
            Expr::Number(0.0)
        );
        let result = exec.eval_expr(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BasicError::DivisionByZero));
    }

    // Requirement: 算术运算符 - 乘方
    #[test]
    fn test_power() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(2.0),
            BinaryOperator::Power,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    // Requirement: 一元运算符 - 一元负号
    #[test]
    fn test_unary_minus() {
        let mut exec = Executor::new();
        let expr = Expr::unary(UnaryOperator::Minus, Expr::Number(5.0));
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-5.0));
    }

    // Requirement: 关系运算符 - 等于
    #[test]
    fn test_equal() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Equal,
            Expr::Number(5.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-1.0)); // BASIC true = -1
    }

    // Requirement: 关系运算符 - 不等于
    #[test]
    fn test_not_equal() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::NotEqual,
            Expr::Number(4.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-1.0));
    }

    // Requirement: 字符串运算符 - 字符串连接
    #[test]
    fn test_string_concatenation() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::String("HELLO".to_string()),
            BinaryOperator::Add,
            Expr::String(" WORLD".to_string())
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::String("HELLO WORLD".to_string()));
    }

    // Test: 变量读取
    #[test]
    fn test_variable_read() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(42.0)).unwrap();
        
        let expr = Expr::Variable("A".to_string());
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    // Test: LET 语句执行
    #[test]
    fn test_let_statement() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Let {
            target: AssignTarget::Variable("X".to_string()),
            value: Expr::Number(100.0),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.variables.get("X"), Value::Number(100.0));
    }

    // Test: DIM 语句执行
    #[test]
    fn test_dim_statement() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Dim {
            arrays: vec![
                ArrayDim {
                    name: "A".to_string(),
                    dimensions: vec![Expr::Number(10.0)],
                }
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert!(exec.variables.has_array("A"));
    }

    // Test: 数学函数
    #[test]
    fn test_math_functions() {
        let mut exec = Executor::new();
        
        // ABS
        let expr = Expr::FunctionCall {
            name: "ABS".to_string(),
            args: vec![Expr::Number(-42.0)],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(42.0));
        
        // INT
        let expr = Expr::FunctionCall {
            name: "INT".to_string(),
            args: vec![Expr::Number(3.7)],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    // Test: 字符串函数
    #[test]
    fn test_string_functions() {
        let mut exec = Executor::new();
        
        // LEN
        let expr = Expr::FunctionCall {
            name: "LEN".to_string(),
            args: vec![Expr::String("HELLO".to_string())],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));
        
        // LEFT$
        let expr = Expr::FunctionCall {
            name: "LEFT$".to_string(),
            args: vec![
                Expr::String("HELLO".to_string()),
                Expr::Number(3.0),
            ],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::String("HEL".to_string()));
    }

    // Test: 复杂表达式
    #[test]
    fn test_complex_expression() {
        let mut exec = Executor::new();
        
        // 2 + 3 * 4 = 14
        let expr = Expr::binary(
            Expr::Number(2.0),
            BinaryOperator::Add,
            Expr::binary(
                Expr::Number(3.0),
                BinaryOperator::Multiply,
                Expr::Number(4.0)
            )
        );
        
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(14.0));
    }

    // Requirement: PRINT 语句 - 打印数值
    #[test]
    fn test_print_number() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(42.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 42 \n");
    }

    // Requirement: PRINT 语句 - 打印字符串
    #[test]
    fn test_print_string() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("HELLO".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "HELLO\n");
    }

    // Requirement: PRINT 语句 - 打印变量
    #[test]
    fn test_print_variable() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(10.0)).unwrap();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Variable("A".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 10 \n");
    }

    // Requirement: PRINT 语句 - 分号分隔（紧密连接）
    #[test]
    fn test_print_semicolon() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(1.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::Number(2.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::Number(3.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 1  2  3 \n");
    }

    // Requirement: PRINT 语句 - 行尾分号（不换行）
    #[test]
    fn test_print_no_newline() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(42.0)),
                PrintItem::Semicolon,
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 42 ");
    }

    // Requirement: PRINT 语句 - 空 PRINT
    #[test]
    fn test_print_empty() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "\n");
    }

    // Requirement: PRINT 语句 - 逗号分隔（列对齐）
    #[test]
    fn test_print_comma_alignment() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(1.0)),
                PrintItem::Comma,
                PrintItem::Expr(Expr::Number(2.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // 第一个数 " 1 " 占 3 列，逗号应该对齐到第 14 列
        assert!(output.starts_with(" 1 "));
        assert!(output.contains(" 2 "));
    }

    // Requirement: GOTO 语句
    #[test]
    fn test_goto_statement() {
        let mut exec = Executor::new();
        
        // 添加程序行
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("A".to_string()),
                value: Expr::Number(1.0),
            }]
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("B".to_string()),
                value: Expr::Number(99.0),
            }]
        });
        
        let stmt = Statement::Goto {
            line_number: Expr::Number(100.0),
        };
        exec.execute_statement(&stmt).unwrap();
        
        // 验证跳转成功（下一行应该是 100）
        assert_eq!(exec.runtime().get_current_line(), Some(100));
    }

    // Requirement: IF...THEN 语句 - 条件为真
    #[test]
    fn test_if_then_true() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(15.0)).unwrap();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行来设置初始状态
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::LineNumber(100)),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.runtime().get_current_line(), Some(100));
    }

    // Requirement: IF...THEN 语句 - 条件为假
    #[test]
    fn test_if_then_false() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(5.0)).unwrap();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![],
        });
        
        let current_line = exec.runtime().get_current_line();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::LineNumber(100)),
        };
        
        exec.execute_statement(&stmt).unwrap();
        // 条件为假，不应该跳转
        assert_eq!(exec.runtime().get_current_line(), current_line);
    }

    // Requirement: IF...THEN 语句 - THEN 后跟语句
    #[test]
    fn test_if_then_statement() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(15.0)).unwrap();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::Statement(
                Statement::Print {
                    items: vec![
                        PrintItem::Expr(Expr::String("TRUE".to_string())),
                    ],
                }
            )),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "TRUE\n");
    }

    // Test: TAB 函数
    #[test]
    fn test_tab_function() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("A".to_string())),
                PrintItem::Semicolon,
                PrintItem::Tab(Expr::Number(10.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::String("B".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // A 在列 0，TAB(10) 跳到列 10，然后是 B
        assert!(output.starts_with("A"));
        assert!(output.contains("B"));
    }

    // Test: SPC 函数
    #[test]
    fn test_spc_function() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("A".to_string())),
                PrintItem::Semicolon,
                PrintItem::Spc(Expr::Number(5.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::String("B".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // A + 5个空格 + B
        assert_eq!(output, "A     B\n");
    }

    // Requirement: GOSUB 和 RETURN 语句 - 子程序调用
    #[test]
    fn test_gosub_statement() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 500,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        let stmt = Statement::Gosub {
            line_number: Expr::Number(500.0),
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证跳转到子程序
        assert_eq!(exec.runtime().get_current_line(), Some(500));
        // 验证调用栈深度
        assert_eq!(exec.runtime().stack_depth(), 1);
    }

    // Requirement: GOSUB 和 RETURN 语句 - 子程序返回
    #[test]
    fn test_return_statement() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 500,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行并设置调用栈
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        exec.runtime_mut().push_gosub(20, 0).unwrap();
        exec.runtime_mut().set_execution_position(500, 0).unwrap();
        
        let stmt = Statement::Return;
        exec.execute_statement(&stmt).unwrap();
        
        // 验证返回到调用点
        assert_eq!(exec.runtime().get_current_line(), Some(20));
        // 验证调用栈已弹出
        assert_eq!(exec.runtime().stack_depth(), 0);
    }

    // Requirement: GOSUB 和 RETURN 语句 - 嵌套子程序
    #[test]
    fn test_nested_gosub() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 200,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // 第一次 GOSUB
        exec.execute_statement(&Statement::Gosub {
            line_number: Expr::Number(100.0),
        }).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 1);
        
        // 第二次 GOSUB（嵌套）
        exec.execute_statement(&Statement::Gosub {
            line_number: Expr::Number(200.0),
        }).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 2);
        assert_eq!(exec.runtime().get_current_line(), Some(200));
        
        // 第一次 RETURN
        exec.execute_statement(&Statement::Return).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 1);
        assert_eq!(exec.runtime().get_current_line(), Some(100));
        
        // 第二次 RETURN
        exec.execute_statement(&Statement::Return).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 0);
        assert_eq!(exec.runtime().get_current_line(), Some(10));
    }

    // Requirement: INPUT 语句 - 基本输入
    #[test]
    fn test_input_basic() {
        let mut exec = Executor::new();
        
        // 设置输入回调
        exec.set_input_callback(|_| Some("42".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证输出提示符
        assert!(exec.get_output().contains("? "));
        
        // 验证变量赋值
        assert_eq!(exec.variables.get("A"), Value::Number(42.0));
    }

    // Requirement: INPUT 语句 - 带提示符的输入
    #[test]
    fn test_input_with_prompt() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("100".to_string()));
        
        let stmt = Statement::Input {
            prompt: Some("ENTER VALUE".to_string()),
            variables: vec![AssignTarget::Variable("X".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证提示符
        assert!(exec.get_output().contains("ENTER VALUE? "));
        assert_eq!(exec.variables.get("X"), Value::Number(100.0));
    }

    // Requirement: INPUT 语句 - 输入多个变量
    #[test]
    fn test_input_multiple_variables() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("10, 20, 30".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string()),
                AssignTarget::Variable("C".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(10.0));
        assert_eq!(exec.variables.get("B"), Value::Number(20.0));
        assert_eq!(exec.variables.get("C"), Value::Number(30.0));
    }

    // Requirement: INPUT 语句 - 字符串输入
    #[test]
    fn test_input_string() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("HELLO".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A$".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A$"), Value::String("HELLO".to_string()));
    }

    // Requirement: INPUT 语句 - 字符串带引号
    #[test]
    fn test_input_string_with_quotes() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("\"HELLO, WORLD\"".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A$".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A$"), Value::String("HELLO, WORLD".to_string()));
    }

    // Requirement: DATA/READ 机制 - DATA 存储和 READ 读取
    #[test]
    fn test_data_read() {
        let mut exec = Executor::new();
        
        // 添加 DATA 值
        exec.add_data_value(DataValue::Number(1.0));
        exec.add_data_value(DataValue::Number(2.0));
        exec.add_data_value(DataValue::Number(3.0));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string()),
                AssignTarget::Variable("C".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(1.0));
        assert_eq!(exec.variables.get("B"), Value::Number(2.0));
        assert_eq!(exec.variables.get("C"), Value::Number(3.0));
    }

    // Requirement: DATA/READ 机制 - 混合数据类型
    #[test]
    fn test_data_read_mixed_types() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(42.0));
        exec.add_data_value(DataValue::String("HELLO".to_string()));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B$".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(42.0));
        assert_eq!(exec.variables.get("B$"), Value::String("HELLO".to_string()));
    }

    // Requirement: DATA/READ 机制 - OUT OF DATA 错误
    #[test]
    fn test_out_of_data_error() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(1.0));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string())
            ],
        };
        
        let result = exec.execute_statement(&stmt);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BasicError::OutOfData));
    }

    // Requirement: RESTORE 数据指针 - RESTORE 重置到开头
    #[test]
    fn test_restore() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(1.0));
        exec.add_data_value(DataValue::Number(2.0));
        
        // 第一次 READ
        exec.execute_statement(&Statement::Read {
            variables: vec![AssignTarget::Variable("A".to_string())],
        }).unwrap();
        assert_eq!(exec.variables.get("A"), Value::Number(1.0));
        
        // RESTORE
        exec.execute_statement(&Statement::Restore {
            line_number: None,
        }).unwrap();
        
        // 第二次 READ（应该重新从头开始）
        exec.execute_statement(&Statement::Read {
            variables: vec![AssignTarget::Variable("B".to_string())],
        }).unwrap();
        assert_eq!(exec.variables.get("B"), Value::Number(1.0));
    }

    // Requirement: FOR...NEXT 循环 - 正向循环
    #[test]
    fn test_for_next_basic() {
        let mut exec = Executor::new();
        
        // 添加测试程序：FOR I=1 TO 3: PRINT I: NEXT I
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(1.0),
                    end: Expr::Number(3.0),
                    step: None,
                },
            ],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Next { var: Some("I".to_string()) }],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // 第一次循环：I=1
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(1.0),
            end: Expr::Number(3.0),
            step: None,
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
        
        // NEXT：I=2
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=3
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
        
        // NEXT：循环结束 (I递增到4但不再循环)
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        // 循环已结束，变量值应该为循环后的值 4
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
    }

    // Requirement: FOR...NEXT 循环 - 步长为 2
    #[test]
    fn test_for_next_step() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(0.0),
                    end: Expr::Number(4.0),
                    step: Some(Expr::Number(2.0)),
                },
            ],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // FOR I=0 TO 4 STEP 2
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(0.0),
            end: Expr::Number(4.0),
            step: Some(Expr::Number(2.0)),
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(0.0));
        
        // NEXT：I=2
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=4
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(4.0));
        
        // NEXT：循环结束
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(4.0));
    }

    // Requirement: FOR...NEXT 循环 - 负步长
    #[test]
    fn test_for_next_negative_step() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(3.0),
                    end: Expr::Number(1.0),
                    step: Some(Expr::Number(-1.0)),
                },
            ],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // FOR I=3 TO 1 STEP -1
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(3.0),
            end: Expr::Number(1.0),
            step: Some(Expr::Number(-1.0)),
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
        
        // NEXT：I=2
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=1
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
        
        // NEXT：循环结束
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
    }

    // Requirement: ON...GOTO - 基于表达式的跳转
    #[test]
    fn test_on_goto() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 200,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // ON 2 GOTO 100,200,300
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(2.0),
            targets: vec![100, 200, 300],
            is_gosub: false,
        }).unwrap();
        
        // 应该跳转到 200
        assert_eq!(exec.runtime().get_current_line(), Some(200));
    }

    // Requirement: ON...GOSUB - 基于表达式的子程序调用
    #[test]
    fn test_on_gosub() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // ON 1 GOSUB 100,200
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(1.0),
            targets: vec![100, 200],
            is_gosub: true,
        }).unwrap();
        
        // 应该跳转到 100
        assert_eq!(exec.runtime().get_current_line(), Some(100));
        // 栈深度应该为 1
        assert_eq!(exec.runtime().stack_depth(), 1);
    }

    // Requirement: ON...GOTO - 值超出范围
    #[test]
    fn test_on_goto_out_of_range() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        let current_line = exec.runtime().get_current_line();
        
        // ON 5 GOTO 100,200  (5 超出范围)
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(5.0),
            targets: vec![100, 200],
            is_gosub: false,
        }).unwrap();
        
        // 应该继续在当前行
        assert_eq!(exec.runtime().get_current_line(), current_line);
    }
    
    #[test]
    fn test_save_and_load() {
        use std::fs;
        
        let mut exec = Executor::new();
        
        // 添加一些程序行
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Print {
                items: vec![PrintItem::Expr(Expr::String("HELLO".to_string()))],
            }],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("A".to_string()),
                value: Expr::Number(42.0),
            }],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 30,
            statements: vec![Statement::End],
        });
        
        // 保存程序到文件
        let filename = "test_program.bas";
        exec.execute_statement(&Statement::Save {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证文件存在
        assert!(fs::metadata(filename).is_ok());
        
        // 清空程序
        exec.runtime_mut().clear_program();
        assert_eq!(exec.runtime().line_count(), 0);
        
        // 加载程序
        exec.execute_statement(&Statement::Load {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证程序已加载
        assert_eq!(exec.runtime().line_count(), 3);
        assert!(exec.runtime().get_line(10).is_some());
        assert!(exec.runtime().get_line(20).is_some());
        assert!(exec.runtime().get_line(30).is_some());
        
        // 清理测试文件
        fs::remove_file(filename).ok();
    }
    
    #[test]
    fn test_save_empty_program() {
        let mut exec = Executor::new();
        
        // 尝试保存空程序应该失败
        let result = exec.execute_statement(&Statement::Save {
            filename: "empty.bas".to_string(),
        });
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_load_nonexistent_file() {
        let mut exec = Executor::new();
        
        // 尝试加载不存在的文件应该失败
        let result = exec.execute_statement(&Statement::Load {
            filename: "nonexistent.bas".to_string(),
        });
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_save_complex_program() {
        use std::fs;
        
        let mut exec = Executor::new();
        
        // 创建一个更复杂的程序
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(1.0),
                    end: Expr::Number(10.0),
                    step: Some(Expr::Number(1.0)),
                },
            ],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![
                Statement::Print {
                    items: vec![PrintItem::Expr(Expr::Variable("I".to_string()))],
                },
            ],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 30,
            statements: vec![Statement::Next { var: Some("I".to_string()) }],
        });
        
        // 保存并重新加载
        let filename = "test_complex.bas";
        exec.execute_statement(&Statement::Save {
            filename: filename.to_string(),
        }).unwrap();
        
        exec.runtime_mut().clear_program();
        
        exec.execute_statement(&Statement::Load {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证程序结构
        assert_eq!(exec.runtime().line_count(), 3);
        
        // 清理
        fs::remove_file(filename).ok();
    }
}


```

```rust
/// 执行引擎
///
/// 求值表达式并执行语句

use crate::ast::*;
use crate::error::{BasicError, Result};
use crate::runtime::Runtime;
use crate::variables::{Value, Variables};

/// 输入回调函数类型
pub type InputCallback = Box<dyn FnMut(&str) -> Option<String>>;

/// 执行引擎
pub struct Executor {
    runtime: Runtime,
    variables: Variables,
    /// 输出缓冲区（用于测试和捕获输出）
    output_buffer: Vec<String>,
    /// 当前打印列位置
    print_column: usize,
    /// DATA 数据存储
    data_values: Vec<DataValue>,
    /// DATA 数据指针（当前读取位置）
    data_pointer: usize,
    /// 输入回调函数（用于测试）
    input_callback: Option<InputCallback>,
}

/// DATA 值类型
#[derive(Debug, Clone, PartialEq)]
pub enum DataValue {
    Number(f64),
    String(String),
}

impl Executor {
    /// 创建新的执行器
    pub fn new() -> Self {
        Executor {
            runtime: Runtime::new(),
            variables: Variables::new(),
            output_buffer: Vec::new(),
            print_column: 0,
            data_values: Vec::new(),
            data_pointer: 0,
            input_callback: None,
        }
    }
    
    /// 设置输入回调函数（用于测试）
    pub fn set_input_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&str) -> Option<String> + 'static,
    {
        self.input_callback = Some(Box::new(callback));
    }
    
    /// 添加 DATA 值
    pub fn add_data_value(&mut self, value: DataValue) {
        self.data_values.push(value);
    }
    
    /// 重置 DATA 指针
    pub fn restore_data(&mut self) {
        self.data_pointer = 0;
    }
    
    /// 读取下一个 DATA 值
    fn read_data_value(&mut self) -> Result<DataValue> {
        if self.data_pointer >= self.data_values.len() {
            return Err(BasicError::OutOfData);
        }
        
        let value = self.data_values[self.data_pointer].clone();
        self.data_pointer += 1;
        Ok(value)
    }
    
    /// 获取输出内容（用于测试）
    pub fn get_output(&self) -> String {
        self.output_buffer.join("")
    }
    
    /// 清空输出缓冲区
    pub fn clear_output(&mut self) {
        self.output_buffer.clear();
        self.print_column = 0;
    }
    
    /// 输出文本（添加到缓冲区并打印到终端）
    fn output(&mut self, text: &str) {
        // 打印到终端
        print!("{}", text);
        use std::io::Write;
        std::io::stdout().flush().ok();
        
        // 同时添加到缓冲区（用于测试）
        self.output_buffer.push(text.to_string());
        
        // 更新列位置
        for ch in text.chars() {
            if ch == '\n' {
                self.print_column = 0;
            } else {
                self.print_column += 1;
            }
        }
    }
    
    /// 输出换行
    fn output_newline(&mut self) {
        self.output("\n");
    }

    /// 获取运行时引用
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    /// 获取变量存储引用
    pub fn variables(&self) -> &Variables {
        &self.variables
    }

    /// 获取运行时可变引用
    pub fn runtime_mut(&mut self) -> &mut Runtime {
        &mut self.runtime
    }

    /// 获取变量存储可变引用
    pub fn variables_mut(&mut self) -> &mut Variables {
        &mut self.variables
    }

    /// 求值表达式
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            
            Expr::String(s) => Ok(Value::String(s.clone())),
            
            Expr::Variable(name) => {
                Ok(self.variables.get(name))
            }
            
            Expr::ArrayAccess { name, indices } => {
                // 求值所有索引
                let idx_values: Result<Vec<usize>> = indices.iter()
                    .map(|idx_expr| {
                        self.eval_expr(idx_expr)?
                            .as_number()
                            .and_then(|n| {
                                if n < 0.0 {
                                    Err(BasicError::SubscriptOutOfRange(
                                        "Negative array index".to_string()
                                    ))
                                } else {
                                    Ok(n as usize)
                                }
                            })
                    })
                    .collect();
                
                let indices_usize = idx_values?;
                self.variables.get_array_element(name, &indices_usize)
            }
            
            Expr::FunctionCall { name, args } => {
                self.eval_function_call(name, args)
            }
            
            Expr::BinaryOp { left, op, right } => {
                self.eval_binary_op(left, *op, right)
            }
            
            Expr::UnaryOp { op, operand } => {
                self.eval_unary_op(*op, operand)
            }
        }
    }

    /// 求值二元运算
    fn eval_binary_op(&mut self, left: &Expr, op: BinaryOperator, right: &Expr) -> Result<Value> {
        use BinaryOperator::*;

        let left_val = self.eval_expr(left)?;
        let right_val = self.eval_expr(right)?;

        match op {
            // 算术运算符
            Add => {
                if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    Ok(Value::Number(l + r))
                } else if left_val.is_string() && right_val.is_string() {
                    // 字符串连接
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    Ok(Value::String(format!("{}{}", l, r)))
                } else {
                    Err(BasicError::TypeMismatch(
                        "Cannot add incompatible types".to_string()
                    ))
                }
            }
            
            Subtract => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l - r))
            }
            
            Multiply => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l * r))
            }
            
            Divide => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                if r == 0.0 {
                    return Err(BasicError::DivisionByZero);
                }
                Ok(Value::Number(l / r))
            }
            
            Power => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l.powf(r)))
            }
            
            // 关系运算符（BASIC 中 true = -1, false = 0）
            Equal => {
                let result = if left_val == right_val { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            NotEqual => {
                let result = if left_val != right_val { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            Less => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l < r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l < r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            Greater => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l > r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l > r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            LessEqual => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l <= r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l <= r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            GreaterEqual => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l >= r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l >= r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            // 逻辑运算符（按位）
            And => {
                let l = left_val.as_number()? as i32;
                let r = right_val.as_number()? as i32;
                Ok(Value::Number((l & r) as f64))
            }
            
            Or => {
                let l = left_val.as_number()? as i32;
                let r = right_val.as_number()? as i32;
                Ok(Value::Number((l | r) as f64))
            }
        }
    }

    /// 求值一元运算
    fn eval_unary_op(&mut self, op: UnaryOperator, operand: &Expr) -> Result<Value> {
        let val = self.eval_expr(operand)?;
        
        match op {
            UnaryOperator::Minus => {
                let n = val.as_number()?;
                Ok(Value::Number(-n))
            }
            UnaryOperator::Not => {
                let n = val.as_number()? as i32;
                Ok(Value::Number((!n) as f64))
            }
        }
    }

    /// 求值函数调用（内置函数）
    fn eval_function_call(&mut self, name: &str, args: &[Expr]) -> Result<Value> {
        match name.to_uppercase().as_str() {
            // 数学函数
            "SGN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SGN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                let result = if n > 0.0 { 1.0 } else if n < 0.0 { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            "INT" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("INT requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.floor()))
            }
            
            "ABS" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ABS requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.abs()))
            }
            
            "SQR" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SQR requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n < 0.0 {
                    return Err(BasicError::IllegalQuantity("SQR of negative number".to_string()));
                }
                Ok(Value::Number(n.sqrt()))
            }
            
            "SIN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SIN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.sin()))
            }
            
            "COS" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("COS requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.cos()))
            }
            
            "TAN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("TAN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.tan()))
            }
            
            "ATN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ATN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.atan()))
            }
            
            "LOG" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("LOG requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n <= 0.0 {
                    return Err(BasicError::IllegalQuantity("LOG of non-positive number".to_string()));
                }
                Ok(Value::Number(n.ln()))
            }
            
            "EXP" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("EXP requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.exp()))
            }
            
            // 字符串函数
            "LEN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("LEN requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                Ok(Value::Number(s.len() as f64))
            }
            
            "ASC" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ASC requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                if s.is_empty() {
                    return Err(BasicError::IllegalQuantity("ASC of empty string".to_string()));
                }
                Ok(Value::Number(s.chars().next().unwrap() as u8 as f64))
            }
            
            "CHR$" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("CHR$ requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n < 0.0 || n > 255.0 {
                    return Err(BasicError::IllegalQuantity("CHR$ argument out of range".to_string()));
                }
                let ch = n as u8 as char;
                Ok(Value::String(ch.to_string()))
            }
            
            "STR$" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("STR$ requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                // BASIC 的 STR$ 在正数前加空格
                let s = if n >= 0.0 {
                    format!(" {}", n)
                } else {
                    n.to_string()
                };
                Ok(Value::String(s))
            }
            
            "VAL" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("VAL requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = s.trim().parse::<f64>().unwrap_or(0.0);
                Ok(Value::Number(n))
            }
            
            "LEFT$" => {
                if args.len() != 2 {
                    return Err(BasicError::SyntaxError("LEFT$ requires 2 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = self.eval_expr(&args[1])?.as_number()? as usize;
                let result: String = s.chars().take(n).collect();
                Ok(Value::String(result))
            }
            
            "RIGHT$" => {
                if args.len() != 2 {
                    return Err(BasicError::SyntaxError("RIGHT$ requires 2 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = self.eval_expr(&args[1])?.as_number()? as usize;
                let len = s.chars().count();
                let skip = if n > len { 0 } else { len - n };
                let result: String = s.chars().skip(skip).collect();
                Ok(Value::String(result))
            }
            
            "MID$" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err(BasicError::SyntaxError("MID$ requires 2 or 3 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let start = self.eval_expr(&args[1])?.as_number()? as usize;
                
                // BASIC 的 MID$ 是 1-based
                let start = if start > 0 { start - 1 } else { 0 };
                
                let chars: Vec<char> = s.chars().collect();
                
                if args.len() == 3 {
                    let len = self.eval_expr(&args[2])?.as_number()? as usize;
                    let result: String = chars.iter().skip(start).take(len).collect();
                    Ok(Value::String(result))
                } else {
                    let result: String = chars.iter().skip(start).collect();
                    Ok(Value::String(result))
                }
            }
            
            _ => Err(BasicError::SyntaxError(
                format!("Unknown function: {}", name)
            )),
        }
    }

    /// 执行语句
    pub fn execute_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Let { target, value } => {
                let val = self.eval_expr(value)?;
                
                match target {
                    AssignTarget::Variable(name) => {
                        self.variables.set(name, val)?;
                    }
                    AssignTarget::ArrayElement { name, indices } => {
                        let idx_values: Result<Vec<usize>> = indices.iter()
                            .map(|idx_expr| {
                                self.eval_expr(idx_expr)?
                                    .as_number()
                                    .map(|n| n as usize)
                            })
                            .collect();
                        
                        let indices_usize = idx_values?;
                        self.variables.set_array_element(name, &indices_usize, val)?;
                    }
                }
                
                Ok(())
            }
            
            Statement::End => {
                self.runtime.end_execution();
                Ok(())
            }
            
            Statement::Stop => {
                self.runtime.pause_execution();
                Ok(())
            }
            
            Statement::New => {
                self.runtime.clear_program();
                self.variables.clear();
                Ok(())
            }
            
            Statement::Clear => {
                self.variables.clear();
                Ok(())
            }
            
            Statement::Dim { arrays } => {
                for arr_dim in arrays {
                    let dimensions: Result<Vec<usize>> = arr_dim.dimensions.iter()
                        .map(|dim_expr| {
                            self.eval_expr(dim_expr)?
                                .as_number()
                                .map(|n| n as usize)
                        })
                        .collect();
                    
                    let dims = dimensions?;
                    self.variables.dim_array(&arr_dim.name, dims)?;
                }
                Ok(())
            }
            
            Statement::Print { items } => {
                self.execute_print(items)?;
                Ok(())
            }
            
            Statement::Goto { line_number } => {
                let line_val = self.eval_expr(line_number)?;
                let line = line_val.as_number()? as u16;
                self.runtime.set_execution_position(line, 0)?;
                Ok(())
            }
            
            Statement::If { condition, then_part } => {
                let cond_val = self.eval_expr(condition)?;
                let cond_num = cond_val.as_number()?;
                
                // BASIC 中，任何非零值都是真
                if cond_num != 0.0 {
                    match then_part.as_ref() {
                        ThenPart::LineNumber(line) => {
                            self.runtime.set_execution_position(*line as u16, 0)?;
                        }
                        ThenPart::Statement(stmt) => {
                            self.execute_statement(stmt)?;
                        }
                        ThenPart::Statements(stmts) => {
                            for stmt in stmts {
                                self.execute_statement(stmt)?;
                            }
                        }
                    }
                }
                Ok(())
            }
            
            Statement::Gosub { line_number } => {
                // 保存返回地址（当前行号和语句索引）
                let return_line = self.runtime.get_current_line().unwrap_or(0);
                let return_stmt = 0; // 简化：返回到下一行的第一条语句
                
                // 入栈
                self.runtime.push_gosub(return_line, return_stmt)?;
                
                // 跳转到子程序
                let line_val = self.eval_expr(line_number)?;
                let line = line_val.as_number()? as u16;
                self.runtime.set_execution_position(line, 0)?;
                
                Ok(())
            }
            
            Statement::Return => {
                // 从栈中弹出返回地址
                let (return_line, return_stmt) = self.runtime.pop_gosub()?;
                
                // 跳转回返回地址
                self.runtime.set_execution_position(return_line, return_stmt)?;
                
                Ok(())
            }
            
            Statement::Input { prompt, variables } => {
                // 提取变量名
                let var_names: Vec<String> = variables.iter()
                    .map(|target| match target {
                        AssignTarget::Variable(name) => name.clone(),
                        AssignTarget::ArrayElement { .. } => {
                            // INPUT 不支持数组元素
                            String::new()
                        }
                    })
                    .collect();
                
                self.execute_input(prompt.as_deref(), &var_names)?;
                Ok(())
            }
            
            Statement::Data { values: _ } => {
                // DATA 语句在程序加载时处理，执行时跳过
                // 数据已经存储在 data_values 中
                Ok(())
            }
            
            Statement::Read { variables } => {
                for target in variables {
                    let var_name = match target {
                        AssignTarget::Variable(name) => name.as_str(),
                        AssignTarget::ArrayElement { .. } => {
                            return Err(BasicError::SyntaxError(
                                "READ does not support array elements".to_string()
                            ));
                        }
                    };
                    
                    let data_val = self.read_data_value()?;
                    
                    // 根据变量名判断类型
                    if var_name.ends_with('$') {
                        // 字符串变量
                        let str_val = match data_val {
                            DataValue::String(s) => s,
                            DataValue::Number(n) => n.to_string(),
                        };
                        self.variables.set(var_name, Value::String(str_val))?;
                    } else {
                        // 数值变量
                        let num_val = match data_val {
                            DataValue::Number(n) => n,
                            DataValue::String(s) => {
                                s.trim().parse::<f64>().unwrap_or(0.0)
                            }
                        };
                        self.variables.set(var_name, Value::Number(num_val))?;
                    }
                }
                Ok(())
            }
            
            Statement::Restore { line_number } => {
                if line_number.is_some() {
                    // RESTORE 到指定行（暂不支持，需要跟踪每行的 DATA 位置）
                    return Err(BasicError::SyntaxError(
                        "RESTORE to specific line not yet implemented".to_string()
                    ));
                }
                self.restore_data();
                Ok(())
            }
            
            Statement::For { var, start, end, step } => {
                // 计算起始值、结束值和步长
                let start_val = self.eval_expr(&start)?;
                let end_val = self.eval_expr(&end)?;
                let step_val = if let Some(ref s) = step {
                    self.eval_expr(s)?
                } else {
                    Value::Number(1.0)
                };
                
                // 提取数值
                let start_num = start_val.as_number()?;
                let end_num = end_val.as_number()?;
                let step_num = step_val.as_number()?;
                
                // 检查步长
                if step_num == 0.0 {
                    return Err(BasicError::IllegalQuantity(
                        "FOR loop step cannot be zero".to_string()
                    ));
                }
                
                // 设置循环变量初值
                self.variables.set(var, Value::Number(start_num))?;
                
                // 获取当前位置
                let loop_line = self.runtime.get_current_line()
                    .ok_or_else(|| BasicError::SyntaxError("FOR without line number".to_string()))?;
                let loop_stmt = self.runtime.get_current_stmt_index();
                
                // 将循环信息压入栈
                self.runtime.push_for_loop(
                    var.clone(),
                    end_num,
                    step_num,
                    loop_line,
                    loop_stmt,
                )?;
                
                Ok(())
            }
            
            Statement::Next { var } => {
                // 弹出 FOR 循环信息
                let (loop_var, end_val, step_val, loop_line, loop_stmt) = 
                    self.runtime.pop_for_loop(var.clone())?;
                
                // 获取当前循环变量的值
                let current_val = self.variables.get(&loop_var).as_number()?;
                
                // 递增/递减
                let new_val = current_val + step_val;
                
                // 检查是否继续循环
                let should_continue = if step_val > 0.0 {
                    new_val <= end_val
                } else {
                    new_val >= end_val
                };
                
                if should_continue {
                    // 更新循环变量
                    self.variables.set(&loop_var, Value::Number(new_val))?;
                    
                    // 重新压入栈（继续循环）
                    self.runtime.push_for_loop(
                        loop_var.clone(),
                        end_val,
                        step_val,
                        loop_line,
                        loop_stmt,
                    )?;
                    
                    // 跳转回 FOR 语句的下一条语句
                    self.runtime.set_execution_position(loop_line, loop_stmt + 1)?;
                }
                // 否则继续执行下一条语句（循环结束）
                
                Ok(())
            }
            
            Statement::On { expr, targets, is_gosub } => {
                // 计算表达式的值
                let index_val = self.eval_expr(&expr)?;
                let index = index_val.as_number()? as i32;
                
                // 索引从 1 开始
                if index < 1 || index as usize > targets.len() {
                    // 超出范围，继续执行下一条语句
                    return Ok(());
                }
                
                // 获取目标行号
                let target_line = targets[(index - 1) as usize];
                
                if *is_gosub {
                    // ON...GOSUB：保存返回地址并跳转
                    let return_line = self.runtime.get_current_line()
                        .ok_or_else(|| BasicError::SyntaxError("GOSUB without line number".to_string()))?;
                    let return_stmt = self.runtime.get_current_stmt_index();
                    
                    self.runtime.push_gosub(return_line, return_stmt)?;
                    self.runtime.set_execution_position(target_line, 0)?;
                } else {
                    // ON...GOTO：直接跳转
                    self.runtime.set_execution_position(target_line, 0)?;
                }
                
                Ok(())
            }
            
            Statement::Load { filename } => {
                self.execute_load(filename)?;
                Ok(())
            }
            
            Statement::Save { filename } => {
                self.execute_save(filename)?;
                Ok(())
            }
            
            _ => {
                // 其他语句暂未实现
                Err(BasicError::SyntaxError(
                    "Statement not yet implemented".to_string()
                ))
            }
        }
    }
    
    /// 执行 INPUT 语句
    fn execute_input(&mut self, prompt: Option<&str>, variables: &[String]) -> Result<()> {
        // 显示提示符
        if let Some(p) = prompt {
            self.output(p);
            self.output("? ");
        } else {
            self.output("? ");
        }
        
        // 读取输入
        let input_line = if let Some(ref mut callback) = self.input_callback {
            let prompt_text = prompt.unwrap_or("");
            callback(prompt_text).ok_or_else(|| {
                BasicError::SyntaxError("No input provided".to_string())
            })?
        } else {
            // 在实际 REPL 中，这里会从 stdin 读取
            return Err(BasicError::SyntaxError(
                "No input callback set".to_string()
            ));
        };
        
        // 解析输入值（考虑引号内的逗号）
        let values = Self::parse_input_values(&input_line);
        
        if values.len() != variables.len() {
            self.output("?EXTRA IGNORED\n");
        }
        
        // 赋值给变量
        for (i, var_name) in variables.iter().enumerate() {
            if i >= values.len() {
                break;
            }
            
            let input_val = &values[i];
            
            if var_name.ends_with('$') {
                // 字符串变量
                let str_val = if input_val.starts_with('"') && input_val.ends_with('"') {
                    // 去掉引号
                    input_val[1..input_val.len()-1].to_string()
                } else {
                    input_val.clone()
                };
                self.variables.set(var_name, Value::String(str_val))?;
            } else {
                // 数值变量
                match input_val.parse::<f64>() {
                    Ok(num) => {
                        self.variables.set(var_name, Value::Number(num))?;
                    }
                    Err(_) => {
                        self.output("?REDO FROM START\n");
                        return Err(BasicError::TypeMismatch(
                            "Invalid number input".to_string()
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// 解析输入值，处理带引号的字符串
    fn parse_input_values(input: &str) -> Vec<String> {
        let mut values = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        
        for ch in input.chars() {
            match ch {
                '"' => {
                    in_quotes = !in_quotes;
                    current.push(ch);
                }
                ',' if !in_quotes => {
                    values.push(current.trim().to_string());
                    current.clear();
                }
                _ => {
                    current.push(ch);
                }
            }
        }
        
        if !current.is_empty() || input.ends_with(',') {
            values.push(current.trim().to_string());
        }
        
        values
    }
    
    /// 执行 SAVE 命令 - 保存程序到文件
    fn execute_save(&self, filename: &str) -> Result<()> {
        use std::fs::File;
        use std::io::Write;
        
        let program = self.runtime.clone_program();
        if program.is_empty() {
            return Err(BasicError::SyntaxError("No program to save".to_string()));
        }
        
        let mut file = File::create(filename).map_err(|e| {
            BasicError::SyntaxError(format!("Failed to create file: {}", e))
        })?;
        
        for (_, line) in program.iter() {
            let line_text = Self::serialize_program_line(line);
            writeln!(file, "{}", line_text).map_err(|e| {
                BasicError::SyntaxError(format!("Failed to write to file: {}", e))
            })?;
        }
        
        Ok(())
    }
    
    /// 将程序行序列化为文本
    pub fn serialize_program_line(line: &ProgramLine) -> String {
        let mut result = format!("{}", line.line_number);
        
        for (i, stmt) in line.statements.iter().enumerate() {
            if i > 0 {
                result.push_str(":");
            }
            result.push(' ');
            result.push_str(&Self::serialize_statement(stmt));
        }
        
        result
    }
    
    /// 将语句序列化为文本
    pub fn serialize_statement(stmt: &Statement) -> String {
        match stmt {
            Statement::Let { target, value } => {
                format!("{} = {}", Self::serialize_assign_target(target), Self::serialize_expr(value))
            }
            Statement::Print { items } => {
                let mut result = "PRINT ".to_string();
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        result.push_str(&Self::serialize_print_item(item));
                    } else {
                        result.push_str(&Self::serialize_print_item(item));
                    }
                }
                result
            }
            Statement::If { condition, then_part } => {
                format!("IF {} THEN {}", Self::serialize_expr(condition), Self::serialize_then_part(then_part))
            }
            Statement::Goto { line_number } => {
                format!("GOTO {}", Self::serialize_expr(line_number))
            }
            Statement::Gosub { line_number } => {
                format!("GOSUB {}", Self::serialize_expr(line_number))
            }
            Statement::Return => "RETURN".to_string(),
            Statement::For { var, start, end, step } => {
                let mut result = format!("FOR {} = {} TO {}", var, Self::serialize_expr(start), Self::serialize_expr(end));
                if let Some(step_expr) = step {
                    result.push_str(&format!(" STEP {}", Self::serialize_expr(step_expr)));
                }
                result
            }
            Statement::Next { var } => {
                if let Some(v) = var {
                    format!("NEXT {}", v)
                } else {
                    "NEXT".to_string()
                }
            }
            Statement::On { expr, targets, is_gosub } => {
                let keyword = if *is_gosub { "GOSUB" } else { "GOTO" };
                let target_str = targets.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("ON {} {} {}", Self::serialize_expr(expr), keyword, target_str)
            }
            Statement::Input { prompt, variables } => {
                let mut result = "INPUT ".to_string();
                if let Some(p) = prompt {
                    result.push_str(&format!("\"{}\" ", p));
                }
                let var_str = variables.iter()
                    .map(|v| Self::serialize_assign_target(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                result.push_str(&var_str);
                result
            }
            Statement::Dim { arrays } => {
                let arr_str = arrays.iter()
                    .map(|arr| {
                        let dims = arr.dimensions.iter()
                            .map(|d| Self::serialize_expr(d))
                            .collect::<Vec<_>>()
                            .join(",");
                        format!("{}({})", arr.name, dims)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("DIM {}", arr_str)
            }
            Statement::Data { values } => {
                let val_str = values.iter()
                    .map(|v| match v {
                        crate::ast::DataValue::Number(n) => n.to_string(),
                        crate::ast::DataValue::String(s) => format!("\"{}\"", s),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("DATA {}", val_str)
            }
            Statement::Read { variables } => {
                let var_str = variables.iter()
                    .map(|v| Self::serialize_assign_target(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("READ {}", var_str)
            }
            Statement::Restore { line_number } => {
                if let Some(ln) = line_number {
                    format!("RESTORE {}", ln)
                } else {
                    "RESTORE".to_string()
                }
            }
            Statement::Rem => "REM".to_string(),
            Statement::End => "END".to_string(),
            Statement::Stop => "STOP".to_string(),
            Statement::New => "NEW".to_string(),
            Statement::Clear => "CLEAR".to_string(),
            _ => "REM UNSUPPORTED STATEMENT".to_string(),
        }
    }
    
    /// 将表达式序列化为文本
    fn serialize_expr(expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => n.to_string(),
            Expr::String(s) => format!("\"{}\"", s),
            Expr::Variable(name) => name.clone(),
            Expr::ArrayAccess { name, indices } => {
                let idx_str = indices.iter()
                    .map(|i| Self::serialize_expr(i))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, idx_str)
            }
            Expr::FunctionCall { name, args } => {
                let arg_str = args.iter()
                    .map(|a| Self::serialize_expr(a))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, arg_str)
            }
            Expr::BinaryOp { left, op, right } => {
                let op_str = match op {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Subtract => "-",
                    BinaryOperator::Multiply => "*",
                    BinaryOperator::Divide => "/",
                    BinaryOperator::Power => "^",
                    BinaryOperator::Equal => "=",
                    BinaryOperator::NotEqual => "<>",
                    BinaryOperator::Less => "<",
                    BinaryOperator::Greater => ">",
                    BinaryOperator::LessEqual => "<=",
                    BinaryOperator::GreaterEqual => ">=",
                    BinaryOperator::And => " AND ",
                    BinaryOperator::Or => " OR ",
                };
                format!("({} {} {})", Self::serialize_expr(left), op_str, Self::serialize_expr(right))
            }
            Expr::UnaryOp { op, operand } => {
                let op_str = match op {
                    UnaryOperator::Minus => "-",
                    UnaryOperator::Not => "NOT ",
                };
                format!("{}{}", op_str, Self::serialize_expr(operand))
            }
        }
    }
    
    /// 将赋值目标序列化为文本
    fn serialize_assign_target(target: &AssignTarget) -> String {
        match target {
            AssignTarget::Variable(name) => name.clone(),
            AssignTarget::ArrayElement { name, indices } => {
                let idx_str = indices.iter()
                    .map(|i| Self::serialize_expr(i))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, idx_str)
            }
        }
    }
    
    /// 将THEN部分序列化为文本
    fn serialize_then_part(then_part: &ThenPart) -> String {
        match then_part {
            ThenPart::LineNumber(ln) => ln.to_string(),
            ThenPart::Statement(stmt) => Self::serialize_statement(stmt),
            ThenPart::Statements(stmts) => {
                stmts.iter()
                    .map(|s| Self::serialize_statement(s))
                    .collect::<Vec<_>>()
                    .join(":")
            }
        }
    }
    
    /// 将PRINT项序列化为文本
    fn serialize_print_item(item: &PrintItem) -> String {
        match item {
            PrintItem::Expr(expr) => Self::serialize_expr(expr),
            PrintItem::Tab(expr) => format!("TAB({})", Self::serialize_expr(expr)),
            PrintItem::Spc(expr) => format!("SPC({})", Self::serialize_expr(expr)),
            PrintItem::Comma => ",".to_string(),
            PrintItem::Semicolon => ";".to_string(),
        }
    }
    
    /// 执行 LOAD 命令 - 从文件加载程序
    fn execute_load(&mut self, filename: &str) -> Result<()> {
        use std::fs;
        use crate::tokenizer::Tokenizer;
        use crate::parser::Parser;
        
        // 读取文件内容
        let content = fs::read_to_string(filename).map_err(|e| {
            BasicError::SyntaxError(format!("Failed to read file: {}", e))
        })?;
        
        // 清空当前程序
        self.runtime.clear_program();
        self.variables.clear();
        
        // 逐行解析并添加到程序
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            // 使用tokenizer和parser解析每一行
            let mut tokenizer = Tokenizer::new(line);
            let tokens = tokenizer.tokenize_line()?;
            
            let mut parser = Parser::new(tokens);
            if let Some(program_line) = parser.parse_line()? {
                if program_line.line_number > 0 {
                    self.runtime.add_line(program_line);
                }
            }
        }
        
        Ok(())
    }
    
    /// 执行 PRINT 语句
    fn execute_print(&mut self, items: &[PrintItem]) -> Result<()> {
        if items.is_empty() {
            // 空 PRINT，仅输出换行
            self.output_newline();
            return Ok(());
        }
        
        for item in items.iter() {
            match item {
                PrintItem::Expr(expr) => {
                    let val = self.eval_expr(expr)?;
                    self.print_value(&val)?;
                }
                PrintItem::Tab(expr) => {
                    let target_col = self.eval_expr(expr)?.as_number()? as usize;
                    if target_col > self.print_column {
                        let spaces = target_col - self.print_column;
                        self.output(&" ".repeat(spaces));
                    } else if target_col < self.print_column {
                        // TAB 到更小的列，换行后跳转
                        self.output_newline();
                        self.output(&" ".repeat(target_col));
                    }
                }
                PrintItem::Spc(expr) => {
                    let spaces = self.eval_expr(expr)?.as_number()? as usize;
                    self.output(&" ".repeat(spaces));
                }
                PrintItem::Comma => {
                    // 逗号：对齐到下一个 14 列边界
                    let next_col = ((self.print_column / 14) + 1) * 14;
                    let spaces_needed = next_col - self.print_column;
                    self.output(&" ".repeat(spaces_needed));
                }
                PrintItem::Semicolon => {
                    // 分号：不添加空格（紧密连接）
                }
            }
        }
        
        // 检查最后一项是否是分隔符
        if let Some(last) = items.last() {
            if !matches!(last, PrintItem::Comma | PrintItem::Semicolon) {
                // 如果最后不是分隔符，输出换行
                self.output_newline();
            }
        } else {
            self.output_newline();
        }
        
        Ok(())
    }
    
    /// 打印值（根据 BASIC 格式）
    fn print_value(&mut self, val: &Value) -> Result<()> {
        match val {
            Value::Number(n) => {
                // BASIC 数值格式：正数前后各有空格，负数前有空格
                let formatted = if *n >= 0.0 {
                    format!(" {} ", n)
                } else {
                    format!(" {}", n)
                };
                self.output(&formatted);
            }
            Value::String(s) => {
                // 普通字符串，直接输出
                self.output(s);
            }
        }
        Ok(())
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Requirement: 算术运算符 - 加法
    #[test]
    fn test_addition() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Add,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    // Requirement: 算术运算符 - 减法
    #[test]
    fn test_subtraction() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(10.0),
            BinaryOperator::Subtract,
            Expr::Number(7.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    // Requirement: 算术运算符 - 乘法
    #[test]
    fn test_multiplication() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(4.0),
            BinaryOperator::Multiply,
            Expr::Number(5.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(20.0));
    }

    // Requirement: 算术运算符 - 除法
    #[test]
    fn test_division() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(15.0),
            BinaryOperator::Divide,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    // Requirement: 算术运算符 - 浮点除法
    #[test]
    fn test_float_division() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(10.0),
            BinaryOperator::Divide,
            Expr::Number(4.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(2.5));
    }

    // Requirement: 算术运算符 - 除以零
    #[test]
    fn test_division_by_zero() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Divide,
            Expr::Number(0.0)
        );
        let result = exec.eval_expr(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BasicError::DivisionByZero));
    }

    // Requirement: 算术运算符 - 乘方
    #[test]
    fn test_power() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(2.0),
            BinaryOperator::Power,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    // Requirement: 一元运算符 - 一元负号
    #[test]
    fn test_unary_minus() {
        let mut exec = Executor::new();
        let expr = Expr::unary(UnaryOperator::Minus, Expr::Number(5.0));
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-5.0));
    }

    // Requirement: 关系运算符 - 等于
    #[test]
    fn test_equal() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Equal,
            Expr::Number(5.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-1.0)); // BASIC true = -1
    }

    // Requirement: 关系运算符 - 不等于
    #[test]
    fn test_not_equal() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::NotEqual,
            Expr::Number(4.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-1.0));
    }

    // Requirement: 字符串运算符 - 字符串连接
    #[test]
    fn test_string_concatenation() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::String("HELLO".to_string()),
            BinaryOperator::Add,
            Expr::String(" WORLD".to_string())
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::String("HELLO WORLD".to_string()));
    }

    // Test: 变量读取
    #[test]
    fn test_variable_read() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(42.0)).unwrap();
        
        let expr = Expr::Variable("A".to_string());
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    // Test: LET 语句执行
    #[test]
    fn test_let_statement() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Let {
            target: AssignTarget::Variable("X".to_string()),
            value: Expr::Number(100.0),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.variables.get("X"), Value::Number(100.0));
    }

    // Test: DIM 语句执行
    #[test]
    fn test_dim_statement() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Dim {
            arrays: vec![
                ArrayDim {
                    name: "A".to_string(),
                    dimensions: vec![Expr::Number(10.0)],
                }
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert!(exec.variables.has_array("A"));
    }

    // Test: 数学函数
    #[test]
    fn test_math_functions() {
        let mut exec = Executor::new();
        
        // ABS
        let expr = Expr::FunctionCall {
            name: "ABS".to_string(),
            args: vec![Expr::Number(-42.0)],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(42.0));
        
        // INT
        let expr = Expr::FunctionCall {
            name: "INT".to_string(),
            args: vec![Expr::Number(3.7)],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    // Test: 字符串函数
    #[test]
    fn test_string_functions() {
        let mut exec = Executor::new();
        
        // LEN
        let expr = Expr::FunctionCall {
            name: "LEN".to_string(),
            args: vec![Expr::String("HELLO".to_string())],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));
        
        // LEFT$
        let expr = Expr::FunctionCall {
            name: "LEFT$".to_string(),
            args: vec![
                Expr::String("HELLO".to_string()),
                Expr::Number(3.0),
            ],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::String("HEL".to_string()));
    }

    // Test: 复杂表达式
    #[test]
    fn test_complex_expression() {
        let mut exec = Executor::new();
        
        // 2 + 3 * 4 = 14
        let expr = Expr::binary(
            Expr::Number(2.0),
            BinaryOperator::Add,
            Expr::binary(
                Expr::Number(3.0),
                BinaryOperator::Multiply,
                Expr::Number(4.0)
            )
        );
        
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(14.0));
    }

    // Requirement: PRINT 语句 - 打印数值
    #[test]
    fn test_print_number() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(42.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 42 \n");
    }

    // Requirement: PRINT 语句 - 打印字符串
    #[test]
    fn test_print_string() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("HELLO".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "HELLO\n");
    }

    // Requirement: PRINT 语句 - 打印变量
    #[test]
    fn test_print_variable() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(10.0)).unwrap();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Variable("A".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 10 \n");
    }

    // Requirement: PRINT 语句 - 分号分隔（紧密连接）
    #[test]
    fn test_print_semicolon() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(1.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::Number(2.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::Number(3.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 1  2  3 \n");
    }

    // Requirement: PRINT 语句 - 行尾分号（不换行）
    #[test]
    fn test_print_no_newline() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(42.0)),
                PrintItem::Semicolon,
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 42 ");
    }

    // Requirement: PRINT 语句 - 空 PRINT
    #[test]
    fn test_print_empty() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "\n");
    }

    // Requirement: PRINT 语句 - 逗号分隔（列对齐）
    #[test]
    fn test_print_comma_alignment() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(1.0)),
                PrintItem::Comma,
                PrintItem::Expr(Expr::Number(2.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // 第一个数 " 1 " 占 3 列，逗号应该对齐到第 14 列
        assert!(output.starts_with(" 1 "));
        assert!(output.contains(" 2 "));
    }

    // Requirement: GOTO 语句
    #[test]
    fn test_goto_statement() {
        let mut exec = Executor::new();
        
        // 添加程序行
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("A".to_string()),
                value: Expr::Number(1.0),
            }]
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("B".to_string()),
                value: Expr::Number(99.0),
            }]
        });
        
        let stmt = Statement::Goto {
            line_number: Expr::Number(100.0),
        };
        exec.execute_statement(&stmt).unwrap();
        
        // 验证跳转成功（下一行应该是 100）
        assert_eq!(exec.runtime().get_current_line(), Some(100));
    }

    // Requirement: IF...THEN 语句 - 条件为真
    #[test]
    fn test_if_then_true() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(15.0)).unwrap();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行来设置初始状态
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::LineNumber(100)),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.runtime().get_current_line(), Some(100));
    }

    // Requirement: IF...THEN 语句 - 条件为假
    #[test]
    fn test_if_then_false() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(5.0)).unwrap();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![],
        });
        
        let current_line = exec.runtime().get_current_line();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::LineNumber(100)),
        };
        
        exec.execute_statement(&stmt).unwrap();
        // 条件为假，不应该跳转
        assert_eq!(exec.runtime().get_current_line(), current_line);
    }

    // Requirement: IF...THEN 语句 - THEN 后跟语句
    #[test]
    fn test_if_then_statement() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(15.0)).unwrap();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::Statement(
                Statement::Print {
                    items: vec![
                        PrintItem::Expr(Expr::String("TRUE".to_string())),
                    ],
                }
            )),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "TRUE\n");
    }

    // Test: TAB 函数
    #[test]
    fn test_tab_function() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("A".to_string())),
                PrintItem::Semicolon,
                PrintItem::Tab(Expr::Number(10.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::String("B".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // A 在列 0，TAB(10) 跳到列 10，然后是 B
        assert!(output.starts_with("A"));
        assert!(output.contains("B"));
    }

    // Test: SPC 函数
    #[test]
    fn test_spc_function() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("A".to_string())),
                PrintItem::Semicolon,
                PrintItem::Spc(Expr::Number(5.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::String("B".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // A + 5个空格 + B
        assert_eq!(output, "A     B\n");
    }

    // Requirement: GOSUB 和 RETURN 语句 - 子程序调用
    #[test]
    fn test_gosub_statement() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 500,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        let stmt = Statement::Gosub {
            line_number: Expr::Number(500.0),
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证跳转到子程序
        assert_eq!(exec.runtime().get_current_line(), Some(500));
        // 验证调用栈深度
        assert_eq!(exec.runtime().stack_depth(), 1);
    }

    // Requirement: GOSUB 和 RETURN 语句 - 子程序返回
    #[test]
    fn test_return_statement() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 500,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行并设置调用栈
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        exec.runtime_mut().push_gosub(20, 0).unwrap();
        exec.runtime_mut().set_execution_position(500, 0).unwrap();
        
        let stmt = Statement::Return;
        exec.execute_statement(&stmt).unwrap();
        
        // 验证返回到调用点
        assert_eq!(exec.runtime().get_current_line(), Some(20));
        // 验证调用栈已弹出
        assert_eq!(exec.runtime().stack_depth(), 0);
    }

    // Requirement: GOSUB 和 RETURN 语句 - 嵌套子程序
    #[test]
    fn test_nested_gosub() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 200,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // 第一次 GOSUB
        exec.execute_statement(&Statement::Gosub {
            line_number: Expr::Number(100.0),
        }).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 1);
        
        // 第二次 GOSUB（嵌套）
        exec.execute_statement(&Statement::Gosub {
            line_number: Expr::Number(200.0),
        }).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 2);
        assert_eq!(exec.runtime().get_current_line(), Some(200));
        
        // 第一次 RETURN
        exec.execute_statement(&Statement::Return).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 1);
        assert_eq!(exec.runtime().get_current_line(), Some(100));
        
        // 第二次 RETURN
        exec.execute_statement(&Statement::Return).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 0);
        assert_eq!(exec.runtime().get_current_line(), Some(10));
    }

    // Requirement: INPUT 语句 - 基本输入
    #[test]
    fn test_input_basic() {
        let mut exec = Executor::new();
        
        // 设置输入回调
        exec.set_input_callback(|_| Some("42".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证输出提示符
        assert!(exec.get_output().contains("? "));
        
        // 验证变量赋值
        assert_eq!(exec.variables.get("A"), Value::Number(42.0));
    }

    // Requirement: INPUT 语句 - 带提示符的输入
    #[test]
    fn test_input_with_prompt() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("100".to_string()));
        
        let stmt = Statement::Input {
            prompt: Some("ENTER VALUE".to_string()),
            variables: vec![AssignTarget::Variable("X".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证提示符
        assert!(exec.get_output().contains("ENTER VALUE? "));
        assert_eq!(exec.variables.get("X"), Value::Number(100.0));
    }

    // Requirement: INPUT 语句 - 输入多个变量
    #[test]
    fn test_input_multiple_variables() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("10, 20, 30".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string()),
                AssignTarget::Variable("C".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(10.0));
        assert_eq!(exec.variables.get("B"), Value::Number(20.0));
        assert_eq!(exec.variables.get("C"), Value::Number(30.0));
    }

    // Requirement: INPUT 语句 - 字符串输入
    #[test]
    fn test_input_string() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("HELLO".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A$".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A$"), Value::String("HELLO".to_string()));
    }

    // Requirement: INPUT 语句 - 字符串带引号
    #[test]
    fn test_input_string_with_quotes() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("\"HELLO, WORLD\"".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A$".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A$"), Value::String("HELLO, WORLD".to_string()));
    }

    // Requirement: DATA/READ 机制 - DATA 存储和 READ 读取
    #[test]
    fn test_data_read() {
        let mut exec = Executor::new();
        
        // 添加 DATA 值
        exec.add_data_value(DataValue::Number(1.0));
        exec.add_data_value(DataValue::Number(2.0));
        exec.add_data_value(DataValue::Number(3.0));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string()),
                AssignTarget::Variable("C".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(1.0));
        assert_eq!(exec.variables.get("B"), Value::Number(2.0));
        assert_eq!(exec.variables.get("C"), Value::Number(3.0));
    }

    // Requirement: DATA/READ 机制 - 混合数据类型
    #[test]
    fn test_data_read_mixed_types() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(42.0));
        exec.add_data_value(DataValue::String("HELLO".to_string()));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B$".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(42.0));
        assert_eq!(exec.variables.get("B$"), Value::String("HELLO".to_string()));
    }

    // Requirement: DATA/READ 机制 - OUT OF DATA 错误
    #[test]
    fn test_out_of_data_error() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(1.0));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string())
            ],
        };
        
        let result = exec.execute_statement(&stmt);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BasicError::OutOfData));
    }

    // Requirement: RESTORE 数据指针 - RESTORE 重置到开头
    #[test]
    fn test_restore() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(1.0));
        exec.add_data_value(DataValue::Number(2.0));
        
        // 第一次 READ
        exec.execute_statement(&Statement::Read {
            variables: vec![AssignTarget::Variable("A".to_string())],
        }).unwrap();
        assert_eq!(exec.variables.get("A"), Value::Number(1.0));
        
        // RESTORE
        exec.execute_statement(&Statement::Restore {
            line_number: None,
        }).unwrap();
        
        // 第二次 READ（应该重新从头开始）
        exec.execute_statement(&Statement::Read {
            variables: vec![AssignTarget::Variable("B".to_string())],
        }).unwrap();
        assert_eq!(exec.variables.get("B"), Value::Number(1.0));
    }

    // Requirement: FOR...NEXT 循环 - 正向循环
    #[test]
    fn test_for_next_basic() {
        let mut exec = Executor::new();
        
        // 添加测试程序：FOR I=1 TO 3: PRINT I: NEXT I
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(1.0),
                    end: Expr::Number(3.0),
                    step: None,
                },
            ],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Next { var: Some("I".to_string()) }],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // 第一次循环：I=1
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(1.0),
            end: Expr::Number(3.0),
            step: None,
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
        
        // NEXT：I=2
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=3
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
        
        // NEXT：循环结束 (I递增到4但不再循环)
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        // 循环已结束，变量值应该为循环后的值 4
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
    }

    // Requirement: FOR...NEXT 循环 - 步长为 2
    #[test]
    fn test_for_next_step() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(0.0),
                    end: Expr::Number(4.0),
                    step: Some(Expr::Number(2.0)),
                },
            ],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // FOR I=0 TO 4 STEP 2
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(0.0),
            end: Expr::Number(4.0),
            step: Some(Expr::Number(2.0)),
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(0.0));
        
        // NEXT：I=2
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=4
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(4.0));
        
        // NEXT：循环结束
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(4.0));
    }

    // Requirement: FOR...NEXT 循环 - 负步长
    #[test]
    fn test_for_next_negative_step() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(3.0),
                    end: Expr::Number(1.0),
                    step: Some(Expr::Number(-1.0)),
                },
            ],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // FOR I=3 TO 1 STEP -1
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(3.0),
            end: Expr::Number(1.0),
            step: Some(Expr::Number(-1.0)),
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
        
        // NEXT：I=2
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=1
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
        
        // NEXT：循环结束
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
    }

    // Requirement: ON...GOTO - 基于表达式的跳转
    #[test]
    fn test_on_goto() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 200,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // ON 2 GOTO 100,200,300
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(2.0),
            targets: vec![100, 200, 300],
            is_gosub: false,
        }).unwrap();
        
        // 应该跳转到 200
        assert_eq!(exec.runtime().get_current_line(), Some(200));
    }

    // Requirement: ON...GOSUB - 基于表达式的子程序调用
    #[test]
    fn test_on_gosub() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // ON 1 GOSUB 100,200
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(1.0),
            targets: vec![100, 200],
            is_gosub: true,
        }).unwrap();
        
        // 应该跳转到 100
        assert_eq!(exec.runtime().get_current_line(), Some(100));
        // 栈深度应该为 1
        assert_eq!(exec.runtime().stack_depth(), 1);
    }

    // Requirement: ON...GOTO - 值超出范围
    #[test]
    fn test_on_goto_out_of_range() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        let current_line = exec.runtime().get_current_line();
        
        // ON 5 GOTO 100,200  (5 超出范围)
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(5.0),
            targets: vec![100, 200],
            is_gosub: false,
        }).unwrap();
        
        // 应该继续在当前行
        assert_eq!(exec.runtime().get_current_line(), current_line);
    }
    
    #[test]
    fn test_save_and_load() {
        use std::fs;
        
        let mut exec = Executor::new();
        
        // 添加一些程序行
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Print {
                items: vec![PrintItem::Expr(Expr::String("HELLO".to_string()))],
            }],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("A".to_string()),
                value: Expr::Number(42.0),
            }],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 30,
            statements: vec![Statement::End],
        });
        
        // 保存程序到文件
        let filename = "test_program.bas";
        exec.execute_statement(&Statement::Save {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证文件存在
        assert!(fs::metadata(filename).is_ok());
        
        // 清空程序
        exec.runtime_mut().clear_program();
        assert_eq!(exec.runtime().line_count(), 0);
        
        // 加载程序
        exec.execute_statement(&Statement::Load {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证程序已加载
        assert_eq!(exec.runtime().line_count(), 3);
        assert!(exec.runtime().get_line(10).is_some());
        assert!(exec.runtime().get_line(20).is_some());
        assert!(exec.runtime().get_line(30).is_some());
        
        // 清理测试文件
        fs::remove_file(filename).ok();
    }
    
    #[test]
    fn test_save_empty_program() {
        let mut exec = Executor::new();
        
        // 尝试保存空程序应该失败
        let result = exec.execute_statement(&Statement::Save {
            filename: "empty.bas".to_string(),
        });
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_load_nonexistent_file() {
        let mut exec = Executor::new();
        
        // 尝试加载不存在的文件应该失败
        let result = exec.execute_statement(&Statement::Load {
            filename: "nonexistent.bas".to_string(),
        });
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_save_complex_program() {
        use std::fs;
        
        let mut exec = Executor::new();
        
        // 创建一个更复杂的程序
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(1.0),
                    end: Expr::Number(10.0),
                    step: Some(Expr::Number(1.0)),
                },
            ],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![
                Statement::Print {
                    items: vec![PrintItem::Expr(Expr::Variable("I".to_string()))],
                },
            ],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 30,
            statements: vec![Statement::Next { var: Some("I".to_string()) }],
        });
        
        // 保存并重新加载
        let filename = "test_complex.bas";
        exec.execute_statement(&Statement::Save {
            filename: filename.to_string(),
        }).unwrap();
        
        exec.runtime_mut().clear_program();
        
        exec.execute_statement(&Statement::Load {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证程序结构
        assert_eq!(exec.runtime().line_count(), 3);
        
        // 清理
        fs::remove_file(filename).ok();
    }
}


```

```rust
/// 执行引擎
///
/// 求值表达式并执行语句

use crate::ast::*;
use crate::error::{BasicError, Result};
use crate::runtime::Runtime;
use crate::variables::{Value, Variables};

/// 输入回调函数类型
pub type InputCallback = Box<dyn FnMut(&str) -> Option<String>>;

/// 执行引擎
pub struct Executor {
    runtime: Runtime,
    variables: Variables,
    /// 输出缓冲区（用于测试和捕获输出）
    output_buffer: Vec<String>,
    /// 当前打印列位置
    print_column: usize,
    /// DATA 数据存储
    data_values: Vec<DataValue>,
    /// DATA 数据指针（当前读取位置）
    data_pointer: usize,
    /// 输入回调函数（用于测试）
    input_callback: Option<InputCallback>,
}

/// DATA 值类型
#[derive(Debug, Clone, PartialEq)]
pub enum DataValue {
    Number(f64),
    String(String),
}

impl Executor {
    /// 创建新的执行器
    pub fn new() -> Self {
        Executor {
            runtime: Runtime::new(),
            variables: Variables::new(),
            output_buffer: Vec::new(),
            print_column: 0,
            data_values: Vec::new(),
            data_pointer: 0,
            input_callback: None,
        }
    }
    
    /// 设置输入回调函数（用于测试）
    pub fn set_input_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&str) -> Option<String> + 'static,
    {
        self.input_callback = Some(Box::new(callback));
    }
    
    /// 添加 DATA 值
    pub fn add_data_value(&mut self, value: DataValue) {
        self.data_values.push(value);
    }
    
    /// 重置 DATA 指针
    pub fn restore_data(&mut self) {
        self.data_pointer = 0;
    }
    
    /// 读取下一个 DATA 值
    fn read_data_value(&mut self) -> Result<DataValue> {
        if self.data_pointer >= self.data_values.len() {
            return Err(BasicError::OutOfData);
        }
        
        let value = self.data_values[self.data_pointer].clone();
        self.data_pointer += 1;
        Ok(value)
    }
    
    /// 获取输出内容（用于测试）
    pub fn get_output(&self) -> String {
        self.output_buffer.join("")
    }
    
    /// 清空输出缓冲区
    pub fn clear_output(&mut self) {
        self.output_buffer.clear();
        self.print_column = 0;
    }
    
    /// 输出文本（添加到缓冲区并打印到终端）
    fn output(&mut self, text: &str) {
        // 打印到终端
        print!("{}", text);
        use std::io::Write;
        std::io::stdout().flush().ok();
        
        // 同时添加到缓冲区（用于测试）
        self.output_buffer.push(text.to_string());
        
        // 更新列位置
        for ch in text.chars() {
            if ch == '\n' {
                self.print_column = 0;
            } else {
                self.print_column += 1;
            }
        }
    }
    
    /// 输出换行
    fn output_newline(&mut self) {
        self.output("\n");
    }

    /// 获取运行时引用
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    /// 获取变量存储引用
    pub fn variables(&self) -> &Variables {
        &self.variables
    }

    /// 获取运行时可变引用
    pub fn runtime_mut(&mut self) -> &mut Runtime {
        &mut self.runtime
    }

    /// 获取变量存储可变引用
    pub fn variables_mut(&mut self) -> &mut Variables {
        &mut self.variables
    }

    /// 求值表达式
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            
            Expr::String(s) => Ok(Value::String(s.clone())),
            
            Expr::Variable(name) => {
                Ok(self.variables.get(name))
            }
            
            Expr::ArrayAccess { name, indices } => {
                // 求值所有索引
                let idx_values: Result<Vec<usize>> = indices.iter()
                    .map(|idx_expr| {
                        self.eval_expr(idx_expr)?
                            .as_number()
                            .and_then(|n| {
                                if n < 0.0 {
                                    Err(BasicError::SubscriptOutOfRange(
                                        "Negative array index".to_string()
                                    ))
                                } else {
                                    Ok(n as usize)
                                }
                            })
                    })
                    .collect();
                
                let indices_usize = idx_values?;
                self.variables.get_array_element(name, &indices_usize)
            }
            
            Expr::FunctionCall { name, args } => {
                self.eval_function_call(name, args)
            }
            
            Expr::BinaryOp { left, op, right } => {
                self.eval_binary_op(left, *op, right)
            }
            
            Expr::UnaryOp { op, operand } => {
                self.eval_unary_op(*op, operand)
            }
        }
    }

    /// 求值二元运算
    fn eval_binary_op(&mut self, left: &Expr, op: BinaryOperator, right: &Expr) -> Result<Value> {
        use BinaryOperator::*;

        let left_val = self.eval_expr(left)?;
        let right_val = self.eval_expr(right)?;

        match op {
            // 算术运算符
            Add => {
                if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    Ok(Value::Number(l + r))
                } else if left_val.is_string() && right_val.is_string() {
                    // 字符串连接
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    Ok(Value::String(format!("{}{}", l, r)))
                } else {
                    Err(BasicError::TypeMismatch(
                        "Cannot add incompatible types".to_string()
                    ))
                }
            }
            
            Subtract => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l - r))
            }
            
            Multiply => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l * r))
            }
            
            Divide => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                if r == 0.0 {
                    return Err(BasicError::DivisionByZero);
                }
                Ok(Value::Number(l / r))
            }
            
            Power => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l.powf(r)))
            }
            
            // 关系运算符（BASIC 中 true = -1, false = 0）
            Equal => {
                let result = if left_val == right_val { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            NotEqual => {
                let result = if left_val != right_val { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            Less => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l < r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l < r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            Greater => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l > r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l > r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            LessEqual => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l <= r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l <= r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            GreaterEqual => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l >= r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l >= r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            // 逻辑运算符（按位）
            And => {
                let l = left_val.as_number()? as i32;
                let r = right_val.as_number()? as i32;
                Ok(Value::Number((l & r) as f64))
            }
            
            Or => {
                let l = left_val.as_number()? as i32;
                let r = right_val.as_number()? as i32;
                Ok(Value::Number((l | r) as f64))
            }
        }
    }

    /// 求值一元运算
    fn eval_unary_op(&mut self, op: UnaryOperator, operand: &Expr) -> Result<Value> {
        let val = self.eval_expr(operand)?;
        
        match op {
            UnaryOperator::Minus => {
                let n = val.as_number()?;
                Ok(Value::Number(-n))
            }
            UnaryOperator::Not => {
                let n = val.as_number()? as i32;
                Ok(Value::Number((!n) as f64))
            }
        }
    }

    /// 求值函数调用（内置函数）
    fn eval_function_call(&mut self, name: &str, args: &[Expr]) -> Result<Value> {
        match name.to_uppercase().as_str() {
            // 数学函数
            "SGN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SGN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                let result = if n > 0.0 { 1.0 } else if n < 0.0 { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            "INT" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("INT requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.floor()))
            }
            
            "ABS" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ABS requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.abs()))
            }
            
            "SQR" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SQR requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n < 0.0 {
                    return Err(BasicError::IllegalQuantity("SQR of negative number".to_string()));
                }
                Ok(Value::Number(n.sqrt()))
            }
            
            "SIN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SIN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.sin()))
            }
            
            "COS" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("COS requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.cos()))
            }
            
            "TAN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("TAN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.tan()))
            }
            
            "ATN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ATN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.atan()))
            }
            
            "LOG" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("LOG requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n <= 0.0 {
                    return Err(BasicError::IllegalQuantity("LOG of non-positive number".to_string()));
                }
                Ok(Value::Number(n.ln()))
            }
            
            "EXP" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("EXP requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.exp()))
            }
            
            // 字符串函数
            "LEN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("LEN requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                Ok(Value::Number(s.len() as f64))
            }
            
            "ASC" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ASC requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                if s.is_empty() {
                    return Err(BasicError::IllegalQuantity("ASC of empty string".to_string()));
                }
                Ok(Value::Number(s.chars().next().unwrap() as u8 as f64))
            }
            
            "CHR$" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("CHR$ requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n < 0.0 || n > 255.0 {
                    return Err(BasicError::IllegalQuantity("CHR$ argument out of range".to_string()));
                }
                let ch = n as u8 as char;
                Ok(Value::String(ch.to_string()))
            }
            
            "STR$" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("STR$ requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                // BASIC 的 STR$ 在正数前加空格
                let s = if n >= 0.0 {
                    format!(" {}", n)
                } else {
                    n.to_string()
                };
                Ok(Value::String(s))
            }
            
            "VAL" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("VAL requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = s.trim().parse::<f64>().unwrap_or(0.0);
                Ok(Value::Number(n))
            }
            
            "LEFT$" => {
                if args.len() != 2 {
                    return Err(BasicError::SyntaxError("LEFT$ requires 2 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = self.eval_expr(&args[1])?.as_number()? as usize;
                let result: String = s.chars().take(n).collect();
                Ok(Value::String(result))
            }
            
            "RIGHT$" => {
                if args.len() != 2 {
                    return Err(BasicError::SyntaxError("RIGHT$ requires 2 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = self.eval_expr(&args[1])?.as_number()? as usize;
                let len = s.chars().count();
                let skip = if n > len { 0 } else { len - n };
                let result: String = s.chars().skip(skip).collect();
                Ok(Value::String(result))
            }
            
            "MID$" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err(BasicError::SyntaxError("MID$ requires 2 or 3 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let start = self.eval_expr(&args[1])?.as_number()? as usize;
                
                // BASIC 的 MID$ 是 1-based
                let start = if start > 0 { start - 1 } else { 0 };
                
                let chars: Vec<char> = s.chars().collect();
                
                if args.len() == 3 {
                    let len = self.eval_expr(&args[2])?.as_number()? as usize;
                    let result: String = chars.iter().skip(start).take(len).collect();
                    Ok(Value::String(result))
                } else {
                    let result: String = chars.iter().skip(start).collect();
                    Ok(Value::String(result))
                }
            }
            
            _ => Err(BasicError::SyntaxError(
                format!("Unknown function: {}", name)
            )),
        }
    }

    /// 执行语句
    pub fn execute_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Let { target, value } => {
                let val = self.eval_expr(value)?;
                
                match target {
                    AssignTarget::Variable(name) => {
                        self.variables.set(name, val)?;
                    }
                    AssignTarget::ArrayElement { name, indices } => {
                        let idx_values: Result<Vec<usize>> = indices.iter()
                            .map(|idx_expr| {
                                self.eval_expr(idx_expr)?
                                    .as_number()
                                    .map(|n| n as usize)
                            })
                            .collect();
                        
                        let indices_usize = idx_values?;
                        self.variables.set_array_element(name, &indices_usize, val)?;
                    }
                }
                
                Ok(())
            }
            
            Statement::End => {
                self.runtime.end_execution();
                Ok(())
            }
            
            Statement::Stop => {
                self.runtime.pause_execution();
                Ok(())
            }
            
            Statement::New => {
                self.runtime.clear_program();
                self.variables.clear();
                Ok(())
            }
            
            Statement::Clear => {
                self.variables.clear();
                Ok(())
            }
            
            Statement::Dim { arrays } => {
                for arr_dim in arrays {
                    let dimensions: Result<Vec<usize>> = arr_dim.dimensions.iter()
                        .map(|dim_expr| {
                            self.eval_expr(dim_expr)?
                                .as_number()
                                .map(|n| n as usize)
                        })
                        .collect();
                    
                    let dims = dimensions?;
                    self.variables.dim_array(&arr_dim.name, dims)?;
                }
                Ok(())
            }
            
            Statement::Print { items } => {
                self.execute_print(items)?;
                Ok(())
            }
            
            Statement::Goto { line_number } => {
                let line_val = self.eval_expr(line_number)?;
                let line = line_val.as_number()? as u16;
                self.runtime.set_execution_position(line, 0)?;
                Ok(())
            }
            
            Statement::If { condition, then_part } => {
                let cond_val = self.eval_expr(condition)?;
                let cond_num = cond_val.as_number()?;
                
                // BASIC 中，任何非零值都是真
                if cond_num != 0.0 {
                    match then_part.as_ref() {
                        ThenPart::LineNumber(line) => {
                            self.runtime.set_execution_position(*line as u16, 0)?;
                        }
                        ThenPart::Statement(stmt) => {
                            self.execute_statement(stmt)?;
                        }
                        ThenPart::Statements(stmts) => {
                            for stmt in stmts {
                                self.execute_statement(stmt)?;
                            }
                        }
                    }
                }
                Ok(())
            }
            
            Statement::Gosub { line_number } => {
                // 保存返回地址（当前行号和语句索引）
                let return_line = self.runtime.get_current_line().unwrap_or(0);
                let return_stmt = 0; // 简化：返回到下一行的第一条语句
                
                // 入栈
                self.runtime.push_gosub(return_line, return_stmt)?;
                
                // 跳转到子程序
                let line_val = self.eval_expr(line_number)?;
                let line = line_val.as_number()? as u16;
                self.runtime.set_execution_position(line, 0)?;
                
                Ok(())
            }
            
            Statement::Return => {
                // 从栈中弹出返回地址
                let (return_line, return_stmt) = self.runtime.pop_gosub()?;
                
                // 跳转回返回地址
                self.runtime.set_execution_position(return_line, return_stmt)?;
                
                Ok(())
            }
            
            Statement::Input { prompt, variables } => {
                // 提取变量名
                let var_names: Vec<String> = variables.iter()
                    .map(|target| match target {
                        AssignTarget::Variable(name) => name.clone(),
                        AssignTarget::ArrayElement { .. } => {
                            // INPUT 不支持数组元素
                            String::new()
                        }
                    })
                    .collect();
                
                self.execute_input(prompt.as_deref(), &var_names)?;
                Ok(())
            }
            
            Statement::Data { values: _ } => {
                // DATA 语句在程序加载时处理，执行时跳过
                // 数据已经存储在 data_values 中
                Ok(())
            }
            
            Statement::Read { variables } => {
                for target in variables {
                    let var_name = match target {
                        AssignTarget::Variable(name) => name.as_str(),
                        AssignTarget::ArrayElement { .. } => {
                            return Err(BasicError::SyntaxError(
                                "READ does not support array elements".to_string()
                            ));
                        }
                    };
                    
                    let data_val = self.read_data_value()?;
                    
                    // 根据变量名判断类型
                    if var_name.ends_with('$') {
                        // 字符串变量
                        let str_val = match data_val {
                            DataValue::String(s) => s,
                            DataValue::Number(n) => n.to_string(),
                        };
                        self.variables.set(var_name, Value::String(str_val))?;
                    } else {
                        // 数值变量
                        let num_val = match data_val {
                            DataValue::Number(n) => n,
                            DataValue::String(s) => {
                                s.trim().parse::<f64>().unwrap_or(0.0)
                            }
                        };
                        self.variables.set(var_name, Value::Number(num_val))?;
                    }
                }
                Ok(())
            }
            
            Statement::Restore { line_number } => {
                if line_number.is_some() {
                    // RESTORE 到指定行（暂不支持，需要跟踪每行的 DATA 位置）
                    return Err(BasicError::SyntaxError(
                        "RESTORE to specific line not yet implemented".to_string()
                    ));
                }
                self.restore_data();
                Ok(())
            }
            
            Statement::For { var, start, end, step } => {
                // 计算起始值、结束值和步长
                let start_val = self.eval_expr(&start)?;
                let end_val = self.eval_expr(&end)?;
                let step_val = if let Some(ref s) = step {
                    self.eval_expr(s)?
                } else {
                    Value::Number(1.0)
                };
                
                // 提取数值
                let start_num = start_val.as_number()?;
                let end_num = end_val.as_number()?;
                let step_num = step_val.as_number()?;
                
                // 检查步长
                if step_num == 0.0 {
                    return Err(BasicError::IllegalQuantity(
                        "FOR loop step cannot be zero".to_string()
                    ));
                }
                
                // 设置循环变量初值
                self.variables.set(var, Value::Number(start_num))?;
                
                // 获取当前位置
                let loop_line = self.runtime.get_current_line()
                    .ok_or_else(|| BasicError::SyntaxError("FOR without line number".to_string()))?;
                let loop_stmt = self.runtime.get_current_stmt_index();
                
                // 将循环信息压入栈
                self.runtime.push_for_loop(
                    var.clone(),
                    end_num,
                    step_num,
                    loop_line,
                    loop_stmt,
                )?;
                
                Ok(())
            }
            
            Statement::Next { var } => {
                // 弹出 FOR 循环信息
                let (loop_var, end_val, step_val, loop_line, loop_stmt) = 
                    self.runtime.pop_for_loop(var.clone())?;
                
                // 获取当前循环变量的值
                let current_val = self.variables.get(&loop_var).as_number()?;
                
                // 递增/递减
                let new_val = current_val + step_val;
                
                // 检查是否继续循环
                let should_continue = if step_val > 0.0 {
                    new_val <= end_val
                } else {
                    new_val >= end_val
                };
                
                if should_continue {
                    // 更新循环变量
                    self.variables.set(&loop_var, Value::Number(new_val))?;
                    
                    // 重新压入栈（继续循环）
                    self.runtime.push_for_loop(
                        loop_var.clone(),
                        end_val,
                        step_val,
                        loop_line,
                        loop_stmt,
                    )?;
                    
                    // 跳转回 FOR 语句的下一条语句
                    self.runtime.set_execution_position(loop_line, loop_stmt + 1)?;
                }
                // 否则继续执行下一条语句（循环结束）
                
                Ok(())
            }
            
            Statement::On { expr, targets, is_gosub } => {
                // 计算表达式的值
                let index_val = self.eval_expr(&expr)?;
                let index = index_val.as_number()? as i32;
                
                // 索引从 1 开始
                if index < 1 || index as usize > targets.len() {
                    // 超出范围，继续执行下一条语句
                    return Ok(());
                }
                
                // 获取目标行号
                let target_line = targets[(index - 1) as usize];
                
                if *is_gosub {
                    // ON...GOSUB：保存返回地址并跳转
                    let return_line = self.runtime.get_current_line()
                        .ok_or_else(|| BasicError::SyntaxError("GOSUB without line number".to_string()))?;
                    let return_stmt = self.runtime.get_current_stmt_index();
                    
                    self.runtime.push_gosub(return_line, return_stmt)?;
                    self.runtime.set_execution_position(target_line, 0)?;
                } else {
                    // ON...GOTO：直接跳转
                    self.runtime.set_execution_position(target_line, 0)?;
                }
                
                Ok(())
            }
            
            Statement::Load { filename } => {
                self.execute_load(filename)?;
                Ok(())
            }
            
            Statement::Save { filename } => {
                self.execute_save(filename)?;
                Ok(())
            }
            
            _ => {
                // 其他语句暂未实现
                Err(BasicError::SyntaxError(
                    "Statement not yet implemented".to_string()
                ))
            }
        }
    }
    
    /// 执行 INPUT 语句
    fn execute_input(&mut self, prompt: Option<&str>, variables: &[String]) -> Result<()> {
        // 显示提示符
        if let Some(p) = prompt {
            self.output(p);
            self.output("? ");
        } else {
            self.output("? ");
        }
        
        // 读取输入
        let input_line = if let Some(ref mut callback) = self.input_callback {
            let prompt_text = prompt.unwrap_or("");
            callback(prompt_text).ok_or_else(|| {
                BasicError::SyntaxError("No input provided".to_string())
            })?
        } else {
            // 在实际 REPL 中，这里会从 stdin 读取
            return Err(BasicError::SyntaxError(
                "No input callback set".to_string()
            ));
        };
        
        // 解析输入值（考虑引号内的逗号）
        let values = Self::parse_input_values(&input_line);
        
        if values.len() != variables.len() {
            self.output("?EXTRA IGNORED\n");
        }
        
        // 赋值给变量
        for (i, var_name) in variables.iter().enumerate() {
            if i >= values.len() {
                break;
            }
            
            let input_val = &values[i];
            
            if var_name.ends_with('$') {
                // 字符串变量
                let str_val = if input_val.starts_with('"') && input_val.ends_with('"') {
                    // 去掉引号
                    input_val[1..input_val.len()-1].to_string()
                } else {
                    input_val.clone()
                };
                self.variables.set(var_name, Value::String(str_val))?;
            } else {
                // 数值变量
                match input_val.parse::<f64>() {
                    Ok(num) => {
                        self.variables.set(var_name, Value::Number(num))?;
                    }
                    Err(_) => {
                        self.output("?REDO FROM START\n");
                        return Err(BasicError::TypeMismatch(
                            "Invalid number input".to_string()
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// 解析输入值，处理带引号的字符串
    fn parse_input_values(input: &str) -> Vec<String> {
        let mut values = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        
        for ch in input.chars() {
            match ch {
                '"' => {
                    in_quotes = !in_quotes;
                    current.push(ch);
                }
                ',' if !in_quotes => {
                    values.push(current.trim().to_string());
                    current.clear();
                }
                _ => {
                    current.push(ch);
                }
            }
        }
        
        if !current.is_empty() || input.ends_with(',') {
            values.push(current.trim().to_string());
        }
        
        values
    }
    
    /// 执行 SAVE 命令 - 保存程序到文件
    fn execute_save(&self, filename: &str) -> Result<()> {
        use std::fs::File;
        use std::io::Write;
        
        let program = self.runtime.clone_program();
        if program.is_empty() {
            return Err(BasicError::SyntaxError("No program to save".to_string()));
        }
        
        let mut file = File::create(filename).map_err(|e| {
            BasicError::SyntaxError(format!("Failed to create file: {}", e))
        })?;
        
        for (_, line) in program.iter() {
            let line_text = Self::serialize_program_line(line);
            writeln!(file, "{}", line_text).map_err(|e| {
                BasicError::SyntaxError(format!("Failed to write to file: {}", e))
            })?;
        }
        
        Ok(())
    }
    
    /// 将程序行序列化为文本
    pub fn serialize_program_line(line: &ProgramLine) -> String {
        let mut result = format!("{}", line.line_number);
        
        for (i, stmt) in line.statements.iter().enumerate() {
            if i > 0 {
                result.push_str(":");
            }
            result.push(' ');
            result.push_str(&Self::serialize_statement(stmt));
        }
        
        result
    }
    
    /// 将语句序列化为文本
    pub fn serialize_statement(stmt: &Statement) -> String {
        match stmt {
            Statement::Let { target, value } => {
                format!("{} = {}", Self::serialize_assign_target(target), Self::serialize_expr(value))
            }
            Statement::Print { items } => {
                let mut result = "PRINT ".to_string();
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        result.push_str(&Self::serialize_print_item(item));
                    } else {
                        result.push_str(&Self::serialize_print_item(item));
                    }
                }
                result
            }
            Statement::If { condition, then_part } => {
                format!("IF {} THEN {}", Self::serialize_expr(condition), Self::serialize_then_part(then_part))
            }
            Statement::Goto { line_number } => {
                format!("GOTO {}", Self::serialize_expr(line_number))
            }
            Statement::Gosub { line_number } => {
                format!("GOSUB {}", Self::serialize_expr(line_number))
            }
            Statement::Return => "RETURN".to_string(),
            Statement::For { var, start, end, step } => {
                let mut result = format!("FOR {} = {} TO {}", var, Self::serialize_expr(start), Self::serialize_expr(end));
                if let Some(step_expr) = step {
                    result.push_str(&format!(" STEP {}", Self::serialize_expr(step_expr)));
                }
                result
            }
            Statement::Next { var } => {
                if let Some(v) = var {
                    format!("NEXT {}", v)
                } else {
                    "NEXT".to_string()
                }
            }
            Statement::On { expr, targets, is_gosub } => {
                let keyword = if *is_gosub { "GOSUB" } else { "GOTO" };
                let target_str = targets.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("ON {} {} {}", Self::serialize_expr(expr), keyword, target_str)
            }
            Statement::Input { prompt, variables } => {
                let mut result = "INPUT ".to_string();
                if let Some(p) = prompt {
                    result.push_str(&format!("\"{}\" ", p));
                }
                let var_str = variables.iter()
                    .map(|v| Self::serialize_assign_target(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                result.push_str(&var_str);
                result
            }
            Statement::Dim { arrays } => {
                let arr_str = arrays.iter()
                    .map(|arr| {
                        let dims = arr.dimensions.iter()
                            .map(|d| Self::serialize_expr(d))
                            .collect::<Vec<_>>()
                            .join(",");
                        format!("{}({})", arr.name, dims)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("DIM {}", arr_str)
            }
            Statement::Data { values } => {
                let val_str = values.iter()
                    .map(|v| match v {
                        crate::ast::DataValue::Number(n) => n.to_string(),
                        crate::ast::DataValue::String(s) => format!("\"{}\"", s),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("DATA {}", val_str)
            }
            Statement::Read { variables } => {
                let var_str = variables.iter()
                    .map(|v| Self::serialize_assign_target(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("READ {}", var_str)
            }
            Statement::Restore { line_number } => {
                if let Some(ln) = line_number {
                    format!("RESTORE {}", ln)
                } else {
                    "RESTORE".to_string()
                }
            }
            Statement::Rem => "REM".to_string(),
            Statement::End => "END".to_string(),
            Statement::Stop => "STOP".to_string(),
            Statement::New => "NEW".to_string(),
            Statement::Clear => "CLEAR".to_string(),
            _ => "REM UNSUPPORTED STATEMENT".to_string(),
        }
    }
    
    /// 将表达式序列化为文本
    pub fn serialize_expr(expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => n.to_string(),
            Expr::String(s) => format!("\"{}\"", s),
            Expr::Variable(name) => name.clone(),
            Expr::ArrayAccess { name, indices } => {
                let idx_str = indices.iter()
                    .map(|i| Self::serialize_expr(i))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, idx_str)
            }
            Expr::FunctionCall { name, args } => {
                let arg_str = args.iter()
                    .map(|a| Self::serialize_expr(a))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, arg_str)
            }
            Expr::BinaryOp { left, op, right } => {
                let op_str = match op {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Subtract => "-",
                    BinaryOperator::Multiply => "*",
                    BinaryOperator::Divide => "/",
                    BinaryOperator::Power => "^",
                    BinaryOperator::Equal => "=",
                    BinaryOperator::NotEqual => "<>",
                    BinaryOperator::Less => "<",
                    BinaryOperator::Greater => ">",
                    BinaryOperator::LessEqual => "<=",
                    BinaryOperator::GreaterEqual => ">=",
                    BinaryOperator::And => " AND ",
                    BinaryOperator::Or => " OR ",
                };
                format!("({} {} {})", Self::serialize_expr(left), op_str, Self::serialize_expr(right))
            }
            Expr::UnaryOp { op, operand } => {
                let op_str = match op {
                    UnaryOperator::Minus => "-",
                    UnaryOperator::Not => "NOT ",
                };
                format!("{}{}", op_str, Self::serialize_expr(operand))
            }
        }
    }
    
    /// 将赋值目标序列化为文本
    fn serialize_assign_target(target: &AssignTarget) -> String {
        match target {
            AssignTarget::Variable(name) => name.clone(),
            AssignTarget::ArrayElement { name, indices } => {
                let idx_str = indices.iter()
                    .map(|i| Self::serialize_expr(i))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, idx_str)
            }
        }
    }
    
    /// 将THEN部分序列化为文本
    fn serialize_then_part(then_part: &ThenPart) -> String {
        match then_part {
            ThenPart::LineNumber(ln) => ln.to_string(),
            ThenPart::Statement(stmt) => Self::serialize_statement(stmt),
            ThenPart::Statements(stmts) => {
                stmts.iter()
                    .map(|s| Self::serialize_statement(s))
                    .collect::<Vec<_>>()
                    .join(":")
            }
        }
    }
    
    /// 将PRINT项序列化为文本
    fn serialize_print_item(item: &PrintItem) -> String {
        match item {
            PrintItem::Expr(expr) => Self::serialize_expr(expr),
            PrintItem::Tab(expr) => format!("TAB({})", Self::serialize_expr(expr)),
            PrintItem::Spc(expr) => format!("SPC({})", Self::serialize_expr(expr)),
            PrintItem::Comma => ",".to_string(),
            PrintItem::Semicolon => ";".to_string(),
        }
    }
    
    /// 执行 LOAD 命令 - 从文件加载程序
    fn execute_load(&mut self, filename: &str) -> Result<()> {
        use std::fs;
        use crate::tokenizer::Tokenizer;
        use crate::parser::Parser;
        
        // 读取文件内容
        let content = fs::read_to_string(filename).map_err(|e| {
            BasicError::SyntaxError(format!("Failed to read file: {}", e))
        })?;
        
        // 清空当前程序
        self.runtime.clear_program();
        self.variables.clear();
        
        // 逐行解析并添加到程序
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            // 使用tokenizer和parser解析每一行
            let mut tokenizer = Tokenizer::new(line);
            let tokens = tokenizer.tokenize_line()?;
            
            let mut parser = Parser::new(tokens);
            if let Some(program_line) = parser.parse_line()? {
                if program_line.line_number > 0 {
                    self.runtime.add_line(program_line);
                }
            }
        }
        
        Ok(())
    }
    
    /// 执行 PRINT 语句
    fn execute_print(&mut self, items: &[PrintItem]) -> Result<()> {
        if items.is_empty() {
            // 空 PRINT，仅输出换行
            self.output_newline();
            return Ok(());
        }
        
        for item in items.iter() {
            match item {
                PrintItem::Expr(expr) => {
                    let val = self.eval_expr(expr)?;
                    self.print_value(&val)?;
                }
                PrintItem::Tab(expr) => {
                    let target_col = self.eval_expr(expr)?.as_number()? as usize;
                    if target_col > self.print_column {
                        let spaces = target_col - self.print_column;
                        self.output(&" ".repeat(spaces));
                    } else if target_col < self.print_column {
                        // TAB 到更小的列，换行后跳转
                        self.output_newline();
                        self.output(&" ".repeat(target_col));
                    }
                }
                PrintItem::Spc(expr) => {
                    let spaces = self.eval_expr(expr)?.as_number()? as usize;
                    self.output(&" ".repeat(spaces));
                }
                PrintItem::Comma => {
                    // 逗号：对齐到下一个 14 列边界
                    let next_col = ((self.print_column / 14) + 1) * 14;
                    let spaces_needed = next_col - self.print_column;
                    self.output(&" ".repeat(spaces_needed));
                }
                PrintItem::Semicolon => {
                    // 分号：不添加空格（紧密连接）
                }
            }
        }
        
        // 检查最后一项是否是分隔符
        if let Some(last) = items.last() {
            if !matches!(last, PrintItem::Comma | PrintItem::Semicolon) {
                // 如果最后不是分隔符，输出换行
                self.output_newline();
            }
        } else {
            self.output_newline();
        }
        
        Ok(())
    }
    
    /// 打印值（根据 BASIC 格式）
    fn print_value(&mut self, val: &Value) -> Result<()> {
        match val {
            Value::Number(n) => {
                // BASIC 数值格式：正数前后各有空格，负数前有空格
                let formatted = if *n >= 0.0 {
                    format!(" {} ", n)
                } else {
                    format!(" {}", n)
                };
                self.output(&formatted);
            }
            Value::String(s) => {
                // 普通字符串，直接输出
                self.output(s);
            }
        }
        Ok(())
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Requirement: 算术运算符 - 加法
    #[test]
    fn test_addition() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Add,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    // Requirement: 算术运算符 - 减法
    #[test]
    fn test_subtraction() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(10.0),
            BinaryOperator::Subtract,
            Expr::Number(7.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    // Requirement: 算术运算符 - 乘法
    #[test]
    fn test_multiplication() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(4.0),
            BinaryOperator::Multiply,
            Expr::Number(5.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(20.0));
    }

    // Requirement: 算术运算符 - 除法
    #[test]
    fn test_division() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(15.0),
            BinaryOperator::Divide,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    // Requirement: 算术运算符 - 浮点除法
    #[test]
    fn test_float_division() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(10.0),
            BinaryOperator::Divide,
            Expr::Number(4.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(2.5));
    }

    // Requirement: 算术运算符 - 除以零
    #[test]
    fn test_division_by_zero() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Divide,
            Expr::Number(0.0)
        );
        let result = exec.eval_expr(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BasicError::DivisionByZero));
    }

    // Requirement: 算术运算符 - 乘方
    #[test]
    fn test_power() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(2.0),
            BinaryOperator::Power,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    // Requirement: 一元运算符 - 一元负号
    #[test]
    fn test_unary_minus() {
        let mut exec = Executor::new();
        let expr = Expr::unary(UnaryOperator::Minus, Expr::Number(5.0));
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-5.0));
    }

    // Requirement: 关系运算符 - 等于
    #[test]
    fn test_equal() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Equal,
            Expr::Number(5.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-1.0)); // BASIC true = -1
    }

    // Requirement: 关系运算符 - 不等于
    #[test]
    fn test_not_equal() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::NotEqual,
            Expr::Number(4.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-1.0));
    }

    // Requirement: 字符串运算符 - 字符串连接
    #[test]
    fn test_string_concatenation() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::String("HELLO".to_string()),
            BinaryOperator::Add,
            Expr::String(" WORLD".to_string())
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::String("HELLO WORLD".to_string()));
    }

    // Test: 变量读取
    #[test]
    fn test_variable_read() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(42.0)).unwrap();
        
        let expr = Expr::Variable("A".to_string());
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    // Test: LET 语句执行
    #[test]
    fn test_let_statement() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Let {
            target: AssignTarget::Variable("X".to_string()),
            value: Expr::Number(100.0),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.variables.get("X"), Value::Number(100.0));
    }

    // Test: DIM 语句执行
    #[test]
    fn test_dim_statement() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Dim {
            arrays: vec![
                ArrayDim {
                    name: "A".to_string(),
                    dimensions: vec![Expr::Number(10.0)],
                }
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert!(exec.variables.has_array("A"));
    }

    // Test: 数学函数
    #[test]
    fn test_math_functions() {
        let mut exec = Executor::new();
        
        // ABS
        let expr = Expr::FunctionCall {
            name: "ABS".to_string(),
            args: vec![Expr::Number(-42.0)],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(42.0));
        
        // INT
        let expr = Expr::FunctionCall {
            name: "INT".to_string(),
            args: vec![Expr::Number(3.7)],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    // Test: 字符串函数
    #[test]
    fn test_string_functions() {
        let mut exec = Executor::new();
        
        // LEN
        let expr = Expr::FunctionCall {
            name: "LEN".to_string(),
            args: vec![Expr::String("HELLO".to_string())],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));
        
        // LEFT$
        let expr = Expr::FunctionCall {
            name: "LEFT$".to_string(),
            args: vec![
                Expr::String("HELLO".to_string()),
                Expr::Number(3.0),
            ],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::String("HEL".to_string()));
    }

    // Test: 复杂表达式
    #[test]
    fn test_complex_expression() {
        let mut exec = Executor::new();
        
        // 2 + 3 * 4 = 14
        let expr = Expr::binary(
            Expr::Number(2.0),
            BinaryOperator::Add,
            Expr::binary(
                Expr::Number(3.0),
                BinaryOperator::Multiply,
                Expr::Number(4.0)
            )
        );
        
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(14.0));
    }

    // Requirement: PRINT 语句 - 打印数值
    #[test]
    fn test_print_number() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(42.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 42 \n");
    }

    // Requirement: PRINT 语句 - 打印字符串
    #[test]
    fn test_print_string() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("HELLO".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "HELLO\n");
    }

    // Requirement: PRINT 语句 - 打印变量
    #[test]
    fn test_print_variable() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(10.0)).unwrap();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Variable("A".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 10 \n");
    }

    // Requirement: PRINT 语句 - 分号分隔（紧密连接）
    #[test]
    fn test_print_semicolon() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(1.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::Number(2.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::Number(3.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 1  2  3 \n");
    }

    // Requirement: PRINT 语句 - 行尾分号（不换行）
    #[test]
    fn test_print_no_newline() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(42.0)),
                PrintItem::Semicolon,
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 42 ");
    }

    // Requirement: PRINT 语句 - 空 PRINT
    #[test]
    fn test_print_empty() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "\n");
    }

    // Requirement: PRINT 语句 - 逗号分隔（列对齐）
    #[test]
    fn test_print_comma_alignment() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(1.0)),
                PrintItem::Comma,
                PrintItem::Expr(Expr::Number(2.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // 第一个数 " 1 " 占 3 列，逗号应该对齐到第 14 列
        assert!(output.starts_with(" 1 "));
        assert!(output.contains(" 2 "));
    }

    // Requirement: GOTO 语句
    #[test]
    fn test_goto_statement() {
        let mut exec = Executor::new();
        
        // 添加程序行
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("A".to_string()),
                value: Expr::Number(1.0),
            }]
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("B".to_string()),
                value: Expr::Number(99.0),
            }]
        });
        
        let stmt = Statement::Goto {
            line_number: Expr::Number(100.0),
        };
        exec.execute_statement(&stmt).unwrap();
        
        // 验证跳转成功（下一行应该是 100）
        assert_eq!(exec.runtime().get_current_line(), Some(100));
    }

    // Requirement: IF...THEN 语句 - 条件为真
    #[test]
    fn test_if_then_true() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(15.0)).unwrap();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行来设置初始状态
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::LineNumber(100)),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.runtime().get_current_line(), Some(100));
    }

    // Requirement: IF...THEN 语句 - 条件为假
    #[test]
    fn test_if_then_false() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(5.0)).unwrap();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![],
        });
        
        let current_line = exec.runtime().get_current_line();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::LineNumber(100)),
        };
        
        exec.execute_statement(&stmt).unwrap();
        // 条件为假，不应该跳转
        assert_eq!(exec.runtime().get_current_line(), current_line);
    }

    // Requirement: IF...THEN 语句 - THEN 后跟语句
    #[test]
    fn test_if_then_statement() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(15.0)).unwrap();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::Statement(
                Statement::Print {
                    items: vec![
                        PrintItem::Expr(Expr::String("TRUE".to_string())),
                    ],
                }
            )),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "TRUE\n");
    }

    // Test: TAB 函数
    #[test]
    fn test_tab_function() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("A".to_string())),
                PrintItem::Semicolon,
                PrintItem::Tab(Expr::Number(10.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::String("B".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // A 在列 0，TAB(10) 跳到列 10，然后是 B
        assert!(output.starts_with("A"));
        assert!(output.contains("B"));
    }

    // Test: SPC 函数
    #[test]
    fn test_spc_function() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("A".to_string())),
                PrintItem::Semicolon,
                PrintItem::Spc(Expr::Number(5.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::String("B".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // A + 5个空格 + B
        assert_eq!(output, "A     B\n");
    }

    // Requirement: GOSUB 和 RETURN 语句 - 子程序调用
    #[test]
    fn test_gosub_statement() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 500,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        let stmt = Statement::Gosub {
            line_number: Expr::Number(500.0),
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证跳转到子程序
        assert_eq!(exec.runtime().get_current_line(), Some(500));
        // 验证调用栈深度
        assert_eq!(exec.runtime().stack_depth(), 1);
    }

    // Requirement: GOSUB 和 RETURN 语句 - 子程序返回
    #[test]
    fn test_return_statement() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 500,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行并设置调用栈
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        exec.runtime_mut().push_gosub(20, 0).unwrap();
        exec.runtime_mut().set_execution_position(500, 0).unwrap();
        
        let stmt = Statement::Return;
        exec.execute_statement(&stmt).unwrap();
        
        // 验证返回到调用点
        assert_eq!(exec.runtime().get_current_line(), Some(20));
        // 验证调用栈已弹出
        assert_eq!(exec.runtime().stack_depth(), 0);
    }

    // Requirement: GOSUB 和 RETURN 语句 - 嵌套子程序
    #[test]
    fn test_nested_gosub() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 200,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // 第一次 GOSUB
        exec.execute_statement(&Statement::Gosub {
            line_number: Expr::Number(100.0),
        }).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 1);
        
        // 第二次 GOSUB（嵌套）
        exec.execute_statement(&Statement::Gosub {
            line_number: Expr::Number(200.0),
        }).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 2);
        assert_eq!(exec.runtime().get_current_line(), Some(200));
        
        // 第一次 RETURN
        exec.execute_statement(&Statement::Return).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 1);
        assert_eq!(exec.runtime().get_current_line(), Some(100));
        
        // 第二次 RETURN
        exec.execute_statement(&Statement::Return).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 0);
        assert_eq!(exec.runtime().get_current_line(), Some(10));
    }

    // Requirement: INPUT 语句 - 基本输入
    #[test]
    fn test_input_basic() {
        let mut exec = Executor::new();
        
        // 设置输入回调
        exec.set_input_callback(|_| Some("42".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证输出提示符
        assert!(exec.get_output().contains("? "));
        
        // 验证变量赋值
        assert_eq!(exec.variables.get("A"), Value::Number(42.0));
    }

    // Requirement: INPUT 语句 - 带提示符的输入
    #[test]
    fn test_input_with_prompt() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("100".to_string()));
        
        let stmt = Statement::Input {
            prompt: Some("ENTER VALUE".to_string()),
            variables: vec![AssignTarget::Variable("X".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证提示符
        assert!(exec.get_output().contains("ENTER VALUE? "));
        assert_eq!(exec.variables.get("X"), Value::Number(100.0));
    }

    // Requirement: INPUT 语句 - 输入多个变量
    #[test]
    fn test_input_multiple_variables() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("10, 20, 30".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string()),
                AssignTarget::Variable("C".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(10.0));
        assert_eq!(exec.variables.get("B"), Value::Number(20.0));
        assert_eq!(exec.variables.get("C"), Value::Number(30.0));
    }

    // Requirement: INPUT 语句 - 字符串输入
    #[test]
    fn test_input_string() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("HELLO".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A$".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A$"), Value::String("HELLO".to_string()));
    }

    // Requirement: INPUT 语句 - 字符串带引号
    #[test]
    fn test_input_string_with_quotes() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("\"HELLO, WORLD\"".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A$".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A$"), Value::String("HELLO, WORLD".to_string()));
    }

    // Requirement: DATA/READ 机制 - DATA 存储和 READ 读取
    #[test]
    fn test_data_read() {
        let mut exec = Executor::new();
        
        // 添加 DATA 值
        exec.add_data_value(DataValue::Number(1.0));
        exec.add_data_value(DataValue::Number(2.0));
        exec.add_data_value(DataValue::Number(3.0));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string()),
                AssignTarget::Variable("C".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(1.0));
        assert_eq!(exec.variables.get("B"), Value::Number(2.0));
        assert_eq!(exec.variables.get("C"), Value::Number(3.0));
    }

    // Requirement: DATA/READ 机制 - 混合数据类型
    #[test]
    fn test_data_read_mixed_types() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(42.0));
        exec.add_data_value(DataValue::String("HELLO".to_string()));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B$".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(42.0));
        assert_eq!(exec.variables.get("B$"), Value::String("HELLO".to_string()));
    }

    // Requirement: DATA/READ 机制 - OUT OF DATA 错误
    #[test]
    fn test_out_of_data_error() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(1.0));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string())
            ],
        };
        
        let result = exec.execute_statement(&stmt);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BasicError::OutOfData));
    }

    // Requirement: RESTORE 数据指针 - RESTORE 重置到开头
    #[test]
    fn test_restore() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(1.0));
        exec.add_data_value(DataValue::Number(2.0));
        
        // 第一次 READ
        exec.execute_statement(&Statement::Read {
            variables: vec![AssignTarget::Variable("A".to_string())],
        }).unwrap();
        assert_eq!(exec.variables.get("A"), Value::Number(1.0));
        
        // RESTORE
        exec.execute_statement(&Statement::Restore {
            line_number: None,
        }).unwrap();
        
        // 第二次 READ（应该重新从头开始）
        exec.execute_statement(&Statement::Read {
            variables: vec![AssignTarget::Variable("B".to_string())],
        }).unwrap();
        assert_eq!(exec.variables.get("B"), Value::Number(1.0));
    }

    // Requirement: FOR...NEXT 循环 - 正向循环
    #[test]
    fn test_for_next_basic() {
        let mut exec = Executor::new();
        
        // 添加测试程序：FOR I=1 TO 3: PRINT I: NEXT I
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(1.0),
                    end: Expr::Number(3.0),
                    step: None,
                },
            ],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Next { var: Some("I".to_string()) }],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // 第一次循环：I=1
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(1.0),
            end: Expr::Number(3.0),
            step: None,
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
        
        // NEXT：I=2
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=3
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
        
        // NEXT：循环结束 (I递增到4但不再循环)
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        // 循环已结束，变量值应该为循环后的值 4
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
    }

    // Requirement: FOR...NEXT 循环 - 步长为 2
    #[test]
    fn test_for_next_step() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(0.0),
                    end: Expr::Number(4.0),
                    step: Some(Expr::Number(2.0)),
                },
            ],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // FOR I=0 TO 4 STEP 2
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(0.0),
            end: Expr::Number(4.0),
            step: Some(Expr::Number(2.0)),
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(0.0));
        
        // NEXT：I=2
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=4
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(4.0));
        
        // NEXT：循环结束
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(4.0));
    }

    // Requirement: FOR...NEXT 循环 - 负步长
    #[test]
    fn test_for_next_negative_step() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(3.0),
                    end: Expr::Number(1.0),
                    step: Some(Expr::Number(-1.0)),
                },
            ],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // FOR I=3 TO 1 STEP -1
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(3.0),
            end: Expr::Number(1.0),
            step: Some(Expr::Number(-1.0)),
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
        
        // NEXT：I=2
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=1
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
        
        // NEXT：循环结束
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
    }

    // Requirement: ON...GOTO - 基于表达式的跳转
    #[test]
    fn test_on_goto() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 200,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // ON 2 GOTO 100,200,300
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(2.0),
            targets: vec![100, 200, 300],
            is_gosub: false,
        }).unwrap();
        
        // 应该跳转到 200
        assert_eq!(exec.runtime().get_current_line(), Some(200));
    }

    // Requirement: ON...GOSUB - 基于表达式的子程序调用
    #[test]
    fn test_on_gosub() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // ON 1 GOSUB 100,200
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(1.0),
            targets: vec![100, 200],
            is_gosub: true,
        }).unwrap();
        
        // 应该跳转到 100
        assert_eq!(exec.runtime().get_current_line(), Some(100));
        // 栈深度应该为 1
        assert_eq!(exec.runtime().stack_depth(), 1);
    }

    // Requirement: ON...GOTO - 值超出范围
    #[test]
    fn test_on_goto_out_of_range() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        let current_line = exec.runtime().get_current_line();
        
        // ON 5 GOTO 100,200  (5 超出范围)
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(5.0),
            targets: vec![100, 200],
            is_gosub: false,
        }).unwrap();
        
        // 应该继续在当前行
        assert_eq!(exec.runtime().get_current_line(), current_line);
    }
    
    #[test]
    fn test_save_and_load() {
        use std::fs;
        
        let mut exec = Executor::new();
        
        // 添加一些程序行
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Print {
                items: vec![PrintItem::Expr(Expr::String("HELLO".to_string()))],
            }],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("A".to_string()),
                value: Expr::Number(42.0),
            }],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 30,
            statements: vec![Statement::End],
        });
        
        // 保存程序到文件
        let filename = "test_program.bas";
        exec.execute_statement(&Statement::Save {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证文件存在
        assert!(fs::metadata(filename).is_ok());
        
        // 清空程序
        exec.runtime_mut().clear_program();
        assert_eq!(exec.runtime().line_count(), 0);
        
        // 加载程序
        exec.execute_statement(&Statement::Load {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证程序已加载
        assert_eq!(exec.runtime().line_count(), 3);
        assert!(exec.runtime().get_line(10).is_some());
        assert!(exec.runtime().get_line(20).is_some());
        assert!(exec.runtime().get_line(30).is_some());
        
        // 清理测试文件
        fs::remove_file(filename).ok();
    }
    
    #[test]
    fn test_save_empty_program() {
        let mut exec = Executor::new();
        
        // 尝试保存空程序应该失败
        let result = exec.execute_statement(&Statement::Save {
            filename: "empty.bas".to_string(),
        });
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_load_nonexistent_file() {
        let mut exec = Executor::new();
        
        // 尝试加载不存在的文件应该失败
        let result = exec.execute_statement(&Statement::Load {
            filename: "nonexistent.bas".to_string(),
        });
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_save_complex_program() {
        use std::fs;
        
        let mut exec = Executor::new();
        
        // 创建一个更复杂的程序
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(1.0),
                    end: Expr::Number(10.0),
                    step: Some(Expr::Number(1.0)),
                },
            ],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![
                Statement::Print {
                    items: vec![PrintItem::Expr(Expr::Variable("I".to_string()))],
                },
            ],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 30,
            statements: vec![Statement::Next { var: Some("I".to_string()) }],
        });
        
        // 保存并重新加载
        let filename = "test_complex.bas";
        exec.execute_statement(&Statement::Save {
            filename: filename.to_string(),
        }).unwrap();
        
        exec.runtime_mut().clear_program();
        
        exec.execute_statement(&Statement::Load {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证程序结构
        assert_eq!(exec.runtime().line_count(), 3);
        
        // 清理
        fs::remove_file(filename).ok();
    }
}


```

```rust
/// 执行引擎
///
/// 求值表达式并执行语句

use crate::ast::*;
use crate::error::{BasicError, Result};
use crate::runtime::Runtime;
use crate::variables::{Value, Variables};

/// 输入回调函数类型
pub type InputCallback = Box<dyn FnMut(&str) -> Option<String>>;

/// 执行引擎
pub struct Executor {
    runtime: Runtime,
    variables: Variables,
    /// 输出缓冲区（用于测试和捕获输出）
    output_buffer: Vec<String>,
    /// 当前打印列位置
    print_column: usize,
    /// DATA 数据存储
    data_values: Vec<DataValue>,
    /// DATA 数据指针（当前读取位置）
    data_pointer: usize,
    /// 输入回调函数（用于测试）
    input_callback: Option<InputCallback>,
}

/// DATA 值类型
#[derive(Debug, Clone, PartialEq)]
pub enum DataValue {
    Number(f64),
    String(String),
}

impl Executor {
    /// 创建新的执行器
    pub fn new() -> Self {
        Executor {
            runtime: Runtime::new(),
            variables: Variables::new(),
            output_buffer: Vec::new(),
            print_column: 0,
            data_values: Vec::new(),
            data_pointer: 0,
            input_callback: None,
        }
    }
    
    /// 设置输入回调函数（用于测试）
    pub fn set_input_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&str) -> Option<String> + 'static,
    {
        self.input_callback = Some(Box::new(callback));
    }
    
    /// 添加 DATA 值
    pub fn add_data_value(&mut self, value: DataValue) {
        self.data_values.push(value);
    }
    
    /// 重置 DATA 指针
    pub fn restore_data(&mut self) {
        self.data_pointer = 0;
    }
    
    /// 读取下一个 DATA 值
    fn read_data_value(&mut self) -> Result<DataValue> {
        if self.data_pointer >= self.data_values.len() {
            return Err(BasicError::OutOfData);
        }
        
        let value = self.data_values[self.data_pointer].clone();
        self.data_pointer += 1;
        Ok(value)
    }
    
    /// 获取输出内容（用于测试）
    pub fn get_output(&self) -> String {
        self.output_buffer.join("")
    }
    
    /// 清空输出缓冲区
    pub fn clear_output(&mut self) {
        self.output_buffer.clear();
        self.print_column = 0;
    }
    
    /// 输出文本（添加到缓冲区并打印到终端）
    fn output(&mut self, text: &str) {
        // 打印到终端
        print!("{}", text);
        use std::io::Write;
        std::io::stdout().flush().ok();
        
        // 同时添加到缓冲区（用于测试）
        self.output_buffer.push(text.to_string());
        
        // 更新列位置
        for ch in text.chars() {
            if ch == '\n' {
                self.print_column = 0;
            } else {
                self.print_column += 1;
            }
        }
    }
    
    /// 输出换行
    fn output_newline(&mut self) {
        self.output("\n");
    }

    /// 获取运行时引用
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    /// 获取变量存储引用
    pub fn variables(&self) -> &Variables {
        &self.variables
    }

    /// 获取运行时可变引用
    pub fn runtime_mut(&mut self) -> &mut Runtime {
        &mut self.runtime
    }

    /// 获取变量存储可变引用
    pub fn variables_mut(&mut self) -> &mut Variables {
        &mut self.variables
    }

    /// 求值表达式
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            
            Expr::String(s) => Ok(Value::String(s.clone())),
            
            Expr::Variable(name) => {
                Ok(self.variables.get(name))
            }
            
            Expr::ArrayAccess { name, indices } => {
                // 求值所有索引
                let idx_values: Result<Vec<usize>> = indices.iter()
                    .map(|idx_expr| {
                        self.eval_expr(idx_expr)?
                            .as_number()
                            .and_then(|n| {
                                if n < 0.0 {
                                    Err(BasicError::SubscriptOutOfRange(
                                        "Negative array index".to_string()
                                    ))
                                } else {
                                    Ok(n as usize)
                                }
                            })
                    })
                    .collect();
                
                let indices_usize = idx_values?;
                self.variables.get_array_element(name, &indices_usize)
            }
            
            Expr::FunctionCall { name, args } => {
                self.eval_function_call(name, args)
            }
            
            Expr::BinaryOp { left, op, right } => {
                self.eval_binary_op(left, *op, right)
            }
            
            Expr::UnaryOp { op, operand } => {
                self.eval_unary_op(*op, operand)
            }
        }
    }

    /// 求值二元运算
    fn eval_binary_op(&mut self, left: &Expr, op: BinaryOperator, right: &Expr) -> Result<Value> {
        use BinaryOperator::*;

        let left_val = self.eval_expr(left)?;
        let right_val = self.eval_expr(right)?;

        match op {
            // 算术运算符
            Add => {
                if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    Ok(Value::Number(l + r))
                } else if left_val.is_string() && right_val.is_string() {
                    // 字符串连接
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    Ok(Value::String(format!("{}{}", l, r)))
                } else {
                    Err(BasicError::TypeMismatch(
                        "Cannot add incompatible types".to_string()
                    ))
                }
            }
            
            Subtract => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l - r))
            }
            
            Multiply => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l * r))
            }
            
            Divide => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                if r == 0.0 {
                    return Err(BasicError::DivisionByZero);
                }
                Ok(Value::Number(l / r))
            }
            
            Power => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l.powf(r)))
            }
            
            // 关系运算符（BASIC 中 true = -1, false = 0）
            Equal => {
                let result = if left_val == right_val { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            NotEqual => {
                let result = if left_val != right_val { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            Less => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l < r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l < r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            Greater => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l > r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l > r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            LessEqual => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l <= r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l <= r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            GreaterEqual => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l >= r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l >= r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            // 逻辑运算符（按位）
            And => {
                let l = left_val.as_number()? as i32;
                let r = right_val.as_number()? as i32;
                Ok(Value::Number((l & r) as f64))
            }
            
            Or => {
                let l = left_val.as_number()? as i32;
                let r = right_val.as_number()? as i32;
                Ok(Value::Number((l | r) as f64))
            }
        }
    }

    /// 求值一元运算
    fn eval_unary_op(&mut self, op: UnaryOperator, operand: &Expr) -> Result<Value> {
        let val = self.eval_expr(operand)?;
        
        match op {
            UnaryOperator::Minus => {
                let n = val.as_number()?;
                Ok(Value::Number(-n))
            }
            UnaryOperator::Not => {
                let n = val.as_number()? as i32;
                Ok(Value::Number((!n) as f64))
            }
        }
    }

    /// 求值函数调用（内置函数）
    fn eval_function_call(&mut self, name: &str, args: &[Expr]) -> Result<Value> {
        match name.to_uppercase().as_str() {
            // 数学函数
            "SGN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SGN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                let result = if n > 0.0 { 1.0 } else if n < 0.0 { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            "INT" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("INT requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.floor()))
            }
            
            "ABS" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ABS requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.abs()))
            }
            
            "SQR" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SQR requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n < 0.0 {
                    return Err(BasicError::IllegalQuantity("SQR of negative number".to_string()));
                }
                Ok(Value::Number(n.sqrt()))
            }
            
            "SIN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SIN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.sin()))
            }
            
            "COS" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("COS requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.cos()))
            }
            
            "TAN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("TAN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.tan()))
            }
            
            "ATN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ATN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.atan()))
            }
            
            "LOG" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("LOG requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n <= 0.0 {
                    return Err(BasicError::IllegalQuantity("LOG of non-positive number".to_string()));
                }
                Ok(Value::Number(n.ln()))
            }
            
            "EXP" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("EXP requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.exp()))
            }
            
            // 字符串函数
            "LEN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("LEN requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                Ok(Value::Number(s.len() as f64))
            }
            
            "ASC" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ASC requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                if s.is_empty() {
                    return Err(BasicError::IllegalQuantity("ASC of empty string".to_string()));
                }
                Ok(Value::Number(s.chars().next().unwrap() as u8 as f64))
            }
            
            "CHR$" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("CHR$ requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n < 0.0 || n > 255.0 {
                    return Err(BasicError::IllegalQuantity("CHR$ argument out of range".to_string()));
                }
                let ch = n as u8 as char;
                Ok(Value::String(ch.to_string()))
            }
            
            "STR$" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("STR$ requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                // BASIC 的 STR$ 在正数前加空格
                let s = if n >= 0.0 {
                    format!(" {}", n)
                } else {
                    n.to_string()
                };
                Ok(Value::String(s))
            }
            
            "VAL" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("VAL requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = s.trim().parse::<f64>().unwrap_or(0.0);
                Ok(Value::Number(n))
            }
            
            "LEFT$" => {
                if args.len() != 2 {
                    return Err(BasicError::SyntaxError("LEFT$ requires 2 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = self.eval_expr(&args[1])?.as_number()? as usize;
                let result: String = s.chars().take(n).collect();
                Ok(Value::String(result))
            }
            
            "RIGHT$" => {
                if args.len() != 2 {
                    return Err(BasicError::SyntaxError("RIGHT$ requires 2 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = self.eval_expr(&args[1])?.as_number()? as usize;
                let len = s.chars().count();
                let skip = if n > len { 0 } else { len - n };
                let result: String = s.chars().skip(skip).collect();
                Ok(Value::String(result))
            }
            
            "MID$" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err(BasicError::SyntaxError("MID$ requires 2 or 3 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let start = self.eval_expr(&args[1])?.as_number()? as usize;
                
                // BASIC 的 MID$ 是 1-based
                let start = if start > 0 { start - 1 } else { 0 };
                
                let chars: Vec<char> = s.chars().collect();
                
                if args.len() == 3 {
                    let len = self.eval_expr(&args[2])?.as_number()? as usize;
                    let result: String = chars.iter().skip(start).take(len).collect();
                    Ok(Value::String(result))
                } else {
                    let result: String = chars.iter().skip(start).collect();
                    Ok(Value::String(result))
                }
            }
            
            _ => Err(BasicError::SyntaxError(
                format!("Unknown function: {}", name)
            )),
        }
    }

    /// 执行语句
    pub fn execute_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Let { target, value } => {
                let val = self.eval_expr(value)?;
                
                match target {
                    AssignTarget::Variable(name) => {
                        self.variables.set(name, val)?;
                    }
                    AssignTarget::ArrayElement { name, indices } => {
                        let idx_values: Result<Vec<usize>> = indices.iter()
                            .map(|idx_expr| {
                                self.eval_expr(idx_expr)?
                                    .as_number()
                                    .map(|n| n as usize)
                            })
                            .collect();
                        
                        let indices_usize = idx_values?;
                        self.variables.set_array_element(name, &indices_usize, val)?;
                    }
                }
                
                Ok(())
            }
            
            Statement::End => {
                self.runtime.end_execution();
                Ok(())
            }
            
            Statement::Stop => {
                self.runtime.pause_execution();
                Ok(())
            }
            
            Statement::New => {
                self.runtime.clear_program();
                self.variables.clear();
                Ok(())
            }
            
            Statement::Clear => {
                self.variables.clear();
                Ok(())
            }
            
            Statement::Dim { arrays } => {
                for arr_dim in arrays {
                    let dimensions: Result<Vec<usize>> = arr_dim.dimensions.iter()
                        .map(|dim_expr| {
                            self.eval_expr(dim_expr)?
                                .as_number()
                                .map(|n| n as usize)
                        })
                        .collect();
                    
                    let dims = dimensions?;
                    self.variables.dim_array(&arr_dim.name, dims)?;
                }
                Ok(())
            }
            
            Statement::Print { items } => {
                self.execute_print(items)?;
                Ok(())
            }
            
            Statement::Goto { line_number } => {
                let line_val = self.eval_expr(line_number)?;
                let line = line_val.as_number()? as u16;
                self.runtime.set_execution_position(line, 0)?;
                Ok(())
            }
            
            Statement::If { condition, then_part } => {
                let cond_val = self.eval_expr(condition)?;
                let cond_num = cond_val.as_number()?;
                
                // BASIC 中，任何非零值都是真
                if cond_num != 0.0 {
                    match then_part.as_ref() {
                        ThenPart::LineNumber(line) => {
                            self.runtime.set_execution_position(*line as u16, 0)?;
                        }
                        ThenPart::Statement(stmt) => {
                            self.execute_statement(stmt)?;
                        }
                        ThenPart::Statements(stmts) => {
                            for stmt in stmts {
                                self.execute_statement(stmt)?;
                            }
                        }
                    }
                }
                Ok(())
            }
            
            Statement::Gosub { line_number } => {
                // 保存返回地址（当前行号和语句索引）
                let return_line = self.runtime.get_current_line().unwrap_or(0);
                let return_stmt = 0; // 简化：返回到下一行的第一条语句
                
                // 入栈
                self.runtime.push_gosub(return_line, return_stmt)?;
                
                // 跳转到子程序
                let line_val = self.eval_expr(line_number)?;
                let line = line_val.as_number()? as u16;
                self.runtime.set_execution_position(line, 0)?;
                
                Ok(())
            }
            
            Statement::Return => {
                // 从栈中弹出返回地址
                let (return_line, return_stmt) = self.runtime.pop_gosub()?;
                
                // 跳转回返回地址
                self.runtime.set_execution_position(return_line, return_stmt)?;
                
                Ok(())
            }
            
            Statement::Input { prompt, variables } => {
                // 提取变量名
                let var_names: Vec<String> = variables.iter()
                    .map(|target| match target {
                        AssignTarget::Variable(name) => name.clone(),
                        AssignTarget::ArrayElement { .. } => {
                            // INPUT 不支持数组元素
                            String::new()
                        }
                    })
                    .collect();
                
                self.execute_input(prompt.as_deref(), &var_names)?;
                Ok(())
            }
            
            Statement::Data { values: _ } => {
                // DATA 语句在程序加载时处理，执行时跳过
                // 数据已经存储在 data_values 中
                Ok(())
            }
            
            Statement::Read { variables } => {
                for target in variables {
                    let var_name = match target {
                        AssignTarget::Variable(name) => name.as_str(),
                        AssignTarget::ArrayElement { .. } => {
                            return Err(BasicError::SyntaxError(
                                "READ does not support array elements".to_string()
                            ));
                        }
                    };
                    
                    let data_val = self.read_data_value()?;
                    
                    // 根据变量名判断类型
                    if var_name.ends_with('$') {
                        // 字符串变量
                        let str_val = match data_val {
                            DataValue::String(s) => s,
                            DataValue::Number(n) => n.to_string(),
                        };
                        self.variables.set(var_name, Value::String(str_val))?;
                    } else {
                        // 数值变量
                        let num_val = match data_val {
                            DataValue::Number(n) => n,
                            DataValue::String(s) => {
                                s.trim().parse::<f64>().unwrap_or(0.0)
                            }
                        };
                        self.variables.set(var_name, Value::Number(num_val))?;
                    }
                }
                Ok(())
            }
            
            Statement::Restore { line_number } => {
                if line_number.is_some() {
                    // RESTORE 到指定行（暂不支持，需要跟踪每行的 DATA 位置）
                    return Err(BasicError::SyntaxError(
                        "RESTORE to specific line not yet implemented".to_string()
                    ));
                }
                self.restore_data();
                Ok(())
            }
            
            Statement::For { var, start, end, step } => {
                // 计算起始值、结束值和步长
                let start_val = self.eval_expr(&start)?;
                let end_val = self.eval_expr(&end)?;
                let step_val = if let Some(ref s) = step {
                    self.eval_expr(s)?
                } else {
                    Value::Number(1.0)
                };
                
                // 提取数值
                let start_num = start_val.as_number()?;
                let end_num = end_val.as_number()?;
                let step_num = step_val.as_number()?;
                
                // 检查步长
                if step_num == 0.0 {
                    return Err(BasicError::IllegalQuantity(
                        "FOR loop step cannot be zero".to_string()
                    ));
                }
                
                // 设置循环变量初值
                self.variables.set(var, Value::Number(start_num))?;
                
                // 获取当前位置
                let loop_line = self.runtime.get_current_line()
                    .ok_or_else(|| BasicError::SyntaxError("FOR without line number".to_string()))?;
                let loop_stmt = self.runtime.get_current_stmt_index();
                
                // 将循环信息压入栈
                self.runtime.push_for_loop(
                    var.clone(),
                    end_num,
                    step_num,
                    loop_line,
                    loop_stmt,
                )?;
                
                Ok(())
            }
            
            Statement::Next { var } => {
                // 弹出 FOR 循环信息
                let (loop_var, end_val, step_val, loop_line, loop_stmt) = 
                    self.runtime.pop_for_loop(var.clone())?;
                
                // 获取当前循环变量的值
                let current_val = self.variables.get(&loop_var).as_number()?;
                
                // 递增/递减
                let new_val = current_val + step_val;
                
                // 检查是否继续循环
                let should_continue = if step_val > 0.0 {
                    new_val <= end_val
                } else {
                    new_val >= end_val
                };
                
                if should_continue {
                    // 更新循环变量
                    self.variables.set(&loop_var, Value::Number(new_val))?;
                    
                    // 重新压入栈（继续循环）
                    self.runtime.push_for_loop(
                        loop_var.clone(),
                        end_val,
                        step_val,
                        loop_line,
                        loop_stmt,
                    )?;
                    
                    // 跳转回 FOR 语句的下一条语句
                    self.runtime.set_execution_position(loop_line, loop_stmt + 1)?;
                }
                // 否则继续执行下一条语句（循环结束）
                
                Ok(())
            }
            
            Statement::On { expr, targets, is_gosub } => {
                // 计算表达式的值
                let index_val = self.eval_expr(&expr)?;
                let index = index_val.as_number()? as i32;
                
                // 索引从 1 开始
                if index < 1 || index as usize > targets.len() {
                    // 超出范围，继续执行下一条语句
                    return Ok(());
                }
                
                // 获取目标行号
                let target_line = targets[(index - 1) as usize];
                
                if *is_gosub {
                    // ON...GOSUB：保存返回地址并跳转
                    let return_line = self.runtime.get_current_line()
                        .ok_or_else(|| BasicError::SyntaxError("GOSUB without line number".to_string()))?;
                    let return_stmt = self.runtime.get_current_stmt_index();
                    
                    self.runtime.push_gosub(return_line, return_stmt)?;
                    self.runtime.set_execution_position(target_line, 0)?;
                } else {
                    // ON...GOTO：直接跳转
                    self.runtime.set_execution_position(target_line, 0)?;
                }
                
                Ok(())
            }
            
            Statement::Load { filename } => {
                self.execute_load(filename)?;
                Ok(())
            }
            
            Statement::Save { filename } => {
                self.execute_save(filename)?;
                Ok(())
            }
            
            _ => {
                // 其他语句暂未实现
                Err(BasicError::SyntaxError(
                    "Statement not yet implemented".to_string()
                ))
            }
        }
    }
    
    /// 执行 INPUT 语句
    fn execute_input(&mut self, prompt: Option<&str>, variables: &[String]) -> Result<()> {
        // 显示提示符
        if let Some(p) = prompt {
            self.output(p);
            self.output("? ");
        } else {
            self.output("? ");
        }
        
        // 读取输入
        let input_line = if let Some(ref mut callback) = self.input_callback {
            let prompt_text = prompt.unwrap_or("");
            callback(prompt_text).ok_or_else(|| {
                BasicError::SyntaxError("No input provided".to_string())
            })?
        } else {
            // 在实际 REPL 中，这里会从 stdin 读取
            return Err(BasicError::SyntaxError(
                "No input callback set".to_string()
            ));
        };
        
        // 解析输入值（考虑引号内的逗号）
        let values = Self::parse_input_values(&input_line);
        
        if values.len() != variables.len() {
            self.output("?EXTRA IGNORED\n");
        }
        
        // 赋值给变量
        for (i, var_name) in variables.iter().enumerate() {
            if i >= values.len() {
                break;
            }
            
            let input_val = &values[i];
            
            if var_name.ends_with('$') {
                // 字符串变量
                let str_val = if input_val.starts_with('"') && input_val.ends_with('"') {
                    // 去掉引号
                    input_val[1..input_val.len()-1].to_string()
                } else {
                    input_val.clone()
                };
                self.variables.set(var_name, Value::String(str_val))?;
            } else {
                // 数值变量
                match input_val.parse::<f64>() {
                    Ok(num) => {
                        self.variables.set(var_name, Value::Number(num))?;
                    }
                    Err(_) => {
                        self.output("?REDO FROM START\n");
                        return Err(BasicError::TypeMismatch(
                            "Invalid number input".to_string()
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// 解析输入值，处理带引号的字符串
    fn parse_input_values(input: &str) -> Vec<String> {
        let mut values = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        
        for ch in input.chars() {
            match ch {
                '"' => {
                    in_quotes = !in_quotes;
                    current.push(ch);
                }
                ',' if !in_quotes => {
                    values.push(current.trim().to_string());
                    current.clear();
                }
                _ => {
                    current.push(ch);
                }
            }
        }
        
        if !current.is_empty() || input.ends_with(',') {
            values.push(current.trim().to_string());
        }
        
        values
    }
    
    /// 执行 SAVE 命令 - 保存程序到文件
    fn execute_save(&self, filename: &str) -> Result<()> {
        use std::fs::File;
        use std::io::Write;
        
        let program = self.runtime.clone_program();
        if program.is_empty() {
            return Err(BasicError::SyntaxError("No program to save".to_string()));
        }
        
        let mut file = File::create(filename).map_err(|e| {
            BasicError::SyntaxError(format!("Failed to create file: {}", e))
        })?;
        
        for (_, line) in program.iter() {
            let line_text = Self::serialize_program_line(line);
            writeln!(file, "{}", line_text).map_err(|e| {
                BasicError::SyntaxError(format!("Failed to write to file: {}", e))
            })?;
        }
        
        Ok(())
    }
    
    /// 将程序行序列化为文本
    pub fn serialize_program_line(line: &ProgramLine) -> String {
        let mut result = format!("{}", line.line_number);
        
        for (i, stmt) in line.statements.iter().enumerate() {
            if i > 0 {
                result.push_str(":");
            }
            result.push(' ');
            result.push_str(&Self::serialize_statement(stmt));
        }
        
        result
    }
    
    /// 将语句序列化为文本
    pub fn serialize_statement(stmt: &Statement) -> String {
        match stmt {
            Statement::Let { target, value } => {
                format!("{} = {}", Self::serialize_assign_target(target), Self::serialize_expr(value))
            }
            Statement::Print { items } => {
                let mut result = "PRINT ".to_string();
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        result.push_str(&Self::serialize_print_item(item));
                    } else {
                        result.push_str(&Self::serialize_print_item(item));
                    }
                }
                result
            }
            Statement::If { condition, then_part } => {
                format!("IF {} THEN {}", Self::serialize_expr(condition), Self::serialize_then_part(then_part))
            }
            Statement::Goto { line_number } => {
                format!("GOTO {}", Self::serialize_expr(line_number))
            }
            Statement::Gosub { line_number } => {
                format!("GOSUB {}", Self::serialize_expr(line_number))
            }
            Statement::Return => "RETURN".to_string(),
            Statement::For { var, start, end, step } => {
                let mut result = format!("FOR {} = {} TO {}", var, Self::serialize_expr(start), Self::serialize_expr(end));
                if let Some(step_expr) = step {
                    result.push_str(&format!(" STEP {}", Self::serialize_expr(step_expr)));
                }
                result
            }
            Statement::Next { var } => {
                if let Some(v) = var {
                    format!("NEXT {}", v)
                } else {
                    "NEXT".to_string()
                }
            }
            Statement::On { expr, targets, is_gosub } => {
                let keyword = if *is_gosub { "GOSUB" } else { "GOTO" };
                let target_str = targets.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("ON {} {} {}", Self::serialize_expr(expr), keyword, target_str)
            }
            Statement::Input { prompt, variables } => {
                let mut result = "INPUT ".to_string();
                if let Some(p) = prompt {
                    result.push_str(&format!("\"{}\" ", p));
                }
                let var_str = variables.iter()
                    .map(|v| Self::serialize_assign_target(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                result.push_str(&var_str);
                result
            }
            Statement::Dim { arrays } => {
                let arr_str = arrays.iter()
                    .map(|arr| {
                        let dims = arr.dimensions.iter()
                            .map(|d| Self::serialize_expr(d))
                            .collect::<Vec<_>>()
                            .join(",");
                        format!("{}({})", arr.name, dims)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("DIM {}", arr_str)
            }
            Statement::Data { values } => {
                let val_str = values.iter()
                    .map(|v| match v {
                        crate::ast::DataValue::Number(n) => n.to_string(),
                        crate::ast::DataValue::String(s) => format!("\"{}\"", s),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("DATA {}", val_str)
            }
            Statement::Read { variables } => {
                let var_str = variables.iter()
                    .map(|v| Self::serialize_assign_target(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("READ {}", var_str)
            }
            Statement::Restore { line_number } => {
                if let Some(ln) = line_number {
                    format!("RESTORE {}", ln)
                } else {
                    "RESTORE".to_string()
                }
            }
            Statement::Rem => "REM".to_string(),
            Statement::End => "END".to_string(),
            Statement::Stop => "STOP".to_string(),
            Statement::New => "NEW".to_string(),
            Statement::Clear => "CLEAR".to_string(),
            _ => "REM UNSUPPORTED STATEMENT".to_string(),
        }
    }
    
    /// 将表达式序列化为文本
    pub fn serialize_expr(expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => n.to_string(),
            Expr::String(s) => format!("\"{}\"", s),
            Expr::Variable(name) => name.clone(),
            Expr::ArrayAccess { name, indices } => {
                let idx_str = indices.iter()
                    .map(|i| Self::serialize_expr(i))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, idx_str)
            }
            Expr::FunctionCall { name, args } => {
                let arg_str = args.iter()
                    .map(|a| Self::serialize_expr(a))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, arg_str)
            }
            Expr::BinaryOp { left, op, right } => {
                let op_str = match op {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Subtract => "-",
                    BinaryOperator::Multiply => "*",
                    BinaryOperator::Divide => "/",
                    BinaryOperator::Power => "^",
                    BinaryOperator::Equal => "=",
                    BinaryOperator::NotEqual => "<>",
                    BinaryOperator::Less => "<",
                    BinaryOperator::Greater => ">",
                    BinaryOperator::LessEqual => "<=",
                    BinaryOperator::GreaterEqual => ">=",
                    BinaryOperator::And => " AND ",
                    BinaryOperator::Or => " OR ",
                };
                format!("({} {} {})", Self::serialize_expr(left), op_str, Self::serialize_expr(right))
            }
            Expr::UnaryOp { op, operand } => {
                let op_str = match op {
                    UnaryOperator::Minus => "-",
                    UnaryOperator::Not => "NOT ",
                };
                format!("{}{}", op_str, Self::serialize_expr(operand))
            }
        }
    }
    
    /// 将赋值目标序列化为文本
    pub fn serialize_assign_target(target: &AssignTarget) -> String {
        match target {
            AssignTarget::Variable(name) => name.clone(),
            AssignTarget::ArrayElement { name, indices } => {
                let idx_str = indices.iter()
                    .map(|i| Self::serialize_expr(i))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, idx_str)
            }
        }
    }
    
    /// 将THEN部分序列化为文本
    fn serialize_then_part(then_part: &ThenPart) -> String {
        match then_part {
            ThenPart::LineNumber(ln) => ln.to_string(),
            ThenPart::Statement(stmt) => Self::serialize_statement(stmt),
            ThenPart::Statements(stmts) => {
                stmts.iter()
                    .map(|s| Self::serialize_statement(s))
                    .collect::<Vec<_>>()
                    .join(":")
            }
        }
    }
    
    /// 将PRINT项序列化为文本
    fn serialize_print_item(item: &PrintItem) -> String {
        match item {
            PrintItem::Expr(expr) => Self::serialize_expr(expr),
            PrintItem::Tab(expr) => format!("TAB({})", Self::serialize_expr(expr)),
            PrintItem::Spc(expr) => format!("SPC({})", Self::serialize_expr(expr)),
            PrintItem::Comma => ",".to_string(),
            PrintItem::Semicolon => ";".to_string(),
        }
    }
    
    /// 执行 LOAD 命令 - 从文件加载程序
    fn execute_load(&mut self, filename: &str) -> Result<()> {
        use std::fs;
        use crate::tokenizer::Tokenizer;
        use crate::parser::Parser;
        
        // 读取文件内容
        let content = fs::read_to_string(filename).map_err(|e| {
            BasicError::SyntaxError(format!("Failed to read file: {}", e))
        })?;
        
        // 清空当前程序
        self.runtime.clear_program();
        self.variables.clear();
        
        // 逐行解析并添加到程序
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            // 使用tokenizer和parser解析每一行
            let mut tokenizer = Tokenizer::new(line);
            let tokens = tokenizer.tokenize_line()?;
            
            let mut parser = Parser::new(tokens);
            if let Some(program_line) = parser.parse_line()? {
                if program_line.line_number > 0 {
                    self.runtime.add_line(program_line);
                }
            }
        }
        
        Ok(())
    }
    
    /// 执行 PRINT 语句
    fn execute_print(&mut self, items: &[PrintItem]) -> Result<()> {
        if items.is_empty() {
            // 空 PRINT，仅输出换行
            self.output_newline();
            return Ok(());
        }
        
        for item in items.iter() {
            match item {
                PrintItem::Expr(expr) => {
                    let val = self.eval_expr(expr)?;
                    self.print_value(&val)?;
                }
                PrintItem::Tab(expr) => {
                    let target_col = self.eval_expr(expr)?.as_number()? as usize;
                    if target_col > self.print_column {
                        let spaces = target_col - self.print_column;
                        self.output(&" ".repeat(spaces));
                    } else if target_col < self.print_column {
                        // TAB 到更小的列，换行后跳转
                        self.output_newline();
                        self.output(&" ".repeat(target_col));
                    }
                }
                PrintItem::Spc(expr) => {
                    let spaces = self.eval_expr(expr)?.as_number()? as usize;
                    self.output(&" ".repeat(spaces));
                }
                PrintItem::Comma => {
                    // 逗号：对齐到下一个 14 列边界
                    let next_col = ((self.print_column / 14) + 1) * 14;
                    let spaces_needed = next_col - self.print_column;
                    self.output(&" ".repeat(spaces_needed));
                }
                PrintItem::Semicolon => {
                    // 分号：不添加空格（紧密连接）
                }
            }
        }
        
        // 检查最后一项是否是分隔符
        if let Some(last) = items.last() {
            if !matches!(last, PrintItem::Comma | PrintItem::Semicolon) {
                // 如果最后不是分隔符，输出换行
                self.output_newline();
            }
        } else {
            self.output_newline();
        }
        
        Ok(())
    }
    
    /// 打印值（根据 BASIC 格式）
    fn print_value(&mut self, val: &Value) -> Result<()> {
        match val {
            Value::Number(n) => {
                // BASIC 数值格式：正数前后各有空格，负数前有空格
                let formatted = if *n >= 0.0 {
                    format!(" {} ", n)
                } else {
                    format!(" {}", n)
                };
                self.output(&formatted);
            }
            Value::String(s) => {
                // 普通字符串，直接输出
                self.output(s);
            }
        }
        Ok(())
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Requirement: 算术运算符 - 加法
    #[test]
    fn test_addition() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Add,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    // Requirement: 算术运算符 - 减法
    #[test]
    fn test_subtraction() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(10.0),
            BinaryOperator::Subtract,
            Expr::Number(7.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    // Requirement: 算术运算符 - 乘法
    #[test]
    fn test_multiplication() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(4.0),
            BinaryOperator::Multiply,
            Expr::Number(5.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(20.0));
    }

    // Requirement: 算术运算符 - 除法
    #[test]
    fn test_division() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(15.0),
            BinaryOperator::Divide,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    // Requirement: 算术运算符 - 浮点除法
    #[test]
    fn test_float_division() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(10.0),
            BinaryOperator::Divide,
            Expr::Number(4.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(2.5));
    }

    // Requirement: 算术运算符 - 除以零
    #[test]
    fn test_division_by_zero() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Divide,
            Expr::Number(0.0)
        );
        let result = exec.eval_expr(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BasicError::DivisionByZero));
    }

    // Requirement: 算术运算符 - 乘方
    #[test]
    fn test_power() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(2.0),
            BinaryOperator::Power,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    // Requirement: 一元运算符 - 一元负号
    #[test]
    fn test_unary_minus() {
        let mut exec = Executor::new();
        let expr = Expr::unary(UnaryOperator::Minus, Expr::Number(5.0));
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-5.0));
    }

    // Requirement: 关系运算符 - 等于
    #[test]
    fn test_equal() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Equal,
            Expr::Number(5.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-1.0)); // BASIC true = -1
    }

    // Requirement: 关系运算符 - 不等于
    #[test]
    fn test_not_equal() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::NotEqual,
            Expr::Number(4.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-1.0));
    }

    // Requirement: 字符串运算符 - 字符串连接
    #[test]
    fn test_string_concatenation() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::String("HELLO".to_string()),
            BinaryOperator::Add,
            Expr::String(" WORLD".to_string())
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::String("HELLO WORLD".to_string()));
    }

    // Test: 变量读取
    #[test]
    fn test_variable_read() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(42.0)).unwrap();
        
        let expr = Expr::Variable("A".to_string());
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    // Test: LET 语句执行
    #[test]
    fn test_let_statement() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Let {
            target: AssignTarget::Variable("X".to_string()),
            value: Expr::Number(100.0),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.variables.get("X"), Value::Number(100.0));
    }

    // Test: DIM 语句执行
    #[test]
    fn test_dim_statement() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Dim {
            arrays: vec![
                ArrayDim {
                    name: "A".to_string(),
                    dimensions: vec![Expr::Number(10.0)],
                }
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert!(exec.variables.has_array("A"));
    }

    // Test: 数学函数
    #[test]
    fn test_math_functions() {
        let mut exec = Executor::new();
        
        // ABS
        let expr = Expr::FunctionCall {
            name: "ABS".to_string(),
            args: vec![Expr::Number(-42.0)],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(42.0));
        
        // INT
        let expr = Expr::FunctionCall {
            name: "INT".to_string(),
            args: vec![Expr::Number(3.7)],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    // Test: 字符串函数
    #[test]
    fn test_string_functions() {
        let mut exec = Executor::new();
        
        // LEN
        let expr = Expr::FunctionCall {
            name: "LEN".to_string(),
            args: vec![Expr::String("HELLO".to_string())],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));
        
        // LEFT$
        let expr = Expr::FunctionCall {
            name: "LEFT$".to_string(),
            args: vec![
                Expr::String("HELLO".to_string()),
                Expr::Number(3.0),
            ],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::String("HEL".to_string()));
    }

    // Test: 复杂表达式
    #[test]
    fn test_complex_expression() {
        let mut exec = Executor::new();
        
        // 2 + 3 * 4 = 14
        let expr = Expr::binary(
            Expr::Number(2.0),
            BinaryOperator::Add,
            Expr::binary(
                Expr::Number(3.0),
                BinaryOperator::Multiply,
                Expr::Number(4.0)
            )
        );
        
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(14.0));
    }

    // Requirement: PRINT 语句 - 打印数值
    #[test]
    fn test_print_number() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(42.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 42 \n");
    }

    // Requirement: PRINT 语句 - 打印字符串
    #[test]
    fn test_print_string() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("HELLO".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "HELLO\n");
    }

    // Requirement: PRINT 语句 - 打印变量
    #[test]
    fn test_print_variable() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(10.0)).unwrap();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Variable("A".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 10 \n");
    }

    // Requirement: PRINT 语句 - 分号分隔（紧密连接）
    #[test]
    fn test_print_semicolon() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(1.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::Number(2.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::Number(3.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 1  2  3 \n");
    }

    // Requirement: PRINT 语句 - 行尾分号（不换行）
    #[test]
    fn test_print_no_newline() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(42.0)),
                PrintItem::Semicolon,
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 42 ");
    }

    // Requirement: PRINT 语句 - 空 PRINT
    #[test]
    fn test_print_empty() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "\n");
    }

    // Requirement: PRINT 语句 - 逗号分隔（列对齐）
    #[test]
    fn test_print_comma_alignment() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(1.0)),
                PrintItem::Comma,
                PrintItem::Expr(Expr::Number(2.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // 第一个数 " 1 " 占 3 列，逗号应该对齐到第 14 列
        assert!(output.starts_with(" 1 "));
        assert!(output.contains(" 2 "));
    }

    // Requirement: GOTO 语句
    #[test]
    fn test_goto_statement() {
        let mut exec = Executor::new();
        
        // 添加程序行
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("A".to_string()),
                value: Expr::Number(1.0),
            }]
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("B".to_string()),
                value: Expr::Number(99.0),
            }]
        });
        
        let stmt = Statement::Goto {
            line_number: Expr::Number(100.0),
        };
        exec.execute_statement(&stmt).unwrap();
        
        // 验证跳转成功（下一行应该是 100）
        assert_eq!(exec.runtime().get_current_line(), Some(100));
    }

    // Requirement: IF...THEN 语句 - 条件为真
    #[test]
    fn test_if_then_true() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(15.0)).unwrap();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行来设置初始状态
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::LineNumber(100)),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.runtime().get_current_line(), Some(100));
    }

    // Requirement: IF...THEN 语句 - 条件为假
    #[test]
    fn test_if_then_false() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(5.0)).unwrap();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![],
        });
        
        let current_line = exec.runtime().get_current_line();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::LineNumber(100)),
        };
        
        exec.execute_statement(&stmt).unwrap();
        // 条件为假，不应该跳转
        assert_eq!(exec.runtime().get_current_line(), current_line);
    }

    // Requirement: IF...THEN 语句 - THEN 后跟语句
    #[test]
    fn test_if_then_statement() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(15.0)).unwrap();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::Statement(
                Statement::Print {
                    items: vec![
                        PrintItem::Expr(Expr::String("TRUE".to_string())),
                    ],
                }
            )),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "TRUE\n");
    }

    // Test: TAB 函数
    #[test]
    fn test_tab_function() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("A".to_string())),
                PrintItem::Semicolon,
                PrintItem::Tab(Expr::Number(10.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::String("B".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // A 在列 0，TAB(10) 跳到列 10，然后是 B
        assert!(output.starts_with("A"));
        assert!(output.contains("B"));
    }

    // Test: SPC 函数
    #[test]
    fn test_spc_function() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("A".to_string())),
                PrintItem::Semicolon,
                PrintItem::Spc(Expr::Number(5.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::String("B".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // A + 5个空格 + B
        assert_eq!(output, "A     B\n");
    }

    // Requirement: GOSUB 和 RETURN 语句 - 子程序调用
    #[test]
    fn test_gosub_statement() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 500,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        let stmt = Statement::Gosub {
            line_number: Expr::Number(500.0),
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证跳转到子程序
        assert_eq!(exec.runtime().get_current_line(), Some(500));
        // 验证调用栈深度
        assert_eq!(exec.runtime().stack_depth(), 1);
    }

    // Requirement: GOSUB 和 RETURN 语句 - 子程序返回
    #[test]
    fn test_return_statement() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 500,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行并设置调用栈
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        exec.runtime_mut().push_gosub(20, 0).unwrap();
        exec.runtime_mut().set_execution_position(500, 0).unwrap();
        
        let stmt = Statement::Return;
        exec.execute_statement(&stmt).unwrap();
        
        // 验证返回到调用点
        assert_eq!(exec.runtime().get_current_line(), Some(20));
        // 验证调用栈已弹出
        assert_eq!(exec.runtime().stack_depth(), 0);
    }

    // Requirement: GOSUB 和 RETURN 语句 - 嵌套子程序
    #[test]
    fn test_nested_gosub() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 200,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // 第一次 GOSUB
        exec.execute_statement(&Statement::Gosub {
            line_number: Expr::Number(100.0),
        }).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 1);
        
        // 第二次 GOSUB（嵌套）
        exec.execute_statement(&Statement::Gosub {
            line_number: Expr::Number(200.0),
        }).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 2);
        assert_eq!(exec.runtime().get_current_line(), Some(200));
        
        // 第一次 RETURN
        exec.execute_statement(&Statement::Return).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 1);
        assert_eq!(exec.runtime().get_current_line(), Some(100));
        
        // 第二次 RETURN
        exec.execute_statement(&Statement::Return).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 0);
        assert_eq!(exec.runtime().get_current_line(), Some(10));
    }

    // Requirement: INPUT 语句 - 基本输入
    #[test]
    fn test_input_basic() {
        let mut exec = Executor::new();
        
        // 设置输入回调
        exec.set_input_callback(|_| Some("42".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证输出提示符
        assert!(exec.get_output().contains("? "));
        
        // 验证变量赋值
        assert_eq!(exec.variables.get("A"), Value::Number(42.0));
    }

    // Requirement: INPUT 语句 - 带提示符的输入
    #[test]
    fn test_input_with_prompt() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("100".to_string()));
        
        let stmt = Statement::Input {
            prompt: Some("ENTER VALUE".to_string()),
            variables: vec![AssignTarget::Variable("X".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证提示符
        assert!(exec.get_output().contains("ENTER VALUE? "));
        assert_eq!(exec.variables.get("X"), Value::Number(100.0));
    }

    // Requirement: INPUT 语句 - 输入多个变量
    #[test]
    fn test_input_multiple_variables() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("10, 20, 30".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string()),
                AssignTarget::Variable("C".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(10.0));
        assert_eq!(exec.variables.get("B"), Value::Number(20.0));
        assert_eq!(exec.variables.get("C"), Value::Number(30.0));
    }

    // Requirement: INPUT 语句 - 字符串输入
    #[test]
    fn test_input_string() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("HELLO".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A$".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A$"), Value::String("HELLO".to_string()));
    }

    // Requirement: INPUT 语句 - 字符串带引号
    #[test]
    fn test_input_string_with_quotes() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("\"HELLO, WORLD\"".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A$".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A$"), Value::String("HELLO, WORLD".to_string()));
    }

    // Requirement: DATA/READ 机制 - DATA 存储和 READ 读取
    #[test]
    fn test_data_read() {
        let mut exec = Executor::new();
        
        // 添加 DATA 值
        exec.add_data_value(DataValue::Number(1.0));
        exec.add_data_value(DataValue::Number(2.0));
        exec.add_data_value(DataValue::Number(3.0));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string()),
                AssignTarget::Variable("C".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(1.0));
        assert_eq!(exec.variables.get("B"), Value::Number(2.0));
        assert_eq!(exec.variables.get("C"), Value::Number(3.0));
    }

    // Requirement: DATA/READ 机制 - 混合数据类型
    #[test]
    fn test_data_read_mixed_types() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(42.0));
        exec.add_data_value(DataValue::String("HELLO".to_string()));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B$".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(42.0));
        assert_eq!(exec.variables.get("B$"), Value::String("HELLO".to_string()));
    }

    // Requirement: DATA/READ 机制 - OUT OF DATA 错误
    #[test]
    fn test_out_of_data_error() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(1.0));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string())
            ],
        };
        
        let result = exec.execute_statement(&stmt);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BasicError::OutOfData));
    }

    // Requirement: RESTORE 数据指针 - RESTORE 重置到开头
    #[test]
    fn test_restore() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(1.0));
        exec.add_data_value(DataValue::Number(2.0));
        
        // 第一次 READ
        exec.execute_statement(&Statement::Read {
            variables: vec![AssignTarget::Variable("A".to_string())],
        }).unwrap();
        assert_eq!(exec.variables.get("A"), Value::Number(1.0));
        
        // RESTORE
        exec.execute_statement(&Statement::Restore {
            line_number: None,
        }).unwrap();
        
        // 第二次 READ（应该重新从头开始）
        exec.execute_statement(&Statement::Read {
            variables: vec![AssignTarget::Variable("B".to_string())],
        }).unwrap();
        assert_eq!(exec.variables.get("B"), Value::Number(1.0));
    }

    // Requirement: FOR...NEXT 循环 - 正向循环
    #[test]
    fn test_for_next_basic() {
        let mut exec = Executor::new();
        
        // 添加测试程序：FOR I=1 TO 3: PRINT I: NEXT I
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(1.0),
                    end: Expr::Number(3.0),
                    step: None,
                },
            ],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Next { var: Some("I".to_string()) }],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // 第一次循环：I=1
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(1.0),
            end: Expr::Number(3.0),
            step: None,
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
        
        // NEXT：I=2
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=3
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
        
        // NEXT：循环结束 (I递增到4但不再循环)
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        // 循环已结束，变量值应该为循环后的值 4
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
    }

    // Requirement: FOR...NEXT 循环 - 步长为 2
    #[test]
    fn test_for_next_step() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(0.0),
                    end: Expr::Number(4.0),
                    step: Some(Expr::Number(2.0)),
                },
            ],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // FOR I=0 TO 4 STEP 2
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(0.0),
            end: Expr::Number(4.0),
            step: Some(Expr::Number(2.0)),
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(0.0));
        
        // NEXT：I=2
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=4
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(4.0));
        
        // NEXT：循环结束
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(4.0));
    }

    // Requirement: FOR...NEXT 循环 - 负步长
    #[test]
    fn test_for_next_negative_step() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(3.0),
                    end: Expr::Number(1.0),
                    step: Some(Expr::Number(-1.0)),
                },
            ],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // FOR I=3 TO 1 STEP -1
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(3.0),
            end: Expr::Number(1.0),
            step: Some(Expr::Number(-1.0)),
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
        
        // NEXT：I=2
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=1
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
        
        // NEXT：循环结束
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
    }

    // Requirement: ON...GOTO - 基于表达式的跳转
    #[test]
    fn test_on_goto() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 200,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // ON 2 GOTO 100,200,300
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(2.0),
            targets: vec![100, 200, 300],
            is_gosub: false,
        }).unwrap();
        
        // 应该跳转到 200
        assert_eq!(exec.runtime().get_current_line(), Some(200));
    }

    // Requirement: ON...GOSUB - 基于表达式的子程序调用
    #[test]
    fn test_on_gosub() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // ON 1 GOSUB 100,200
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(1.0),
            targets: vec![100, 200],
            is_gosub: true,
        }).unwrap();
        
        // 应该跳转到 100
        assert_eq!(exec.runtime().get_current_line(), Some(100));
        // 栈深度应该为 1
        assert_eq!(exec.runtime().stack_depth(), 1);
    }

    // Requirement: ON...GOTO - 值超出范围
    #[test]
    fn test_on_goto_out_of_range() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        let current_line = exec.runtime().get_current_line();
        
        // ON 5 GOTO 100,200  (5 超出范围)
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(5.0),
            targets: vec![100, 200],
            is_gosub: false,
        }).unwrap();
        
        // 应该继续在当前行
        assert_eq!(exec.runtime().get_current_line(), current_line);
    }
    
    #[test]
    fn test_save_and_load() {
        use std::fs;
        
        let mut exec = Executor::new();
        
        // 添加一些程序行
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Print {
                items: vec![PrintItem::Expr(Expr::String("HELLO".to_string()))],
            }],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("A".to_string()),
                value: Expr::Number(42.0),
            }],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 30,
            statements: vec![Statement::End],
        });
        
        // 保存程序到文件
        let filename = "test_program.bas";
        exec.execute_statement(&Statement::Save {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证文件存在
        assert!(fs::metadata(filename).is_ok());
        
        // 清空程序
        exec.runtime_mut().clear_program();
        assert_eq!(exec.runtime().line_count(), 0);
        
        // 加载程序
        exec.execute_statement(&Statement::Load {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证程序已加载
        assert_eq!(exec.runtime().line_count(), 3);
        assert!(exec.runtime().get_line(10).is_some());
        assert!(exec.runtime().get_line(20).is_some());
        assert!(exec.runtime().get_line(30).is_some());
        
        // 清理测试文件
        fs::remove_file(filename).ok();
    }
    
    #[test]
    fn test_save_empty_program() {
        let mut exec = Executor::new();
        
        // 尝试保存空程序应该失败
        let result = exec.execute_statement(&Statement::Save {
            filename: "empty.bas".to_string(),
        });
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_load_nonexistent_file() {
        let mut exec = Executor::new();
        
        // 尝试加载不存在的文件应该失败
        let result = exec.execute_statement(&Statement::Load {
            filename: "nonexistent.bas".to_string(),
        });
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_save_complex_program() {
        use std::fs;
        
        let mut exec = Executor::new();
        
        // 创建一个更复杂的程序
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(1.0),
                    end: Expr::Number(10.0),
                    step: Some(Expr::Number(1.0)),
                },
            ],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![
                Statement::Print {
                    items: vec![PrintItem::Expr(Expr::Variable("I".to_string()))],
                },
            ],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 30,
            statements: vec![Statement::Next { var: Some("I".to_string()) }],
        });
        
        // 保存并重新加载
        let filename = "test_complex.bas";
        exec.execute_statement(&Statement::Save {
            filename: filename.to_string(),
        }).unwrap();
        
        exec.runtime_mut().clear_program();
        
        exec.execute_statement(&Statement::Load {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证程序结构
        assert_eq!(exec.runtime().line_count(), 3);
        
        // 清理
        fs::remove_file(filename).ok();
    }
}


```

```rust
/// 执行引擎
///
/// 求值表达式并执行语句

use crate::ast::*;
use crate::error::{BasicError, Result};
use crate::runtime::Runtime;
use crate::variables::{Value, Variables};

/// 输入回调函数类型
pub type InputCallback = Box<dyn FnMut(&str) -> Option<String>>;

/// 执行引擎
pub struct Executor {
    runtime: Runtime,
    variables: Variables,
    /// 输出缓冲区（用于测试和捕获输出）
    output_buffer: Vec<String>,
    /// 当前打印列位置
    print_column: usize,
    /// DATA 数据存储
    data_values: Vec<DataValue>,
    /// DATA 数据指针（当前读取位置）
    data_pointer: usize,
    /// 输入回调函数（用于测试）
    input_callback: Option<InputCallback>,
}

/// DATA 值类型
#[derive(Debug, Clone, PartialEq)]
pub enum DataValue {
    Number(f64),
    String(String),
}

impl Executor {
    /// 创建新的执行器
    pub fn new() -> Self {
        Executor {
            runtime: Runtime::new(),
            variables: Variables::new(),
            output_buffer: Vec::new(),
            print_column: 0,
            data_values: Vec::new(),
            data_pointer: 0,
            input_callback: None,
        }
    }
    
    /// 设置输入回调函数（用于测试）
    pub fn set_input_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&str) -> Option<String> + 'static,
    {
        self.input_callback = Some(Box::new(callback));
    }
    
    /// 添加 DATA 值
    pub fn add_data_value(&mut self, value: DataValue) {
        self.data_values.push(value);
    }
    
    /// 重置 DATA 指针
    pub fn restore_data(&mut self) {
        self.data_pointer = 0;
    }
    
    /// 读取下一个 DATA 值
    fn read_data_value(&mut self) -> Result<DataValue> {
        if self.data_pointer >= self.data_values.len() {
            return Err(BasicError::OutOfData);
        }
        
        let value = self.data_values[self.data_pointer].clone();
        self.data_pointer += 1;
        Ok(value)
    }
    
    /// 获取输出内容（用于测试）
    pub fn get_output(&self) -> String {
        self.output_buffer.join("")
    }
    
    /// 清空输出缓冲区
    pub fn clear_output(&mut self) {
        self.output_buffer.clear();
        self.print_column = 0;
    }
    
    /// 输出文本（添加到缓冲区并打印到终端）
    fn output(&mut self, text: &str) {
        // 打印到终端
        print!("{}", text);
        use std::io::Write;
        std::io::stdout().flush().ok();
        
        // 同时添加到缓冲区（用于测试）
        self.output_buffer.push(text.to_string());
        
        // 更新列位置
        for ch in text.chars() {
            if ch == '\n' {
                self.print_column = 0;
            } else {
                self.print_column += 1;
            }
        }
    }
    
    /// 输出换行
    fn output_newline(&mut self) {
        self.output("\n");
    }

    /// 获取运行时引用
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    /// 获取变量存储引用
    pub fn variables(&self) -> &Variables {
        &self.variables
    }

    /// 获取运行时可变引用
    pub fn runtime_mut(&mut self) -> &mut Runtime {
        &mut self.runtime
    }

    /// 获取变量存储可变引用
    pub fn variables_mut(&mut self) -> &mut Variables {
        &mut self.variables
    }

    /// 求值表达式
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            
            Expr::String(s) => Ok(Value::String(s.clone())),
            
            Expr::Variable(name) => {
                Ok(self.variables.get(name))
            }
            
            Expr::ArrayAccess { name, indices } => {
                // 求值所有索引
                let idx_values: Result<Vec<usize>> = indices.iter()
                    .map(|idx_expr| {
                        self.eval_expr(idx_expr)?
                            .as_number()
                            .and_then(|n| {
                                if n < 0.0 {
                                    Err(BasicError::SubscriptOutOfRange(
                                        "Negative array index".to_string()
                                    ))
                                } else {
                                    Ok(n as usize)
                                }
                            })
                    })
                    .collect();
                
                let indices_usize = idx_values?;
                self.variables.get_array_element(name, &indices_usize)
            }
            
            Expr::FunctionCall { name, args } => {
                self.eval_function_call(name, args)
            }
            
            Expr::BinaryOp { left, op, right } => {
                self.eval_binary_op(left, *op, right)
            }
            
            Expr::UnaryOp { op, operand } => {
                self.eval_unary_op(*op, operand)
            }
        }
    }

    /// 求值二元运算
    fn eval_binary_op(&mut self, left: &Expr, op: BinaryOperator, right: &Expr) -> Result<Value> {
        use BinaryOperator::*;

        let left_val = self.eval_expr(left)?;
        let right_val = self.eval_expr(right)?;

        match op {
            // 算术运算符
            Add => {
                if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    Ok(Value::Number(l + r))
                } else if left_val.is_string() && right_val.is_string() {
                    // 字符串连接
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    Ok(Value::String(format!("{}{}", l, r)))
                } else {
                    Err(BasicError::TypeMismatch(
                        "Cannot add incompatible types".to_string()
                    ))
                }
            }
            
            Subtract => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l - r))
            }
            
            Multiply => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l * r))
            }
            
            Divide => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                if r == 0.0 {
                    return Err(BasicError::DivisionByZero);
                }
                Ok(Value::Number(l / r))
            }
            
            Power => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l.powf(r)))
            }
            
            // 关系运算符（BASIC 中 true = -1, false = 0）
            Equal => {
                let result = if left_val == right_val { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            NotEqual => {
                let result = if left_val != right_val { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            Less => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l < r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l < r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            Greater => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l > r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l > r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            LessEqual => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l <= r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l <= r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            GreaterEqual => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l >= r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l >= r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            // 逻辑运算符（按位）
            And => {
                let l = left_val.as_number()? as i32;
                let r = right_val.as_number()? as i32;
                Ok(Value::Number((l & r) as f64))
            }
            
            Or => {
                let l = left_val.as_number()? as i32;
                let r = right_val.as_number()? as i32;
                Ok(Value::Number((l | r) as f64))
            }
        }
    }

    /// 求值一元运算
    fn eval_unary_op(&mut self, op: UnaryOperator, operand: &Expr) -> Result<Value> {
        let val = self.eval_expr(operand)?;
        
        match op {
            UnaryOperator::Minus => {
                let n = val.as_number()?;
                Ok(Value::Number(-n))
            }
            UnaryOperator::Not => {
                let n = val.as_number()? as i32;
                Ok(Value::Number((!n) as f64))
            }
        }
    }

    /// 求值函数调用（内置函数）
    fn eval_function_call(&mut self, name: &str, args: &[Expr]) -> Result<Value> {
        match name.to_uppercase().as_str() {
            // 数学函数
            "SGN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SGN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                let result = if n > 0.0 { 1.0 } else if n < 0.0 { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            "INT" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("INT requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.floor()))
            }
            
            "ABS" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ABS requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.abs()))
            }
            
            "SQR" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SQR requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n < 0.0 {
                    return Err(BasicError::IllegalQuantity("SQR of negative number".to_string()));
                }
                Ok(Value::Number(n.sqrt()))
            }
            
            "SIN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SIN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.sin()))
            }
            
            "COS" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("COS requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.cos()))
            }
            
            "TAN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("TAN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.tan()))
            }
            
            "ATN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ATN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.atan()))
            }
            
            "LOG" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("LOG requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n <= 0.0 {
                    return Err(BasicError::IllegalQuantity("LOG of non-positive number".to_string()));
                }
                Ok(Value::Number(n.ln()))
            }
            
            "EXP" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("EXP requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.exp()))
            }
            
            // 字符串函数
            "LEN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("LEN requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                Ok(Value::Number(s.len() as f64))
            }
            
            "ASC" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ASC requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                if s.is_empty() {
                    return Err(BasicError::IllegalQuantity("ASC of empty string".to_string()));
                }
                Ok(Value::Number(s.chars().next().unwrap() as u8 as f64))
            }
            
            "CHR$" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("CHR$ requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n < 0.0 || n > 255.0 {
                    return Err(BasicError::IllegalQuantity("CHR$ argument out of range".to_string()));
                }
                let ch = n as u8 as char;
                Ok(Value::String(ch.to_string()))
            }
            
            "STR$" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("STR$ requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                // BASIC 的 STR$ 在正数前加空格
                let s = if n >= 0.0 {
                    format!(" {}", n)
                } else {
                    n.to_string()
                };
                Ok(Value::String(s))
            }
            
            "VAL" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("VAL requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = s.trim().parse::<f64>().unwrap_or(0.0);
                Ok(Value::Number(n))
            }
            
            "LEFT$" => {
                if args.len() != 2 {
                    return Err(BasicError::SyntaxError("LEFT$ requires 2 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = self.eval_expr(&args[1])?.as_number()? as usize;
                let result: String = s.chars().take(n).collect();
                Ok(Value::String(result))
            }
            
            "RIGHT$" => {
                if args.len() != 2 {
                    return Err(BasicError::SyntaxError("RIGHT$ requires 2 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = self.eval_expr(&args[1])?.as_number()? as usize;
                let len = s.chars().count();
                let skip = if n > len { 0 } else { len - n };
                let result: String = s.chars().skip(skip).collect();
                Ok(Value::String(result))
            }
            
            "MID$" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err(BasicError::SyntaxError("MID$ requires 2 or 3 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let start = self.eval_expr(&args[1])?.as_number()? as usize;
                
                // BASIC 的 MID$ 是 1-based
                let start = if start > 0 { start - 1 } else { 0 };
                
                let chars: Vec<char> = s.chars().collect();
                
                if args.len() == 3 {
                    let len = self.eval_expr(&args[2])?.as_number()? as usize;
                    let result: String = chars.iter().skip(start).take(len).collect();
                    Ok(Value::String(result))
                } else {
                    let result: String = chars.iter().skip(start).collect();
                    Ok(Value::String(result))
                }
            }
            
            _ => Err(BasicError::SyntaxError(
                format!("Unknown function: {}", name)
            )),
        }
    }

    /// 执行语句
    pub fn execute_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Let { target, value } => {
                let val = self.eval_expr(value)?;
                
                match target {
                    AssignTarget::Variable(name) => {
                        self.variables.set(name, val)?;
                    }
                    AssignTarget::ArrayElement { name, indices } => {
                        let idx_values: Result<Vec<usize>> = indices.iter()
                            .map(|idx_expr| {
                                self.eval_expr(idx_expr)?
                                    .as_number()
                                    .map(|n| n as usize)
                            })
                            .collect();
                        
                        let indices_usize = idx_values?;
                        self.variables.set_array_element(name, &indices_usize, val)?;
                    }
                }
                
                Ok(())
            }
            
            Statement::End => {
                self.runtime.end_execution();
                Ok(())
            }
            
            Statement::Stop => {
                self.runtime.pause_execution();
                Ok(())
            }
            
            Statement::New => {
                self.runtime.clear_program();
                self.variables.clear();
                Ok(())
            }
            
            Statement::Clear => {
                self.variables.clear();
                Ok(())
            }
            
            Statement::Dim { arrays } => {
                for arr_dim in arrays {
                    let dimensions: Result<Vec<usize>> = arr_dim.dimensions.iter()
                        .map(|dim_expr| {
                            self.eval_expr(dim_expr)?
                                .as_number()
                                .map(|n| n as usize)
                        })
                        .collect();
                    
                    let dims = dimensions?;
                    self.variables.dim_array(&arr_dim.name, dims)?;
                }
                Ok(())
            }
            
            Statement::Print { items } => {
                self.execute_print(items)?;
                Ok(())
            }
            
            Statement::Goto { line_number } => {
                let line_val = self.eval_expr(line_number)?;
                let line = line_val.as_number()? as u16;
                self.runtime.set_execution_position(line, 0)?;
                Ok(())
            }
            
            Statement::If { condition, then_part } => {
                let cond_val = self.eval_expr(condition)?;
                let cond_num = cond_val.as_number()?;
                
                // BASIC 中，任何非零值都是真
                if cond_num != 0.0 {
                    match then_part.as_ref() {
                        ThenPart::LineNumber(line) => {
                            self.runtime.set_execution_position(*line as u16, 0)?;
                        }
                        ThenPart::Statement(stmt) => {
                            self.execute_statement(stmt)?;
                        }
                        ThenPart::Statements(stmts) => {
                            for stmt in stmts {
                                self.execute_statement(stmt)?;
                            }
                        }
                    }
                }
                Ok(())
            }
            
            Statement::Gosub { line_number } => {
                // 保存返回地址（当前行号和语句索引）
                let return_line = self.runtime.get_current_line().unwrap_or(0);
                let return_stmt = 0; // 简化：返回到下一行的第一条语句
                
                // 入栈
                self.runtime.push_gosub(return_line, return_stmt)?;
                
                // 跳转到子程序
                let line_val = self.eval_expr(line_number)?;
                let line = line_val.as_number()? as u16;
                self.runtime.set_execution_position(line, 0)?;
                
                Ok(())
            }
            
            Statement::Return => {
                // 从栈中弹出返回地址
                let (return_line, return_stmt) = self.runtime.pop_gosub()?;
                
                // 跳转回返回地址
                self.runtime.set_execution_position(return_line, return_stmt)?;
                
                Ok(())
            }
            
            Statement::Input { prompt, variables } => {
                // 提取变量名
                let var_names: Vec<String> = variables.iter()
                    .map(|target| match target {
                        AssignTarget::Variable(name) => name.clone(),
                        AssignTarget::ArrayElement { .. } => {
                            // INPUT 不支持数组元素
                            String::new()
                        }
                    })
                    .collect();
                
                self.execute_input(prompt.as_deref(), &var_names)?;
                Ok(())
            }
            
            Statement::Data { values: _ } => {
                // DATA 语句在程序加载时处理，执行时跳过
                // 数据已经存储在 data_values 中
                Ok(())
            }
            
            Statement::Read { variables } => {
                for target in variables {
                    let var_name = match target {
                        AssignTarget::Variable(name) => name.as_str(),
                        AssignTarget::ArrayElement { .. } => {
                            return Err(BasicError::SyntaxError(
                                "READ does not support array elements".to_string()
                            ));
                        }
                    };
                    
                    let data_val = self.read_data_value()?;
                    
                    // 根据变量名判断类型
                    if var_name.ends_with('$') {
                        // 字符串变量
                        let str_val = match data_val {
                            DataValue::String(s) => s,
                            DataValue::Number(n) => n.to_string(),
                        };
                        self.variables.set(var_name, Value::String(str_val))?;
                    } else {
                        // 数值变量
                        let num_val = match data_val {
                            DataValue::Number(n) => n,
                            DataValue::String(s) => {
                                s.trim().parse::<f64>().unwrap_or(0.0)
                            }
                        };
                        self.variables.set(var_name, Value::Number(num_val))?;
                    }
                }
                Ok(())
            }
            
            Statement::Restore { line_number } => {
                if line_number.is_some() {
                    // RESTORE 到指定行（暂不支持，需要跟踪每行的 DATA 位置）
                    return Err(BasicError::SyntaxError(
                        "RESTORE to specific line not yet implemented".to_string()
                    ));
                }
                self.restore_data();
                Ok(())
            }
            
            Statement::For { var, start, end, step } => {
                // 计算起始值、结束值和步长
                let start_val = self.eval_expr(&start)?;
                let end_val = self.eval_expr(&end)?;
                let step_val = if let Some(ref s) = step {
                    self.eval_expr(s)?
                } else {
                    Value::Number(1.0)
                };
                
                // 提取数值
                let start_num = start_val.as_number()?;
                let end_num = end_val.as_number()?;
                let step_num = step_val.as_number()?;
                
                // 检查步长
                if step_num == 0.0 {
                    return Err(BasicError::IllegalQuantity(
                        "FOR loop step cannot be zero".to_string()
                    ));
                }
                
                // 设置循环变量初值
                self.variables.set(var, Value::Number(start_num))?;
                
                // 获取当前位置
                let loop_line = self.runtime.get_current_line()
                    .ok_or_else(|| BasicError::SyntaxError("FOR without line number".to_string()))?;
                let loop_stmt = self.runtime.get_current_stmt_index();
                
                // 将循环信息压入栈
                self.runtime.push_for_loop(
                    var.clone(),
                    end_num,
                    step_num,
                    loop_line,
                    loop_stmt,
                )?;
                
                Ok(())
            }
            
            Statement::Next { var } => {
                // 弹出 FOR 循环信息
                let (loop_var, end_val, step_val, loop_line, loop_stmt) = 
                    self.runtime.pop_for_loop(var.clone())?;
                
                // 获取当前循环变量的值
                let current_val = self.variables.get(&loop_var).as_number()?;
                
                // 递增/递减
                let new_val = current_val + step_val;
                
                // 检查是否继续循环
                let should_continue = if step_val > 0.0 {
                    new_val <= end_val
                } else {
                    new_val >= end_val
                };
                
                if should_continue {
                    // 更新循环变量
                    self.variables.set(&loop_var, Value::Number(new_val))?;
                    
                    // 重新压入栈（继续循环）
                    self.runtime.push_for_loop(
                        loop_var.clone(),
                        end_val,
                        step_val,
                        loop_line,
                        loop_stmt,
                    )?;
                    
                    // 跳转回 FOR 语句的下一条语句
                    self.runtime.set_execution_position(loop_line, loop_stmt + 1)?;
                }
                // 否则继续执行下一条语句（循环结束）
                
                Ok(())
            }
            
            Statement::On { expr, targets, is_gosub } => {
                // 计算表达式的值
                let index_val = self.eval_expr(&expr)?;
                let index = index_val.as_number()? as i32;
                
                // 索引从 1 开始
                if index < 1 || index as usize > targets.len() {
                    // 超出范围，继续执行下一条语句
                    return Ok(());
                }
                
                // 获取目标行号
                let target_line = targets[(index - 1) as usize];
                
                if *is_gosub {
                    // ON...GOSUB：保存返回地址并跳转
                    let return_line = self.runtime.get_current_line()
                        .ok_or_else(|| BasicError::SyntaxError("GOSUB without line number".to_string()))?;
                    let return_stmt = self.runtime.get_current_stmt_index();
                    
                    self.runtime.push_gosub(return_line, return_stmt)?;
                    self.runtime.set_execution_position(target_line, 0)?;
                } else {
                    // ON...GOTO：直接跳转
                    self.runtime.set_execution_position(target_line, 0)?;
                }
                
                Ok(())
            }
            
            Statement::Load { filename } => {
                self.execute_load(filename)?;
                Ok(())
            }
            
            Statement::Save { filename } => {
                self.execute_save(filename)?;
                Ok(())
            }
            
            _ => {
                // 其他语句暂未实现
                Err(BasicError::SyntaxError(
                    "Statement not yet implemented".to_string()
                ))
            }
        }
    }
    
    /// 执行 INPUT 语句
    fn execute_input(&mut self, prompt: Option<&str>, variables: &[String]) -> Result<()> {
        // 显示提示符
        if let Some(p) = prompt {
            self.output(p);
            self.output("? ");
        } else {
            self.output("? ");
        }
        
        // 读取输入
        let input_line = if let Some(ref mut callback) = self.input_callback {
            let prompt_text = prompt.unwrap_or("");
            callback(prompt_text).ok_or_else(|| {
                BasicError::SyntaxError("No input provided".to_string())
            })?
        } else {
            // 在实际 REPL 中，这里会从 stdin 读取
            return Err(BasicError::SyntaxError(
                "No input callback set".to_string()
            ));
        };
        
        // 解析输入值（考虑引号内的逗号）
        let values = Self::parse_input_values(&input_line);
        
        if values.len() != variables.len() {
            self.output("?EXTRA IGNORED\n");
        }
        
        // 赋值给变量
        for (i, var_name) in variables.iter().enumerate() {
            if i >= values.len() {
                break;
            }
            
            let input_val = &values[i];
            
            if var_name.ends_with('$') {
                // 字符串变量
                let str_val = if input_val.starts_with('"') && input_val.ends_with('"') {
                    // 去掉引号
                    input_val[1..input_val.len()-1].to_string()
                } else {
                    input_val.clone()
                };
                self.variables.set(var_name, Value::String(str_val))?;
            } else {
                // 数值变量
                match input_val.parse::<f64>() {
                    Ok(num) => {
                        self.variables.set(var_name, Value::Number(num))?;
                    }
                    Err(_) => {
                        self.output("?REDO FROM START\n");
                        return Err(BasicError::TypeMismatch(
                            "Invalid number input".to_string()
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// 解析输入值，处理带引号的字符串
    fn parse_input_values(input: &str) -> Vec<String> {
        let mut values = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        
        for ch in input.chars() {
            match ch {
                '"' => {
                    in_quotes = !in_quotes;
                    current.push(ch);
                }
                ',' if !in_quotes => {
                    values.push(current.trim().to_string());
                    current.clear();
                }
                _ => {
                    current.push(ch);
                }
            }
        }
        
        if !current.is_empty() || input.ends_with(',') {
            values.push(current.trim().to_string());
        }
        
        values
    }
    
    /// 执行 SAVE 命令 - 保存程序到文件
    fn execute_save(&self, filename: &str) -> Result<()> {
        use std::fs::File;
        use std::io::Write;
        
        let program = self.runtime.clone_program();
        if program.is_empty() {
            return Err(BasicError::SyntaxError("No program to save".to_string()));
        }
        
        let mut file = File::create(filename).map_err(|e| {
            BasicError::SyntaxError(format!("Failed to create file: {}", e))
        })?;
        
        for (_, line) in program.iter() {
            let line_text = Self::serialize_program_line(line);
            writeln!(file, "{}", line_text).map_err(|e| {
                BasicError::SyntaxError(format!("Failed to write to file: {}", e))
            })?;
        }
        
        Ok(())
    }
    
    /// 将程序行序列化为文本
    pub fn serialize_program_line(line: &ProgramLine) -> String {
        let mut result = format!("{}", line.line_number);
        
        for (i, stmt) in line.statements.iter().enumerate() {
            if i > 0 {
                result.push_str(":");
            }
            result.push(' ');
            result.push_str(&Self::serialize_statement(stmt));
        }
        
        result
    }
    
    /// 将语句序列化为文本
    pub fn serialize_statement(stmt: &Statement) -> String {
        match stmt {
            Statement::Let { target, value } => {
                format!("{} = {}", Self::serialize_assign_target(target), Self::serialize_expr(value))
            }
            Statement::Print { items } => {
                let mut result = "PRINT ".to_string();
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        result.push_str(&Self::serialize_print_item(item));
                    } else {
                        result.push_str(&Self::serialize_print_item(item));
                    }
                }
                result
            }
            Statement::If { condition, then_part } => {
                format!("IF {} THEN {}", Self::serialize_expr(condition), Self::serialize_then_part(then_part))
            }
            Statement::Goto { line_number } => {
                format!("GOTO {}", Self::serialize_expr(line_number))
            }
            Statement::Gosub { line_number } => {
                format!("GOSUB {}", Self::serialize_expr(line_number))
            }
            Statement::Return => "RETURN".to_string(),
            Statement::For { var, start, end, step } => {
                let mut result = format!("FOR {} = {} TO {}", var, Self::serialize_expr(start), Self::serialize_expr(end));
                if let Some(step_expr) = step {
                    result.push_str(&format!(" STEP {}", Self::serialize_expr(step_expr)));
                }
                result
            }
            Statement::Next { var } => {
                if let Some(v) = var {
                    format!("NEXT {}", v)
                } else {
                    "NEXT".to_string()
                }
            }
            Statement::On { expr, targets, is_gosub } => {
                let keyword = if *is_gosub { "GOSUB" } else { "GOTO" };
                let target_str = targets.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("ON {} {} {}", Self::serialize_expr(expr), keyword, target_str)
            }
            Statement::Input { prompt, variables } => {
                let mut result = "INPUT ".to_string();
                if let Some(p) = prompt {
                    result.push_str(&format!("\"{}\" ", p));
                }
                let var_str = variables.iter()
                    .map(|v| Self::serialize_assign_target(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                result.push_str(&var_str);
                result
            }
            Statement::Dim { arrays } => {
                let arr_str = arrays.iter()
                    .map(|arr| {
                        let dims = arr.dimensions.iter()
                            .map(|d| Self::serialize_expr(d))
                            .collect::<Vec<_>>()
                            .join(",");
                        format!("{}({})", arr.name, dims)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("DIM {}", arr_str)
            }
            Statement::Data { values } => {
                let val_str = values.iter()
                    .map(|v| match v {
                        crate::ast::DataValue::Number(n) => n.to_string(),
                        crate::ast::DataValue::String(s) => format!("\"{}\"", s),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("DATA {}", val_str)
            }
            Statement::Read { variables } => {
                let var_str = variables.iter()
                    .map(|v| Self::serialize_assign_target(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("READ {}", var_str)
            }
            Statement::Restore { line_number } => {
                if let Some(ln) = line_number {
                    format!("RESTORE {}", ln)
                } else {
                    "RESTORE".to_string()
                }
            }
            Statement::Rem => "REM".to_string(),
            Statement::End => "END".to_string(),
            Statement::Stop => "STOP".to_string(),
            Statement::New => "NEW".to_string(),
            Statement::Clear => "CLEAR".to_string(),
            _ => "REM UNSUPPORTED STATEMENT".to_string(),
        }
    }
    
    /// 将表达式序列化为文本
    pub fn serialize_expr(expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => n.to_string(),
            Expr::String(s) => format!("\"{}\"", s),
            Expr::Variable(name) => name.clone(),
            Expr::ArrayAccess { name, indices } => {
                let idx_str = indices.iter()
                    .map(|i| Self::serialize_expr(i))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, idx_str)
            }
            Expr::FunctionCall { name, args } => {
                let arg_str = args.iter()
                    .map(|a| Self::serialize_expr(a))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, arg_str)
            }
            Expr::BinaryOp { left, op, right } => {
                let op_str = match op {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Subtract => "-",
                    BinaryOperator::Multiply => "*",
                    BinaryOperator::Divide => "/",
                    BinaryOperator::Power => "^",
                    BinaryOperator::Equal => "=",
                    BinaryOperator::NotEqual => "<>",
                    BinaryOperator::Less => "<",
                    BinaryOperator::Greater => ">",
                    BinaryOperator::LessEqual => "<=",
                    BinaryOperator::GreaterEqual => ">=",
                    BinaryOperator::And => " AND ",
                    BinaryOperator::Or => " OR ",
                };
                format!("({} {} {})", Self::serialize_expr(left), op_str, Self::serialize_expr(right))
            }
            Expr::UnaryOp { op, operand } => {
                let op_str = match op {
                    UnaryOperator::Minus => "-",
                    UnaryOperator::Not => "NOT ",
                };
                format!("{}{}", op_str, Self::serialize_expr(operand))
            }
        }
    }
    
    /// 将赋值目标序列化为文本
    pub fn serialize_assign_target(target: &AssignTarget) -> String {
        match target {
            AssignTarget::Variable(name) => name.clone(),
            AssignTarget::ArrayElement { name, indices } => {
                let idx_str = indices.iter()
                    .map(|i| Self::serialize_expr(i))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, idx_str)
            }
        }
    }
    
    /// 将THEN部分序列化为文本
    pub fn serialize_then_part(then_part: &ThenPart) -> String {
        match then_part {
            ThenPart::LineNumber(ln) => ln.to_string(),
            ThenPart::Statement(stmt) => Self::serialize_statement(stmt),
            ThenPart::Statements(stmts) => {
                stmts.iter()
                    .map(|s| Self::serialize_statement(s))
                    .collect::<Vec<_>>()
                    .join(":")
            }
        }
    }
    
    /// 将PRINT项序列化为文本
    fn serialize_print_item(item: &PrintItem) -> String {
        match item {
            PrintItem::Expr(expr) => Self::serialize_expr(expr),
            PrintItem::Tab(expr) => format!("TAB({})", Self::serialize_expr(expr)),
            PrintItem::Spc(expr) => format!("SPC({})", Self::serialize_expr(expr)),
            PrintItem::Comma => ",".to_string(),
            PrintItem::Semicolon => ";".to_string(),
        }
    }
    
    /// 执行 LOAD 命令 - 从文件加载程序
    fn execute_load(&mut self, filename: &str) -> Result<()> {
        use std::fs;
        use crate::tokenizer::Tokenizer;
        use crate::parser::Parser;
        
        // 读取文件内容
        let content = fs::read_to_string(filename).map_err(|e| {
            BasicError::SyntaxError(format!("Failed to read file: {}", e))
        })?;
        
        // 清空当前程序
        self.runtime.clear_program();
        self.variables.clear();
        
        // 逐行解析并添加到程序
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            // 使用tokenizer和parser解析每一行
            let mut tokenizer = Tokenizer::new(line);
            let tokens = tokenizer.tokenize_line()?;
            
            let mut parser = Parser::new(tokens);
            if let Some(program_line) = parser.parse_line()? {
                if program_line.line_number > 0 {
                    self.runtime.add_line(program_line);
                }
            }
        }
        
        Ok(())
    }
    
    /// 执行 PRINT 语句
    fn execute_print(&mut self, items: &[PrintItem]) -> Result<()> {
        if items.is_empty() {
            // 空 PRINT，仅输出换行
            self.output_newline();
            return Ok(());
        }
        
        for item in items.iter() {
            match item {
                PrintItem::Expr(expr) => {
                    let val = self.eval_expr(expr)?;
                    self.print_value(&val)?;
                }
                PrintItem::Tab(expr) => {
                    let target_col = self.eval_expr(expr)?.as_number()? as usize;
                    if target_col > self.print_column {
                        let spaces = target_col - self.print_column;
                        self.output(&" ".repeat(spaces));
                    } else if target_col < self.print_column {
                        // TAB 到更小的列，换行后跳转
                        self.output_newline();
                        self.output(&" ".repeat(target_col));
                    }
                }
                PrintItem::Spc(expr) => {
                    let spaces = self.eval_expr(expr)?.as_number()? as usize;
                    self.output(&" ".repeat(spaces));
                }
                PrintItem::Comma => {
                    // 逗号：对齐到下一个 14 列边界
                    let next_col = ((self.print_column / 14) + 1) * 14;
                    let spaces_needed = next_col - self.print_column;
                    self.output(&" ".repeat(spaces_needed));
                }
                PrintItem::Semicolon => {
                    // 分号：不添加空格（紧密连接）
                }
            }
        }
        
        // 检查最后一项是否是分隔符
        if let Some(last) = items.last() {
            if !matches!(last, PrintItem::Comma | PrintItem::Semicolon) {
                // 如果最后不是分隔符，输出换行
                self.output_newline();
            }
        } else {
            self.output_newline();
        }
        
        Ok(())
    }
    
    /// 打印值（根据 BASIC 格式）
    fn print_value(&mut self, val: &Value) -> Result<()> {
        match val {
            Value::Number(n) => {
                // BASIC 数值格式：正数前后各有空格，负数前有空格
                let formatted = if *n >= 0.0 {
                    format!(" {} ", n)
                } else {
                    format!(" {}", n)
                };
                self.output(&formatted);
            }
            Value::String(s) => {
                // 普通字符串，直接输出
                self.output(s);
            }
        }
        Ok(())
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Requirement: 算术运算符 - 加法
    #[test]
    fn test_addition() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Add,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    // Requirement: 算术运算符 - 减法
    #[test]
    fn test_subtraction() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(10.0),
            BinaryOperator::Subtract,
            Expr::Number(7.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    // Requirement: 算术运算符 - 乘法
    #[test]
    fn test_multiplication() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(4.0),
            BinaryOperator::Multiply,
            Expr::Number(5.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(20.0));
    }

    // Requirement: 算术运算符 - 除法
    #[test]
    fn test_division() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(15.0),
            BinaryOperator::Divide,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    // Requirement: 算术运算符 - 浮点除法
    #[test]
    fn test_float_division() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(10.0),
            BinaryOperator::Divide,
            Expr::Number(4.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(2.5));
    }

    // Requirement: 算术运算符 - 除以零
    #[test]
    fn test_division_by_zero() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Divide,
            Expr::Number(0.0)
        );
        let result = exec.eval_expr(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BasicError::DivisionByZero));
    }

    // Requirement: 算术运算符 - 乘方
    #[test]
    fn test_power() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(2.0),
            BinaryOperator::Power,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    // Requirement: 一元运算符 - 一元负号
    #[test]
    fn test_unary_minus() {
        let mut exec = Executor::new();
        let expr = Expr::unary(UnaryOperator::Minus, Expr::Number(5.0));
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-5.0));
    }

    // Requirement: 关系运算符 - 等于
    #[test]
    fn test_equal() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Equal,
            Expr::Number(5.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-1.0)); // BASIC true = -1
    }

    // Requirement: 关系运算符 - 不等于
    #[test]
    fn test_not_equal() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::NotEqual,
            Expr::Number(4.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-1.0));
    }

    // Requirement: 字符串运算符 - 字符串连接
    #[test]
    fn test_string_concatenation() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::String("HELLO".to_string()),
            BinaryOperator::Add,
            Expr::String(" WORLD".to_string())
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::String("HELLO WORLD".to_string()));
    }

    // Test: 变量读取
    #[test]
    fn test_variable_read() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(42.0)).unwrap();
        
        let expr = Expr::Variable("A".to_string());
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    // Test: LET 语句执行
    #[test]
    fn test_let_statement() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Let {
            target: AssignTarget::Variable("X".to_string()),
            value: Expr::Number(100.0),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.variables.get("X"), Value::Number(100.0));
    }

    // Test: DIM 语句执行
    #[test]
    fn test_dim_statement() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Dim {
            arrays: vec![
                ArrayDim {
                    name: "A".to_string(),
                    dimensions: vec![Expr::Number(10.0)],
                }
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert!(exec.variables.has_array("A"));
    }

    // Test: 数学函数
    #[test]
    fn test_math_functions() {
        let mut exec = Executor::new();
        
        // ABS
        let expr = Expr::FunctionCall {
            name: "ABS".to_string(),
            args: vec![Expr::Number(-42.0)],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(42.0));
        
        // INT
        let expr = Expr::FunctionCall {
            name: "INT".to_string(),
            args: vec![Expr::Number(3.7)],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    // Test: 字符串函数
    #[test]
    fn test_string_functions() {
        let mut exec = Executor::new();
        
        // LEN
        let expr = Expr::FunctionCall {
            name: "LEN".to_string(),
            args: vec![Expr::String("HELLO".to_string())],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));
        
        // LEFT$
        let expr = Expr::FunctionCall {
            name: "LEFT$".to_string(),
            args: vec![
                Expr::String("HELLO".to_string()),
                Expr::Number(3.0),
            ],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::String("HEL".to_string()));
    }

    // Test: 复杂表达式
    #[test]
    fn test_complex_expression() {
        let mut exec = Executor::new();
        
        // 2 + 3 * 4 = 14
        let expr = Expr::binary(
            Expr::Number(2.0),
            BinaryOperator::Add,
            Expr::binary(
                Expr::Number(3.0),
                BinaryOperator::Multiply,
                Expr::Number(4.0)
            )
        );
        
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(14.0));
    }

    // Requirement: PRINT 语句 - 打印数值
    #[test]
    fn test_print_number() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(42.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 42 \n");
    }

    // Requirement: PRINT 语句 - 打印字符串
    #[test]
    fn test_print_string() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("HELLO".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "HELLO\n");
    }

    // Requirement: PRINT 语句 - 打印变量
    #[test]
    fn test_print_variable() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(10.0)).unwrap();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Variable("A".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 10 \n");
    }

    // Requirement: PRINT 语句 - 分号分隔（紧密连接）
    #[test]
    fn test_print_semicolon() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(1.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::Number(2.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::Number(3.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 1  2  3 \n");
    }

    // Requirement: PRINT 语句 - 行尾分号（不换行）
    #[test]
    fn test_print_no_newline() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(42.0)),
                PrintItem::Semicolon,
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 42 ");
    }

    // Requirement: PRINT 语句 - 空 PRINT
    #[test]
    fn test_print_empty() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "\n");
    }

    // Requirement: PRINT 语句 - 逗号分隔（列对齐）
    #[test]
    fn test_print_comma_alignment() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(1.0)),
                PrintItem::Comma,
                PrintItem::Expr(Expr::Number(2.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // 第一个数 " 1 " 占 3 列，逗号应该对齐到第 14 列
        assert!(output.starts_with(" 1 "));
        assert!(output.contains(" 2 "));
    }

    // Requirement: GOTO 语句
    #[test]
    fn test_goto_statement() {
        let mut exec = Executor::new();
        
        // 添加程序行
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("A".to_string()),
                value: Expr::Number(1.0),
            }]
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("B".to_string()),
                value: Expr::Number(99.0),
            }]
        });
        
        let stmt = Statement::Goto {
            line_number: Expr::Number(100.0),
        };
        exec.execute_statement(&stmt).unwrap();
        
        // 验证跳转成功（下一行应该是 100）
        assert_eq!(exec.runtime().get_current_line(), Some(100));
    }

    // Requirement: IF...THEN 语句 - 条件为真
    #[test]
    fn test_if_then_true() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(15.0)).unwrap();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行来设置初始状态
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::LineNumber(100)),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.runtime().get_current_line(), Some(100));
    }

    // Requirement: IF...THEN 语句 - 条件为假
    #[test]
    fn test_if_then_false() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(5.0)).unwrap();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![],
        });
        
        let current_line = exec.runtime().get_current_line();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::LineNumber(100)),
        };
        
        exec.execute_statement(&stmt).unwrap();
        // 条件为假，不应该跳转
        assert_eq!(exec.runtime().get_current_line(), current_line);
    }

    // Requirement: IF...THEN 语句 - THEN 后跟语句
    #[test]
    fn test_if_then_statement() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(15.0)).unwrap();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::Statement(
                Statement::Print {
                    items: vec![
                        PrintItem::Expr(Expr::String("TRUE".to_string())),
                    ],
                }
            )),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "TRUE\n");
    }

    // Test: TAB 函数
    #[test]
    fn test_tab_function() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("A".to_string())),
                PrintItem::Semicolon,
                PrintItem::Tab(Expr::Number(10.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::String("B".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // A 在列 0，TAB(10) 跳到列 10，然后是 B
        assert!(output.starts_with("A"));
        assert!(output.contains("B"));
    }

    // Test: SPC 函数
    #[test]
    fn test_spc_function() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("A".to_string())),
                PrintItem::Semicolon,
                PrintItem::Spc(Expr::Number(5.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::String("B".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // A + 5个空格 + B
        assert_eq!(output, "A     B\n");
    }

    // Requirement: GOSUB 和 RETURN 语句 - 子程序调用
    #[test]
    fn test_gosub_statement() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 500,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        let stmt = Statement::Gosub {
            line_number: Expr::Number(500.0),
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证跳转到子程序
        assert_eq!(exec.runtime().get_current_line(), Some(500));
        // 验证调用栈深度
        assert_eq!(exec.runtime().stack_depth(), 1);
    }

    // Requirement: GOSUB 和 RETURN 语句 - 子程序返回
    #[test]
    fn test_return_statement() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 500,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行并设置调用栈
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        exec.runtime_mut().push_gosub(20, 0).unwrap();
        exec.runtime_mut().set_execution_position(500, 0).unwrap();
        
        let stmt = Statement::Return;
        exec.execute_statement(&stmt).unwrap();
        
        // 验证返回到调用点
        assert_eq!(exec.runtime().get_current_line(), Some(20));
        // 验证调用栈已弹出
        assert_eq!(exec.runtime().stack_depth(), 0);
    }

    // Requirement: GOSUB 和 RETURN 语句 - 嵌套子程序
    #[test]
    fn test_nested_gosub() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 200,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // 第一次 GOSUB
        exec.execute_statement(&Statement::Gosub {
            line_number: Expr::Number(100.0),
        }).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 1);
        
        // 第二次 GOSUB（嵌套）
        exec.execute_statement(&Statement::Gosub {
            line_number: Expr::Number(200.0),
        }).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 2);
        assert_eq!(exec.runtime().get_current_line(), Some(200));
        
        // 第一次 RETURN
        exec.execute_statement(&Statement::Return).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 1);
        assert_eq!(exec.runtime().get_current_line(), Some(100));
        
        // 第二次 RETURN
        exec.execute_statement(&Statement::Return).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 0);
        assert_eq!(exec.runtime().get_current_line(), Some(10));
    }

    // Requirement: INPUT 语句 - 基本输入
    #[test]
    fn test_input_basic() {
        let mut exec = Executor::new();
        
        // 设置输入回调
        exec.set_input_callback(|_| Some("42".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证输出提示符
        assert!(exec.get_output().contains("? "));
        
        // 验证变量赋值
        assert_eq!(exec.variables.get("A"), Value::Number(42.0));
    }

    // Requirement: INPUT 语句 - 带提示符的输入
    #[test]
    fn test_input_with_prompt() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("100".to_string()));
        
        let stmt = Statement::Input {
            prompt: Some("ENTER VALUE".to_string()),
            variables: vec![AssignTarget::Variable("X".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证提示符
        assert!(exec.get_output().contains("ENTER VALUE? "));
        assert_eq!(exec.variables.get("X"), Value::Number(100.0));
    }

    // Requirement: INPUT 语句 - 输入多个变量
    #[test]
    fn test_input_multiple_variables() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("10, 20, 30".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string()),
                AssignTarget::Variable("C".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(10.0));
        assert_eq!(exec.variables.get("B"), Value::Number(20.0));
        assert_eq!(exec.variables.get("C"), Value::Number(30.0));
    }

    // Requirement: INPUT 语句 - 字符串输入
    #[test]
    fn test_input_string() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("HELLO".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A$".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A$"), Value::String("HELLO".to_string()));
    }

    // Requirement: INPUT 语句 - 字符串带引号
    #[test]
    fn test_input_string_with_quotes() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("\"HELLO, WORLD\"".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A$".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A$"), Value::String("HELLO, WORLD".to_string()));
    }

    // Requirement: DATA/READ 机制 - DATA 存储和 READ 读取
    #[test]
    fn test_data_read() {
        let mut exec = Executor::new();
        
        // 添加 DATA 值
        exec.add_data_value(DataValue::Number(1.0));
        exec.add_data_value(DataValue::Number(2.0));
        exec.add_data_value(DataValue::Number(3.0));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string()),
                AssignTarget::Variable("C".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(1.0));
        assert_eq!(exec.variables.get("B"), Value::Number(2.0));
        assert_eq!(exec.variables.get("C"), Value::Number(3.0));
    }

    // Requirement: DATA/READ 机制 - 混合数据类型
    #[test]
    fn test_data_read_mixed_types() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(42.0));
        exec.add_data_value(DataValue::String("HELLO".to_string()));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B$".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(42.0));
        assert_eq!(exec.variables.get("B$"), Value::String("HELLO".to_string()));
    }

    // Requirement: DATA/READ 机制 - OUT OF DATA 错误
    #[test]
    fn test_out_of_data_error() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(1.0));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string())
            ],
        };
        
        let result = exec.execute_statement(&stmt);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BasicError::OutOfData));
    }

    // Requirement: RESTORE 数据指针 - RESTORE 重置到开头
    #[test]
    fn test_restore() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(1.0));
        exec.add_data_value(DataValue::Number(2.0));
        
        // 第一次 READ
        exec.execute_statement(&Statement::Read {
            variables: vec![AssignTarget::Variable("A".to_string())],
        }).unwrap();
        assert_eq!(exec.variables.get("A"), Value::Number(1.0));
        
        // RESTORE
        exec.execute_statement(&Statement::Restore {
            line_number: None,
        }).unwrap();
        
        // 第二次 READ（应该重新从头开始）
        exec.execute_statement(&Statement::Read {
            variables: vec![AssignTarget::Variable("B".to_string())],
        }).unwrap();
        assert_eq!(exec.variables.get("B"), Value::Number(1.0));
    }

    // Requirement: FOR...NEXT 循环 - 正向循环
    #[test]
    fn test_for_next_basic() {
        let mut exec = Executor::new();
        
        // 添加测试程序：FOR I=1 TO 3: PRINT I: NEXT I
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(1.0),
                    end: Expr::Number(3.0),
                    step: None,
                },
            ],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Next { var: Some("I".to_string()) }],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // 第一次循环：I=1
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(1.0),
            end: Expr::Number(3.0),
            step: None,
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
        
        // NEXT：I=2
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=3
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
        
        // NEXT：循环结束 (I递增到4但不再循环)
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        // 循环已结束，变量值应该为循环后的值 4
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
    }

    // Requirement: FOR...NEXT 循环 - 步长为 2
    #[test]
    fn test_for_next_step() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(0.0),
                    end: Expr::Number(4.0),
                    step: Some(Expr::Number(2.0)),
                },
            ],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // FOR I=0 TO 4 STEP 2
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(0.0),
            end: Expr::Number(4.0),
            step: Some(Expr::Number(2.0)),
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(0.0));
        
        // NEXT：I=2
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=4
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(4.0));
        
        // NEXT：循环结束
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(4.0));
    }

    // Requirement: FOR...NEXT 循环 - 负步长
    #[test]
    fn test_for_next_negative_step() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(3.0),
                    end: Expr::Number(1.0),
                    step: Some(Expr::Number(-1.0)),
                },
            ],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // FOR I=3 TO 1 STEP -1
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(3.0),
            end: Expr::Number(1.0),
            step: Some(Expr::Number(-1.0)),
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
        
        // NEXT：I=2
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=1
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
        
        // NEXT：循环结束
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
    }

    // Requirement: ON...GOTO - 基于表达式的跳转
    #[test]
    fn test_on_goto() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 200,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // ON 2 GOTO 100,200,300
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(2.0),
            targets: vec![100, 200, 300],
            is_gosub: false,
        }).unwrap();
        
        // 应该跳转到 200
        assert_eq!(exec.runtime().get_current_line(), Some(200));
    }

    // Requirement: ON...GOSUB - 基于表达式的子程序调用
    #[test]
    fn test_on_gosub() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // ON 1 GOSUB 100,200
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(1.0),
            targets: vec![100, 200],
            is_gosub: true,
        }).unwrap();
        
        // 应该跳转到 100
        assert_eq!(exec.runtime().get_current_line(), Some(100));
        // 栈深度应该为 1
        assert_eq!(exec.runtime().stack_depth(), 1);
    }

    // Requirement: ON...GOTO - 值超出范围
    #[test]
    fn test_on_goto_out_of_range() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        let current_line = exec.runtime().get_current_line();
        
        // ON 5 GOTO 100,200  (5 超出范围)
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(5.0),
            targets: vec![100, 200],
            is_gosub: false,
        }).unwrap();
        
        // 应该继续在当前行
        assert_eq!(exec.runtime().get_current_line(), current_line);
    }
    
    #[test]
    fn test_save_and_load() {
        use std::fs;
        
        let mut exec = Executor::new();
        
        // 添加一些程序行
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Print {
                items: vec![PrintItem::Expr(Expr::String("HELLO".to_string()))],
            }],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("A".to_string()),
                value: Expr::Number(42.0),
            }],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 30,
            statements: vec![Statement::End],
        });
        
        // 保存程序到文件
        let filename = "test_program.bas";
        exec.execute_statement(&Statement::Save {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证文件存在
        assert!(fs::metadata(filename).is_ok());
        
        // 清空程序
        exec.runtime_mut().clear_program();
        assert_eq!(exec.runtime().line_count(), 0);
        
        // 加载程序
        exec.execute_statement(&Statement::Load {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证程序已加载
        assert_eq!(exec.runtime().line_count(), 3);
        assert!(exec.runtime().get_line(10).is_some());
        assert!(exec.runtime().get_line(20).is_some());
        assert!(exec.runtime().get_line(30).is_some());
        
        // 清理测试文件
        fs::remove_file(filename).ok();
    }
    
    #[test]
    fn test_save_empty_program() {
        let mut exec = Executor::new();
        
        // 尝试保存空程序应该失败
        let result = exec.execute_statement(&Statement::Save {
            filename: "empty.bas".to_string(),
        });
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_load_nonexistent_file() {
        let mut exec = Executor::new();
        
        // 尝试加载不存在的文件应该失败
        let result = exec.execute_statement(&Statement::Load {
            filename: "nonexistent.bas".to_string(),
        });
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_save_complex_program() {
        use std::fs;
        
        let mut exec = Executor::new();
        
        // 创建一个更复杂的程序
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(1.0),
                    end: Expr::Number(10.0),
                    step: Some(Expr::Number(1.0)),
                },
            ],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![
                Statement::Print {
                    items: vec![PrintItem::Expr(Expr::Variable("I".to_string()))],
                },
            ],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 30,
            statements: vec![Statement::Next { var: Some("I".to_string()) }],
        });
        
        // 保存并重新加载
        let filename = "test_complex.bas";
        exec.execute_statement(&Statement::Save {
            filename: filename.to_string(),
        }).unwrap();
        
        exec.runtime_mut().clear_program();
        
        exec.execute_statement(&Statement::Load {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证程序结构
        assert_eq!(exec.runtime().line_count(), 3);
        
        // 清理
        fs::remove_file(filename).ok();
    }
}


```

```rust
/// 执行引擎
///
/// 求值表达式并执行语句

use crate::ast::*;
use crate::error::{BasicError, Result};
use crate::runtime::Runtime;
use crate::variables::{Value, Variables};

/// 输入回调函数类型
pub type InputCallback = Box<dyn FnMut(&str) -> Option<String>>;

/// 执行引擎
pub struct Executor {
    runtime: Runtime,
    variables: Variables,
    /// 输出缓冲区（用于测试和捕获输出）
    output_buffer: Vec<String>,
    /// 当前打印列位置
    print_column: usize,
    /// DATA 数据存储
    data_values: Vec<DataValue>,
    /// DATA 数据指针（当前读取位置）
    data_pointer: usize,
    /// 输入回调函数（用于测试）
    input_callback: Option<InputCallback>,
}

/// DATA 值类型
#[derive(Debug, Clone, PartialEq)]
pub enum DataValue {
    Number(f64),
    String(String),
}

impl Executor {
    /// 创建新的执行器
    pub fn new() -> Self {
        Executor {
            runtime: Runtime::new(),
            variables: Variables::new(),
            output_buffer: Vec::new(),
            print_column: 0,
            data_values: Vec::new(),
            data_pointer: 0,
            input_callback: None,
        }
    }
    
    /// 设置输入回调函数（用于测试）
    pub fn set_input_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&str) -> Option<String> + 'static,
    {
        self.input_callback = Some(Box::new(callback));
    }
    
    /// 添加 DATA 值
    pub fn add_data_value(&mut self, value: DataValue) {
        self.data_values.push(value);
    }
    
    /// 重置 DATA 指针
    pub fn restore_data(&mut self) {
        self.data_pointer = 0;
    }
    
    /// 读取下一个 DATA 值
    fn read_data_value(&mut self) -> Result<DataValue> {
        if self.data_pointer >= self.data_values.len() {
            return Err(BasicError::OutOfData);
        }
        
        let value = self.data_values[self.data_pointer].clone();
        self.data_pointer += 1;
        Ok(value)
    }
    
    /// 获取输出内容（用于测试）
    pub fn get_output(&self) -> String {
        self.output_buffer.join("")
    }
    
    /// 清空输出缓冲区
    pub fn clear_output(&mut self) {
        self.output_buffer.clear();
        self.print_column = 0;
    }
    
    /// 输出文本（添加到缓冲区并打印到终端）
    fn output(&mut self, text: &str) {
        // 打印到终端
        print!("{}", text);
        use std::io::Write;
        std::io::stdout().flush().ok();
        
        // 同时添加到缓冲区（用于测试）
        self.output_buffer.push(text.to_string());
        
        // 更新列位置
        for ch in text.chars() {
            if ch == '\n' {
                self.print_column = 0;
            } else {
                self.print_column += 1;
            }
        }
    }
    
    /// 输出换行
    fn output_newline(&mut self) {
        self.output("\n");
    }

    /// 获取运行时引用
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    /// 获取变量存储引用
    pub fn variables(&self) -> &Variables {
        &self.variables
    }

    /// 获取运行时可变引用
    pub fn runtime_mut(&mut self) -> &mut Runtime {
        &mut self.runtime
    }

    /// 获取变量存储可变引用
    pub fn variables_mut(&mut self) -> &mut Variables {
        &mut self.variables
    }

    /// 求值表达式
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            
            Expr::String(s) => Ok(Value::String(s.clone())),
            
            Expr::Variable(name) => {
                Ok(self.variables.get(name))
            }
            
            Expr::ArrayAccess { name, indices } => {
                // 求值所有索引
                let idx_values: Result<Vec<usize>> = indices.iter()
                    .map(|idx_expr| {
                        self.eval_expr(idx_expr)?
                            .as_number()
                            .and_then(|n| {
                                if n < 0.0 {
                                    Err(BasicError::SubscriptOutOfRange(
                                        "Negative array index".to_string()
                                    ))
                                } else {
                                    Ok(n as usize)
                                }
                            })
                    })
                    .collect();
                
                let indices_usize = idx_values?;
                self.variables.get_array_element(name, &indices_usize)
            }
            
            Expr::FunctionCall { name, args } => {
                self.eval_function_call(name, args)
            }
            
            Expr::BinaryOp { left, op, right } => {
                self.eval_binary_op(left, *op, right)
            }
            
            Expr::UnaryOp { op, operand } => {
                self.eval_unary_op(*op, operand)
            }
        }
    }

    /// 求值二元运算
    fn eval_binary_op(&mut self, left: &Expr, op: BinaryOperator, right: &Expr) -> Result<Value> {
        use BinaryOperator::*;

        let left_val = self.eval_expr(left)?;
        let right_val = self.eval_expr(right)?;

        match op {
            // 算术运算符
            Add => {
                if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    Ok(Value::Number(l + r))
                } else if left_val.is_string() && right_val.is_string() {
                    // 字符串连接
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    Ok(Value::String(format!("{}{}", l, r)))
                } else {
                    Err(BasicError::TypeMismatch(
                        "Cannot add incompatible types".to_string()
                    ))
                }
            }
            
            Subtract => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l - r))
            }
            
            Multiply => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l * r))
            }
            
            Divide => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                if r == 0.0 {
                    return Err(BasicError::DivisionByZero);
                }
                Ok(Value::Number(l / r))
            }
            
            Power => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l.powf(r)))
            }
            
            // 关系运算符（BASIC 中 true = -1, false = 0）
            Equal => {
                let result = if left_val == right_val { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            NotEqual => {
                let result = if left_val != right_val { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            Less => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l < r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l < r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            Greater => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l > r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l > r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            LessEqual => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l <= r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l <= r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            GreaterEqual => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l >= r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l >= r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            // 逻辑运算符（按位）
            And => {
                let l = left_val.as_number()? as i32;
                let r = right_val.as_number()? as i32;
                Ok(Value::Number((l & r) as f64))
            }
            
            Or => {
                let l = left_val.as_number()? as i32;
                let r = right_val.as_number()? as i32;
                Ok(Value::Number((l | r) as f64))
            }
        }
    }

    /// 求值一元运算
    fn eval_unary_op(&mut self, op: UnaryOperator, operand: &Expr) -> Result<Value> {
        let val = self.eval_expr(operand)?;
        
        match op {
            UnaryOperator::Minus => {
                let n = val.as_number()?;
                Ok(Value::Number(-n))
            }
            UnaryOperator::Not => {
                let n = val.as_number()? as i32;
                Ok(Value::Number((!n) as f64))
            }
        }
    }

    /// 求值函数调用（内置函数）
    fn eval_function_call(&mut self, name: &str, args: &[Expr]) -> Result<Value> {
        match name.to_uppercase().as_str() {
            // 数学函数
            "SGN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SGN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                let result = if n > 0.0 { 1.0 } else if n < 0.0 { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            "INT" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("INT requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.floor()))
            }
            
            "ABS" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ABS requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.abs()))
            }
            
            "SQR" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SQR requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n < 0.0 {
                    return Err(BasicError::IllegalQuantity("SQR of negative number".to_string()));
                }
                Ok(Value::Number(n.sqrt()))
            }
            
            "SIN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SIN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.sin()))
            }
            
            "COS" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("COS requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.cos()))
            }
            
            "TAN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("TAN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.tan()))
            }
            
            "ATN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ATN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.atan()))
            }
            
            "LOG" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("LOG requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n <= 0.0 {
                    return Err(BasicError::IllegalQuantity("LOG of non-positive number".to_string()));
                }
                Ok(Value::Number(n.ln()))
            }
            
            "EXP" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("EXP requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.exp()))
            }
            
            // 字符串函数
            "LEN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("LEN requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                Ok(Value::Number(s.len() as f64))
            }
            
            "ASC" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ASC requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                if s.is_empty() {
                    return Err(BasicError::IllegalQuantity("ASC of empty string".to_string()));
                }
                Ok(Value::Number(s.chars().next().unwrap() as u8 as f64))
            }
            
            "CHR$" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("CHR$ requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n < 0.0 || n > 255.0 {
                    return Err(BasicError::IllegalQuantity("CHR$ argument out of range".to_string()));
                }
                let ch = n as u8 as char;
                Ok(Value::String(ch.to_string()))
            }
            
            "STR$" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("STR$ requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                // BASIC 的 STR$ 在正数前加空格
                let s = if n >= 0.0 {
                    format!(" {}", n)
                } else {
                    n.to_string()
                };
                Ok(Value::String(s))
            }
            
            "VAL" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("VAL requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = s.trim().parse::<f64>().unwrap_or(0.0);
                Ok(Value::Number(n))
            }
            
            "LEFT$" => {
                if args.len() != 2 {
                    return Err(BasicError::SyntaxError("LEFT$ requires 2 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = self.eval_expr(&args[1])?.as_number()? as usize;
                let result: String = s.chars().take(n).collect();
                Ok(Value::String(result))
            }
            
            "RIGHT$" => {
                if args.len() != 2 {
                    return Err(BasicError::SyntaxError("RIGHT$ requires 2 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = self.eval_expr(&args[1])?.as_number()? as usize;
                let len = s.chars().count();
                let skip = if n > len { 0 } else { len - n };
                let result: String = s.chars().skip(skip).collect();
                Ok(Value::String(result))
            }
            
            "MID$" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err(BasicError::SyntaxError("MID$ requires 2 or 3 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let start = self.eval_expr(&args[1])?.as_number()? as usize;
                
                // BASIC 的 MID$ 是 1-based
                let start = if start > 0 { start - 1 } else { 0 };
                
                let chars: Vec<char> = s.chars().collect();
                
                if args.len() == 3 {
                    let len = self.eval_expr(&args[2])?.as_number()? as usize;
                    let result: String = chars.iter().skip(start).take(len).collect();
                    Ok(Value::String(result))
                } else {
                    let result: String = chars.iter().skip(start).collect();
                    Ok(Value::String(result))
                }
            }
            
            _ => Err(BasicError::SyntaxError(
                format!("Unknown function: {}", name)
            )),
        }
    }

    /// 执行语句
    pub fn execute_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Let { target, value } => {
                let val = self.eval_expr(value)?;
                
                match target {
                    AssignTarget::Variable(name) => {
                        self.variables.set(name, val)?;
                    }
                    AssignTarget::ArrayElement { name, indices } => {
                        let idx_values: Result<Vec<usize>> = indices.iter()
                            .map(|idx_expr| {
                                self.eval_expr(idx_expr)?
                                    .as_number()
                                    .map(|n| n as usize)
                            })
                            .collect();
                        
                        let indices_usize = idx_values?;
                        self.variables.set_array_element(name, &indices_usize, val)?;
                    }
                }
                
                Ok(())
            }
            
            Statement::End => {
                self.runtime.end_execution();
                Ok(())
            }
            
            Statement::Stop => {
                self.runtime.pause_execution();
                Ok(())
            }
            
            Statement::New => {
                self.runtime.clear_program();
                self.variables.clear();
                Ok(())
            }
            
            Statement::Clear => {
                self.variables.clear();
                Ok(())
            }
            
            Statement::Dim { arrays } => {
                for arr_dim in arrays {
                    let dimensions: Result<Vec<usize>> = arr_dim.dimensions.iter()
                        .map(|dim_expr| {
                            self.eval_expr(dim_expr)?
                                .as_number()
                                .map(|n| n as usize)
                        })
                        .collect();
                    
                    let dims = dimensions?;
                    self.variables.dim_array(&arr_dim.name, dims)?;
                }
                Ok(())
            }
            
            Statement::Print { items } => {
                self.execute_print(items)?;
                Ok(())
            }
            
            Statement::Goto { line_number } => {
                let line_val = self.eval_expr(line_number)?;
                let line = line_val.as_number()? as u16;
                self.runtime.set_execution_position(line, 0)?;
                Ok(())
            }
            
            Statement::If { condition, then_part } => {
                let cond_val = self.eval_expr(condition)?;
                let cond_num = cond_val.as_number()?;
                
                // BASIC 中，任何非零值都是真
                if cond_num != 0.0 {
                    match then_part.as_ref() {
                        ThenPart::LineNumber(line) => {
                            self.runtime.set_execution_position(*line as u16, 0)?;
                        }
                        ThenPart::Statement(stmt) => {
                            self.execute_statement(stmt)?;
                        }
                        ThenPart::Statements(stmts) => {
                            for stmt in stmts {
                                self.execute_statement(stmt)?;
                            }
                        }
                    }
                }
                Ok(())
            }
            
            Statement::Gosub { line_number } => {
                // 保存返回地址（当前行号和语句索引）
                let return_line = self.runtime.get_current_line().unwrap_or(0);
                let return_stmt = 0; // 简化：返回到下一行的第一条语句
                
                // 入栈
                self.runtime.push_gosub(return_line, return_stmt)?;
                
                // 跳转到子程序
                let line_val = self.eval_expr(line_number)?;
                let line = line_val.as_number()? as u16;
                self.runtime.set_execution_position(line, 0)?;
                
                Ok(())
            }
            
            Statement::Return => {
                // 从栈中弹出返回地址
                let (return_line, return_stmt) = self.runtime.pop_gosub()?;
                
                // 跳转回返回地址
                self.runtime.set_execution_position(return_line, return_stmt)?;
                
                Ok(())
            }
            
            Statement::Input { prompt, variables } => {
                // 提取变量名
                let var_names: Vec<String> = variables.iter()
                    .map(|target| match target {
                        AssignTarget::Variable(name) => name.clone(),
                        AssignTarget::ArrayElement { .. } => {
                            // INPUT 不支持数组元素
                            String::new()
                        }
                    })
                    .collect();
                
                self.execute_input(prompt.as_deref(), &var_names)?;
                Ok(())
            }
            
            Statement::Data { values: _ } => {
                // DATA 语句在程序加载时处理，执行时跳过
                // 数据已经存储在 data_values 中
                Ok(())
            }
            
            Statement::Read { variables } => {
                for target in variables {
                    let var_name = match target {
                        AssignTarget::Variable(name) => name.as_str(),
                        AssignTarget::ArrayElement { .. } => {
                            return Err(BasicError::SyntaxError(
                                "READ does not support array elements".to_string()
                            ));
                        }
                    };
                    
                    let data_val = self.read_data_value()?;
                    
                    // 根据变量名判断类型
                    if var_name.ends_with('$') {
                        // 字符串变量
                        let str_val = match data_val {
                            DataValue::String(s) => s,
                            DataValue::Number(n) => n.to_string(),
                        };
                        self.variables.set(var_name, Value::String(str_val))?;
                    } else {
                        // 数值变量
                        let num_val = match data_val {
                            DataValue::Number(n) => n,
                            DataValue::String(s) => {
                                s.trim().parse::<f64>().unwrap_or(0.0)
                            }
                        };
                        self.variables.set(var_name, Value::Number(num_val))?;
                    }
                }
                Ok(())
            }
            
            Statement::Restore { line_number } => {
                if line_number.is_some() {
                    // RESTORE 到指定行（暂不支持，需要跟踪每行的 DATA 位置）
                    return Err(BasicError::SyntaxError(
                        "RESTORE to specific line not yet implemented".to_string()
                    ));
                }
                self.restore_data();
                Ok(())
            }
            
            Statement::For { var, start, end, step } => {
                // 计算起始值、结束值和步长
                let start_val = self.eval_expr(&start)?;
                let end_val = self.eval_expr(&end)?;
                let step_val = if let Some(ref s) = step {
                    self.eval_expr(s)?
                } else {
                    Value::Number(1.0)
                };
                
                // 提取数值
                let start_num = start_val.as_number()?;
                let end_num = end_val.as_number()?;
                let step_num = step_val.as_number()?;
                
                // 检查步长
                if step_num == 0.0 {
                    return Err(BasicError::IllegalQuantity(
                        "FOR loop step cannot be zero".to_string()
                    ));
                }
                
                // 设置循环变量初值
                self.variables.set(var, Value::Number(start_num))?;
                
                // 获取当前位置
                let loop_line = self.runtime.get_current_line()
                    .ok_or_else(|| BasicError::SyntaxError("FOR without line number".to_string()))?;
                let loop_stmt = self.runtime.get_current_stmt_index();
                
                // 将循环信息压入栈
                self.runtime.push_for_loop(
                    var.clone(),
                    end_num,
                    step_num,
                    loop_line,
                    loop_stmt,
                )?;
                
                Ok(())
            }
            
            Statement::Next { var } => {
                // 弹出 FOR 循环信息
                let (loop_var, end_val, step_val, loop_line, loop_stmt) = 
                    self.runtime.pop_for_loop(var.clone())?;
                
                // 获取当前循环变量的值
                let current_val = self.variables.get(&loop_var).as_number()?;
                
                // 递增/递减
                let new_val = current_val + step_val;
                
                // 检查是否继续循环
                let should_continue = if step_val > 0.0 {
                    new_val <= end_val
                } else {
                    new_val >= end_val
                };
                
                if should_continue {
                    // 更新循环变量
                    self.variables.set(&loop_var, Value::Number(new_val))?;
                    
                    // 重新压入栈（继续循环）
                    self.runtime.push_for_loop(
                        loop_var.clone(),
                        end_val,
                        step_val,
                        loop_line,
                        loop_stmt,
                    )?;
                    
                    // 跳转回 FOR 语句的下一条语句
                    self.runtime.set_execution_position(loop_line, loop_stmt + 1)?;
                }
                // 否则继续执行下一条语句（循环结束）
                
                Ok(())
            }
            
            Statement::On { expr, targets, is_gosub } => {
                // 计算表达式的值
                let index_val = self.eval_expr(&expr)?;
                let index = index_val.as_number()? as i32;
                
                // 索引从 1 开始
                if index < 1 || index as usize > targets.len() {
                    // 超出范围，继续执行下一条语句
                    return Ok(());
                }
                
                // 获取目标行号
                let target_line = targets[(index - 1) as usize];
                
                if *is_gosub {
                    // ON...GOSUB：保存返回地址并跳转
                    let return_line = self.runtime.get_current_line()
                        .ok_or_else(|| BasicError::SyntaxError("GOSUB without line number".to_string()))?;
                    let return_stmt = self.runtime.get_current_stmt_index();
                    
                    self.runtime.push_gosub(return_line, return_stmt)?;
                    self.runtime.set_execution_position(target_line, 0)?;
                } else {
                    // ON...GOTO：直接跳转
                    self.runtime.set_execution_position(target_line, 0)?;
                }
                
                Ok(())
            }
            
            Statement::Load { filename } => {
                self.execute_load(filename)?;
                Ok(())
            }
            
            Statement::Save { filename } => {
                self.execute_save(filename)?;
                Ok(())
            }
            
            _ => {
                // 其他语句暂未实现
                Err(BasicError::SyntaxError(
                    "Statement not yet implemented".to_string()
                ))
            }
        }
    }
    
    /// 执行 INPUT 语句
    fn execute_input(&mut self, prompt: Option<&str>, variables: &[String]) -> Result<()> {
        // 显示提示符
        if let Some(p) = prompt {
            self.output(p);
            self.output("? ");
        } else {
            self.output("? ");
        }
        
        // 读取输入
        let input_line = if let Some(ref mut callback) = self.input_callback {
            let prompt_text = prompt.unwrap_or("");
            callback(prompt_text).ok_or_else(|| {
                BasicError::SyntaxError("No input provided".to_string())
            })?
        } else {
            // 在实际 REPL 中，这里会从 stdin 读取
            return Err(BasicError::SyntaxError(
                "No input callback set".to_string()
            ));
        };
        
        // 解析输入值（考虑引号内的逗号）
        let values = Self::parse_input_values(&input_line);
        
        if values.len() != variables.len() {
            self.output("?EXTRA IGNORED\n");
        }
        
        // 赋值给变量
        for (i, var_name) in variables.iter().enumerate() {
            if i >= values.len() {
                break;
            }
            
            let input_val = &values[i];
            
            if var_name.ends_with('$') {
                // 字符串变量
                let str_val = if input_val.starts_with('"') && input_val.ends_with('"') {
                    // 去掉引号
                    input_val[1..input_val.len()-1].to_string()
                } else {
                    input_val.clone()
                };
                self.variables.set(var_name, Value::String(str_val))?;
            } else {
                // 数值变量
                match input_val.parse::<f64>() {
                    Ok(num) => {
                        self.variables.set(var_name, Value::Number(num))?;
                    }
                    Err(_) => {
                        self.output("?REDO FROM START\n");
                        return Err(BasicError::TypeMismatch(
                            "Invalid number input".to_string()
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// 解析输入值，处理带引号的字符串
    fn parse_input_values(input: &str) -> Vec<String> {
        let mut values = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        
        for ch in input.chars() {
            match ch {
                '"' => {
                    in_quotes = !in_quotes;
                    current.push(ch);
                }
                ',' if !in_quotes => {
                    values.push(current.trim().to_string());
                    current.clear();
                }
                _ => {
                    current.push(ch);
                }
            }
        }
        
        if !current.is_empty() || input.ends_with(',') {
            values.push(current.trim().to_string());
        }
        
        values
    }
    
    /// 执行 SAVE 命令 - 保存程序到文件
    fn execute_save(&self, filename: &str) -> Result<()> {
        use std::fs::File;
        use std::io::Write;
        
        let program = self.runtime.clone_program();
        if program.is_empty() {
            return Err(BasicError::SyntaxError("No program to save".to_string()));
        }
        
        let mut file = File::create(filename).map_err(|e| {
            BasicError::SyntaxError(format!("Failed to create file: {}", e))
        })?;
        
        for (_, line) in program.iter() {
            let line_text = Self::serialize_program_line(line);
            writeln!(file, "{}", line_text).map_err(|e| {
                BasicError::SyntaxError(format!("Failed to write to file: {}", e))
            })?;
        }
        
        Ok(())
    }
    
    /// 将程序行序列化为文本
    pub fn serialize_program_line(line: &ProgramLine) -> String {
        let mut result = format!("{}", line.line_number);
        
        for (i, stmt) in line.statements.iter().enumerate() {
            if i > 0 {
                result.push_str(":");
            }
            result.push(' ');
            result.push_str(&Self::serialize_statement(stmt));
        }
        
        result
    }
    
    /// 将语句序列化为文本
    pub fn serialize_statement(stmt: &Statement) -> String {
        match stmt {
            Statement::Let { target, value } => {
                format!("{} = {}", Self::serialize_assign_target(target), Self::serialize_expr(value))
            }
            Statement::Print { items } => {
                let mut result = "PRINT ".to_string();
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        result.push_str(&Self::serialize_print_item(item));
                    } else {
                        result.push_str(&Self::serialize_print_item(item));
                    }
                }
                result
            }
            Statement::If { condition, then_part } => {
                format!("IF {} THEN {}", Self::serialize_expr(condition), Self::serialize_then_part(then_part))
            }
            Statement::Goto { line_number } => {
                format!("GOTO {}", Self::serialize_expr(line_number))
            }
            Statement::Gosub { line_number } => {
                format!("GOSUB {}", Self::serialize_expr(line_number))
            }
            Statement::Return => "RETURN".to_string(),
            Statement::For { var, start, end, step } => {
                let mut result = format!("FOR {} = {} TO {}", var, Self::serialize_expr(start), Self::serialize_expr(end));
                if let Some(step_expr) = step {
                    result.push_str(&format!(" STEP {}", Self::serialize_expr(step_expr)));
                }
                result
            }
            Statement::Next { var } => {
                if let Some(v) = var {
                    format!("NEXT {}", v)
                } else {
                    "NEXT".to_string()
                }
            }
            Statement::On { expr, targets, is_gosub } => {
                let keyword = if *is_gosub { "GOSUB" } else { "GOTO" };
                let target_str = targets.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("ON {} {} {}", Self::serialize_expr(expr), keyword, target_str)
            }
            Statement::Input { prompt, variables } => {
                let mut result = "INPUT ".to_string();
                if let Some(p) = prompt {
                    result.push_str(&format!("\"{}\" ", p));
                }
                let var_str = variables.iter()
                    .map(|v| Self::serialize_assign_target(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                result.push_str(&var_str);
                result
            }
            Statement::Dim { arrays } => {
                let arr_str = arrays.iter()
                    .map(|arr| {
                        let dims = arr.dimensions.iter()
                            .map(|d| Self::serialize_expr(d))
                            .collect::<Vec<_>>()
                            .join(",");
                        format!("{}({})", arr.name, dims)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("DIM {}", arr_str)
            }
            Statement::Data { values } => {
                let val_str = values.iter()
                    .map(|v| match v {
                        crate::ast::DataValue::Number(n) => n.to_string(),
                        crate::ast::DataValue::String(s) => format!("\"{}\"", s),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("DATA {}", val_str)
            }
            Statement::Read { variables } => {
                let var_str = variables.iter()
                    .map(|v| Self::serialize_assign_target(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("READ {}", var_str)
            }
            Statement::Restore { line_number } => {
                if let Some(ln) = line_number {
                    format!("RESTORE {}", ln)
                } else {
                    "RESTORE".to_string()
                }
            }
            Statement::Rem => "REM".to_string(),
            Statement::End => "END".to_string(),
            Statement::Stop => "STOP".to_string(),
            Statement::New => "NEW".to_string(),
            Statement::Clear => "CLEAR".to_string(),
            _ => "REM UNSUPPORTED STATEMENT".to_string(),
        }
    }
    
    /// 将表达式序列化为文本
    pub fn serialize_expr(expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => n.to_string(),
            Expr::String(s) => format!("\"{}\"", s),
            Expr::Variable(name) => name.clone(),
            Expr::ArrayAccess { name, indices } => {
                let idx_str = indices.iter()
                    .map(|i| Self::serialize_expr(i))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, idx_str)
            }
            Expr::FunctionCall { name, args } => {
                let arg_str = args.iter()
                    .map(|a| Self::serialize_expr(a))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, arg_str)
            }
            Expr::BinaryOp { left, op, right } => {
                let op_str = match op {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Subtract => "-",
                    BinaryOperator::Multiply => "*",
                    BinaryOperator::Divide => "/",
                    BinaryOperator::Power => "^",
                    BinaryOperator::Equal => "=",
                    BinaryOperator::NotEqual => "<>",
                    BinaryOperator::Less => "<",
                    BinaryOperator::Greater => ">",
                    BinaryOperator::LessEqual => "<=",
                    BinaryOperator::GreaterEqual => ">=",
                    BinaryOperator::And => " AND ",
                    BinaryOperator::Or => " OR ",
                };
                format!("({} {} {})", Self::serialize_expr(left), op_str, Self::serialize_expr(right))
            }
            Expr::UnaryOp { op, operand } => {
                let op_str = match op {
                    UnaryOperator::Minus => "-",
                    UnaryOperator::Not => "NOT ",
                };
                format!("{}{}", op_str, Self::serialize_expr(operand))
            }
        }
    }
    
    /// 将赋值目标序列化为文本
    pub fn serialize_assign_target(target: &AssignTarget) -> String {
        match target {
            AssignTarget::Variable(name) => name.clone(),
            AssignTarget::ArrayElement { name, indices } => {
                let idx_str = indices.iter()
                    .map(|i| Self::serialize_expr(i))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, idx_str)
            }
        }
    }
    
    /// 将THEN部分序列化为文本
    pub fn serialize_then_part(then_part: &ThenPart) -> String {
        match then_part {
            ThenPart::LineNumber(ln) => ln.to_string(),
            ThenPart::Statement(stmt) => Self::serialize_statement(stmt),
            ThenPart::Statements(stmts) => {
                stmts.iter()
                    .map(|s| Self::serialize_statement(s))
                    .collect::<Vec<_>>()
                    .join(":")
            }
        }
    }
    
    /// 将PRINT项序列化为文本
    pub fn serialize_print_item(item: &PrintItem) -> String {
        match item {
            PrintItem::Expr(expr) => Self::serialize_expr(expr),
            PrintItem::Tab(expr) => format!("TAB({})", Self::serialize_expr(expr)),
            PrintItem::Spc(expr) => format!("SPC({})", Self::serialize_expr(expr)),
            PrintItem::Comma => ",".to_string(),
            PrintItem::Semicolon => ";".to_string(),
        }
    }
    
    /// 执行 LOAD 命令 - 从文件加载程序
    fn execute_load(&mut self, filename: &str) -> Result<()> {
        use std::fs;
        use crate::tokenizer::Tokenizer;
        use crate::parser::Parser;
        
        // 读取文件内容
        let content = fs::read_to_string(filename).map_err(|e| {
            BasicError::SyntaxError(format!("Failed to read file: {}", e))
        })?;
        
        // 清空当前程序
        self.runtime.clear_program();
        self.variables.clear();
        
        // 逐行解析并添加到程序
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            // 使用tokenizer和parser解析每一行
            let mut tokenizer = Tokenizer::new(line);
            let tokens = tokenizer.tokenize_line()?;
            
            let mut parser = Parser::new(tokens);
            if let Some(program_line) = parser.parse_line()? {
                if program_line.line_number > 0 {
                    self.runtime.add_line(program_line);
                }
            }
        }
        
        Ok(())
    }
    
    /// 执行 PRINT 语句
    fn execute_print(&mut self, items: &[PrintItem]) -> Result<()> {
        if items.is_empty() {
            // 空 PRINT，仅输出换行
            self.output_newline();
            return Ok(());
        }
        
        for item in items.iter() {
            match item {
                PrintItem::Expr(expr) => {
                    let val = self.eval_expr(expr)?;
                    self.print_value(&val)?;
                }
                PrintItem::Tab(expr) => {
                    let target_col = self.eval_expr(expr)?.as_number()? as usize;
                    if target_col > self.print_column {
                        let spaces = target_col - self.print_column;
                        self.output(&" ".repeat(spaces));
                    } else if target_col < self.print_column {
                        // TAB 到更小的列，换行后跳转
                        self.output_newline();
                        self.output(&" ".repeat(target_col));
                    }
                }
                PrintItem::Spc(expr) => {
                    let spaces = self.eval_expr(expr)?.as_number()? as usize;
                    self.output(&" ".repeat(spaces));
                }
                PrintItem::Comma => {
                    // 逗号：对齐到下一个 14 列边界
                    let next_col = ((self.print_column / 14) + 1) * 14;
                    let spaces_needed = next_col - self.print_column;
                    self.output(&" ".repeat(spaces_needed));
                }
                PrintItem::Semicolon => {
                    // 分号：不添加空格（紧密连接）
                }
            }
        }
        
        // 检查最后一项是否是分隔符
        if let Some(last) = items.last() {
            if !matches!(last, PrintItem::Comma | PrintItem::Semicolon) {
                // 如果最后不是分隔符，输出换行
                self.output_newline();
            }
        } else {
            self.output_newline();
        }
        
        Ok(())
    }
    
    /// 打印值（根据 BASIC 格式）
    fn print_value(&mut self, val: &Value) -> Result<()> {
        match val {
            Value::Number(n) => {
                // BASIC 数值格式：正数前后各有空格，负数前有空格
                let formatted = if *n >= 0.0 {
                    format!(" {} ", n)
                } else {
                    format!(" {}", n)
                };
                self.output(&formatted);
            }
            Value::String(s) => {
                // 普通字符串，直接输出
                self.output(s);
            }
        }
        Ok(())
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Requirement: 算术运算符 - 加法
    #[test]
    fn test_addition() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Add,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    // Requirement: 算术运算符 - 减法
    #[test]
    fn test_subtraction() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(10.0),
            BinaryOperator::Subtract,
            Expr::Number(7.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    // Requirement: 算术运算符 - 乘法
    #[test]
    fn test_multiplication() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(4.0),
            BinaryOperator::Multiply,
            Expr::Number(5.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(20.0));
    }

    // Requirement: 算术运算符 - 除法
    #[test]
    fn test_division() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(15.0),
            BinaryOperator::Divide,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    // Requirement: 算术运算符 - 浮点除法
    #[test]
    fn test_float_division() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(10.0),
            BinaryOperator::Divide,
            Expr::Number(4.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(2.5));
    }

    // Requirement: 算术运算符 - 除以零
    #[test]
    fn test_division_by_zero() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Divide,
            Expr::Number(0.0)
        );
        let result = exec.eval_expr(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BasicError::DivisionByZero));
    }

    // Requirement: 算术运算符 - 乘方
    #[test]
    fn test_power() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(2.0),
            BinaryOperator::Power,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    // Requirement: 一元运算符 - 一元负号
    #[test]
    fn test_unary_minus() {
        let mut exec = Executor::new();
        let expr = Expr::unary(UnaryOperator::Minus, Expr::Number(5.0));
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-5.0));
    }

    // Requirement: 关系运算符 - 等于
    #[test]
    fn test_equal() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Equal,
            Expr::Number(5.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-1.0)); // BASIC true = -1
    }

    // Requirement: 关系运算符 - 不等于
    #[test]
    fn test_not_equal() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::NotEqual,
            Expr::Number(4.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-1.0));
    }

    // Requirement: 字符串运算符 - 字符串连接
    #[test]
    fn test_string_concatenation() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::String("HELLO".to_string()),
            BinaryOperator::Add,
            Expr::String(" WORLD".to_string())
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::String("HELLO WORLD".to_string()));
    }

    // Test: 变量读取
    #[test]
    fn test_variable_read() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(42.0)).unwrap();
        
        let expr = Expr::Variable("A".to_string());
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    // Test: LET 语句执行
    #[test]
    fn test_let_statement() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Let {
            target: AssignTarget::Variable("X".to_string()),
            value: Expr::Number(100.0),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.variables.get("X"), Value::Number(100.0));
    }

    // Test: DIM 语句执行
    #[test]
    fn test_dim_statement() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Dim {
            arrays: vec![
                ArrayDim {
                    name: "A".to_string(),
                    dimensions: vec![Expr::Number(10.0)],
                }
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert!(exec.variables.has_array("A"));
    }

    // Test: 数学函数
    #[test]
    fn test_math_functions() {
        let mut exec = Executor::new();
        
        // ABS
        let expr = Expr::FunctionCall {
            name: "ABS".to_string(),
            args: vec![Expr::Number(-42.0)],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(42.0));
        
        // INT
        let expr = Expr::FunctionCall {
            name: "INT".to_string(),
            args: vec![Expr::Number(3.7)],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    // Test: 字符串函数
    #[test]
    fn test_string_functions() {
        let mut exec = Executor::new();
        
        // LEN
        let expr = Expr::FunctionCall {
            name: "LEN".to_string(),
            args: vec![Expr::String("HELLO".to_string())],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));
        
        // LEFT$
        let expr = Expr::FunctionCall {
            name: "LEFT$".to_string(),
            args: vec![
                Expr::String("HELLO".to_string()),
                Expr::Number(3.0),
            ],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::String("HEL".to_string()));
    }

    // Test: 复杂表达式
    #[test]
    fn test_complex_expression() {
        let mut exec = Executor::new();
        
        // 2 + 3 * 4 = 14
        let expr = Expr::binary(
            Expr::Number(2.0),
            BinaryOperator::Add,
            Expr::binary(
                Expr::Number(3.0),
                BinaryOperator::Multiply,
                Expr::Number(4.0)
            )
        );
        
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(14.0));
    }

    // Requirement: PRINT 语句 - 打印数值
    #[test]
    fn test_print_number() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(42.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 42 \n");
    }

    // Requirement: PRINT 语句 - 打印字符串
    #[test]
    fn test_print_string() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("HELLO".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "HELLO\n");
    }

    // Requirement: PRINT 语句 - 打印变量
    #[test]
    fn test_print_variable() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(10.0)).unwrap();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Variable("A".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 10 \n");
    }

    // Requirement: PRINT 语句 - 分号分隔（紧密连接）
    #[test]
    fn test_print_semicolon() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(1.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::Number(2.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::Number(3.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 1  2  3 \n");
    }

    // Requirement: PRINT 语句 - 行尾分号（不换行）
    #[test]
    fn test_print_no_newline() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(42.0)),
                PrintItem::Semicolon,
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 42 ");
    }

    // Requirement: PRINT 语句 - 空 PRINT
    #[test]
    fn test_print_empty() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "\n");
    }

    // Requirement: PRINT 语句 - 逗号分隔（列对齐）
    #[test]
    fn test_print_comma_alignment() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(1.0)),
                PrintItem::Comma,
                PrintItem::Expr(Expr::Number(2.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // 第一个数 " 1 " 占 3 列，逗号应该对齐到第 14 列
        assert!(output.starts_with(" 1 "));
        assert!(output.contains(" 2 "));
    }

    // Requirement: GOTO 语句
    #[test]
    fn test_goto_statement() {
        let mut exec = Executor::new();
        
        // 添加程序行
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("A".to_string()),
                value: Expr::Number(1.0),
            }]
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("B".to_string()),
                value: Expr::Number(99.0),
            }]
        });
        
        let stmt = Statement::Goto {
            line_number: Expr::Number(100.0),
        };
        exec.execute_statement(&stmt).unwrap();
        
        // 验证跳转成功（下一行应该是 100）
        assert_eq!(exec.runtime().get_current_line(), Some(100));
    }

    // Requirement: IF...THEN 语句 - 条件为真
    #[test]
    fn test_if_then_true() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(15.0)).unwrap();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行来设置初始状态
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::LineNumber(100)),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.runtime().get_current_line(), Some(100));
    }

    // Requirement: IF...THEN 语句 - 条件为假
    #[test]
    fn test_if_then_false() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(5.0)).unwrap();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![],
        });
        
        let current_line = exec.runtime().get_current_line();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::LineNumber(100)),
        };
        
        exec.execute_statement(&stmt).unwrap();
        // 条件为假，不应该跳转
        assert_eq!(exec.runtime().get_current_line(), current_line);
    }

    // Requirement: IF...THEN 语句 - THEN 后跟语句
    #[test]
    fn test_if_then_statement() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(15.0)).unwrap();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::Statement(
                Statement::Print {
                    items: vec![
                        PrintItem::Expr(Expr::String("TRUE".to_string())),
                    ],
                }
            )),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "TRUE\n");
    }

    // Test: TAB 函数
    #[test]
    fn test_tab_function() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("A".to_string())),
                PrintItem::Semicolon,
                PrintItem::Tab(Expr::Number(10.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::String("B".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // A 在列 0，TAB(10) 跳到列 10，然后是 B
        assert!(output.starts_with("A"));
        assert!(output.contains("B"));
    }

    // Test: SPC 函数
    #[test]
    fn test_spc_function() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("A".to_string())),
                PrintItem::Semicolon,
                PrintItem::Spc(Expr::Number(5.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::String("B".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // A + 5个空格 + B
        assert_eq!(output, "A     B\n");
    }

    // Requirement: GOSUB 和 RETURN 语句 - 子程序调用
    #[test]
    fn test_gosub_statement() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 500,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        let stmt = Statement::Gosub {
            line_number: Expr::Number(500.0),
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证跳转到子程序
        assert_eq!(exec.runtime().get_current_line(), Some(500));
        // 验证调用栈深度
        assert_eq!(exec.runtime().stack_depth(), 1);
    }

    // Requirement: GOSUB 和 RETURN 语句 - 子程序返回
    #[test]
    fn test_return_statement() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 500,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行并设置调用栈
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        exec.runtime_mut().push_gosub(20, 0).unwrap();
        exec.runtime_mut().set_execution_position(500, 0).unwrap();
        
        let stmt = Statement::Return;
        exec.execute_statement(&stmt).unwrap();
        
        // 验证返回到调用点
        assert_eq!(exec.runtime().get_current_line(), Some(20));
        // 验证调用栈已弹出
        assert_eq!(exec.runtime().stack_depth(), 0);
    }

    // Requirement: GOSUB 和 RETURN 语句 - 嵌套子程序
    #[test]
    fn test_nested_gosub() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 200,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // 第一次 GOSUB
        exec.execute_statement(&Statement::Gosub {
            line_number: Expr::Number(100.0),
        }).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 1);
        
        // 第二次 GOSUB（嵌套）
        exec.execute_statement(&Statement::Gosub {
            line_number: Expr::Number(200.0),
        }).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 2);
        assert_eq!(exec.runtime().get_current_line(), Some(200));
        
        // 第一次 RETURN
        exec.execute_statement(&Statement::Return).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 1);
        assert_eq!(exec.runtime().get_current_line(), Some(100));
        
        // 第二次 RETURN
        exec.execute_statement(&Statement::Return).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 0);
        assert_eq!(exec.runtime().get_current_line(), Some(10));
    }

    // Requirement: INPUT 语句 - 基本输入
    #[test]
    fn test_input_basic() {
        let mut exec = Executor::new();
        
        // 设置输入回调
        exec.set_input_callback(|_| Some("42".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证输出提示符
        assert!(exec.get_output().contains("? "));
        
        // 验证变量赋值
        assert_eq!(exec.variables.get("A"), Value::Number(42.0));
    }

    // Requirement: INPUT 语句 - 带提示符的输入
    #[test]
    fn test_input_with_prompt() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("100".to_string()));
        
        let stmt = Statement::Input {
            prompt: Some("ENTER VALUE".to_string()),
            variables: vec![AssignTarget::Variable("X".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证提示符
        assert!(exec.get_output().contains("ENTER VALUE? "));
        assert_eq!(exec.variables.get("X"), Value::Number(100.0));
    }

    // Requirement: INPUT 语句 - 输入多个变量
    #[test]
    fn test_input_multiple_variables() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("10, 20, 30".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string()),
                AssignTarget::Variable("C".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(10.0));
        assert_eq!(exec.variables.get("B"), Value::Number(20.0));
        assert_eq!(exec.variables.get("C"), Value::Number(30.0));
    }

    // Requirement: INPUT 语句 - 字符串输入
    #[test]
    fn test_input_string() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("HELLO".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A$".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A$"), Value::String("HELLO".to_string()));
    }

    // Requirement: INPUT 语句 - 字符串带引号
    #[test]
    fn test_input_string_with_quotes() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("\"HELLO, WORLD\"".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A$".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A$"), Value::String("HELLO, WORLD".to_string()));
    }

    // Requirement: DATA/READ 机制 - DATA 存储和 READ 读取
    #[test]
    fn test_data_read() {
        let mut exec = Executor::new();
        
        // 添加 DATA 值
        exec.add_data_value(DataValue::Number(1.0));
        exec.add_data_value(DataValue::Number(2.0));
        exec.add_data_value(DataValue::Number(3.0));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string()),
                AssignTarget::Variable("C".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(1.0));
        assert_eq!(exec.variables.get("B"), Value::Number(2.0));
        assert_eq!(exec.variables.get("C"), Value::Number(3.0));
    }

    // Requirement: DATA/READ 机制 - 混合数据类型
    #[test]
    fn test_data_read_mixed_types() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(42.0));
        exec.add_data_value(DataValue::String("HELLO".to_string()));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B$".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(42.0));
        assert_eq!(exec.variables.get("B$"), Value::String("HELLO".to_string()));
    }

    // Requirement: DATA/READ 机制 - OUT OF DATA 错误
    #[test]
    fn test_out_of_data_error() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(1.0));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string())
            ],
        };
        
        let result = exec.execute_statement(&stmt);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BasicError::OutOfData));
    }

    // Requirement: RESTORE 数据指针 - RESTORE 重置到开头
    #[test]
    fn test_restore() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(1.0));
        exec.add_data_value(DataValue::Number(2.0));
        
        // 第一次 READ
        exec.execute_statement(&Statement::Read {
            variables: vec![AssignTarget::Variable("A".to_string())],
        }).unwrap();
        assert_eq!(exec.variables.get("A"), Value::Number(1.0));
        
        // RESTORE
        exec.execute_statement(&Statement::Restore {
            line_number: None,
        }).unwrap();
        
        // 第二次 READ（应该重新从头开始）
        exec.execute_statement(&Statement::Read {
            variables: vec![AssignTarget::Variable("B".to_string())],
        }).unwrap();
        assert_eq!(exec.variables.get("B"), Value::Number(1.0));
    }

    // Requirement: FOR...NEXT 循环 - 正向循环
    #[test]
    fn test_for_next_basic() {
        let mut exec = Executor::new();
        
        // 添加测试程序：FOR I=1 TO 3: PRINT I: NEXT I
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(1.0),
                    end: Expr::Number(3.0),
                    step: None,
                },
            ],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Next { var: Some("I".to_string()) }],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // 第一次循环：I=1
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(1.0),
            end: Expr::Number(3.0),
            step: None,
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
        
        // NEXT：I=2
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=3
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
        
        // NEXT：循环结束 (I递增到4但不再循环)
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        // 循环已结束，变量值应该为循环后的值 4
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
    }

    // Requirement: FOR...NEXT 循环 - 步长为 2
    #[test]
    fn test_for_next_step() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(0.0),
                    end: Expr::Number(4.0),
                    step: Some(Expr::Number(2.0)),
                },
            ],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // FOR I=0 TO 4 STEP 2
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(0.0),
            end: Expr::Number(4.0),
            step: Some(Expr::Number(2.0)),
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(0.0));
        
        // NEXT：I=2
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=4
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(4.0));
        
        // NEXT：循环结束
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(4.0));
    }

    // Requirement: FOR...NEXT 循环 - 负步长
    #[test]
    fn test_for_next_negative_step() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(3.0),
                    end: Expr::Number(1.0),
                    step: Some(Expr::Number(-1.0)),
                },
            ],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // FOR I=3 TO 1 STEP -1
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(3.0),
            end: Expr::Number(1.0),
            step: Some(Expr::Number(-1.0)),
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
        
        // NEXT：I=2
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=1
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
        
        // NEXT：循环结束
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
    }

    // Requirement: ON...GOTO - 基于表达式的跳转
    #[test]
    fn test_on_goto() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 200,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // ON 2 GOTO 100,200,300
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(2.0),
            targets: vec![100, 200, 300],
            is_gosub: false,
        }).unwrap();
        
        // 应该跳转到 200
        assert_eq!(exec.runtime().get_current_line(), Some(200));
    }

    // Requirement: ON...GOSUB - 基于表达式的子程序调用
    #[test]
    fn test_on_gosub() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // ON 1 GOSUB 100,200
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(1.0),
            targets: vec![100, 200],
            is_gosub: true,
        }).unwrap();
        
        // 应该跳转到 100
        assert_eq!(exec.runtime().get_current_line(), Some(100));
        // 栈深度应该为 1
        assert_eq!(exec.runtime().stack_depth(), 1);
    }

    // Requirement: ON...GOTO - 值超出范围
    #[test]
    fn test_on_goto_out_of_range() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        let current_line = exec.runtime().get_current_line();
        
        // ON 5 GOTO 100,200  (5 超出范围)
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(5.0),
            targets: vec![100, 200],
            is_gosub: false,
        }).unwrap();
        
        // 应该继续在当前行
        assert_eq!(exec.runtime().get_current_line(), current_line);
    }
    
    #[test]
    fn test_save_and_load() {
        use std::fs;
        
        let mut exec = Executor::new();
        
        // 添加一些程序行
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Print {
                items: vec![PrintItem::Expr(Expr::String("HELLO".to_string()))],
            }],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("A".to_string()),
                value: Expr::Number(42.0),
            }],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 30,
            statements: vec![Statement::End],
        });
        
        // 保存程序到文件
        let filename = "test_program.bas";
        exec.execute_statement(&Statement::Save {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证文件存在
        assert!(fs::metadata(filename).is_ok());
        
        // 清空程序
        exec.runtime_mut().clear_program();
        assert_eq!(exec.runtime().line_count(), 0);
        
        // 加载程序
        exec.execute_statement(&Statement::Load {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证程序已加载
        assert_eq!(exec.runtime().line_count(), 3);
        assert!(exec.runtime().get_line(10).is_some());
        assert!(exec.runtime().get_line(20).is_some());
        assert!(exec.runtime().get_line(30).is_some());
        
        // 清理测试文件
        fs::remove_file(filename).ok();
    }
    
    #[test]
    fn test_save_empty_program() {
        let mut exec = Executor::new();
        
        // 尝试保存空程序应该失败
        let result = exec.execute_statement(&Statement::Save {
            filename: "empty.bas".to_string(),
        });
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_load_nonexistent_file() {
        let mut exec = Executor::new();
        
        // 尝试加载不存在的文件应该失败
        let result = exec.execute_statement(&Statement::Load {
            filename: "nonexistent.bas".to_string(),
        });
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_save_complex_program() {
        use std::fs;
        
        let mut exec = Executor::new();
        
        // 创建一个更复杂的程序
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(1.0),
                    end: Expr::Number(10.0),
                    step: Some(Expr::Number(1.0)),
                },
            ],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![
                Statement::Print {
                    items: vec![PrintItem::Expr(Expr::Variable("I".to_string()))],
                },
            ],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 30,
            statements: vec![Statement::Next { var: Some("I".to_string()) }],
        });
        
        // 保存并重新加载
        let filename = "test_complex.bas";
        exec.execute_statement(&Statement::Save {
            filename: filename.to_string(),
        }).unwrap();
        
        exec.runtime_mut().clear_program();
        
        exec.execute_statement(&Statement::Load {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证程序结构
        assert_eq!(exec.runtime().line_count(), 3);
        
        // 清理
        fs::remove_file(filename).ok();
    }
}


```

现在让我修改 `main.rs` 中的 `list_program` 函数，使用完整的序列化函数：

```rust
use basic_m6502::{
    BasicError, Executor, Parser, Result, Statement, Tokenizer,
};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() -> Result<()> {
    println!("Microsoft BASIC 6502 Interpreter (Rust Edition)");
    println!("Ready.");
    println!();

    // 创建执行器
    let mut executor = Executor::new();

    // 创建 rustyline 编辑器（带历史记录）
    let mut rl = DefaultEditor::new().map_err(|e| {
        BasicError::SyntaxError(format!("Failed to initialize editor: {}", e))
    })?;

    // 加载历史记录（如果存在）
    let history_file = ".basic_history";
    let _ = rl.load_history(history_file);

    // REPL 主循环
    loop {
        // 读取一行
        let readline = rl.readline("");
        
        match readline {
            Ok(line) => {
                // 添加到历史记录
                rl.add_history_entry(line.as_str()).ok();
                
                // 处理输入行
                if let Err(e) = process_line(&mut executor, &line) {
                    // 输出错误
                    match e {
                        BasicError::CantContinue => {
                            // 正常退出信号，不输出错误
                        }
                        _ => {
                            eprintln!("?{}", format_error(&e));
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C 中断
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("\n?BREAK IN {}", line);
                } else {
                    println!("\n^C");
                }
                executor.runtime_mut().interrupt();
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    // 保存历史记录
    rl.save_history(history_file).ok();
    
    Ok(())
}

/// 处理一行输入
fn process_line(executor: &mut Executor, line: &str) -> Result<()> {
    let line = line.trim();
    
    // 空行
    if line.is_empty() {
        return Ok(());
    }
    
    // 词法分析
    let mut tokenizer = Tokenizer::new(line);
    let tokens = tokenizer.tokenize_line()?;
    
    // 语法分析
    let mut parser = Parser::new(tokens);
    
    // 解析程序行
    if let Some(program_line) = parser.parse_line()? {
        // 检查是否是带行号的程序行
        if program_line.line_number > 0 {
            // 添加或删除程序行
            if program_line.statements.is_empty() {
                // 空行：删除该行
                executor.runtime_mut().delete_line(program_line.line_number);
            } else {
                // 非空行：添加到程序
                executor.runtime_mut().add_line(program_line);
            }
            return Ok(());
        }
        
        // 直接模式：执行语句
        if program_line.statements.is_empty() {
            return Ok(());
        }
        let statement = program_line.statements.into_iter().next().unwrap();
        
        // 特殊命令处理
        match &statement {
            Statement::List { start, end } => {
                list_program(executor, *start, *end);
                Ok(())
            }
            Statement::Run { line_number } => {
                run_program(executor, *line_number)
            }
            Statement::New => {
                executor.execute_statement(&statement)?;
                println!("New program started.");
                Ok(())
            }
            Statement::End => {
                executor.execute_statement(&statement)?;
                Ok(())
            }
            Statement::Stop => {
                executor.execute_statement(&statement)?;
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("?BREAK IN {}", line);
                }
                Ok(())
            }
            Statement::Cont => {
                continue_program(executor)
            }
            _ => {
                // 其他语句：直接执行
                executor.execute_statement(&statement)
            }
        }
    } else {
        Ok(())
    }
}

/// 列出程序
fn list_program(executor: &Executor, start: Option<u16>, end: Option<u16>) {
    let lines = executor.runtime().get_program_lines(start, end);
    
    if lines.is_empty() {
        if start.is_some() || end.is_some() {
            // 有范围但没有内容，不输出
            return;
        }
        println!("No program loaded.");
        return;
    }
    
    for line in lines {
        // 使用完整的序列化函数
        println!("{}", Executor::serialize_program_line(line));
    }
}

/// 运行程序
fn run_program(executor: &mut Executor, line_number: Option<u16>) -> Result<()> {
    // 启动执行
    executor.runtime_mut().start_execution(line_number)?;
    
    // 执行循环
    loop {
        let stmt = match executor.runtime_mut().get_next_statement() {
            Some(s) => s,
            None => break, // 程序结束
        };
        
        // 执行语句
        if let Err(e) = executor.execute_statement(&stmt) {
            // 输出错误和行号
            if let Some(line) = executor.runtime().get_current_line() {
                eprintln!("?{} IN {}", format_error(&e), line);
            } else {
                eprintln!("?{}", format_error(&e));
            }
            return Err(e);
        }
        
        // 检查是否应该停止
        if executor.runtime().is_stopped() {
            break;
        }
    }
    
    Ok(())
}

/// 继续执行程序
fn continue_program(executor: &mut Executor) -> Result<()> {
    if !executor.runtime().can_continue() {
        println!("?CAN'T CONTINUE");
        return Err(BasicError::CantContinue);
    }
    
    // 从暂停点恢复执行
    executor.runtime_mut().continue_execution()?;
    run_program(executor, None)
}

/// 格式化错误信息
fn format_error(error: &BasicError) -> String {
    match error {
        BasicError::SyntaxError(_) => "SYNTAX ERROR".to_string(),
        BasicError::DivisionByZero => "DIVISION BY ZERO".to_string(),
        BasicError::TypeMismatch(_) => "TYPE MISMATCH".to_string(),
        BasicError::UndefinedLine(n) => format!("UNDEFINED LINE {}", n),
        BasicError::UndefinedVariable(v) => format!("UNDEFINED VARIABLE {}", v),
        BasicError::SubscriptOutOfRange(_) => "SUBSCRIPT OUT OF RANGE".to_string(),
        BasicError::OutOfData => "OUT OF DATA".to_string(),
        BasicError::ReturnWithoutGosub => "RETURN WITHOUT GOSUB".to_string(),
        BasicError::NextWithoutFor(_) => "NEXT WITHOUT FOR".to_string(),
        BasicError::CantContinue => "CAN'T CONTINUE".to_string(),
        _ => format!("{:?}", error),
    }
}

/// 格式化语句（简化版本）
fn format_statement(stmt: &Statement) -> String {
    match stmt {
        Statement::Print { items } => {
            format!("PRINT {}", items.len())
        }
        Statement::Let { target, value } => {
            format!("LET {:?} = {:?}", target, value)
        }
        Statement::Goto { line_number } => {
            format!("GOTO {:?}", line_number)
        }
        Statement::If { .. } => "IF...THEN".to_string(),
        Statement::For { .. } => "FOR".to_string(),
        Statement::Next { .. } => "NEXT".to_string(),
        Statement::Rem => "REM".to_string(),
        Statement::End => "END".to_string(),
        Statement::Stop => "STOP".to_string(),
        _ => format!("{:?}", stmt),
    }
}


```

现在删除不再需要的 `format_statement` 函数：

```rust
use basic_m6502::{
    BasicError, Executor, Parser, Result, Statement, Tokenizer,
};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() -> Result<()> {
    println!("Microsoft BASIC 6502 Interpreter (Rust Edition)");
    println!("Ready.");
    println!();

    // 创建执行器
    let mut executor = Executor::new();

    // 创建 rustyline 编辑器（带历史记录）
    let mut rl = DefaultEditor::new().map_err(|e| {
        BasicError::SyntaxError(format!("Failed to initialize editor: {}", e))
    })?;

    // 加载历史记录（如果存在）
    let history_file = ".basic_history";
    let _ = rl.load_history(history_file);

    // REPL 主循环
    loop {
        // 读取一行
        let readline = rl.readline("");
        
        match readline {
            Ok(line) => {
                // 添加到历史记录
                rl.add_history_entry(line.as_str()).ok();
                
                // 处理输入行
                if let Err(e) = process_line(&mut executor, &line) {
                    // 输出错误
                    match e {
                        BasicError::CantContinue => {
                            // 正常退出信号，不输出错误
                        }
                        _ => {
                            eprintln!("?{}", format_error(&e));
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C 中断
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("\n?BREAK IN {}", line);
                } else {
                    println!("\n^C");
                }
                executor.runtime_mut().interrupt();
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    // 保存历史记录
    rl.save_history(history_file).ok();
    
    Ok(())
}

/// 处理一行输入
fn process_line(executor: &mut Executor, line: &str) -> Result<()> {
    let line = line.trim();
    
    // 空行
    if line.is_empty() {
        return Ok(());
    }
    
    // 词法分析
    let mut tokenizer = Tokenizer::new(line);
    let tokens = tokenizer.tokenize_line()?;
    
    // 语法分析
    let mut parser = Parser::new(tokens);
    
    // 解析程序行
    if let Some(program_line) = parser.parse_line()? {
        // 检查是否是带行号的程序行
        if program_line.line_number > 0 {
            // 添加或删除程序行
            if program_line.statements.is_empty() {
                // 空行：删除该行
                executor.runtime_mut().delete_line(program_line.line_number);
            } else {
                // 非空行：添加到程序
                executor.runtime_mut().add_line(program_line);
            }
            return Ok(());
        }
        
        // 直接模式：执行语句
        if program_line.statements.is_empty() {
            return Ok(());
        }
        let statement = program_line.statements.into_iter().next().unwrap();
        
        // 特殊命令处理
        match &statement {
            Statement::List { start, end } => {
                list_program(executor, *start, *end);
                Ok(())
            }
            Statement::Run { line_number } => {
                run_program(executor, *line_number)
            }
            Statement::New => {
                executor.execute_statement(&statement)?;
                println!("New program started.");
                Ok(())
            }
            Statement::End => {
                executor.execute_statement(&statement)?;
                Ok(())
            }
            Statement::Stop => {
                executor.execute_statement(&statement)?;
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("?BREAK IN {}", line);
                }
                Ok(())
            }
            Statement::Cont => {
                continue_program(executor)
            }
            _ => {
                // 其他语句：直接执行
                executor.execute_statement(&statement)
            }
        }
    } else {
        Ok(())
    }
}

/// 列出程序
fn list_program(executor: &Executor, start: Option<u16>, end: Option<u16>) {
    let lines = executor.runtime().get_program_lines(start, end);
    
    if lines.is_empty() {
        if start.is_some() || end.is_some() {
            // 有范围但没有内容，不输出
            return;
        }
        println!("No program loaded.");
        return;
    }
    
    for line in lines {
        // 使用完整的序列化函数
        println!("{}", Executor::serialize_program_line(line));
    }
}

/// 运行程序
fn run_program(executor: &mut Executor, line_number: Option<u16>) -> Result<()> {
    // 启动执行
    executor.runtime_mut().start_execution(line_number)?;
    
    // 执行循环
    loop {
        let stmt = match executor.runtime_mut().get_next_statement() {
            Some(s) => s,
            None => break, // 程序结束
        };
        
        // 执行语句
        if let Err(e) = executor.execute_statement(&stmt) {
            // 输出错误和行号
            if let Some(line) = executor.runtime().get_current_line() {
                eprintln!("?{} IN {}", format_error(&e), line);
            } else {
                eprintln!("?{}", format_error(&e));
            }
            return Err(e);
        }
        
        // 检查是否应该停止
        if executor.runtime().is_stopped() {
            break;
        }
    }
    
    Ok(())
}

/// 继续执行程序
fn continue_program(executor: &mut Executor) -> Result<()> {
    if !executor.runtime().can_continue() {
        println!("?CAN'T CONTINUE");
        return Err(BasicError::CantContinue);
    }
    
    // 从暂停点恢复执行
    executor.runtime_mut().continue_execution()?;
    run_program(executor, None)
}

/// 格式化错误信息
fn format_error(error: &BasicError) -> String {
    match error {
        BasicError::SyntaxError(_) => "SYNTAX ERROR".to_string(),
        BasicError::DivisionByZero => "DIVISION BY ZERO".to_string(),
        BasicError::TypeMismatch(_) => "TYPE MISMATCH".to_string(),
        BasicError::UndefinedLine(n) => format!("UNDEFINED LINE {}", n),
        BasicError::UndefinedVariable(v) => format!("UNDEFINED VARIABLE {}", v),
        BasicError::SubscriptOutOfRange(_) => "SUBSCRIPT OUT OF RANGE".to_string(),
        BasicError::OutOfData => "OUT OF DATA".to_string(),
        BasicError::ReturnWithoutGosub => "RETURN WITHOUT GOSUB".to_string(),
        BasicError::NextWithoutFor(_) => "NEXT WITHOUT FOR".to_string(),
        BasicError::CantContinue => "CAN'T CONTINUE".to_string(),
        _ => format!("{:?}", error),
    }
}


```

让我检查一下 PRINT 语句的序列化，看看是否需要修复分隔符的输出：

让我编译并测试这些修复：

现在让我测试用户的场景：

完美！现在 `LIST` 命令正常工作了。让我再测试几个更复杂的场景来确保完整性：

非常好！让我测试一下更多复杂的语句类型：

太好了！所有语句都能正确显示。让我再测试一下 SAVE/LOAD 功能，确保序列化也适用于文件保存：

看起来文件名中的下划线有问题。让我用一个简单的文件名测试：

完美！让我清理一下测试文件：

让我再优化一下 PRINT 语句的序列化代码，去除重复逻辑：

```rust
/// 执行引擎
///
/// 求值表达式并执行语句

use crate::ast::*;
use crate::error::{BasicError, Result};
use crate::runtime::Runtime;
use crate::variables::{Value, Variables};

/// 输入回调函数类型
pub type InputCallback = Box<dyn FnMut(&str) -> Option<String>>;

/// 执行引擎
pub struct Executor {
    runtime: Runtime,
    variables: Variables,
    /// 输出缓冲区（用于测试和捕获输出）
    output_buffer: Vec<String>,
    /// 当前打印列位置
    print_column: usize,
    /// DATA 数据存储
    data_values: Vec<DataValue>,
    /// DATA 数据指针（当前读取位置）
    data_pointer: usize,
    /// 输入回调函数（用于测试）
    input_callback: Option<InputCallback>,
}

/// DATA 值类型
#[derive(Debug, Clone, PartialEq)]
pub enum DataValue {
    Number(f64),
    String(String),
}

impl Executor {
    /// 创建新的执行器
    pub fn new() -> Self {
        Executor {
            runtime: Runtime::new(),
            variables: Variables::new(),
            output_buffer: Vec::new(),
            print_column: 0,
            data_values: Vec::new(),
            data_pointer: 0,
            input_callback: None,
        }
    }
    
    /// 设置输入回调函数（用于测试）
    pub fn set_input_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&str) -> Option<String> + 'static,
    {
        self.input_callback = Some(Box::new(callback));
    }
    
    /// 添加 DATA 值
    pub fn add_data_value(&mut self, value: DataValue) {
        self.data_values.push(value);
    }
    
    /// 重置 DATA 指针
    pub fn restore_data(&mut self) {
        self.data_pointer = 0;
    }
    
    /// 读取下一个 DATA 值
    fn read_data_value(&mut self) -> Result<DataValue> {
        if self.data_pointer >= self.data_values.len() {
            return Err(BasicError::OutOfData);
        }
        
        let value = self.data_values[self.data_pointer].clone();
        self.data_pointer += 1;
        Ok(value)
    }
    
    /// 获取输出内容（用于测试）
    pub fn get_output(&self) -> String {
        self.output_buffer.join("")
    }
    
    /// 清空输出缓冲区
    pub fn clear_output(&mut self) {
        self.output_buffer.clear();
        self.print_column = 0;
    }
    
    /// 输出文本（添加到缓冲区并打印到终端）
    fn output(&mut self, text: &str) {
        // 打印到终端
        print!("{}", text);
        use std::io::Write;
        std::io::stdout().flush().ok();
        
        // 同时添加到缓冲区（用于测试）
        self.output_buffer.push(text.to_string());
        
        // 更新列位置
        for ch in text.chars() {
            if ch == '\n' {
                self.print_column = 0;
            } else {
                self.print_column += 1;
            }
        }
    }
    
    /// 输出换行
    fn output_newline(&mut self) {
        self.output("\n");
    }

    /// 获取运行时引用
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    /// 获取变量存储引用
    pub fn variables(&self) -> &Variables {
        &self.variables
    }

    /// 获取运行时可变引用
    pub fn runtime_mut(&mut self) -> &mut Runtime {
        &mut self.runtime
    }

    /// 获取变量存储可变引用
    pub fn variables_mut(&mut self) -> &mut Variables {
        &mut self.variables
    }

    /// 求值表达式
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            
            Expr::String(s) => Ok(Value::String(s.clone())),
            
            Expr::Variable(name) => {
                Ok(self.variables.get(name))
            }
            
            Expr::ArrayAccess { name, indices } => {
                // 求值所有索引
                let idx_values: Result<Vec<usize>> = indices.iter()
                    .map(|idx_expr| {
                        self.eval_expr(idx_expr)?
                            .as_number()
                            .and_then(|n| {
                                if n < 0.0 {
                                    Err(BasicError::SubscriptOutOfRange(
                                        "Negative array index".to_string()
                                    ))
                                } else {
                                    Ok(n as usize)
                                }
                            })
                    })
                    .collect();
                
                let indices_usize = idx_values?;
                self.variables.get_array_element(name, &indices_usize)
            }
            
            Expr::FunctionCall { name, args } => {
                self.eval_function_call(name, args)
            }
            
            Expr::BinaryOp { left, op, right } => {
                self.eval_binary_op(left, *op, right)
            }
            
            Expr::UnaryOp { op, operand } => {
                self.eval_unary_op(*op, operand)
            }
        }
    }

    /// 求值二元运算
    fn eval_binary_op(&mut self, left: &Expr, op: BinaryOperator, right: &Expr) -> Result<Value> {
        use BinaryOperator::*;

        let left_val = self.eval_expr(left)?;
        let right_val = self.eval_expr(right)?;

        match op {
            // 算术运算符
            Add => {
                if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    Ok(Value::Number(l + r))
                } else if left_val.is_string() && right_val.is_string() {
                    // 字符串连接
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    Ok(Value::String(format!("{}{}", l, r)))
                } else {
                    Err(BasicError::TypeMismatch(
                        "Cannot add incompatible types".to_string()
                    ))
                }
            }
            
            Subtract => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l - r))
            }
            
            Multiply => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l * r))
            }
            
            Divide => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                if r == 0.0 {
                    return Err(BasicError::DivisionByZero);
                }
                Ok(Value::Number(l / r))
            }
            
            Power => {
                let l = left_val.as_number()?;
                let r = right_val.as_number()?;
                Ok(Value::Number(l.powf(r)))
            }
            
            // 关系运算符（BASIC 中 true = -1, false = 0）
            Equal => {
                let result = if left_val == right_val { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            NotEqual => {
                let result = if left_val != right_val { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            Less => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l < r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l < r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            Greater => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l > r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l > r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            LessEqual => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l <= r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l <= r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            GreaterEqual => {
                let result = if left_val.is_number() && right_val.is_number() {
                    let l = left_val.as_number()?;
                    let r = right_val.as_number()?;
                    if l >= r { -1.0 } else { 0.0 }
                } else if left_val.is_string() && right_val.is_string() {
                    let l = left_val.as_string()?;
                    let r = right_val.as_string()?;
                    if l >= r { -1.0 } else { 0.0 }
                } else {
                    return Err(BasicError::TypeMismatch("Cannot compare".to_string()));
                };
                Ok(Value::Number(result))
            }
            
            // 逻辑运算符（按位）
            And => {
                let l = left_val.as_number()? as i32;
                let r = right_val.as_number()? as i32;
                Ok(Value::Number((l & r) as f64))
            }
            
            Or => {
                let l = left_val.as_number()? as i32;
                let r = right_val.as_number()? as i32;
                Ok(Value::Number((l | r) as f64))
            }
        }
    }

    /// 求值一元运算
    fn eval_unary_op(&mut self, op: UnaryOperator, operand: &Expr) -> Result<Value> {
        let val = self.eval_expr(operand)?;
        
        match op {
            UnaryOperator::Minus => {
                let n = val.as_number()?;
                Ok(Value::Number(-n))
            }
            UnaryOperator::Not => {
                let n = val.as_number()? as i32;
                Ok(Value::Number((!n) as f64))
            }
        }
    }

    /// 求值函数调用（内置函数）
    fn eval_function_call(&mut self, name: &str, args: &[Expr]) -> Result<Value> {
        match name.to_uppercase().as_str() {
            // 数学函数
            "SGN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SGN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                let result = if n > 0.0 { 1.0 } else if n < 0.0 { -1.0 } else { 0.0 };
                Ok(Value::Number(result))
            }
            
            "INT" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("INT requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.floor()))
            }
            
            "ABS" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ABS requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.abs()))
            }
            
            "SQR" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SQR requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n < 0.0 {
                    return Err(BasicError::IllegalQuantity("SQR of negative number".to_string()));
                }
                Ok(Value::Number(n.sqrt()))
            }
            
            "SIN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("SIN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.sin()))
            }
            
            "COS" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("COS requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.cos()))
            }
            
            "TAN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("TAN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.tan()))
            }
            
            "ATN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ATN requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.atan()))
            }
            
            "LOG" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("LOG requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n <= 0.0 {
                    return Err(BasicError::IllegalQuantity("LOG of non-positive number".to_string()));
                }
                Ok(Value::Number(n.ln()))
            }
            
            "EXP" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("EXP requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                Ok(Value::Number(n.exp()))
            }
            
            // 字符串函数
            "LEN" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("LEN requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                Ok(Value::Number(s.len() as f64))
            }
            
            "ASC" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("ASC requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                if s.is_empty() {
                    return Err(BasicError::IllegalQuantity("ASC of empty string".to_string()));
                }
                Ok(Value::Number(s.chars().next().unwrap() as u8 as f64))
            }
            
            "CHR$" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("CHR$ requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                if n < 0.0 || n > 255.0 {
                    return Err(BasicError::IllegalQuantity("CHR$ argument out of range".to_string()));
                }
                let ch = n as u8 as char;
                Ok(Value::String(ch.to_string()))
            }
            
            "STR$" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("STR$ requires 1 argument".to_string()));
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                // BASIC 的 STR$ 在正数前加空格
                let s = if n >= 0.0 {
                    format!(" {}", n)
                } else {
                    n.to_string()
                };
                Ok(Value::String(s))
            }
            
            "VAL" => {
                if args.len() != 1 {
                    return Err(BasicError::SyntaxError("VAL requires 1 argument".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = s.trim().parse::<f64>().unwrap_or(0.0);
                Ok(Value::Number(n))
            }
            
            "LEFT$" => {
                if args.len() != 2 {
                    return Err(BasicError::SyntaxError("LEFT$ requires 2 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = self.eval_expr(&args[1])?.as_number()? as usize;
                let result: String = s.chars().take(n).collect();
                Ok(Value::String(result))
            }
            
            "RIGHT$" => {
                if args.len() != 2 {
                    return Err(BasicError::SyntaxError("RIGHT$ requires 2 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let n = self.eval_expr(&args[1])?.as_number()? as usize;
                let len = s.chars().count();
                let skip = if n > len { 0 } else { len - n };
                let result: String = s.chars().skip(skip).collect();
                Ok(Value::String(result))
            }
            
            "MID$" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err(BasicError::SyntaxError("MID$ requires 2 or 3 arguments".to_string()));
                }
                let s = self.eval_expr(&args[0])?.as_string()?;
                let start = self.eval_expr(&args[1])?.as_number()? as usize;
                
                // BASIC 的 MID$ 是 1-based
                let start = if start > 0 { start - 1 } else { 0 };
                
                let chars: Vec<char> = s.chars().collect();
                
                if args.len() == 3 {
                    let len = self.eval_expr(&args[2])?.as_number()? as usize;
                    let result: String = chars.iter().skip(start).take(len).collect();
                    Ok(Value::String(result))
                } else {
                    let result: String = chars.iter().skip(start).collect();
                    Ok(Value::String(result))
                }
            }
            
            _ => Err(BasicError::SyntaxError(
                format!("Unknown function: {}", name)
            )),
        }
    }

    /// 执行语句
    pub fn execute_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Let { target, value } => {
                let val = self.eval_expr(value)?;
                
                match target {
                    AssignTarget::Variable(name) => {
                        self.variables.set(name, val)?;
                    }
                    AssignTarget::ArrayElement { name, indices } => {
                        let idx_values: Result<Vec<usize>> = indices.iter()
                            .map(|idx_expr| {
                                self.eval_expr(idx_expr)?
                                    .as_number()
                                    .map(|n| n as usize)
                            })
                            .collect();
                        
                        let indices_usize = idx_values?;
                        self.variables.set_array_element(name, &indices_usize, val)?;
                    }
                }
                
                Ok(())
            }
            
            Statement::End => {
                self.runtime.end_execution();
                Ok(())
            }
            
            Statement::Stop => {
                self.runtime.pause_execution();
                Ok(())
            }
            
            Statement::New => {
                self.runtime.clear_program();
                self.variables.clear();
                Ok(())
            }
            
            Statement::Clear => {
                self.variables.clear();
                Ok(())
            }
            
            Statement::Dim { arrays } => {
                for arr_dim in arrays {
                    let dimensions: Result<Vec<usize>> = arr_dim.dimensions.iter()
                        .map(|dim_expr| {
                            self.eval_expr(dim_expr)?
                                .as_number()
                                .map(|n| n as usize)
                        })
                        .collect();
                    
                    let dims = dimensions?;
                    self.variables.dim_array(&arr_dim.name, dims)?;
                }
                Ok(())
            }
            
            Statement::Print { items } => {
                self.execute_print(items)?;
                Ok(())
            }
            
            Statement::Goto { line_number } => {
                let line_val = self.eval_expr(line_number)?;
                let line = line_val.as_number()? as u16;
                self.runtime.set_execution_position(line, 0)?;
                Ok(())
            }
            
            Statement::If { condition, then_part } => {
                let cond_val = self.eval_expr(condition)?;
                let cond_num = cond_val.as_number()?;
                
                // BASIC 中，任何非零值都是真
                if cond_num != 0.0 {
                    match then_part.as_ref() {
                        ThenPart::LineNumber(line) => {
                            self.runtime.set_execution_position(*line as u16, 0)?;
                        }
                        ThenPart::Statement(stmt) => {
                            self.execute_statement(stmt)?;
                        }
                        ThenPart::Statements(stmts) => {
                            for stmt in stmts {
                                self.execute_statement(stmt)?;
                            }
                        }
                    }
                }
                Ok(())
            }
            
            Statement::Gosub { line_number } => {
                // 保存返回地址（当前行号和语句索引）
                let return_line = self.runtime.get_current_line().unwrap_or(0);
                let return_stmt = 0; // 简化：返回到下一行的第一条语句
                
                // 入栈
                self.runtime.push_gosub(return_line, return_stmt)?;
                
                // 跳转到子程序
                let line_val = self.eval_expr(line_number)?;
                let line = line_val.as_number()? as u16;
                self.runtime.set_execution_position(line, 0)?;
                
                Ok(())
            }
            
            Statement::Return => {
                // 从栈中弹出返回地址
                let (return_line, return_stmt) = self.runtime.pop_gosub()?;
                
                // 跳转回返回地址
                self.runtime.set_execution_position(return_line, return_stmt)?;
                
                Ok(())
            }
            
            Statement::Input { prompt, variables } => {
                // 提取变量名
                let var_names: Vec<String> = variables.iter()
                    .map(|target| match target {
                        AssignTarget::Variable(name) => name.clone(),
                        AssignTarget::ArrayElement { .. } => {
                            // INPUT 不支持数组元素
                            String::new()
                        }
                    })
                    .collect();
                
                self.execute_input(prompt.as_deref(), &var_names)?;
                Ok(())
            }
            
            Statement::Data { values: _ } => {
                // DATA 语句在程序加载时处理，执行时跳过
                // 数据已经存储在 data_values 中
                Ok(())
            }
            
            Statement::Read { variables } => {
                for target in variables {
                    let var_name = match target {
                        AssignTarget::Variable(name) => name.as_str(),
                        AssignTarget::ArrayElement { .. } => {
                            return Err(BasicError::SyntaxError(
                                "READ does not support array elements".to_string()
                            ));
                        }
                    };
                    
                    let data_val = self.read_data_value()?;
                    
                    // 根据变量名判断类型
                    if var_name.ends_with('$') {
                        // 字符串变量
                        let str_val = match data_val {
                            DataValue::String(s) => s,
                            DataValue::Number(n) => n.to_string(),
                        };
                        self.variables.set(var_name, Value::String(str_val))?;
                    } else {
                        // 数值变量
                        let num_val = match data_val {
                            DataValue::Number(n) => n,
                            DataValue::String(s) => {
                                s.trim().parse::<f64>().unwrap_or(0.0)
                            }
                        };
                        self.variables.set(var_name, Value::Number(num_val))?;
                    }
                }
                Ok(())
            }
            
            Statement::Restore { line_number } => {
                if line_number.is_some() {
                    // RESTORE 到指定行（暂不支持，需要跟踪每行的 DATA 位置）
                    return Err(BasicError::SyntaxError(
                        "RESTORE to specific line not yet implemented".to_string()
                    ));
                }
                self.restore_data();
                Ok(())
            }
            
            Statement::For { var, start, end, step } => {
                // 计算起始值、结束值和步长
                let start_val = self.eval_expr(&start)?;
                let end_val = self.eval_expr(&end)?;
                let step_val = if let Some(ref s) = step {
                    self.eval_expr(s)?
                } else {
                    Value::Number(1.0)
                };
                
                // 提取数值
                let start_num = start_val.as_number()?;
                let end_num = end_val.as_number()?;
                let step_num = step_val.as_number()?;
                
                // 检查步长
                if step_num == 0.0 {
                    return Err(BasicError::IllegalQuantity(
                        "FOR loop step cannot be zero".to_string()
                    ));
                }
                
                // 设置循环变量初值
                self.variables.set(var, Value::Number(start_num))?;
                
                // 获取当前位置
                let loop_line = self.runtime.get_current_line()
                    .ok_or_else(|| BasicError::SyntaxError("FOR without line number".to_string()))?;
                let loop_stmt = self.runtime.get_current_stmt_index();
                
                // 将循环信息压入栈
                self.runtime.push_for_loop(
                    var.clone(),
                    end_num,
                    step_num,
                    loop_line,
                    loop_stmt,
                )?;
                
                Ok(())
            }
            
            Statement::Next { var } => {
                // 弹出 FOR 循环信息
                let (loop_var, end_val, step_val, loop_line, loop_stmt) = 
                    self.runtime.pop_for_loop(var.clone())?;
                
                // 获取当前循环变量的值
                let current_val = self.variables.get(&loop_var).as_number()?;
                
                // 递增/递减
                let new_val = current_val + step_val;
                
                // 检查是否继续循环
                let should_continue = if step_val > 0.0 {
                    new_val <= end_val
                } else {
                    new_val >= end_val
                };
                
                if should_continue {
                    // 更新循环变量
                    self.variables.set(&loop_var, Value::Number(new_val))?;
                    
                    // 重新压入栈（继续循环）
                    self.runtime.push_for_loop(
                        loop_var.clone(),
                        end_val,
                        step_val,
                        loop_line,
                        loop_stmt,
                    )?;
                    
                    // 跳转回 FOR 语句的下一条语句
                    self.runtime.set_execution_position(loop_line, loop_stmt + 1)?;
                }
                // 否则继续执行下一条语句（循环结束）
                
                Ok(())
            }
            
            Statement::On { expr, targets, is_gosub } => {
                // 计算表达式的值
                let index_val = self.eval_expr(&expr)?;
                let index = index_val.as_number()? as i32;
                
                // 索引从 1 开始
                if index < 1 || index as usize > targets.len() {
                    // 超出范围，继续执行下一条语句
                    return Ok(());
                }
                
                // 获取目标行号
                let target_line = targets[(index - 1) as usize];
                
                if *is_gosub {
                    // ON...GOSUB：保存返回地址并跳转
                    let return_line = self.runtime.get_current_line()
                        .ok_or_else(|| BasicError::SyntaxError("GOSUB without line number".to_string()))?;
                    let return_stmt = self.runtime.get_current_stmt_index();
                    
                    self.runtime.push_gosub(return_line, return_stmt)?;
                    self.runtime.set_execution_position(target_line, 0)?;
                } else {
                    // ON...GOTO：直接跳转
                    self.runtime.set_execution_position(target_line, 0)?;
                }
                
                Ok(())
            }
            
            Statement::Load { filename } => {
                self.execute_load(filename)?;
                Ok(())
            }
            
            Statement::Save { filename } => {
                self.execute_save(filename)?;
                Ok(())
            }
            
            _ => {
                // 其他语句暂未实现
                Err(BasicError::SyntaxError(
                    "Statement not yet implemented".to_string()
                ))
            }
        }
    }
    
    /// 执行 INPUT 语句
    fn execute_input(&mut self, prompt: Option<&str>, variables: &[String]) -> Result<()> {
        // 显示提示符
        if let Some(p) = prompt {
            self.output(p);
            self.output("? ");
        } else {
            self.output("? ");
        }
        
        // 读取输入
        let input_line = if let Some(ref mut callback) = self.input_callback {
            let prompt_text = prompt.unwrap_or("");
            callback(prompt_text).ok_or_else(|| {
                BasicError::SyntaxError("No input provided".to_string())
            })?
        } else {
            // 在实际 REPL 中，这里会从 stdin 读取
            return Err(BasicError::SyntaxError(
                "No input callback set".to_string()
            ));
        };
        
        // 解析输入值（考虑引号内的逗号）
        let values = Self::parse_input_values(&input_line);
        
        if values.len() != variables.len() {
            self.output("?EXTRA IGNORED\n");
        }
        
        // 赋值给变量
        for (i, var_name) in variables.iter().enumerate() {
            if i >= values.len() {
                break;
            }
            
            let input_val = &values[i];
            
            if var_name.ends_with('$') {
                // 字符串变量
                let str_val = if input_val.starts_with('"') && input_val.ends_with('"') {
                    // 去掉引号
                    input_val[1..input_val.len()-1].to_string()
                } else {
                    input_val.clone()
                };
                self.variables.set(var_name, Value::String(str_val))?;
            } else {
                // 数值变量
                match input_val.parse::<f64>() {
                    Ok(num) => {
                        self.variables.set(var_name, Value::Number(num))?;
                    }
                    Err(_) => {
                        self.output("?REDO FROM START\n");
                        return Err(BasicError::TypeMismatch(
                            "Invalid number input".to_string()
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// 解析输入值，处理带引号的字符串
    fn parse_input_values(input: &str) -> Vec<String> {
        let mut values = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        
        for ch in input.chars() {
            match ch {
                '"' => {
                    in_quotes = !in_quotes;
                    current.push(ch);
                }
                ',' if !in_quotes => {
                    values.push(current.trim().to_string());
                    current.clear();
                }
                _ => {
                    current.push(ch);
                }
            }
        }
        
        if !current.is_empty() || input.ends_with(',') {
            values.push(current.trim().to_string());
        }
        
        values
    }
    
    /// 执行 SAVE 命令 - 保存程序到文件
    fn execute_save(&self, filename: &str) -> Result<()> {
        use std::fs::File;
        use std::io::Write;
        
        let program = self.runtime.clone_program();
        if program.is_empty() {
            return Err(BasicError::SyntaxError("No program to save".to_string()));
        }
        
        let mut file = File::create(filename).map_err(|e| {
            BasicError::SyntaxError(format!("Failed to create file: {}", e))
        })?;
        
        for (_, line) in program.iter() {
            let line_text = Self::serialize_program_line(line);
            writeln!(file, "{}", line_text).map_err(|e| {
                BasicError::SyntaxError(format!("Failed to write to file: {}", e))
            })?;
        }
        
        Ok(())
    }
    
    /// 将程序行序列化为文本
    pub fn serialize_program_line(line: &ProgramLine) -> String {
        let mut result = format!("{}", line.line_number);
        
        for (i, stmt) in line.statements.iter().enumerate() {
            if i > 0 {
                result.push_str(":");
            }
            result.push(' ');
            result.push_str(&Self::serialize_statement(stmt));
        }
        
        result
    }
    
    /// 将语句序列化为文本
    pub fn serialize_statement(stmt: &Statement) -> String {
        match stmt {
            Statement::Let { target, value } => {
                format!("{} = {}", Self::serialize_assign_target(target), Self::serialize_expr(value))
            }
            Statement::Print { items } => {
                let mut result = "PRINT".to_string();
                for item in items.iter() {
                    result.push(' ');
                    result.push_str(&Self::serialize_print_item(item));
                }
                result
            }
            Statement::If { condition, then_part } => {
                format!("IF {} THEN {}", Self::serialize_expr(condition), Self::serialize_then_part(then_part))
            }
            Statement::Goto { line_number } => {
                format!("GOTO {}", Self::serialize_expr(line_number))
            }
            Statement::Gosub { line_number } => {
                format!("GOSUB {}", Self::serialize_expr(line_number))
            }
            Statement::Return => "RETURN".to_string(),
            Statement::For { var, start, end, step } => {
                let mut result = format!("FOR {} = {} TO {}", var, Self::serialize_expr(start), Self::serialize_expr(end));
                if let Some(step_expr) = step {
                    result.push_str(&format!(" STEP {}", Self::serialize_expr(step_expr)));
                }
                result
            }
            Statement::Next { var } => {
                if let Some(v) = var {
                    format!("NEXT {}", v)
                } else {
                    "NEXT".to_string()
                }
            }
            Statement::On { expr, targets, is_gosub } => {
                let keyword = if *is_gosub { "GOSUB" } else { "GOTO" };
                let target_str = targets.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("ON {} {} {}", Self::serialize_expr(expr), keyword, target_str)
            }
            Statement::Input { prompt, variables } => {
                let mut result = "INPUT ".to_string();
                if let Some(p) = prompt {
                    result.push_str(&format!("\"{}\" ", p));
                }
                let var_str = variables.iter()
                    .map(|v| Self::serialize_assign_target(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                result.push_str(&var_str);
                result
            }
            Statement::Dim { arrays } => {
                let arr_str = arrays.iter()
                    .map(|arr| {
                        let dims = arr.dimensions.iter()
                            .map(|d| Self::serialize_expr(d))
                            .collect::<Vec<_>>()
                            .join(",");
                        format!("{}({})", arr.name, dims)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("DIM {}", arr_str)
            }
            Statement::Data { values } => {
                let val_str = values.iter()
                    .map(|v| match v {
                        crate::ast::DataValue::Number(n) => n.to_string(),
                        crate::ast::DataValue::String(s) => format!("\"{}\"", s),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("DATA {}", val_str)
            }
            Statement::Read { variables } => {
                let var_str = variables.iter()
                    .map(|v| Self::serialize_assign_target(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("READ {}", var_str)
            }
            Statement::Restore { line_number } => {
                if let Some(ln) = line_number {
                    format!("RESTORE {}", ln)
                } else {
                    "RESTORE".to_string()
                }
            }
            Statement::Rem => "REM".to_string(),
            Statement::End => "END".to_string(),
            Statement::Stop => "STOP".to_string(),
            Statement::New => "NEW".to_string(),
            Statement::Clear => "CLEAR".to_string(),
            _ => "REM UNSUPPORTED STATEMENT".to_string(),
        }
    }
    
    /// 将表达式序列化为文本
    pub fn serialize_expr(expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => n.to_string(),
            Expr::String(s) => format!("\"{}\"", s),
            Expr::Variable(name) => name.clone(),
            Expr::ArrayAccess { name, indices } => {
                let idx_str = indices.iter()
                    .map(|i| Self::serialize_expr(i))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, idx_str)
            }
            Expr::FunctionCall { name, args } => {
                let arg_str = args.iter()
                    .map(|a| Self::serialize_expr(a))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, arg_str)
            }
            Expr::BinaryOp { left, op, right } => {
                let op_str = match op {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Subtract => "-",
                    BinaryOperator::Multiply => "*",
                    BinaryOperator::Divide => "/",
                    BinaryOperator::Power => "^",
                    BinaryOperator::Equal => "=",
                    BinaryOperator::NotEqual => "<>",
                    BinaryOperator::Less => "<",
                    BinaryOperator::Greater => ">",
                    BinaryOperator::LessEqual => "<=",
                    BinaryOperator::GreaterEqual => ">=",
                    BinaryOperator::And => " AND ",
                    BinaryOperator::Or => " OR ",
                };
                format!("({} {} {})", Self::serialize_expr(left), op_str, Self::serialize_expr(right))
            }
            Expr::UnaryOp { op, operand } => {
                let op_str = match op {
                    UnaryOperator::Minus => "-",
                    UnaryOperator::Not => "NOT ",
                };
                format!("{}{}", op_str, Self::serialize_expr(operand))
            }
        }
    }
    
    /// 将赋值目标序列化为文本
    pub fn serialize_assign_target(target: &AssignTarget) -> String {
        match target {
            AssignTarget::Variable(name) => name.clone(),
            AssignTarget::ArrayElement { name, indices } => {
                let idx_str = indices.iter()
                    .map(|i| Self::serialize_expr(i))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}({})", name, idx_str)
            }
        }
    }
    
    /// 将THEN部分序列化为文本
    pub fn serialize_then_part(then_part: &ThenPart) -> String {
        match then_part {
            ThenPart::LineNumber(ln) => ln.to_string(),
            ThenPart::Statement(stmt) => Self::serialize_statement(stmt),
            ThenPart::Statements(stmts) => {
                stmts.iter()
                    .map(|s| Self::serialize_statement(s))
                    .collect::<Vec<_>>()
                    .join(":")
            }
        }
    }
    
    /// 将PRINT项序列化为文本
    pub fn serialize_print_item(item: &PrintItem) -> String {
        match item {
            PrintItem::Expr(expr) => Self::serialize_expr(expr),
            PrintItem::Tab(expr) => format!("TAB({})", Self::serialize_expr(expr)),
            PrintItem::Spc(expr) => format!("SPC({})", Self::serialize_expr(expr)),
            PrintItem::Comma => ",".to_string(),
            PrintItem::Semicolon => ";".to_string(),
        }
    }
    
    /// 执行 LOAD 命令 - 从文件加载程序
    fn execute_load(&mut self, filename: &str) -> Result<()> {
        use std::fs;
        use crate::tokenizer::Tokenizer;
        use crate::parser::Parser;
        
        // 读取文件内容
        let content = fs::read_to_string(filename).map_err(|e| {
            BasicError::SyntaxError(format!("Failed to read file: {}", e))
        })?;
        
        // 清空当前程序
        self.runtime.clear_program();
        self.variables.clear();
        
        // 逐行解析并添加到程序
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            // 使用tokenizer和parser解析每一行
            let mut tokenizer = Tokenizer::new(line);
            let tokens = tokenizer.tokenize_line()?;
            
            let mut parser = Parser::new(tokens);
            if let Some(program_line) = parser.parse_line()? {
                if program_line.line_number > 0 {
                    self.runtime.add_line(program_line);
                }
            }
        }
        
        Ok(())
    }
    
    /// 执行 PRINT 语句
    fn execute_print(&mut self, items: &[PrintItem]) -> Result<()> {
        if items.is_empty() {
            // 空 PRINT，仅输出换行
            self.output_newline();
            return Ok(());
        }
        
        for item in items.iter() {
            match item {
                PrintItem::Expr(expr) => {
                    let val = self.eval_expr(expr)?;
                    self.print_value(&val)?;
                }
                PrintItem::Tab(expr) => {
                    let target_col = self.eval_expr(expr)?.as_number()? as usize;
                    if target_col > self.print_column {
                        let spaces = target_col - self.print_column;
                        self.output(&" ".repeat(spaces));
                    } else if target_col < self.print_column {
                        // TAB 到更小的列，换行后跳转
                        self.output_newline();
                        self.output(&" ".repeat(target_col));
                    }
                }
                PrintItem::Spc(expr) => {
                    let spaces = self.eval_expr(expr)?.as_number()? as usize;
                    self.output(&" ".repeat(spaces));
                }
                PrintItem::Comma => {
                    // 逗号：对齐到下一个 14 列边界
                    let next_col = ((self.print_column / 14) + 1) * 14;
                    let spaces_needed = next_col - self.print_column;
                    self.output(&" ".repeat(spaces_needed));
                }
                PrintItem::Semicolon => {
                    // 分号：不添加空格（紧密连接）
                }
            }
        }
        
        // 检查最后一项是否是分隔符
        if let Some(last) = items.last() {
            if !matches!(last, PrintItem::Comma | PrintItem::Semicolon) {
                // 如果最后不是分隔符，输出换行
                self.output_newline();
            }
        } else {
            self.output_newline();
        }
        
        Ok(())
    }
    
    /// 打印值（根据 BASIC 格式）
    fn print_value(&mut self, val: &Value) -> Result<()> {
        match val {
            Value::Number(n) => {
                // BASIC 数值格式：正数前后各有空格，负数前有空格
                let formatted = if *n >= 0.0 {
                    format!(" {} ", n)
                } else {
                    format!(" {}", n)
                };
                self.output(&formatted);
            }
            Value::String(s) => {
                // 普通字符串，直接输出
                self.output(s);
            }
        }
        Ok(())
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Requirement: 算术运算符 - 加法
    #[test]
    fn test_addition() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Add,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    // Requirement: 算术运算符 - 减法
    #[test]
    fn test_subtraction() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(10.0),
            BinaryOperator::Subtract,
            Expr::Number(7.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    // Requirement: 算术运算符 - 乘法
    #[test]
    fn test_multiplication() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(4.0),
            BinaryOperator::Multiply,
            Expr::Number(5.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(20.0));
    }

    // Requirement: 算术运算符 - 除法
    #[test]
    fn test_division() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(15.0),
            BinaryOperator::Divide,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    // Requirement: 算术运算符 - 浮点除法
    #[test]
    fn test_float_division() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(10.0),
            BinaryOperator::Divide,
            Expr::Number(4.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(2.5));
    }

    // Requirement: 算术运算符 - 除以零
    #[test]
    fn test_division_by_zero() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Divide,
            Expr::Number(0.0)
        );
        let result = exec.eval_expr(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BasicError::DivisionByZero));
    }

    // Requirement: 算术运算符 - 乘方
    #[test]
    fn test_power() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(2.0),
            BinaryOperator::Power,
            Expr::Number(3.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    // Requirement: 一元运算符 - 一元负号
    #[test]
    fn test_unary_minus() {
        let mut exec = Executor::new();
        let expr = Expr::unary(UnaryOperator::Minus, Expr::Number(5.0));
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-5.0));
    }

    // Requirement: 关系运算符 - 等于
    #[test]
    fn test_equal() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::Equal,
            Expr::Number(5.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-1.0)); // BASIC true = -1
    }

    // Requirement: 关系运算符 - 不等于
    #[test]
    fn test_not_equal() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::Number(5.0),
            BinaryOperator::NotEqual,
            Expr::Number(4.0)
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(-1.0));
    }

    // Requirement: 字符串运算符 - 字符串连接
    #[test]
    fn test_string_concatenation() {
        let mut exec = Executor::new();
        let expr = Expr::binary(
            Expr::String("HELLO".to_string()),
            BinaryOperator::Add,
            Expr::String(" WORLD".to_string())
        );
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::String("HELLO WORLD".to_string()));
    }

    // Test: 变量读取
    #[test]
    fn test_variable_read() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(42.0)).unwrap();
        
        let expr = Expr::Variable("A".to_string());
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    // Test: LET 语句执行
    #[test]
    fn test_let_statement() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Let {
            target: AssignTarget::Variable("X".to_string()),
            value: Expr::Number(100.0),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.variables.get("X"), Value::Number(100.0));
    }

    // Test: DIM 语句执行
    #[test]
    fn test_dim_statement() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Dim {
            arrays: vec![
                ArrayDim {
                    name: "A".to_string(),
                    dimensions: vec![Expr::Number(10.0)],
                }
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert!(exec.variables.has_array("A"));
    }

    // Test: 数学函数
    #[test]
    fn test_math_functions() {
        let mut exec = Executor::new();
        
        // ABS
        let expr = Expr::FunctionCall {
            name: "ABS".to_string(),
            args: vec![Expr::Number(-42.0)],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(42.0));
        
        // INT
        let expr = Expr::FunctionCall {
            name: "INT".to_string(),
            args: vec![Expr::Number(3.7)],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    // Test: 字符串函数
    #[test]
    fn test_string_functions() {
        let mut exec = Executor::new();
        
        // LEN
        let expr = Expr::FunctionCall {
            name: "LEN".to_string(),
            args: vec![Expr::String("HELLO".to_string())],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(5.0));
        
        // LEFT$
        let expr = Expr::FunctionCall {
            name: "LEFT$".to_string(),
            args: vec![
                Expr::String("HELLO".to_string()),
                Expr::Number(3.0),
            ],
        };
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::String("HEL".to_string()));
    }

    // Test: 复杂表达式
    #[test]
    fn test_complex_expression() {
        let mut exec = Executor::new();
        
        // 2 + 3 * 4 = 14
        let expr = Expr::binary(
            Expr::Number(2.0),
            BinaryOperator::Add,
            Expr::binary(
                Expr::Number(3.0),
                BinaryOperator::Multiply,
                Expr::Number(4.0)
            )
        );
        
        let result = exec.eval_expr(&expr).unwrap();
        assert_eq!(result, Value::Number(14.0));
    }

    // Requirement: PRINT 语句 - 打印数值
    #[test]
    fn test_print_number() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(42.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 42 \n");
    }

    // Requirement: PRINT 语句 - 打印字符串
    #[test]
    fn test_print_string() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("HELLO".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "HELLO\n");
    }

    // Requirement: PRINT 语句 - 打印变量
    #[test]
    fn test_print_variable() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(10.0)).unwrap();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Variable("A".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 10 \n");
    }

    // Requirement: PRINT 语句 - 分号分隔（紧密连接）
    #[test]
    fn test_print_semicolon() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(1.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::Number(2.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::Number(3.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 1  2  3 \n");
    }

    // Requirement: PRINT 语句 - 行尾分号（不换行）
    #[test]
    fn test_print_no_newline() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(42.0)),
                PrintItem::Semicolon,
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), " 42 ");
    }

    // Requirement: PRINT 语句 - 空 PRINT
    #[test]
    fn test_print_empty() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![],
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "\n");
    }

    // Requirement: PRINT 语句 - 逗号分隔（列对齐）
    #[test]
    fn test_print_comma_alignment() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::Number(1.0)),
                PrintItem::Comma,
                PrintItem::Expr(Expr::Number(2.0)),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // 第一个数 " 1 " 占 3 列，逗号应该对齐到第 14 列
        assert!(output.starts_with(" 1 "));
        assert!(output.contains(" 2 "));
    }

    // Requirement: GOTO 语句
    #[test]
    fn test_goto_statement() {
        let mut exec = Executor::new();
        
        // 添加程序行
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("A".to_string()),
                value: Expr::Number(1.0),
            }]
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("B".to_string()),
                value: Expr::Number(99.0),
            }]
        });
        
        let stmt = Statement::Goto {
            line_number: Expr::Number(100.0),
        };
        exec.execute_statement(&stmt).unwrap();
        
        // 验证跳转成功（下一行应该是 100）
        assert_eq!(exec.runtime().get_current_line(), Some(100));
    }

    // Requirement: IF...THEN 语句 - 条件为真
    #[test]
    fn test_if_then_true() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(15.0)).unwrap();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行来设置初始状态
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::LineNumber(100)),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.runtime().get_current_line(), Some(100));
    }

    // Requirement: IF...THEN 语句 - 条件为假
    #[test]
    fn test_if_then_false() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(5.0)).unwrap();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![],
        });
        
        let current_line = exec.runtime().get_current_line();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::LineNumber(100)),
        };
        
        exec.execute_statement(&stmt).unwrap();
        // 条件为假，不应该跳转
        assert_eq!(exec.runtime().get_current_line(), current_line);
    }

    // Requirement: IF...THEN 语句 - THEN 后跟语句
    #[test]
    fn test_if_then_statement() {
        let mut exec = Executor::new();
        exec.variables.set("A", Value::Number(15.0)).unwrap();
        
        let stmt = Statement::If {
            condition: Expr::binary(
                Expr::Variable("A".to_string()),
                BinaryOperator::Greater,
                Expr::Number(10.0),
            ),
            then_part: Box::new(ThenPart::Statement(
                Statement::Print {
                    items: vec![
                        PrintItem::Expr(Expr::String("TRUE".to_string())),
                    ],
                }
            )),
        };
        
        exec.execute_statement(&stmt).unwrap();
        assert_eq!(exec.get_output(), "TRUE\n");
    }

    // Test: TAB 函数
    #[test]
    fn test_tab_function() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("A".to_string())),
                PrintItem::Semicolon,
                PrintItem::Tab(Expr::Number(10.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::String("B".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // A 在列 0，TAB(10) 跳到列 10，然后是 B
        assert!(output.starts_with("A"));
        assert!(output.contains("B"));
    }

    // Test: SPC 函数
    #[test]
    fn test_spc_function() {
        let mut exec = Executor::new();
        
        let stmt = Statement::Print {
            items: vec![
                PrintItem::Expr(Expr::String("A".to_string())),
                PrintItem::Semicolon,
                PrintItem::Spc(Expr::Number(5.0)),
                PrintItem::Semicolon,
                PrintItem::Expr(Expr::String("B".to_string())),
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        let output = exec.get_output();
        // A + 5个空格 + B
        assert_eq!(output, "A     B\n");
    }

    // Requirement: GOSUB 和 RETURN 语句 - 子程序调用
    #[test]
    fn test_gosub_statement() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 500,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        let stmt = Statement::Gosub {
            line_number: Expr::Number(500.0),
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证跳转到子程序
        assert_eq!(exec.runtime().get_current_line(), Some(500));
        // 验证调用栈深度
        assert_eq!(exec.runtime().stack_depth(), 1);
    }

    // Requirement: GOSUB 和 RETURN 语句 - 子程序返回
    #[test]
    fn test_return_statement() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 500,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行并设置调用栈
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        exec.runtime_mut().push_gosub(20, 0).unwrap();
        exec.runtime_mut().set_execution_position(500, 0).unwrap();
        
        let stmt = Statement::Return;
        exec.execute_statement(&stmt).unwrap();
        
        // 验证返回到调用点
        assert_eq!(exec.runtime().get_current_line(), Some(20));
        // 验证调用栈已弹出
        assert_eq!(exec.runtime().stack_depth(), 0);
    }

    // Requirement: GOSUB 和 RETURN 语句 - 嵌套子程序
    #[test]
    fn test_nested_gosub() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 200,
            statements: vec![Statement::Rem],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // 第一次 GOSUB
        exec.execute_statement(&Statement::Gosub {
            line_number: Expr::Number(100.0),
        }).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 1);
        
        // 第二次 GOSUB（嵌套）
        exec.execute_statement(&Statement::Gosub {
            line_number: Expr::Number(200.0),
        }).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 2);
        assert_eq!(exec.runtime().get_current_line(), Some(200));
        
        // 第一次 RETURN
        exec.execute_statement(&Statement::Return).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 1);
        assert_eq!(exec.runtime().get_current_line(), Some(100));
        
        // 第二次 RETURN
        exec.execute_statement(&Statement::Return).unwrap();
        assert_eq!(exec.runtime().stack_depth(), 0);
        assert_eq!(exec.runtime().get_current_line(), Some(10));
    }

    // Requirement: INPUT 语句 - 基本输入
    #[test]
    fn test_input_basic() {
        let mut exec = Executor::new();
        
        // 设置输入回调
        exec.set_input_callback(|_| Some("42".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证输出提示符
        assert!(exec.get_output().contains("? "));
        
        // 验证变量赋值
        assert_eq!(exec.variables.get("A"), Value::Number(42.0));
    }

    // Requirement: INPUT 语句 - 带提示符的输入
    #[test]
    fn test_input_with_prompt() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("100".to_string()));
        
        let stmt = Statement::Input {
            prompt: Some("ENTER VALUE".to_string()),
            variables: vec![AssignTarget::Variable("X".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        // 验证提示符
        assert!(exec.get_output().contains("ENTER VALUE? "));
        assert_eq!(exec.variables.get("X"), Value::Number(100.0));
    }

    // Requirement: INPUT 语句 - 输入多个变量
    #[test]
    fn test_input_multiple_variables() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("10, 20, 30".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string()),
                AssignTarget::Variable("C".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(10.0));
        assert_eq!(exec.variables.get("B"), Value::Number(20.0));
        assert_eq!(exec.variables.get("C"), Value::Number(30.0));
    }

    // Requirement: INPUT 语句 - 字符串输入
    #[test]
    fn test_input_string() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("HELLO".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A$".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A$"), Value::String("HELLO".to_string()));
    }

    // Requirement: INPUT 语句 - 字符串带引号
    #[test]
    fn test_input_string_with_quotes() {
        let mut exec = Executor::new();
        
        exec.set_input_callback(|_| Some("\"HELLO, WORLD\"".to_string()));
        
        let stmt = Statement::Input {
            prompt: None,
            variables: vec![AssignTarget::Variable("A$".to_string())],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A$"), Value::String("HELLO, WORLD".to_string()));
    }

    // Requirement: DATA/READ 机制 - DATA 存储和 READ 读取
    #[test]
    fn test_data_read() {
        let mut exec = Executor::new();
        
        // 添加 DATA 值
        exec.add_data_value(DataValue::Number(1.0));
        exec.add_data_value(DataValue::Number(2.0));
        exec.add_data_value(DataValue::Number(3.0));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string()),
                AssignTarget::Variable("C".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(1.0));
        assert_eq!(exec.variables.get("B"), Value::Number(2.0));
        assert_eq!(exec.variables.get("C"), Value::Number(3.0));
    }

    // Requirement: DATA/READ 机制 - 混合数据类型
    #[test]
    fn test_data_read_mixed_types() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(42.0));
        exec.add_data_value(DataValue::String("HELLO".to_string()));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B$".to_string())
            ],
        };
        
        exec.execute_statement(&stmt).unwrap();
        
        assert_eq!(exec.variables.get("A"), Value::Number(42.0));
        assert_eq!(exec.variables.get("B$"), Value::String("HELLO".to_string()));
    }

    // Requirement: DATA/READ 机制 - OUT OF DATA 错误
    #[test]
    fn test_out_of_data_error() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(1.0));
        
        let stmt = Statement::Read {
            variables: vec![
                AssignTarget::Variable("A".to_string()),
                AssignTarget::Variable("B".to_string())
            ],
        };
        
        let result = exec.execute_statement(&stmt);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), BasicError::OutOfData));
    }

    // Requirement: RESTORE 数据指针 - RESTORE 重置到开头
    #[test]
    fn test_restore() {
        let mut exec = Executor::new();
        
        exec.add_data_value(DataValue::Number(1.0));
        exec.add_data_value(DataValue::Number(2.0));
        
        // 第一次 READ
        exec.execute_statement(&Statement::Read {
            variables: vec![AssignTarget::Variable("A".to_string())],
        }).unwrap();
        assert_eq!(exec.variables.get("A"), Value::Number(1.0));
        
        // RESTORE
        exec.execute_statement(&Statement::Restore {
            line_number: None,
        }).unwrap();
        
        // 第二次 READ（应该重新从头开始）
        exec.execute_statement(&Statement::Read {
            variables: vec![AssignTarget::Variable("B".to_string())],
        }).unwrap();
        assert_eq!(exec.variables.get("B"), Value::Number(1.0));
    }

    // Requirement: FOR...NEXT 循环 - 正向循环
    #[test]
    fn test_for_next_basic() {
        let mut exec = Executor::new();
        
        // 添加测试程序：FOR I=1 TO 3: PRINT I: NEXT I
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(1.0),
                    end: Expr::Number(3.0),
                    step: None,
                },
            ],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Next { var: Some("I".to_string()) }],
        });
        
        // 启动执行
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // 第一次循环：I=1
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(1.0),
            end: Expr::Number(3.0),
            step: None,
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
        
        // NEXT：I=2
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=3
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
        
        // NEXT：循环结束 (I递增到4但不再循环)
        exec.runtime_mut().set_execution_position(20, 0).unwrap();
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        // 循环已结束，变量值应该为循环后的值 4
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
    }

    // Requirement: FOR...NEXT 循环 - 步长为 2
    #[test]
    fn test_for_next_step() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(0.0),
                    end: Expr::Number(4.0),
                    step: Some(Expr::Number(2.0)),
                },
            ],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // FOR I=0 TO 4 STEP 2
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(0.0),
            end: Expr::Number(4.0),
            step: Some(Expr::Number(2.0)),
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(0.0));
        
        // NEXT：I=2
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=4
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(4.0));
        
        // NEXT：循环结束
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(4.0));
    }

    // Requirement: FOR...NEXT 循环 - 负步长
    #[test]
    fn test_for_next_negative_step() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(3.0),
                    end: Expr::Number(1.0),
                    step: Some(Expr::Number(-1.0)),
                },
            ],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // FOR I=3 TO 1 STEP -1
        exec.execute_statement(&Statement::For {
            var: "I".to_string(),
            start: Expr::Number(3.0),
            end: Expr::Number(1.0),
            step: Some(Expr::Number(-1.0)),
        }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(3.0));
        
        // NEXT：I=2
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(2.0));
        
        // NEXT：I=1
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
        
        // NEXT：循环结束
        exec.execute_statement(&Statement::Next { var: Some("I".to_string()) }).unwrap();
        assert_eq!(exec.variables.get("I"), Value::Number(1.0));
    }

    // Requirement: ON...GOTO - 基于表达式的跳转
    #[test]
    fn test_on_goto() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 200,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // ON 2 GOTO 100,200,300
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(2.0),
            targets: vec![100, 200, 300],
            is_gosub: false,
        }).unwrap();
        
        // 应该跳转到 200
        assert_eq!(exec.runtime().get_current_line(), Some(200));
    }

    // Requirement: ON...GOSUB - 基于表达式的子程序调用
    #[test]
    fn test_on_gosub() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 100,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        
        // ON 1 GOSUB 100,200
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(1.0),
            targets: vec![100, 200],
            is_gosub: true,
        }).unwrap();
        
        // 应该跳转到 100
        assert_eq!(exec.runtime().get_current_line(), Some(100));
        // 栈深度应该为 1
        assert_eq!(exec.runtime().stack_depth(), 1);
    }

    // Requirement: ON...GOTO - 值超出范围
    #[test]
    fn test_on_goto_out_of_range() {
        let mut exec = Executor::new();
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Rem],
        });
        
        exec.runtime_mut().start_execution(Some(10)).unwrap();
        let current_line = exec.runtime().get_current_line();
        
        // ON 5 GOTO 100,200  (5 超出范围)
        exec.execute_statement(&Statement::On {
            expr: Expr::Number(5.0),
            targets: vec![100, 200],
            is_gosub: false,
        }).unwrap();
        
        // 应该继续在当前行
        assert_eq!(exec.runtime().get_current_line(), current_line);
    }
    
    #[test]
    fn test_save_and_load() {
        use std::fs;
        
        let mut exec = Executor::new();
        
        // 添加一些程序行
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![Statement::Print {
                items: vec![PrintItem::Expr(Expr::String("HELLO".to_string()))],
            }],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![Statement::Let {
                target: AssignTarget::Variable("A".to_string()),
                value: Expr::Number(42.0),
            }],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 30,
            statements: vec![Statement::End],
        });
        
        // 保存程序到文件
        let filename = "test_program.bas";
        exec.execute_statement(&Statement::Save {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证文件存在
        assert!(fs::metadata(filename).is_ok());
        
        // 清空程序
        exec.runtime_mut().clear_program();
        assert_eq!(exec.runtime().line_count(), 0);
        
        // 加载程序
        exec.execute_statement(&Statement::Load {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证程序已加载
        assert_eq!(exec.runtime().line_count(), 3);
        assert!(exec.runtime().get_line(10).is_some());
        assert!(exec.runtime().get_line(20).is_some());
        assert!(exec.runtime().get_line(30).is_some());
        
        // 清理测试文件
        fs::remove_file(filename).ok();
    }
    
    #[test]
    fn test_save_empty_program() {
        let mut exec = Executor::new();
        
        // 尝试保存空程序应该失败
        let result = exec.execute_statement(&Statement::Save {
            filename: "empty.bas".to_string(),
        });
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_load_nonexistent_file() {
        let mut exec = Executor::new();
        
        // 尝试加载不存在的文件应该失败
        let result = exec.execute_statement(&Statement::Load {
            filename: "nonexistent.bas".to_string(),
        });
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_save_complex_program() {
        use std::fs;
        
        let mut exec = Executor::new();
        
        // 创建一个更复杂的程序
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 10,
            statements: vec![
                Statement::For {
                    var: "I".to_string(),
                    start: Expr::Number(1.0),
                    end: Expr::Number(10.0),
                    step: Some(Expr::Number(1.0)),
                },
            ],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 20,
            statements: vec![
                Statement::Print {
                    items: vec![PrintItem::Expr(Expr::Variable("I".to_string()))],
                },
            ],
        });
        
        exec.runtime_mut().add_line(ProgramLine {
            line_number: 30,
            statements: vec![Statement::Next { var: Some("I".to_string()) }],
        });
        
        // 保存并重新加载
        let filename = "test_complex.bas";
        exec.execute_statement(&Statement::Save {
            filename: filename.to_string(),
        }).unwrap();
        
        exec.runtime_mut().clear_program();
        
        exec.execute_statement(&Statement::Load {
            filename: filename.to_string(),
        }).unwrap();
        
        // 验证程序结构
        assert_eq!(exec.runtime().line_count(), 3);
        
        // 清理
        fs::remove_file(filename).ok();
    }
}


```

让我测试一下这个改动：

现在让我用用户最初提供的场景进行最终测试：

完美！问题已经完全修复了。让我总结一下所做的更改：

## 修复内容

### 问题
`LIST` 命令显示的程序内容不完整，只显示简化的语句信息，例如：
- `10 print 123` 显示为 `10 PRINT 1`
- `20 for i=1 to 3` 显示为 `20 FOR`

### 根本原因
`main.rs` 中的 `format_statement()` 函数是一个简化版本，仅用于调试，没有正确序列化完整的语句内容。

### 解决方案

1. **将 `executor.rs` 中的序列化函数改为公开**：
   - `serialize_program_line()` - 序列化整个程序行
   - `serialize_statement()` - 序列化语句
   - `serialize_expr()` - 序列化表达式
   - `serialize_print_item()` - 序列化打印项
   - `serialize_assign_target()` - 序列化赋值目标
   - `serialize_then_part()` - 序列化 THEN 部分

2. **修改 `main.rs` 中的 `list_program()` 函数**：
   - 使用完整的序列化函数替代简化的 `format_statement()`
   - 删除不再需要的 `format_statement()` 函数

3. **优化 PRINT 语句的序列化**：
   - 简化代码逻辑，去除重复

### 测试结果

现在 `LIST` 命令能正确显示所有语句类型：
- ✅ PRINT 语句显示完整内容和分隔符
- ✅ FOR 循环显示变量、范围和步长
- ✅ IF...THEN 显示完整条件和动作
- ✅ 数组、INPUT、DATA/READ 等都能正确显示
- ✅ SAVE/LOAD 功能也正常工作（使用同样的序列化函数）

---

**User**

交互式运行时，只在最开始打印了Ready. 而不是每次回车都再打印Ready. 这跟6502经典实现一致吗？

---

**Cursor**

你提出了一个很好的问题！让我查看一下经典的 6502 BASIC 实现的行为。

在经典的 BASIC 实现中（如 Commodore 64 BASIC、Apple II BASIC、Microsoft BASIC），通常的行为是：

1. **启动时**显示 "READY."
2. **每次执行完直接命令后**再次显示 "READY."
3. **输入程序行（带行号）后**不显示 READY，只是静默接受
4. **RUN 程序执行完成后**显示 "READY."

让我检查一下当前的实现：

你说得对！经典的 BASIC 实现（如 Commodore 64、Apple II）确实会在每次执行完直接命令后显示 "READY." 提示符。让我修改代码以符合这个经典行为：

```rust
use basic_m6502::{
    BasicError, Executor, Parser, Result, Statement, Tokenizer,
};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() -> Result<()> {
    println!("Microsoft BASIC 6502 Interpreter (Rust Edition)");
    println!();

    // 创建执行器
    let mut executor = Executor::new();

    // 创建 rustyline 编辑器（带历史记录）
    let mut rl = DefaultEditor::new().map_err(|e| {
        BasicError::SyntaxError(format!("Failed to initialize editor: {}", e))
    })?;

    // 加载历史记录（如果存在）
    let history_file = ".basic_history";
    let _ = rl.load_history(history_file);

    // 初始提示符
    println!("Ready.");

    // REPL 主循环
    loop {
        // 读取一行
        let readline = rl.readline("");
        
        match readline {
            Ok(line) => {
                // 添加到历史记录
                rl.add_history_entry(line.as_str()).ok();
                
                // 处理输入行
                match process_line(&mut executor, &line) {
                    Ok(should_print_ready) => {
                        // 执行成功后，根据返回值决定是否打印 Ready
                        if should_print_ready {
                            println!();
                            println!("Ready.");
                        }
                    }
                    Err(e) => {
                        // 输出错误
                        match e {
                            BasicError::CantContinue => {
                                // 正常退出信号，不输出错误
                            }
                            _ => {
                                eprintln!("?{}", format_error(&e));
                                println!();
                                println!("Ready.");
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C 中断
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("\n?BREAK IN {}", line);
                } else {
                    println!("\n^C");
                }
                executor.runtime_mut().interrupt();
                println!();
                println!("Ready.");
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    // 保存历史记录
    rl.save_history(history_file).ok();
    
    Ok(())
}

/// 处理一行输入
fn process_line(executor: &mut Executor, line: &str) -> Result<()> {
    let line = line.trim();
    
    // 空行
    if line.is_empty() {
        return Ok(());
    }
    
    // 词法分析
    let mut tokenizer = Tokenizer::new(line);
    let tokens = tokenizer.tokenize_line()?;
    
    // 语法分析
    let mut parser = Parser::new(tokens);
    
    // 解析程序行
    if let Some(program_line) = parser.parse_line()? {
        // 检查是否是带行号的程序行
        if program_line.line_number > 0 {
            // 添加或删除程序行
            if program_line.statements.is_empty() {
                // 空行：删除该行
                executor.runtime_mut().delete_line(program_line.line_number);
            } else {
                // 非空行：添加到程序
                executor.runtime_mut().add_line(program_line);
            }
            return Ok(());
        }
        
        // 直接模式：执行语句
        if program_line.statements.is_empty() {
            return Ok(());
        }
        let statement = program_line.statements.into_iter().next().unwrap();
        
        // 特殊命令处理
        match &statement {
            Statement::List { start, end } => {
                list_program(executor, *start, *end);
                Ok(())
            }
            Statement::Run { line_number } => {
                run_program(executor, *line_number)
            }
            Statement::New => {
                executor.execute_statement(&statement)?;
                println!("New program started.");
                Ok(())
            }
            Statement::End => {
                executor.execute_statement(&statement)?;
                Ok(())
            }
            Statement::Stop => {
                executor.execute_statement(&statement)?;
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("?BREAK IN {}", line);
                }
                Ok(())
            }
            Statement::Cont => {
                continue_program(executor)
            }
            _ => {
                // 其他语句：直接执行
                executor.execute_statement(&statement)
            }
        }
    } else {
        Ok(())
    }
}

/// 列出程序
fn list_program(executor: &Executor, start: Option<u16>, end: Option<u16>) {
    let lines = executor.runtime().get_program_lines(start, end);
    
    if lines.is_empty() {
        if start.is_some() || end.is_some() {
            // 有范围但没有内容，不输出
            return;
        }
        println!("No program loaded.");
        return;
    }
    
    for line in lines {
        // 使用完整的序列化函数
        println!("{}", Executor::serialize_program_line(line));
    }
}

/// 运行程序
fn run_program(executor: &mut Executor, line_number: Option<u16>) -> Result<()> {
    // 启动执行
    executor.runtime_mut().start_execution(line_number)?;
    
    // 执行循环
    loop {
        let stmt = match executor.runtime_mut().get_next_statement() {
            Some(s) => s,
            None => break, // 程序结束
        };
        
        // 执行语句
        if let Err(e) = executor.execute_statement(&stmt) {
            // 输出错误和行号
            if let Some(line) = executor.runtime().get_current_line() {
                eprintln!("?{} IN {}", format_error(&e), line);
            } else {
                eprintln!("?{}", format_error(&e));
            }
            return Err(e);
        }
        
        // 检查是否应该停止
        if executor.runtime().is_stopped() {
            break;
        }
    }
    
    Ok(())
}

/// 继续执行程序
fn continue_program(executor: &mut Executor) -> Result<()> {
    if !executor.runtime().can_continue() {
        println!("?CAN'T CONTINUE");
        return Err(BasicError::CantContinue);
    }
    
    // 从暂停点恢复执行
    executor.runtime_mut().continue_execution()?;
    run_program(executor, None)
}

/// 格式化错误信息
fn format_error(error: &BasicError) -> String {
    match error {
        BasicError::SyntaxError(_) => "SYNTAX ERROR".to_string(),
        BasicError::DivisionByZero => "DIVISION BY ZERO".to_string(),
        BasicError::TypeMismatch(_) => "TYPE MISMATCH".to_string(),
        BasicError::UndefinedLine(n) => format!("UNDEFINED LINE {}", n),
        BasicError::UndefinedVariable(v) => format!("UNDEFINED VARIABLE {}", v),
        BasicError::SubscriptOutOfRange(_) => "SUBSCRIPT OUT OF RANGE".to_string(),
        BasicError::OutOfData => "OUT OF DATA".to_string(),
        BasicError::ReturnWithoutGosub => "RETURN WITHOUT GOSUB".to_string(),
        BasicError::NextWithoutFor(_) => "NEXT WITHOUT FOR".to_string(),
        BasicError::CantContinue => "CAN'T CONTINUE".to_string(),
        _ => format!("{:?}", error),
    }
}


```

现在修改 `process_line` 函数，让它返回一个布尔值来指示是否应该打印 "Ready."：

```rust
use basic_m6502::{
    BasicError, Executor, Parser, Result, Statement, Tokenizer,
};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() -> Result<()> {
    println!("Microsoft BASIC 6502 Interpreter (Rust Edition)");
    println!();

    // 创建执行器
    let mut executor = Executor::new();

    // 创建 rustyline 编辑器（带历史记录）
    let mut rl = DefaultEditor::new().map_err(|e| {
        BasicError::SyntaxError(format!("Failed to initialize editor: {}", e))
    })?;

    // 加载历史记录（如果存在）
    let history_file = ".basic_history";
    let _ = rl.load_history(history_file);

    // 初始提示符
    println!("Ready.");

    // REPL 主循环
    loop {
        // 读取一行
        let readline = rl.readline("");
        
        match readline {
            Ok(line) => {
                // 添加到历史记录
                rl.add_history_entry(line.as_str()).ok();
                
                // 处理输入行
                match process_line(&mut executor, &line) {
                    Ok(should_print_ready) => {
                        // 执行成功后，根据返回值决定是否打印 Ready
                        if should_print_ready {
                            println!();
                            println!("Ready.");
                        }
                    }
                    Err(e) => {
                        // 输出错误
                        match e {
                            BasicError::CantContinue => {
                                // 正常退出信号，不输出错误
                            }
                            _ => {
                                eprintln!("?{}", format_error(&e));
                                println!();
                                println!("Ready.");
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C 中断
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("\n?BREAK IN {}", line);
                } else {
                    println!("\n^C");
                }
                executor.runtime_mut().interrupt();
                println!();
                println!("Ready.");
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    // 保存历史记录
    rl.save_history(history_file).ok();
    
    Ok(())
}

/// 处理一行输入
/// 返回值：Ok(bool) - true 表示应该打印 "Ready.", false 表示不打印
fn process_line(executor: &mut Executor, line: &str) -> Result<bool> {
    let line = line.trim();
    
    // 空行
    if line.is_empty() {
        return Ok(false);
    }
    
    // 词法分析
    let mut tokenizer = Tokenizer::new(line);
    let tokens = tokenizer.tokenize_line()?;
    
    // 语法分析
    let mut parser = Parser::new(tokens);
    
    // 解析程序行
    if let Some(program_line) = parser.parse_line()? {
        // 检查是否是带行号的程序行
        if program_line.line_number > 0 {
            // 添加或删除程序行
            if program_line.statements.is_empty() {
                // 空行：删除该行
                executor.runtime_mut().delete_line(program_line.line_number);
            } else {
                // 非空行：添加到程序
                executor.runtime_mut().add_line(program_line);
            }
            // 程序行输入后不打印 Ready
            return Ok(false);
        }
        
        // 直接模式：执行语句
        if program_line.statements.is_empty() {
            return Ok(false);
        }
        let statement = program_line.statements.into_iter().next().unwrap();
        
        // 特殊命令处理
        match &statement {
            Statement::List { start, end } => {
                list_program(executor, *start, *end);
                Ok(true)
            }
            Statement::Run { line_number } => {
                run_program(executor, *line_number)?;
                Ok(true)
            }
            Statement::New => {
                executor.execute_statement(&statement)?;
                println!("New program started.");
                Ok(true)
            }
            Statement::End => {
                executor.execute_statement(&statement)?;
                Ok(true)
            }
            Statement::Stop => {
                executor.execute_statement(&statement)?;
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("?BREAK IN {}", line);
                }
                Ok(true)
            }
            Statement::Cont => {
                continue_program(executor)?;
                Ok(true)
            }
            _ => {
                // 其他语句：直接执行
                executor.execute_statement(&statement)?;
                Ok(true)
            }
        }
    } else {
        Ok(false)
    }
}

/// 列出程序
fn list_program(executor: &Executor, start: Option<u16>, end: Option<u16>) {
    let lines = executor.runtime().get_program_lines(start, end);
    
    if lines.is_empty() {
        if start.is_some() || end.is_some() {
            // 有范围但没有内容，不输出
            return;
        }
        println!("No program loaded.");
        return;
    }
    
    for line in lines {
        // 使用完整的序列化函数
        println!("{}", Executor::serialize_program_line(line));
    }
}

/// 运行程序
fn run_program(executor: &mut Executor, line_number: Option<u16>) -> Result<()> {
    // 启动执行
    executor.runtime_mut().start_execution(line_number)?;
    
    // 执行循环
    loop {
        let stmt = match executor.runtime_mut().get_next_statement() {
            Some(s) => s,
            None => break, // 程序结束
        };
        
        // 执行语句
        if let Err(e) = executor.execute_statement(&stmt) {
            // 输出错误和行号
            if let Some(line) = executor.runtime().get_current_line() {
                eprintln!("?{} IN {}", format_error(&e), line);
            } else {
                eprintln!("?{}", format_error(&e));
            }
            return Err(e);
        }
        
        // 检查是否应该停止
        if executor.runtime().is_stopped() {
            break;
        }
    }
    
    Ok(())
}

/// 继续执行程序
fn continue_program(executor: &mut Executor) -> Result<()> {
    if !executor.runtime().can_continue() {
        println!("?CAN'T CONTINUE");
        return Err(BasicError::CantContinue);
    }
    
    // 从暂停点恢复执行
    executor.runtime_mut().continue_execution()?;
    run_program(executor, None)
}

/// 格式化错误信息
fn format_error(error: &BasicError) -> String {
    match error {
        BasicError::SyntaxError(_) => "SYNTAX ERROR".to_string(),
        BasicError::DivisionByZero => "DIVISION BY ZERO".to_string(),
        BasicError::TypeMismatch(_) => "TYPE MISMATCH".to_string(),
        BasicError::UndefinedLine(n) => format!("UNDEFINED LINE {}", n),
        BasicError::UndefinedVariable(v) => format!("UNDEFINED VARIABLE {}", v),
        BasicError::SubscriptOutOfRange(_) => "SUBSCRIPT OUT OF RANGE".to_string(),
        BasicError::OutOfData => "OUT OF DATA".to_string(),
        BasicError::ReturnWithoutGosub => "RETURN WITHOUT GOSUB".to_string(),
        BasicError::NextWithoutFor(_) => "NEXT WITHOUT FOR".to_string(),
        BasicError::CantContinue => "CAN'T CONTINUE".to_string(),
        _ => format!("{:?}", error),
    }
}


```

现在让我编译并测试这些修改：

太好了！编译成功。现在让我测试新的行为，看看是否符合经典 BASIC 的风格：

很好！现在让我测试程序行输入和 RUN 的行为：

完美！让我再测试一下空行和错误处理：

让我重新测试错误处理：

让我测试一下程序运行时的错误处理：

看起来错误被打印了两次。让我检查一下 `run_program` 函数的错误处理：

我发现问题了！`run_program` 在打印错误后又返回了错误，导致 `main` 循环中再次打印。让我修复这个问题：

```rust
use basic_m6502::{
    BasicError, Executor, Parser, Result, Statement, Tokenizer,
};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() -> Result<()> {
    println!("Microsoft BASIC 6502 Interpreter (Rust Edition)");
    println!();

    // 创建执行器
    let mut executor = Executor::new();

    // 创建 rustyline 编辑器（带历史记录）
    let mut rl = DefaultEditor::new().map_err(|e| {
        BasicError::SyntaxError(format!("Failed to initialize editor: {}", e))
    })?;

    // 加载历史记录（如果存在）
    let history_file = ".basic_history";
    let _ = rl.load_history(history_file);

    // 初始提示符
    println!("Ready.");

    // REPL 主循环
    loop {
        // 读取一行
        let readline = rl.readline("");
        
        match readline {
            Ok(line) => {
                // 添加到历史记录
                rl.add_history_entry(line.as_str()).ok();
                
                // 处理输入行
                match process_line(&mut executor, &line) {
                    Ok(should_print_ready) => {
                        // 执行成功后，根据返回值决定是否打印 Ready
                        if should_print_ready {
                            println!();
                            println!("Ready.");
                        }
                    }
                    Err(e) => {
                        // 输出错误
                        match e {
                            BasicError::CantContinue => {
                                // 正常退出信号，不输出错误
                            }
                            _ => {
                                eprintln!("?{}", format_error(&e));
                                println!();
                                println!("Ready.");
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C 中断
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("\n?BREAK IN {}", line);
                } else {
                    println!("\n^C");
                }
                executor.runtime_mut().interrupt();
                println!();
                println!("Ready.");
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    // 保存历史记录
    rl.save_history(history_file).ok();
    
    Ok(())
}

/// 处理一行输入
/// 返回值：Ok(bool) - true 表示应该打印 "Ready.", false 表示不打印
fn process_line(executor: &mut Executor, line: &str) -> Result<bool> {
    let line = line.trim();
    
    // 空行
    if line.is_empty() {
        return Ok(false);
    }
    
    // 词法分析
    let mut tokenizer = Tokenizer::new(line);
    let tokens = tokenizer.tokenize_line()?;
    
    // 语法分析
    let mut parser = Parser::new(tokens);
    
    // 解析程序行
    if let Some(program_line) = parser.parse_line()? {
        // 检查是否是带行号的程序行
        if program_line.line_number > 0 {
            // 添加或删除程序行
            if program_line.statements.is_empty() {
                // 空行：删除该行
                executor.runtime_mut().delete_line(program_line.line_number);
            } else {
                // 非空行：添加到程序
                executor.runtime_mut().add_line(program_line);
            }
            // 程序行输入后不打印 Ready
            return Ok(false);
        }
        
        // 直接模式：执行语句
        if program_line.statements.is_empty() {
            return Ok(false);
        }
        let statement = program_line.statements.into_iter().next().unwrap();
        
        // 特殊命令处理
        match &statement {
            Statement::List { start, end } => {
                list_program(executor, *start, *end);
                Ok(true)
            }
            Statement::Run { line_number } => {
                run_program(executor, *line_number)?;
                Ok(true)
            }
            Statement::New => {
                executor.execute_statement(&statement)?;
                println!("New program started.");
                Ok(true)
            }
            Statement::End => {
                executor.execute_statement(&statement)?;
                Ok(true)
            }
            Statement::Stop => {
                executor.execute_statement(&statement)?;
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("?BREAK IN {}", line);
                }
                Ok(true)
            }
            Statement::Cont => {
                continue_program(executor)?;
                Ok(true)
            }
            _ => {
                // 其他语句：直接执行
                executor.execute_statement(&statement)?;
                Ok(true)
            }
        }
    } else {
        Ok(false)
    }
}

/// 列出程序
fn list_program(executor: &Executor, start: Option<u16>, end: Option<u16>) {
    let lines = executor.runtime().get_program_lines(start, end);
    
    if lines.is_empty() {
        if start.is_some() || end.is_some() {
            // 有范围但没有内容，不输出
            return;
        }
        println!("No program loaded.");
        return;
    }
    
    for line in lines {
        // 使用完整的序列化函数
        println!("{}", Executor::serialize_program_line(line));
    }
}

/// 运行程序
fn run_program(executor: &mut Executor, line_number: Option<u16>) -> Result<()> {
    // 启动执行
    executor.runtime_mut().start_execution(line_number)?;
    
    // 执行循环
    loop {
        let stmt = match executor.runtime_mut().get_next_statement() {
            Some(s) => s,
            None => break, // 程序结束
        };
        
        // 执行语句
        if let Err(e) = executor.execute_statement(&stmt) {
            // 输出错误和行号
            if let Some(line) = executor.runtime().get_current_line() {
                eprintln!("?{} IN {}", format_error(&e), line);
            } else {
                eprintln!("?{}", format_error(&e));
            }
            // 错误已经打印，不再向上传播，直接返回 Ok
            return Ok(());
        }
        
        // 检查是否应该停止
        if executor.runtime().is_stopped() {
            break;
        }
    }
    
    Ok(())
}

/// 继续执行程序
fn continue_program(executor: &mut Executor) -> Result<()> {
    if !executor.runtime().can_continue() {
        println!("?CAN'T CONTINUE");
        return Err(BasicError::CantContinue);
    }
    
    // 从暂停点恢复执行
    executor.runtime_mut().continue_execution()?;
    run_program(executor, None)
}

/// 格式化错误信息
fn format_error(error: &BasicError) -> String {
    match error {
        BasicError::SyntaxError(_) => "SYNTAX ERROR".to_string(),
        BasicError::DivisionByZero => "DIVISION BY ZERO".to_string(),
        BasicError::TypeMismatch(_) => "TYPE MISMATCH".to_string(),
        BasicError::UndefinedLine(n) => format!("UNDEFINED LINE {}", n),
        BasicError::UndefinedVariable(v) => format!("UNDEFINED VARIABLE {}", v),
        BasicError::SubscriptOutOfRange(_) => "SUBSCRIPT OUT OF RANGE".to_string(),
        BasicError::OutOfData => "OUT OF DATA".to_string(),
        BasicError::ReturnWithoutGosub => "RETURN WITHOUT GOSUB".to_string(),
        BasicError::NextWithoutFor(_) => "NEXT WITHOUT FOR".to_string(),
        BasicError::CantContinue => "CAN'T CONTINUE".to_string(),
        _ => format!("{:?}", error),
    }
}


```

让我也检查并修复 `continue_program` 函数：

```rust
use basic_m6502::{
    BasicError, Executor, Parser, Result, Statement, Tokenizer,
};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() -> Result<()> {
    println!("Microsoft BASIC 6502 Interpreter (Rust Edition)");
    println!();

    // 创建执行器
    let mut executor = Executor::new();

    // 创建 rustyline 编辑器（带历史记录）
    let mut rl = DefaultEditor::new().map_err(|e| {
        BasicError::SyntaxError(format!("Failed to initialize editor: {}", e))
    })?;

    // 加载历史记录（如果存在）
    let history_file = ".basic_history";
    let _ = rl.load_history(history_file);

    // 初始提示符
    println!("Ready.");

    // REPL 主循环
    loop {
        // 读取一行
        let readline = rl.readline("");
        
        match readline {
            Ok(line) => {
                // 添加到历史记录
                rl.add_history_entry(line.as_str()).ok();
                
                // 处理输入行
                match process_line(&mut executor, &line) {
                    Ok(should_print_ready) => {
                        // 执行成功后，根据返回值决定是否打印 Ready
                        if should_print_ready {
                            println!();
                            println!("Ready.");
                        }
                    }
                    Err(e) => {
                        // 输出错误
                        match e {
                            BasicError::CantContinue => {
                                // 正常退出信号，不输出错误
                            }
                            _ => {
                                eprintln!("?{}", format_error(&e));
                                println!();
                                println!("Ready.");
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C 中断
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("\n?BREAK IN {}", line);
                } else {
                    println!("\n^C");
                }
                executor.runtime_mut().interrupt();
                println!();
                println!("Ready.");
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    // 保存历史记录
    rl.save_history(history_file).ok();
    
    Ok(())
}

/// 处理一行输入
/// 返回值：Ok(bool) - true 表示应该打印 "Ready.", false 表示不打印
fn process_line(executor: &mut Executor, line: &str) -> Result<bool> {
    let line = line.trim();
    
    // 空行
    if line.is_empty() {
        return Ok(false);
    }
    
    // 词法分析
    let mut tokenizer = Tokenizer::new(line);
    let tokens = tokenizer.tokenize_line()?;
    
    // 语法分析
    let mut parser = Parser::new(tokens);
    
    // 解析程序行
    if let Some(program_line) = parser.parse_line()? {
        // 检查是否是带行号的程序行
        if program_line.line_number > 0 {
            // 添加或删除程序行
            if program_line.statements.is_empty() {
                // 空行：删除该行
                executor.runtime_mut().delete_line(program_line.line_number);
            } else {
                // 非空行：添加到程序
                executor.runtime_mut().add_line(program_line);
            }
            // 程序行输入后不打印 Ready
            return Ok(false);
        }
        
        // 直接模式：执行语句
        if program_line.statements.is_empty() {
            return Ok(false);
        }
        let statement = program_line.statements.into_iter().next().unwrap();
        
        // 特殊命令处理
        match &statement {
            Statement::List { start, end } => {
                list_program(executor, *start, *end);
                Ok(true)
            }
            Statement::Run { line_number } => {
                run_program(executor, *line_number)?;
                Ok(true)
            }
            Statement::New => {
                executor.execute_statement(&statement)?;
                println!("New program started.");
                Ok(true)
            }
            Statement::End => {
                executor.execute_statement(&statement)?;
                Ok(true)
            }
            Statement::Stop => {
                executor.execute_statement(&statement)?;
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("?BREAK IN {}", line);
                }
                Ok(true)
            }
            Statement::Cont => {
                continue_program(executor)?;
                Ok(true)
            }
            _ => {
                // 其他语句：直接执行
                executor.execute_statement(&statement)?;
                Ok(true)
            }
        }
    } else {
        Ok(false)
    }
}

/// 列出程序
fn list_program(executor: &Executor, start: Option<u16>, end: Option<u16>) {
    let lines = executor.runtime().get_program_lines(start, end);
    
    if lines.is_empty() {
        if start.is_some() || end.is_some() {
            // 有范围但没有内容，不输出
            return;
        }
        println!("No program loaded.");
        return;
    }
    
    for line in lines {
        // 使用完整的序列化函数
        println!("{}", Executor::serialize_program_line(line));
    }
}

/// 运行程序
fn run_program(executor: &mut Executor, line_number: Option<u16>) -> Result<()> {
    // 启动执行
    executor.runtime_mut().start_execution(line_number)?;
    
    // 执行循环
    loop {
        let stmt = match executor.runtime_mut().get_next_statement() {
            Some(s) => s,
            None => break, // 程序结束
        };
        
        // 执行语句
        if let Err(e) = executor.execute_statement(&stmt) {
            // 输出错误和行号
            if let Some(line) = executor.runtime().get_current_line() {
                eprintln!("?{} IN {}", format_error(&e), line);
            } else {
                eprintln!("?{}", format_error(&e));
            }
            // 错误已经打印，不再向上传播，直接返回 Ok
            return Ok(());
        }
        
        // 检查是否应该停止
        if executor.runtime().is_stopped() {
            break;
        }
    }
    
    Ok(())
}

/// 继续执行程序
fn continue_program(executor: &mut Executor) -> Result<()> {
    if !executor.runtime().can_continue() {
        println!("?CAN'T CONTINUE");
        // 错误已经打印，返回 Ok 以便显示 Ready
        return Ok(());
    }
    
    // 从暂停点恢复执行
    executor.runtime_mut().continue_execution()?;
    run_program(executor, None)
}

/// 格式化错误信息
fn format_error(error: &BasicError) -> String {
    match error {
        BasicError::SyntaxError(_) => "SYNTAX ERROR".to_string(),
        BasicError::DivisionByZero => "DIVISION BY ZERO".to_string(),
        BasicError::TypeMismatch(_) => "TYPE MISMATCH".to_string(),
        BasicError::UndefinedLine(n) => format!("UNDEFINED LINE {}", n),
        BasicError::UndefinedVariable(v) => format!("UNDEFINED VARIABLE {}", v),
        BasicError::SubscriptOutOfRange(_) => "SUBSCRIPT OUT OF RANGE".to_string(),
        BasicError::OutOfData => "OUT OF DATA".to_string(),
        BasicError::ReturnWithoutGosub => "RETURN WITHOUT GOSUB".to_string(),
        BasicError::NextWithoutFor(_) => "NEXT WITHOUT FOR".to_string(),
        BasicError::CantContinue => "CAN'T CONTINUE".to_string(),
        _ => format!("{:?}", error),
    }
}


```

现在编译并测试这些修改：

完美！错误不再重复打印了。让我进行更全面的测试：

优秀！让我最后测试一下空行和各种命令：

完美！现在让我创建一份总结文档说明所做的改进。

## 修改总结

你的观察非常敏锐！我修改了交互模式的提示符行为，使其符合经典 6502 BASIC（如 Commodore 64 BASIC）的标准：

### 经典 BASIC 的 "READY." 提示符规则

1. **启动时**：显示一次 "Ready."
2. **执行直接命令后**：再次显示 "Ready."（如 PRINT, LIST, RUN 等）
3. **输入程序行后**（带行号）：不显示 "Ready."，静默接受
4. **空行**：不显示 "Ready."
5. **错误后**：显示 "Ready."，准备接受新命令
6. **程序运行结束后**：显示 "Ready."

### 实现的改进

1. **修改 `main()` 函数**：
   - 将提示符打印移到 REPL 循环内
   - 根据命令执行结果决定是否打印 "Ready."

2. **修改 `process_line()` 函数**：
   - 返回值改为 `Result<bool>`，指示是否应该打印 "Ready."
   - 程序行输入（带行号）返回 `false`
   - 直接命令执行返回 `true`
   - 空行返回 `false`

3. **修复错误处理**：
   - `run_program()` 中的错误只打印一次，不再向上传播
   - `continue_program()` 中的错误也只打印一次
   - 错误后显示 "Ready." 提示符

### 测试验证

所有场景都符合经典 BASIC 行为：
- ✅ 直接命令后显示 "Ready."
- ✅ 程序行输入后不显示
- ✅ RUN 后显示 "Ready."
- ✅ 空行不显示
- ✅ 错误后显示 "Ready."

这样的交互体验与 Commodore 64、Apple II 等经典平台的 BASIC 解释器保持一致！

---

**User**

貌似Ready.后面多了一个回车，不是应该在同一行输入语句吗？如下：cargo run
warning: variable does not need to be mutable
   --> src/parser.rs:668:13
    |
668 |         let mut left = self.parse_additive_expression()?;
    |             ----^^^^
    |             |
    |             help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` on by default

warning: `basic_m6502` (lib) generated 1 warning (run `cargo fix --lib -p basic_m6502` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/basic`
Microsoft BASIC 6502 Interpreter (Rust Edition)

Ready.
print 123
 123

Ready.


---

**Cursor**

```rust
if should_print_ready {
    println!();  // 这里打印了一个空行
    println!("Ready.");
}
```

```plaintext
命令输出
Ready.
用户输入
```

```plaintext
命令输出

Ready.
用户输入
```

你说得对！`Ready.` 前面不应该有额外的空行。经典 BASIC 的格式应该是：

```
命令输出
Ready.
用户输入
```

而不是当前的：
```
命令输出

Ready.
用户输入
```

让我修复这个问题：

```plaintext
命令输出
Ready.
用户输入
```

```plaintext
命令输出

Ready.
用户输入
```

```rust
use basic_m6502::{
    BasicError, Executor, Parser, Result, Statement, Tokenizer,
};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() -> Result<()> {
    println!("Microsoft BASIC 6502 Interpreter (Rust Edition)");
    println!();

    // 创建执行器
    let mut executor = Executor::new();

    // 创建 rustyline 编辑器（带历史记录）
    let mut rl = DefaultEditor::new().map_err(|e| {
        BasicError::SyntaxError(format!("Failed to initialize editor: {}", e))
    })?;

    // 加载历史记录（如果存在）
    let history_file = ".basic_history";
    let _ = rl.load_history(history_file);

    // 初始提示符
    println!("Ready.");

    // REPL 主循环
    loop {
        // 读取一行
        let readline = rl.readline("");
        
        match readline {
            Ok(line) => {
                // 添加到历史记录
                rl.add_history_entry(line.as_str()).ok();
                
                // 处理输入行
                match process_line(&mut executor, &line) {
                    Ok(should_print_ready) => {
                        // 执行成功后，根据返回值决定是否打印 Ready
                        if should_print_ready {
                            println!("Ready.");
                        }
                    }
                    Err(e) => {
                        // 输出错误
                        match e {
                            BasicError::CantContinue => {
                                // 正常退出信号，不输出错误
                            }
                            _ => {
                                eprintln!("?{}", format_error(&e));
                                println!("Ready.");
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C 中断
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("\n?BREAK IN {}", line);
                } else {
                    println!("\n^C");
                }
                executor.runtime_mut().interrupt();
                println!();
                println!("Ready.");
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    // 保存历史记录
    rl.save_history(history_file).ok();
    
    Ok(())
}

/// 处理一行输入
/// 返回值：Ok(bool) - true 表示应该打印 "Ready.", false 表示不打印
fn process_line(executor: &mut Executor, line: &str) -> Result<bool> {
    let line = line.trim();
    
    // 空行
    if line.is_empty() {
        return Ok(false);
    }
    
    // 词法分析
    let mut tokenizer = Tokenizer::new(line);
    let tokens = tokenizer.tokenize_line()?;
    
    // 语法分析
    let mut parser = Parser::new(tokens);
    
    // 解析程序行
    if let Some(program_line) = parser.parse_line()? {
        // 检查是否是带行号的程序行
        if program_line.line_number > 0 {
            // 添加或删除程序行
            if program_line.statements.is_empty() {
                // 空行：删除该行
                executor.runtime_mut().delete_line(program_line.line_number);
            } else {
                // 非空行：添加到程序
                executor.runtime_mut().add_line(program_line);
            }
            // 程序行输入后不打印 Ready
            return Ok(false);
        }
        
        // 直接模式：执行语句
        if program_line.statements.is_empty() {
            return Ok(false);
        }
        let statement = program_line.statements.into_iter().next().unwrap();
        
        // 特殊命令处理
        match &statement {
            Statement::List { start, end } => {
                list_program(executor, *start, *end);
                Ok(true)
            }
            Statement::Run { line_number } => {
                run_program(executor, *line_number)?;
                Ok(true)
            }
            Statement::New => {
                executor.execute_statement(&statement)?;
                println!("New program started.");
                Ok(true)
            }
            Statement::End => {
                executor.execute_statement(&statement)?;
                Ok(true)
            }
            Statement::Stop => {
                executor.execute_statement(&statement)?;
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("?BREAK IN {}", line);
                }
                Ok(true)
            }
            Statement::Cont => {
                continue_program(executor)?;
                Ok(true)
            }
            _ => {
                // 其他语句：直接执行
                executor.execute_statement(&statement)?;
                Ok(true)
            }
        }
    } else {
        Ok(false)
    }
}

/// 列出程序
fn list_program(executor: &Executor, start: Option<u16>, end: Option<u16>) {
    let lines = executor.runtime().get_program_lines(start, end);
    
    if lines.is_empty() {
        if start.is_some() || end.is_some() {
            // 有范围但没有内容，不输出
            return;
        }
        println!("No program loaded.");
        return;
    }
    
    for line in lines {
        // 使用完整的序列化函数
        println!("{}", Executor::serialize_program_line(line));
    }
}

/// 运行程序
fn run_program(executor: &mut Executor, line_number: Option<u16>) -> Result<()> {
    // 启动执行
    executor.runtime_mut().start_execution(line_number)?;
    
    // 执行循环
    loop {
        let stmt = match executor.runtime_mut().get_next_statement() {
            Some(s) => s,
            None => break, // 程序结束
        };
        
        // 执行语句
        if let Err(e) = executor.execute_statement(&stmt) {
            // 输出错误和行号
            if let Some(line) = executor.runtime().get_current_line() {
                eprintln!("?{} IN {}", format_error(&e), line);
            } else {
                eprintln!("?{}", format_error(&e));
            }
            // 错误已经打印，不再向上传播，直接返回 Ok
            return Ok(());
        }
        
        // 检查是否应该停止
        if executor.runtime().is_stopped() {
            break;
        }
    }
    
    Ok(())
}

/// 继续执行程序
fn continue_program(executor: &mut Executor) -> Result<()> {
    if !executor.runtime().can_continue() {
        println!("?CAN'T CONTINUE");
        // 错误已经打印，返回 Ok 以便显示 Ready
        return Ok(());
    }
    
    // 从暂停点恢复执行
    executor.runtime_mut().continue_execution()?;
    run_program(executor, None)
}

/// 格式化错误信息
fn format_error(error: &BasicError) -> String {
    match error {
        BasicError::SyntaxError(_) => "SYNTAX ERROR".to_string(),
        BasicError::DivisionByZero => "DIVISION BY ZERO".to_string(),
        BasicError::TypeMismatch(_) => "TYPE MISMATCH".to_string(),
        BasicError::UndefinedLine(n) => format!("UNDEFINED LINE {}", n),
        BasicError::UndefinedVariable(v) => format!("UNDEFINED VARIABLE {}", v),
        BasicError::SubscriptOutOfRange(_) => "SUBSCRIPT OUT OF RANGE".to_string(),
        BasicError::OutOfData => "OUT OF DATA".to_string(),
        BasicError::ReturnWithoutGosub => "RETURN WITHOUT GOSUB".to_string(),
        BasicError::NextWithoutFor(_) => "NEXT WITHOUT FOR".to_string(),
        BasicError::CantContinue => "CAN'T CONTINUE".to_string(),
        _ => format!("{:?}", error),
    }
}


```

```rust
use basic_m6502::{
    BasicError, Executor, Parser, Result, Statement, Tokenizer,
};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() -> Result<()> {
    println!("Microsoft BASIC 6502 Interpreter (Rust Edition)");
    println!();

    // 创建执行器
    let mut executor = Executor::new();

    // 创建 rustyline 编辑器（带历史记录）
    let mut rl = DefaultEditor::new().map_err(|e| {
        BasicError::SyntaxError(format!("Failed to initialize editor: {}", e))
    })?;

    // 加载历史记录（如果存在）
    let history_file = ".basic_history";
    let _ = rl.load_history(history_file);

    // 初始提示符
    println!("Ready.");

    // REPL 主循环
    loop {
        // 读取一行
        let readline = rl.readline("");
        
        match readline {
            Ok(line) => {
                // 添加到历史记录
                rl.add_history_entry(line.as_str()).ok();
                
                // 处理输入行
                match process_line(&mut executor, &line) {
                    Ok(should_print_ready) => {
                        // 执行成功后，根据返回值决定是否打印 Ready
                        if should_print_ready {
                            println!("Ready.");
                        }
                    }
                    Err(e) => {
                        // 输出错误
                        match e {
                            BasicError::CantContinue => {
                                // 正常退出信号，不输出错误
                            }
                            _ => {
                                eprintln!("?{}", format_error(&e));
                                println!("Ready.");
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C 中断
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("\n?BREAK IN {}", line);
                } else {
                    println!("\n^C");
                }
                executor.runtime_mut().interrupt();
                println!("Ready.");
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    // 保存历史记录
    rl.save_history(history_file).ok();
    
    Ok(())
}

/// 处理一行输入
/// 返回值：Ok(bool) - true 表示应该打印 "Ready.", false 表示不打印
fn process_line(executor: &mut Executor, line: &str) -> Result<bool> {
    let line = line.trim();
    
    // 空行
    if line.is_empty() {
        return Ok(false);
    }
    
    // 词法分析
    let mut tokenizer = Tokenizer::new(line);
    let tokens = tokenizer.tokenize_line()?;
    
    // 语法分析
    let mut parser = Parser::new(tokens);
    
    // 解析程序行
    if let Some(program_line) = parser.parse_line()? {
        // 检查是否是带行号的程序行
        if program_line.line_number > 0 {
            // 添加或删除程序行
            if program_line.statements.is_empty() {
                // 空行：删除该行
                executor.runtime_mut().delete_line(program_line.line_number);
            } else {
                // 非空行：添加到程序
                executor.runtime_mut().add_line(program_line);
            }
            // 程序行输入后不打印 Ready
            return Ok(false);
        }
        
        // 直接模式：执行语句
        if program_line.statements.is_empty() {
            return Ok(false);
        }
        let statement = program_line.statements.into_iter().next().unwrap();
        
        // 特殊命令处理
        match &statement {
            Statement::List { start, end } => {
                list_program(executor, *start, *end);
                Ok(true)
            }
            Statement::Run { line_number } => {
                run_program(executor, *line_number)?;
                Ok(true)
            }
            Statement::New => {
                executor.execute_statement(&statement)?;
                println!("New program started.");
                Ok(true)
            }
            Statement::End => {
                executor.execute_statement(&statement)?;
                Ok(true)
            }
            Statement::Stop => {
                executor.execute_statement(&statement)?;
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("?BREAK IN {}", line);
                }
                Ok(true)
            }
            Statement::Cont => {
                continue_program(executor)?;
                Ok(true)
            }
            _ => {
                // 其他语句：直接执行
                executor.execute_statement(&statement)?;
                Ok(true)
            }
        }
    } else {
        Ok(false)
    }
}

/// 列出程序
fn list_program(executor: &Executor, start: Option<u16>, end: Option<u16>) {
    let lines = executor.runtime().get_program_lines(start, end);
    
    if lines.is_empty() {
        if start.is_some() || end.is_some() {
            // 有范围但没有内容，不输出
            return;
        }
        println!("No program loaded.");
        return;
    }
    
    for line in lines {
        // 使用完整的序列化函数
        println!("{}", Executor::serialize_program_line(line));
    }
}

/// 运行程序
fn run_program(executor: &mut Executor, line_number: Option<u16>) -> Result<()> {
    // 启动执行
    executor.runtime_mut().start_execution(line_number)?;
    
    // 执行循环
    loop {
        let stmt = match executor.runtime_mut().get_next_statement() {
            Some(s) => s,
            None => break, // 程序结束
        };
        
        // 执行语句
        if let Err(e) = executor.execute_statement(&stmt) {
            // 输出错误和行号
            if let Some(line) = executor.runtime().get_current_line() {
                eprintln!("?{} IN {}", format_error(&e), line);
            } else {
                eprintln!("?{}", format_error(&e));
            }
            // 错误已经打印，不再向上传播，直接返回 Ok
            return Ok(());
        }
        
        // 检查是否应该停止
        if executor.runtime().is_stopped() {
            break;
        }
    }
    
    Ok(())
}

/// 继续执行程序
fn continue_program(executor: &mut Executor) -> Result<()> {
    if !executor.runtime().can_continue() {
        println!("?CAN'T CONTINUE");
        // 错误已经打印，返回 Ok 以便显示 Ready
        return Ok(());
    }
    
    // 从暂停点恢复执行
    executor.runtime_mut().continue_execution()?;
    run_program(executor, None)
}

/// 格式化错误信息
fn format_error(error: &BasicError) -> String {
    match error {
        BasicError::SyntaxError(_) => "SYNTAX ERROR".to_string(),
        BasicError::DivisionByZero => "DIVISION BY ZERO".to_string(),
        BasicError::TypeMismatch(_) => "TYPE MISMATCH".to_string(),
        BasicError::UndefinedLine(n) => format!("UNDEFINED LINE {}", n),
        BasicError::UndefinedVariable(v) => format!("UNDEFINED VARIABLE {}", v),
        BasicError::SubscriptOutOfRange(_) => "SUBSCRIPT OUT OF RANGE".to_string(),
        BasicError::OutOfData => "OUT OF DATA".to_string(),
        BasicError::ReturnWithoutGosub => "RETURN WITHOUT GOSUB".to_string(),
        BasicError::NextWithoutFor(_) => "NEXT WITHOUT FOR".to_string(),
        BasicError::CantContinue => "CAN'T CONTINUE".to_string(),
        _ => format!("{:?}", error),
    }
}


```

现在让我编译并测试：

太好了！现在没有额外的空行了。让我再测试几个场景：

---

**User**

等下啊，我手工测试，还是有换行，我的意思，是否应该调用print!("Ready.")而不是println!("Ready.")

---

**Cursor**

```plaintext
Ready.
用户输入在同一行
```

```plaintext
Ready.
（新行）用户输入
```

```rust
let readline = rl.readline("");
```

```rust
let readline = rl.readline("Ready.");
```

```plaintext
Ready.
█（光标在下一行开始）
```

```plaintext
Ready.█（光标紧跟在后面）
```

你说得对！经典的 BASIC 通常是光标紧跟在 `Ready.` 后面，在同一行等待输入。但是由于我们使用了 `rustyline`，我们需要将 `Ready.` 作为 rustyline 的提示符。

让我修改代码：

```rust
use basic_m6502::{
    BasicError, Executor, Parser, Result, Statement, Tokenizer,
};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() -> Result<()> {
    println!("Microsoft BASIC 6502 Interpreter (Rust Edition)");
    println!();

    // 创建执行器
    let mut executor = Executor::new();

    // 创建 rustyline 编辑器（带历史记录）
    let mut rl = DefaultEditor::new().map_err(|e| {
        BasicError::SyntaxError(format!("Failed to initialize editor: {}", e))
    })?;

    // 加载历史记录（如果存在）
    let history_file = ".basic_history";
    let _ = rl.load_history(history_file);

    // REPL 主循环
    let mut prompt = "Ready.\n";
    
    loop {
        // 读取一行（带提示符）
        let readline = rl.readline(prompt);
        
        match readline {
            Ok(line) => {
                // 添加到历史记录
                rl.add_history_entry(line.as_str()).ok();
                
                // 处理输入行
                match process_line(&mut executor, &line) {
                    Ok(should_print_ready) => {
                        // 执行成功后，根据返回值决定是否显示提示符
                        if should_print_ready {
                            prompt = "Ready.\n";
                        } else {
                            prompt = "";
                        }
                    }
                    Err(e) => {
                        // 输出错误
                        match e {
                            BasicError::CantContinue => {
                                // 正常退出信号，不输出错误
                                prompt = "";
                            }
                            _ => {
                                eprintln!("?{}", format_error(&e));
                                prompt = "Ready.\n";
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C 中断
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("\n?BREAK IN {}", line);
                } else {
                    println!("\n^C");
                }
                executor.runtime_mut().interrupt();
                println!("Ready.");
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    // 保存历史记录
    rl.save_history(history_file).ok();
    
    Ok(())
}

/// 处理一行输入
/// 返回值：Ok(bool) - true 表示应该打印 "Ready.", false 表示不打印
fn process_line(executor: &mut Executor, line: &str) -> Result<bool> {
    let line = line.trim();
    
    // 空行
    if line.is_empty() {
        return Ok(false);
    }
    
    // 词法分析
    let mut tokenizer = Tokenizer::new(line);
    let tokens = tokenizer.tokenize_line()?;
    
    // 语法分析
    let mut parser = Parser::new(tokens);
    
    // 解析程序行
    if let Some(program_line) = parser.parse_line()? {
        // 检查是否是带行号的程序行
        if program_line.line_number > 0 {
            // 添加或删除程序行
            if program_line.statements.is_empty() {
                // 空行：删除该行
                executor.runtime_mut().delete_line(program_line.line_number);
            } else {
                // 非空行：添加到程序
                executor.runtime_mut().add_line(program_line);
            }
            // 程序行输入后不打印 Ready
            return Ok(false);
        }
        
        // 直接模式：执行语句
        if program_line.statements.is_empty() {
            return Ok(false);
        }
        let statement = program_line.statements.into_iter().next().unwrap();
        
        // 特殊命令处理
        match &statement {
            Statement::List { start, end } => {
                list_program(executor, *start, *end);
                Ok(true)
            }
            Statement::Run { line_number } => {
                run_program(executor, *line_number)?;
                Ok(true)
            }
            Statement::New => {
                executor.execute_statement(&statement)?;
                println!("New program started.");
                Ok(true)
            }
            Statement::End => {
                executor.execute_statement(&statement)?;
                Ok(true)
            }
            Statement::Stop => {
                executor.execute_statement(&statement)?;
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("?BREAK IN {}", line);
                }
                Ok(true)
            }
            Statement::Cont => {
                continue_program(executor)?;
                Ok(true)
            }
            _ => {
                // 其他语句：直接执行
                executor.execute_statement(&statement)?;
                Ok(true)
            }
        }
    } else {
        Ok(false)
    }
}

/// 列出程序
fn list_program(executor: &Executor, start: Option<u16>, end: Option<u16>) {
    let lines = executor.runtime().get_program_lines(start, end);
    
    if lines.is_empty() {
        if start.is_some() || end.is_some() {
            // 有范围但没有内容，不输出
            return;
        }
        println!("No program loaded.");
        return;
    }
    
    for line in lines {
        // 使用完整的序列化函数
        println!("{}", Executor::serialize_program_line(line));
    }
}

/// 运行程序
fn run_program(executor: &mut Executor, line_number: Option<u16>) -> Result<()> {
    // 启动执行
    executor.runtime_mut().start_execution(line_number)?;
    
    // 执行循环
    loop {
        let stmt = match executor.runtime_mut().get_next_statement() {
            Some(s) => s,
            None => break, // 程序结束
        };
        
        // 执行语句
        if let Err(e) = executor.execute_statement(&stmt) {
            // 输出错误和行号
            if let Some(line) = executor.runtime().get_current_line() {
                eprintln!("?{} IN {}", format_error(&e), line);
            } else {
                eprintln!("?{}", format_error(&e));
            }
            // 错误已经打印，不再向上传播，直接返回 Ok
            return Ok(());
        }
        
        // 检查是否应该停止
        if executor.runtime().is_stopped() {
            break;
        }
    }
    
    Ok(())
}

/// 继续执行程序
fn continue_program(executor: &mut Executor) -> Result<()> {
    if !executor.runtime().can_continue() {
        println!("?CAN'T CONTINUE");
        // 错误已经打印，返回 Ok 以便显示 Ready
        return Ok(());
    }
    
    // 从暂停点恢复执行
    executor.runtime_mut().continue_execution()?;
    run_program(executor, None)
}

/// 格式化错误信息
fn format_error(error: &BasicError) -> String {
    match error {
        BasicError::SyntaxError(_) => "SYNTAX ERROR".to_string(),
        BasicError::DivisionByZero => "DIVISION BY ZERO".to_string(),
        BasicError::TypeMismatch(_) => "TYPE MISMATCH".to_string(),
        BasicError::UndefinedLine(n) => format!("UNDEFINED LINE {}", n),
        BasicError::UndefinedVariable(v) => format!("UNDEFINED VARIABLE {}", v),
        BasicError::SubscriptOutOfRange(_) => "SUBSCRIPT OUT OF RANGE".to_string(),
        BasicError::OutOfData => "OUT OF DATA".to_string(),
        BasicError::ReturnWithoutGosub => "RETURN WITHOUT GOSUB".to_string(),
        BasicError::NextWithoutFor(_) => "NEXT WITHOUT FOR".to_string(),
        BasicError::CantContinue => "CAN'T CONTINUE".to_string(),
        _ => format!("{:?}", error),
    }
}


```

```rust
use basic_m6502::{
    BasicError, Executor, Parser, Result, Statement, Tokenizer,
};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() -> Result<()> {
    println!("Microsoft BASIC 6502 Interpreter (Rust Edition)");
    println!();

    // 创建执行器
    let mut executor = Executor::new();

    // 创建 rustyline 编辑器（带历史记录）
    let mut rl = DefaultEditor::new().map_err(|e| {
        BasicError::SyntaxError(format!("Failed to initialize editor: {}", e))
    })?;

    // 加载历史记录（如果存在）
    let history_file = ".basic_history";
    let _ = rl.load_history(history_file);

    // REPL 主循环
    let mut prompt = "Ready.\n";
    
    loop {
        // 读取一行（带提示符）
        let readline = rl.readline(prompt);
        
        match readline {
            Ok(line) => {
                // 添加到历史记录
                rl.add_history_entry(line.as_str()).ok();
                
                // 处理输入行
                match process_line(&mut executor, &line) {
                    Ok(should_print_ready) => {
                        // 执行成功后，根据返回值决定是否显示提示符
                        if should_print_ready {
                            prompt = "Ready.\n";
                        } else {
                            prompt = "";
                        }
                    }
                    Err(e) => {
                        // 输出错误
                        match e {
                            BasicError::CantContinue => {
                                // 正常退出信号，不输出错误
                                prompt = "";
                            }
                            _ => {
                                eprintln!("?{}", format_error(&e));
                                prompt = "Ready.\n";
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C 中断
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("?BREAK IN {}", line);
                } else {
                    println!("^C");
                }
                executor.runtime_mut().interrupt();
                prompt = "Ready.\n";
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    // 保存历史记录
    rl.save_history(history_file).ok();
    
    Ok(())
}

/// 处理一行输入
/// 返回值：Ok(bool) - true 表示应该打印 "Ready.", false 表示不打印
fn process_line(executor: &mut Executor, line: &str) -> Result<bool> {
    let line = line.trim();
    
    // 空行
    if line.is_empty() {
        return Ok(false);
    }
    
    // 词法分析
    let mut tokenizer = Tokenizer::new(line);
    let tokens = tokenizer.tokenize_line()?;
    
    // 语法分析
    let mut parser = Parser::new(tokens);
    
    // 解析程序行
    if let Some(program_line) = parser.parse_line()? {
        // 检查是否是带行号的程序行
        if program_line.line_number > 0 {
            // 添加或删除程序行
            if program_line.statements.is_empty() {
                // 空行：删除该行
                executor.runtime_mut().delete_line(program_line.line_number);
            } else {
                // 非空行：添加到程序
                executor.runtime_mut().add_line(program_line);
            }
            // 程序行输入后不打印 Ready
            return Ok(false);
        }
        
        // 直接模式：执行语句
        if program_line.statements.is_empty() {
            return Ok(false);
        }
        let statement = program_line.statements.into_iter().next().unwrap();
        
        // 特殊命令处理
        match &statement {
            Statement::List { start, end } => {
                list_program(executor, *start, *end);
                Ok(true)
            }
            Statement::Run { line_number } => {
                run_program(executor, *line_number)?;
                Ok(true)
            }
            Statement::New => {
                executor.execute_statement(&statement)?;
                println!("New program started.");
                Ok(true)
            }
            Statement::End => {
                executor.execute_statement(&statement)?;
                Ok(true)
            }
            Statement::Stop => {
                executor.execute_statement(&statement)?;
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("?BREAK IN {}", line);
                }
                Ok(true)
            }
            Statement::Cont => {
                continue_program(executor)?;
                Ok(true)
            }
            _ => {
                // 其他语句：直接执行
                executor.execute_statement(&statement)?;
                Ok(true)
            }
        }
    } else {
        Ok(false)
    }
}

/// 列出程序
fn list_program(executor: &Executor, start: Option<u16>, end: Option<u16>) {
    let lines = executor.runtime().get_program_lines(start, end);
    
    if lines.is_empty() {
        if start.is_some() || end.is_some() {
            // 有范围但没有内容，不输出
            return;
        }
        println!("No program loaded.");
        return;
    }
    
    for line in lines {
        // 使用完整的序列化函数
        println!("{}", Executor::serialize_program_line(line));
    }
}

/// 运行程序
fn run_program(executor: &mut Executor, line_number: Option<u16>) -> Result<()> {
    // 启动执行
    executor.runtime_mut().start_execution(line_number)?;
    
    // 执行循环
    loop {
        let stmt = match executor.runtime_mut().get_next_statement() {
            Some(s) => s,
            None => break, // 程序结束
        };
        
        // 执行语句
        if let Err(e) = executor.execute_statement(&stmt) {
            // 输出错误和行号
            if let Some(line) = executor.runtime().get_current_line() {
                eprintln!("?{} IN {}", format_error(&e), line);
            } else {
                eprintln!("?{}", format_error(&e));
            }
            // 错误已经打印，不再向上传播，直接返回 Ok
            return Ok(());
        }
        
        // 检查是否应该停止
        if executor.runtime().is_stopped() {
            break;
        }
    }
    
    Ok(())
}

/// 继续执行程序
fn continue_program(executor: &mut Executor) -> Result<()> {
    if !executor.runtime().can_continue() {
        println!("?CAN'T CONTINUE");
        // 错误已经打印，返回 Ok 以便显示 Ready
        return Ok(());
    }
    
    // 从暂停点恢复执行
    executor.runtime_mut().continue_execution()?;
    run_program(executor, None)
}

/// 格式化错误信息
fn format_error(error: &BasicError) -> String {
    match error {
        BasicError::SyntaxError(_) => "SYNTAX ERROR".to_string(),
        BasicError::DivisionByZero => "DIVISION BY ZERO".to_string(),
        BasicError::TypeMismatch(_) => "TYPE MISMATCH".to_string(),
        BasicError::UndefinedLine(n) => format!("UNDEFINED LINE {}", n),
        BasicError::UndefinedVariable(v) => format!("UNDEFINED VARIABLE {}", v),
        BasicError::SubscriptOutOfRange(_) => "SUBSCRIPT OUT OF RANGE".to_string(),
        BasicError::OutOfData => "OUT OF DATA".to_string(),
        BasicError::ReturnWithoutGosub => "RETURN WITHOUT GOSUB".to_string(),
        BasicError::NextWithoutFor(_) => "NEXT WITHOUT FOR".to_string(),
        BasicError::CantContinue => "CAN'T CONTINUE".to_string(),
        _ => format!("{:?}", error),
    }
}


```

等等，我重新理解你的需求。如果你想要光标紧跟在 `Ready.` 后面在同一行，那应该去掉换行符：

```rust
    // REPL 主循环
    let mut prompt = "Ready.\n";
    
    loop {
        // 读取一行（带提示符）
        let readline = rl.readline(prompt);
        
        match readline {
            Ok(line) => {
                // 添加到历史记录
                rl.add_history_entry(line.as_str()).ok();
                
                // 处理输入行
                match process_line(&mut executor, &line) {
                    Ok(should_print_ready) => {
                        // 执行成功后，根据返回值决定是否显示提示符
                        if should_print_ready {
                            prompt = "Ready.\n";
                        } else {
                            prompt = "";
                        }
                    }
                    Err(e) => {
                        // 输出错误
                        match e {
                            BasicError::CantContinue => {
                                // 正常退出信号，不输出错误
                                prompt = "";
                            }
                            _ => {
                                eprintln!("?{}", format_error(&e));
                                prompt = "Ready.\n";
                            }
                        }
                    }
                }
```

```rust
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C 中断
                if let Some(line) = executor.runtime().get_current_line() {
                    println!("?BREAK IN {}", line);
                } else {
                    println!("^C");
                }
                executor.runtime_mut().interrupt();
                prompt = "Ready.\n";
            }
```

```plaintext
Ready.█（光标在这里）
```

让我先编译并测试当前的代码，看看效果如何：

现在让我们手动测试一下。请你运行 `cargo run` 并手动测试，看看 `Ready.` 的显示效果如何。目前我设置的是 `"Ready.\n"`，这会让 Ready. 显示后换行再等待输入。

如果你想要光标紧跟在 `Ready.` 后面（如 `Ready.█`），请告诉我，我会改成 `"Ready."` (去掉 `\n`)。

让我展示两种风格的区别：

**当前实现 (`"Ready.\n"`)**:
```
Ready.
print 123█
```

**如果改成 (`"Ready."`)**:
```
Ready.print 123█
```

经典的 Commodore 64 BASIC 实际上用的是第一种（Ready. 后换行）。你更喜欢哪种风格？

```plaintext
Ready.
print 123█
```

```plaintext
Ready.print 123█
```

---

**User**

确认第一种是经典的实现方式吗？如果是的话不用改这个问题了。

---

**Cursor**

```plaintext
Ready.
█光标在下一行
```

```plaintext
Ready.█光标紧跟
```

是的，我可以确认！第一种（`Ready.` 后换行）确实是经典的实现方式。

在经典的 6502 BASIC 实现中，如：
- **Commodore 64 BASIC V2**
- **Apple II Integer BASIC / Applesoft BASIC**
- **BBC Micro BASIC**

它们都是这样的格式：

```
READY.
█（光标在下一行）
```

而不是光标紧跟在 `READY.` 后面。

所以你当前的实现 `let mut prompt = "Ready.\n";` 是正确的，完全符合经典 6502 BASIC 的风格！

现在你的实现已经非常接近经典的 BASIC 体验了：
- ✅ `Ready.` 后换行等待输入
- ✅ 执行直接命令后显示 `Ready.`
- ✅ 输入程序行（带行号）后不显示 `Ready.`
- ✅ 运行程序后显示 `Ready.`
- ✅ 错误后显示 `Ready.`

你的 BASIC 解释器已经具备了经典的交互体验！🎉

```plaintext
READY.
█（光标在下一行）
```

