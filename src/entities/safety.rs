use super::{SafetyClassification, UnsafeReason};

/// Represents an unsafe block within Rust code.
///
/// It includes the start and end line numbers, reasons for using unsafe,
/// an optional explanation, and the name of the containing function (if any).
#[derive(Debug, Clone)]
pub struct UnsafeBlock {
    /// The starting line number of the unsafe block.
    pub start_line: usize,
    /// The ending line number of the unsafe block.
    pub end_line: usize,
    /// The list of reasons why this block is marked as unsafe.
    pub reasons: Vec<UnsafeReason>,
    /// Optional documentation explaining why unsafe is needed.
    pub explanation: Option<String>,
    /// Optional name of the function that contains this unsafe block.
    pub containing_function: Option<String>,
}

impl UnsafeBlock {
    /// Creates a new `UnsafeBlock` with the specified start and end line numbers.
    ///
    /// # Arguments
    ///
    /// * `start_line` - The starting line number.
    /// * `end_line` - The ending line number.
    pub fn new(start_line: usize, end_line: usize) -> Self {
        Self {
            start_line,
            end_line,
            reasons: Vec::new(),
            explanation: None,
            containing_function: None,
        }
    }
}

/// Analyzes and tracks safety-related information in Rust code.
///
/// This structure aggregates various pieces of data related to unsafe operations,
/// such as unsafe blocks, function calls, raw pointer usages, FFI interactions, and more.
#[derive(Debug, Clone)]
pub struct SafetyAnalysis {
    /// The overall safety classification.
    pub classification: SafetyClassification,
    /// A list of unsafe blocks encountered in the code.
    pub unsafe_blocks: Vec<UnsafeBlock>,
    /// A list of unsafe function calls detected.
    pub unsafe_fn_calls: Vec<String>,
    /// Indicates if raw pointer usage was found.
    pub raw_pointer_usage: bool,
    /// Indicates if FFI interactions were found.
    pub ffi_interactions: bool,
    /// A list of unsafe traits used.
    pub unsafe_traits_used: Vec<String>,
    /// A list of mutable static variables.
    pub mutable_statics: Vec<String>,
    /// Optional safety comments.
    pub safety_comments: Option<String>,
}

impl SafetyAnalysis {
    /// Creates a new `SafetyAnalysis` with the given classification.
    ///
    /// All other fields are initialized to their default values.
    ///
    /// # Arguments
    ///
    /// * `classification` - The overall safety classification.
    pub fn new(classification: SafetyClassification) -> Self {
        Self {
            classification,
            unsafe_blocks: Vec::new(),
            unsafe_fn_calls: Vec::new(),
            raw_pointer_usage: false,
            ffi_interactions: false,
            unsafe_traits_used: Vec::new(),
            mutable_statics: Vec::new(),
            safety_comments: None,
        }
    }
}
