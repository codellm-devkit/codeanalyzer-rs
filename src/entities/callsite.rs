use serde::{Deserialize, Serialize};

use super::rtype::RustType;

/// Represents a location where a function is called.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallSite {
    /// The line number where the function is called
    pub line_number: u32,

    /// The name of the function containing this call site (optional)
    pub caller_function: Option<String>,

    /// The module path containing this call site (optional)
    pub caller_module: Option<String>,

    /// List of argument types used in the function call
    pub argument_types: Vec<RustType>,

    /// Indicates if the call occurs within an unsafe context
    pub is_unsafe_context: bool,
}

impl CallSite {
    /// Creates a new CallSite instance
    pub fn new(line_number: u32) -> Self {
        Self {
            line_number,
            caller_function: None,
            caller_module: None,
            argument_types: Vec::new(),
            is_unsafe_context: false,
        }
    }
}
