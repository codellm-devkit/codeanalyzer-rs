use serde::{Deserialize, Serialize};

use crate::entities::rtype::RustType;

/// Represents a generic type parameter in Rust.
///
/// This struct holds information about a generic parameter, including its name,
/// trait bounds, default type (if any), and whether it's a const generic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustGenericParam {
    /// The name of the generic parameter
    pub name: String,

    /// List of trait bounds associated with this generic parameter
    /// (e.g., "Send", "Sync", "Display")
    pub bounds: Vec<String>,

    /// Optional default type for this generic parameter
    pub default_type: Option<String>,

    /// Indicates whether this is a const generic parameter
    pub is_const: bool,
}

impl RustGenericParam {
    /// Creates a new generic parameter with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            bounds: Vec::new(),
            default_type: None,
            is_const: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustParameter {
    /// Name of the parameter
    pub name: String,
    /// Type of the parameter
    pub rust_type: RustType,
    /// Indicates if this is a self parameter (self, &self, &mut self)
    pub is_self: bool,
    /// Indicates if this is a mutable binding
    pub is_mut: bool,
    /// Optional default value for the parameter
    pub default_value: Option<String>,
}

impl RustParameter {
    pub fn new(name: String, rust_type: RustType) -> Self {
        RustParameter {
            name,
            rust_type,
            is_self: false,
            is_mut: false,
            default_value: None,
        }
    }
}
