use std::fmt;

/// Represents a Rust type.
#[derive(Debug, Clone, PartialEq)]
pub struct RustType {
    /// Name of the type
    pub name: String,
    /// Whether the type is a reference
    pub is_reference: bool,
    /// Whether the type is mutable
    pub is_mutable: bool,
    /// Optional lifetime parameter
    pub lifetime: Option<String>,
    /// Generic type parameters
    pub generic_params: Vec<String>,
    /// Whether the type is sized
    pub is_sized: bool,
    /// Whether the type is static
    pub is_static: bool,
    /// Whether the type contains raw pointers
    pub contains_raw_pointers: bool,
    /// Whether the type is a union
    pub is_union: bool,
}

impl Default for RustType {
    fn default() -> Self {
        Self {
            name: String::new(),
            is_reference: false,
            is_mutable: false,
            lifetime: None,
            generic_params: Vec::new(),
            is_sized: true,
            is_static: false,
            contains_raw_pointers: false,
            is_union: false,
        }
    }
}

impl RustType {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}

impl fmt::Display for RustType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
