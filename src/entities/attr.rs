use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustAttribute {
    pub name: String,
    pub arguments: Vec<String>,
    pub is_inner: bool,
}

impl RustAttribute {
    pub fn new(name: String, arguments: Vec<String>, is_inner: bool) -> Self {
        RustAttribute {
            name,
            arguments,
            is_inner,
        }
    }
}

pub trait AttributeOperations {
    fn get_name(&self) -> &str;
    fn get_arguments(&self) -> &[String];
    fn is_inner_attribute(&self) -> bool;
}

impl AttributeOperations for RustAttribute {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_arguments(&self) -> &[String] {
        &self.arguments
    }

    fn is_inner_attribute(&self) -> bool {
        self.is_inner
    }
}
