use std::collections::HashMap;

use super::{
    callable::RustCallable, lifetime::RustLifetimeParam, param::RustGenericParam, rtype::RustType,
};

/// Represents a Rust impl block.
#[derive(Debug, Clone)]
pub struct RustImpl {
    /// The type name that the impl block is for.
    pub type_name: String,
    /// Optional trait name; None for inherent impls.
    pub trait_name: Option<String>,
    /// Generic parameters of the impl.
    pub generic_params: Vec<RustGenericParam>,
    /// Lifetime parameters of the impl.
    pub lifetime_params: Vec<RustLifetimeParam>,
    /// Where clauses associated with the impl.
    pub where_clauses: Vec<String>,
    /// Methods in the impl; the key is the method name.
    pub methods: HashMap<String, RustCallable>,
    /// Associated types defined in the impl.
    pub associated_types: HashMap<String, RustType>,
    /// Associated constants defined in the impl.
    pub associated_consts: HashMap<String, String>,
    /// Whether the impl is marked as unsafe.
    pub is_unsafe: bool,
    /// Whether the impl is a negative impl (e.g., `!Send`).
    pub is_negative: bool,
    /// The starting line number of the impl block in the source.
    pub start_line: usize,
    /// The ending line number of the impl block in the source.
    pub end_line: usize,
}
