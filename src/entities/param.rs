use crate::entities::rtype::RustType;

#[derive(Debug, Clone, PartialEq)]
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
