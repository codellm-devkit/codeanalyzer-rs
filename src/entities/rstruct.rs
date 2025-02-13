use std::collections::HashMap;

use super::{
    RustStructKind, RustVisibility, attr::RustAttribute, callable::RustCallable,
    lifetime::RustLifetimeParam, param::RustGenericParam, rtype::RustType,
};

/// Represents a field in a Rust struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustStructField {
    /// Name of the field.
    pub name: String,
    /// The type of the field.
    pub ty: RustType,
    /// Visibility of the field. Defaults to `RustVisibility::PRIVATE`.
    pub visibility: RustVisibility,
    /// Optional documentation comment.
    pub doc_comment: Option<String>,
    /// List of attributes associated with the field.
    pub attributes: Vec<RustAttribute>,
}

impl RustStructField {
    /// Creates a new `RustStructField` with the given name and type.
    /// Visibility is set to `PRIVATE`, no doc comment, and an empty attributes list.
    pub fn new(name: String, ty: RustType) -> Self {
        Self {
            name,
            ty,
            visibility: RustVisibility::Private,
            doc_comment: None,
            attributes: Vec::new(),
        }
    }
}

/// Represents a Rust struct definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustStruct {
    /// Name of the Rust struct.
    pub name: String,
    /// Kind of the struct. Defaults to `RustStructKind::NORMAL`.
    pub kind: RustStructKind,
    /// Visibility of the struct. Defaults to `RustVisibility::PRIVATE`.
    pub visibility: RustVisibility,
    /// Optional documentation comment.
    pub doc_comment: Option<String>,
    /// List of attributes associated with the struct.
    pub attributes: Vec<RustAttribute>,
    /// List of fields contained in the struct.
    pub fields: Vec<RustStructField>,
    /// Generic parameters for the struct.
    pub generic_params: Vec<RustGenericParam>,
    /// Lifetime parameters for the struct.
    pub lifetime_params: Vec<RustLifetimeParam>,
    /// Where clauses applied to the struct.
    pub where_clauses: Vec<String>,
    /// Derives applied to the struct.
    pub derives: Vec<String>,
    /// Associated items (e.g., methods, functions) where key is the item name.
    pub associated_items: HashMap<String, RustCallable>,
    /// Traits that this struct implements.
    pub impl_traits: Vec<String>,
    /// Indicates whether the struct is public.
    pub is_public: bool,
    /// Indicates whether the struct contains unsafe code.
    pub contains_unsafe: bool,
    /// The starting line number of the struct definition in the source file.
    pub start_line: usize,
    /// The ending line number of the struct definition in the source file.
    pub end_line: usize,
}

impl RustStruct {
    /// Creates a new `RustStruct` with the given name, start_line, and end_line.
    /// Default values are used for the remaining fields.
    pub fn new(name: String, start_line: usize, end_line: usize) -> Self {
        Self {
            name,
            kind: RustStructKind::Normal,
            visibility: RustVisibility::Private,
            doc_comment: None,
            attributes: Vec::new(),
            fields: Vec::new(),
            generic_params: Vec::new(),
            lifetime_params: Vec::new(),
            where_clauses: Vec::new(),
            derives: Vec::new(),
            associated_items: HashMap::new(),
            impl_traits: Vec::new(),
            is_public: false,
            contains_unsafe: false,
            start_line,
            end_line,
        }
    }
}
