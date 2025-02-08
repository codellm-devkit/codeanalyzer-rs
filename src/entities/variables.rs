use crate::entities::{RustVisibility, attr::RustAttribute, rtype::RustType};
/// Represents a variable declaration in Rust.
///
/// # Fields
///
/// * `name` - The name of the variable
/// * `type_info` - Optional type information of the variable
/// * `is_mut` - Flag indicating if the variable is mutable
/// * `is_static` - Flag indicating if the variable is static
/// * `is_const` - Flag indicating if the variable is a constant
/// * `initializer` - Optional initialization expression as a string
/// * `visibility` - Visibility level of the variable (public, private, etc.)
/// * `doc_comment` - Optional documentation comment associated with the variable
/// * `attributes` - Vector of attributes associated with the variable
/// * `line_number` - Line number where the variable is declared in the source code
///
/// # Examples
///
/// ```
/// let var = RustVariableDeclaration::new(String::from("my_var"), 1);
/// assert_eq!(var.name, "my_var");
/// assert_eq!(var.line_number, 1);
/// ```
#[derive(Debug, Clone)]
pub struct RustVariableDeclaration {
    /// The name of the variable
    pub name: String,
    /// The type of the variable (optional)
    pub type_info: Option<RustType>,
    /// Whether the variable is mutable
    pub is_mut: bool,
    /// Whether the variable is static
    pub is_static: bool,
    /// Whether the variable is const
    pub is_const: bool,
    /// The initializer expression (optional)
    pub initializer: Option<String>,
    /// The visibility of the variable
    pub visibility: RustVisibility,
    /// Documentation comment (optional)
    pub doc_comment: Option<String>,
    /// Associated attributes
    pub attributes: Vec<RustAttribute>,
    /// Line number in source code
    pub line_number: usize,
}

impl RustVariableDeclaration {
    pub fn new(name: String, line_number: usize) -> Self {
        Self {
            name,
            type_info: None,
            is_mut: false,
            is_static: false,
            is_const: false,
            initializer: None,
            visibility: RustVisibility::Private,
            doc_comment: None,
            attributes: Vec::new(),
            line_number,
        }
    }
}
