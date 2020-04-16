mod attribute;
pub use attribute::AttributeElementExt;
mod only_child;
pub use only_child::OnlyChildElementExt;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to find attribute '{1}' in element '{0}'")]
    AttributeNotFound(String, String),
    #[error("No children matching predicate found in Element '{0}'")]
    NoChildrenFound(String),
    #[error("No children with name '{1}' in Element '{0}'")]
    NoChildren(String, String),
    #[error("Multiple children matching predicate found in Element '{0}' (found {1} elements)")]
    MultipleChildrenFound(String, usize),
    #[error("Multiple children with name '{1}' in Element '{0}' (found {2} elements)")]
    MultipleChildren(String, String, usize),
    #[error("Failed to parse and convert the value '{value}' of attribute '{attribute_name}' in element '{element_name}'")]
    ParseError {
        element_name: String,
        attribute_name: String,
        value: String,
        #[source]
        source: anyhow::Error,
    },
}
