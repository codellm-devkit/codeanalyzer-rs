use std::fmt;

use serde::{Deserialize, Serialize};

use super::{
    RustVisibility, attr::RustAttribute, lifetime::RustLifetimeParam, param::RustGenericParam,
};

/// Represents a Rust type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
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

/// Represents a Rust type alias.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct RustTypeAlias {
    /// Name of the type alias.
    pub name: String,
    /// Visibility of the type alias.
    pub visibility: RustVisibility,
    /// Optional documentation comment.
    pub doc_comment: Option<String>,
    /// Attributes associated with the type alias.
    pub attributes: Vec<RustAttribute>,
    /// Generic parameters for the type alias.
    pub generic_params: Vec<RustGenericParam>,
    /// Lifetime parameters for the type alias.
    pub lifetime_params: Vec<RustLifetimeParam>,
    /// Where clauses for the type alias.
    pub where_clauses: Vec<String>,
    /// The target type that the alias refers to.
    pub target_type: RustType,
    /// Starting line number in the source file.
    pub start_line: usize,
    /// Ending line number in the source file.
    pub end_line: usize,
}
