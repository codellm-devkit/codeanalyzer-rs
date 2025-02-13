use std::collections::HashMap;

use super::{
    attr::RustAttribute, callable::RustCallable, lifetime::RustLifetimeParam,
    param::RustGenericParam, rstruct::RustStructField, rtype::RustType, *,
};

/// Represents a Rust enum.
#[derive(Debug, Clone)]
pub struct RustEnum {
    /// The name of the enum.
    pub name: String,
    /// The visibility of the enum.
    pub visibility: RustVisibility,
    /// Optional documentation comment.
    pub doc_comment: Option<String>,
    /// Attributes associated with the enum.
    pub attributes: Vec<RustAttribute>,
    /// Variants defined for the enum.
    pub variants: Vec<RustEnumVariant>,
    /// Generic parameters.
    pub generic_params: Vec<RustGenericParam>,
    /// Lifetime parameters.
    pub lifetime_params: Vec<RustLifetimeParam>,
    /// Where clauses.
    pub where_clauses: Vec<String>,
    /// Traits to derive.
    pub derives: Vec<String>,
    /// Associated items mapped by their name.
    pub associated_items: HashMap<String, RustCallable>,
    /// Traits implemented for the enum.
    pub impl_traits: Vec<String>,
    /// Indicates if the enum is public.
    pub is_public: bool,
    /// The starting line in the source file.
    pub start_line: i32,
    /// The ending line in the source file.
    pub end_line: i32,
}

/// Represents a variant in a Rust enum.
#[derive(Debug, Clone)]
pub struct RustEnumVariant {
    /// The name of the variant.
    pub name: String,
    /// Fields for a struct variant.
    pub fields: Option<Vec<RustStructField>>,
    /// Types for a tuple variant.
    pub tuple_types: Option<Vec<RustType>>,
    /// Explicit discriminant for the variant.
    pub discriminant: Option<String>,
    /// Optional documentation comment.
    pub doc_comment: Option<String>,
    /// Attributes associated with the variant.
    pub attributes: Vec<RustAttribute>,
}
