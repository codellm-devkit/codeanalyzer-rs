use std::collections::HashMap;

use super::{
    RustVisibility,
    attr::RustAttribute,
    callable::RustCallable,
    renum::RustEnum,
    rimpl::RustImpl,
    rmacro::RustMacro,
    rstruct::RustStruct,
    rtrait::RustTrait,
    rtype::{RustType, RustTypeAlias},
    variables::RustVariableDeclaration,
};

/// Represents a Rust module with all possible items.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustModule {
    /// The name of the module.
    pub name: String,
    /// Documentation comment for the module.
    pub doc_comment: Option<String>,
    /// Attributes applied to the module.
    pub attributes: Vec<RustAttribute>,
    /// Visibility of the module.
    pub visibility: RustVisibility,

    // Type definitions
    /// Map of type definitions.
    pub types: HashMap<String, RustType>,
    /// Map of structs.
    pub structs: HashMap<String, RustStruct>,
    /// Map of enums.
    pub enums: HashMap<String, RustEnum>,
    /// Map of traits.
    pub traits: HashMap<String, RustTrait>,
    /// List of implementations.
    pub impls: Vec<RustImpl>,
    /// Map of type aliases.
    pub type_aliases: HashMap<String, RustTypeAlias>,

    // Functions and macros
    /// Map of functions.
    pub functions: HashMap<String, RustCallable>,
    /// Map of safe functions.
    pub safe_functions: HashMap<String, RustCallable>,
    /// Map of unsafe functions.
    pub unsafe_functions: HashMap<String, RustCallable>,
    /// Map of macros.
    pub macros: HashMap<String, RustMacro>,

    // Module structure
    /// Map of submodules.
    pub submodules: HashMap<String, RustModule>,
    /// Constants declared in the module.
    pub constants: Vec<RustVariableDeclaration>,
    /// List of use declarations.
    pub use_declarations: Vec<String>,
    /// List of extern crate declarations.
    pub extern_crates: Vec<String>,

    // Module properties
    /// Indicates if the module is marked as unsafe.
    pub is_unsafe: bool,
    /// The file path of the module.
    pub file_path: Option<String>,
    /// True if the file is named mod.rs.
    pub is_mod_rs: bool,
    /// True if this is the root module.
    pub is_root_module: bool,
}

impl RustModule {
    /// Creates a new `RustModule` with the given name.
    pub fn new(name: String) -> Self {
        Self {
            name,
            doc_comment: None,
            attributes: Vec::new(),
            visibility: RustVisibility::Private,
            types: HashMap::new(),
            structs: HashMap::new(),
            enums: HashMap::new(),
            traits: HashMap::new(),
            impls: Vec::new(),
            type_aliases: HashMap::new(),
            functions: HashMap::new(),
            safe_functions: HashMap::new(),
            unsafe_functions: HashMap::new(),
            macros: HashMap::new(),
            submodules: HashMap::new(),
            constants: Vec::new(),
            use_declarations: Vec::new(),
            extern_crates: Vec::new(),
            is_unsafe: false,
            file_path: None,
            is_mod_rs: false,
            is_root_module: false,
        }
    }
}
