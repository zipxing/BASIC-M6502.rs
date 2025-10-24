//! Floating point utilities
//!
//! Helper functions for floating point operations.

#[allow(dead_code)]

/// Helper function for floating point comparison
pub fn float_eq(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

/// Format a float like BASIC would
pub fn format_basic_float(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}.0", value)
    } else {
        format!("{}", value)
    }
}