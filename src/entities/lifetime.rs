use std::vec::Vec;

/// Represents a lifetime parameter in Rust.
///
/// This struct holds information about a lifetime parameter, including its name
/// and any bounds associated with it.
#[derive(Debug, Clone, PartialEq)]
pub struct RustLifetimeParam {
    /// The name of the lifetime parameter (e.g., 'a, 'b)
    pub name: String,

    /// A list of lifetime bounds that constrain this lifetime parameter
    /// For example, in 'a: 'b, 'b would be a bound on 'a
    pub bounds: Vec<String>,
}

impl RustLifetimeParam {
    /// Creates a new `RustLifetimeParam` with the given name and empty bounds
    pub fn new(name: String) -> Self {
        Self {
            name,
            bounds: Vec::new(),
        }
    }

    /// Creates a new `RustLifetimeParam` with the given name and bounds
    pub fn with_bounds(name: String, bounds: Vec<String>) -> Self {
        Self { name, bounds }
    }
}
