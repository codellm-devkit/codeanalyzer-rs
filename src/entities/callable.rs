use super::{
    RustVisibility,
    attr::RustAttribute,
    callsite::CallSite,
    lifetime::RustLifetimeParam,
    param::{RustGenericParam, RustParameter},
    rtype::RustType,
    safety::SafetyAnalysis,
    variables::RustVariableDeclaration,
};

/// Represents a Rust function or method.
#[derive(Debug, Clone)]
pub struct RustCallable {
    /// The name of the function or method.
    pub name: String,
    /// Visibility of the function (e.g., public, private).
    pub visibility: RustVisibility,
    /// Documentation comment associated with the function.
    pub doc_comment: Option<String>,
    /// Attributes attached to the function.
    pub attributes: Vec<RustAttribute>,
    /// Parameters of the function.
    pub parameters: Vec<RustParameter>,
    /// Return type of the function.
    pub return_type: Option<RustType>,
    /// Indicates if the function is asynchronous.
    pub is_async: bool,
    /// Indicates if the function is a const function.
    pub is_const: bool,
    /// Indicates if the function is marked as unsafe.
    pub is_unsafe: bool,
    /// Indicates if the function is declared as extern.
    pub is_extern: bool,
    /// ABI for the extern function.
    pub extern_abi: Option<String>,
    /// Generic parameters for the function.
    pub generic_params: Vec<RustGenericParam>,
    /// Lifetime parameters for the function.
    pub lifetime_params: Vec<RustLifetimeParam>,
    /// Where clauses associated with the function.
    pub where_clauses: Vec<String>,
    /// The source code of the function.
    pub code: String,
    /// The starting line number of the function definition.
    pub start_line: usize,
    /// The ending line number of the function definition.
    pub end_line: usize,
    /// List of types referenced in the function.
    pub referenced_types: Vec<String>,
    /// Variables accessed within the function.
    pub accessed_variables: Vec<String>,
    /// Call sites present in the function.
    pub call_sites: Vec<CallSite>,
    /// Variable declarations within the function.
    pub variable_declarations: Vec<RustVariableDeclaration>,
    /// Cyclomatic complexity metric of the function.
    pub cyclomatic_complexity: Option<usize>,
    /// Safety analysis information.
    pub safety_analysis: SafetyAnalysis,
}

impl RustCallable {
    /// Creates a new instance of `RustCallable` with the required fields.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function.
    /// * `code` - The source code of the function.
    /// * `start_line` - The starting line number of the function definition.
    /// * `end_line` - The ending line number of the function definition.
    /// * `safety_analysis` - The safety analysis details for the function.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::entities::callable::RustCallable;
    /// use crate::entities::RustVisibility;
    /// use crate::entities::SafetyAnalysis;
    ///
    /// let safety = SafetyAnalysis::default(); // Assuming a default implementation is available.
    /// let callable = RustCallable::new("my_function".to_string(), "fn my_function() {}".to_string(), 1, 3, safety);
    /// ```
    pub fn new(
        name: String,
        code: String,
        start_line: usize,
        end_line: usize,
        safety_analysis: SafetyAnalysis,
    ) -> Self {
        Self {
            name,
            visibility: RustVisibility::Private,
            doc_comment: None,
            attributes: Vec::new(),
            parameters: Vec::new(),
            return_type: None,
            is_async: false,
            is_const: false,
            is_unsafe: false,
            is_extern: false,
            extern_abi: None,
            generic_params: Vec::new(),
            lifetime_params: Vec::new(),
            where_clauses: Vec::new(),
            code,
            start_line,
            end_line,
            referenced_types: Vec::new(),
            accessed_variables: Vec::new(),
            call_sites: Vec::new(),
            variable_declarations: Vec::new(),
            cyclomatic_complexity: None,
            safety_analysis,
        }
    }
}
