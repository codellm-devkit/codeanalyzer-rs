use super::{RustVisibility, attr::RustAttribute};

/// Represents a Rust macro definition.
///
/// This struct encapsulates details about a macro defined
/// in Rust source code. It contains the macro's name, visibility,
/// documentation comment, any attributes associated with it,
/// its defining rules, and metadata regarding its type and location.
#[derive(Debug, Clone)]
pub struct RustMacro {
    /// The name of the macro.
    pub name: String,
    /// The visibility of the macro (e.g., public or private).
    pub visibility: RustVisibility,
    /// An optional documentation comment for the macro.
    pub doc_comment: Option<String>,
    /// Attributes applied to the macro, such as annotations.
    pub attributes: Vec<RustAttribute>,
    /// Rules or tokens that define the macro's behavior.
    pub rules: Vec<String>,
    /// Indicates if the macro is procedural.
    pub is_procedural: bool,
    /// Indicates if the macro is a derive macro.
    pub is_derive: bool,
    /// Indicates if the macro is an attribute macro.
    pub is_attribute: bool,
    /// Indicates if the macro follows the function-like syntax.
    pub is_function_like: bool,
    /// True if the macro is exported via a macro_use directive.
    pub exported_from_macro_use: bool,
    /// The starting line number of the macro definition.
    pub start_line: usize,
    /// The ending line number of the macro definition.
    pub end_line: usize,
}

impl RustMacro {
    /// Creates a new `RustMacro` instance with default values.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the macro.
    /// * `start_line` - The starting line of the macro definition.
    /// * `end_line` - The ending line of the macro definition.
    pub fn new(name: String, start_line: usize, end_line: usize) -> Self {
        Self {
            name,
            visibility: RustVisibility::Private, // Defaults to PRIVATE visibility.
            doc_comment: None,
            attributes: Vec::new(),
            rules: Vec::new(),
            is_procedural: false,
            is_derive: false,
            is_attribute: false,
            is_function_like: true,
            exported_from_macro_use: false,
            start_line,
            end_line,
        }
    }
}
