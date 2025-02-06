/// Represents a generic type parameter in Rust.
///
/// This struct holds information about a generic parameter, including its name,
/// trait bounds, default type (if any), and whether it's a const generic.
#[derive(Debug, Clone, PartialEq)]
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
