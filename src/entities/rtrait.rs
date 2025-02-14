use std::collections::HashMap;

use super::{
    attr::RustAttribute, callable::RustCallable, lifetime::RustLifetimeParam,
    param::RustGenericParam, rtype::RustType,
};

/// Represents a trait bound in Rust.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustTraitBound {
    /// The name of the trait.
    pub trait_name: String,

    /// A list of generic parameters.
    pub generic_params: Vec<String>,

    /// Indicates if the trait is `Sized`.
    pub is_sized: bool,

    /// Indicates if the trait is optional (e.g., for `?Sized`).
    pub is_optional: bool,

    /// A list of lifetime bounds.
    pub lifetime_bounds: Vec<String>,
}

impl Default for RustTraitBound {
    fn default() -> Self {
        Self {
            trait_name: String::new(),
            generic_params: Vec::new(),
            is_sized: true,
            is_optional: false,
            lifetime_bounds: Vec::new(),
        }
    }
}

/// Represents a Rust trait definition.
///
/// Assumes that types like `RustVisibility`, `RustAttribute`,
/// `RustGenericParam`, `RustLifetimeParam`, `RustType`, and
/// `RustCallable` exist in the parent module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustTrait {
    /// The name of the trait.
    pub name: String,

    /// The visibility of the trait.
    pub visibility: super::RustVisibility,

    /// The documentation comment for the trait.
    pub doc_comment: Option<String>,

    /// A list of attributes attached to the trait.
    pub attributes: Vec<RustAttribute>,

    /// A list of generic parameters.
    pub generic_params: Vec<RustGenericParam>,

    /// A list of lifetime parameters.
    pub lifetime_params: Vec<RustLifetimeParam>,

    /// A list of where clause strings.
    pub where_clauses: Vec<String>,

    /// A list of super traits for the trait.
    pub super_traits: Vec<RustTraitBound>,

    /// A mapping of associated types.
    pub associated_types: HashMap<String, RustType>,

    /// A mapping of associated constants.
    pub associated_consts: HashMap<String, String>,

    /// A mapping of method names to their definitions.
    pub methods: HashMap<String, RustCallable>,

    /// Indicates if the trait is declared as unsafe.
    pub is_unsafe: bool,

    /// Indicates if the trait is auto.
    pub is_auto: bool,

    /// The starting line number of the trait in the source code.
    pub start_line: usize,

    /// The ending line number of the trait in the source code.
    pub end_line: usize,
}
