/// BASIC value types (first cut: number and string).

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Str(String),
}

impl Value {
    pub fn as_number(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            Value::Str(s) => s.parse::<f64>().unwrap_or(0.0),
        }
    }
}

