//! Module for representing a Rust dependency.
//!
//! This module defines the RustDependency struct, which represents a dependency
//! in a Rust project. Dependencies can be internal or external, and an associated
//! RustCrate (defined in the parent module) can be tied to the dependency.

use super::rcrate::RustCrate;

/// Represents a Rust dependency, which could be internal or external.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RustDependency {
    /// The name of the dependency.
    pub name: String,
    /// Indicates if the dependency is external. Defaults to true.
    pub is_external: bool,
    /// Associated RustCrate, defined in the parent module.
    pub crate_ref: Option<RustCrate>,
}

impl RustDependency {
    /// Creates a new RustDependency with the given name.
    ///
    /// The `is_external` field is set to `true` by default.
    ///
    /// # Arguments
    ///
    /// * `name` - A string representing the dependency's name.
    /// * `crate_ref` - An optional reference to a RustCrate from the parent module.
    pub fn new(name: String, crate_ref: Option<RustCrate>) -> Self {
        RustDependency {
            name,
            is_external: true,
            crate_ref,
        }
    }
}
