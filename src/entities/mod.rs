pub mod attr;
pub mod callsite;
pub mod lifetime;
pub mod param;
pub mod rtype;
pub mod variables;
#[derive(Debug, Clone, PartialEq)]
pub enum RustVisibility {
    Public,
    Private,
    Crate,
    Super,
    InPath(String),
}

impl RustVisibility {
    pub fn as_str(&self) -> &str {
        match self {
            RustVisibility::Public => "pub",
            RustVisibility::Private => "",
            RustVisibility::Crate => "pub(crate)",
            RustVisibility::Super => "pub(super)",
            RustVisibility::InPath(_) => "pub(in path)",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SafetyClassification {
    Safe,
    Unsafe,
    UnsafeContainer,
    Ffi,
}

/// Returns a string slice representing the safety classification.
///
/// # Returns
/// - `"safe"` for `SafetyClassification::Safe`
/// - `"unsafe"` for `SafetyClassification::Unsafe`
/// - `"unsafe_container"` for `SafetyClassification::UnsafeContainer`
/// - `"ffi"` for `SafetyClassification::Ffi`
impl SafetyClassification {
    pub fn as_str(&self) -> &str {
        match self {
            SafetyClassification::Safe => "safe",
            SafetyClassification::Unsafe => "unsafe",
            SafetyClassification::UnsafeContainer => "unsafe_container",
            SafetyClassification::Ffi => "ffi",
        }
    }
}

/// Represents the various reasons why code might be marked as unsafe.
///
/// This implementation provides a method to convert the enum variants into
/// string representations, which can be useful for logging, debugging,
/// or displaying the unsafe reason in a human-readable format.
///
/// # Examples
///
/// ```
/// let reason = UnsafeReason::RawPointerDeref;
/// assert_eq!(reason.as_str(), "raw_pointer_deref");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum UnsafeReason {
    RawPointerDeref,
    MutableStatic,
    FfiCall,
    UnionFieldAccess,
    InlineAssembly,
    UnsafeTraitImpl,
    Custom(String),
}

impl UnsafeReason {
    /// Converts the unsafe reason into a static string slice.
    ///
    /// Returns a string representation of the unsafe reason:
    /// - `"raw_pointer_deref"` for raw pointer dereferences
    /// - `"mutable_static"` for mutable static variables
    /// - `"ffi_call"` for foreign function interface calls
    /// - `"union_field_access"` for union field access
    /// - `"inline_assembly"` for inline assembly code
    /// - `"unsafe_trait_impl"` for unsafe trait implementations
    /// - `"custom"` for custom unsafe reasons
    ///
    /// # Returns
    ///
    /// A string slice (`&str`) representing the unsafe reason.
    pub fn as_str(&self) -> &str {
        match self {
            UnsafeReason::RawPointerDeref => "raw_pointer_deref",
            UnsafeReason::MutableStatic => "mutable_static",
            UnsafeReason::FfiCall => "ffi_call",
            UnsafeReason::UnionFieldAccess => "union_field_access",
            UnsafeReason::InlineAssembly => "inline_assembly",
            UnsafeReason::UnsafeTraitImpl => "unsafe_trait_impl",
            UnsafeReason::Custom(_) => "custom",
        }
    }
}
