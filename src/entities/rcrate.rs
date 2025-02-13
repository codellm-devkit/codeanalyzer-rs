use super::{dep::RustDependency, module::RustModule};

/// Represents a complete Rust crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustCrate {
    /// The name of the crate.
    pub name: String,
    /// The version of the crate.
    pub version: String,
    /// Indicates if the crate is a library.
    pub is_lib: bool,
    /// List of modules in the crate.
    pub modules: Vec<RustModule>,
    /// The edition of the crate (default is "2021").
    pub edition: String,
    /// Features enabled in the crate.
    pub features: Vec<String>,
    /// Dependencies of the crate.
    pub dependencies: Vec<RustDependency>,
}

impl RustCrate {
    /// Creates a new `RustCrate` with the given name, version, and modules.
    ///
    /// # Arguments
    ///
    /// * `name` - A string representing the crate's name.
    /// * `version` - A string representing the crate's version.
    /// * `modules` - A vector containing the crate's modules.
    ///
    /// # Returns
    ///
    /// A new instance of `RustCrate` with default values:
    /// - `is_lib` set to `false`
    /// - `edition` set to `"2021"`
    /// - Empty vectors for `features` and `dependencies`
    pub fn new(name: String, version: String, modules: Vec<RustModule>) -> Self {
        RustCrate {
            name,
            version,
            is_lib: false,
            modules,
            edition: "2021".to_owned(),
            features: Vec::new(),
            dependencies: Vec::new(),
        }
    }
}
