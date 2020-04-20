#![deny(missing_docs)]

//! Extension traits and implementations for [`Element`] (see [`minidom`] crate).
//!
//! ## Description
//! - [`OnlyChildElementExt`]: provides helper to extract one and only one child
//!   of an [`Element`]
//! - [`AttributeElementExt`]: provides helper to extract and parse into desired
//!   type an attribute of an [`Element`]
//!
//! ## Examples
//! Follow the links to see some example:
//! - [`OnlyChildElementExt::try_find_only_child`]
//! - [`OnlyChildElementExt::try_only_child`]
//! - [`AttributeElementExt::try_attribute`]
//!
//! [`AttributeElementExt`]: trait.AttributeElementExt.html
//! [`Element`]: ../minidom/element/struct.Element.html
//! [`minidom`]: ../minidom/index.html
//! [`OnlyChildElementExt`]: trait.OnlyChildElementExt.html
//! [`OnlyChildElementExt::try_find_only_child`]: trait.OnlyChildElementExt.html#impl-OnlyChildElementExt-for-Element
//! [`OnlyChildElementExt::try_only_child`]: trait.OnlyChildElementExt.html#impl-OnlyChildElementExt-for-Element
//! [`AttributeElementExt::try_attribute`]: trait.AttributeElementExt.html#impl-AttributeElementExt-for-Element

mod attribute;
pub use attribute::AttributeElementExt;
mod only_child;
pub use only_child::OnlyChildElementExt;

use thiserror::Error;

/// Error type for `minidom_ext`
#[derive(Debug, Error)]
pub enum Error {
    /// Returned when the attribute could not be found by name.
    /// First parameter is the element's name, second parameter is
    /// attribute's name.
    #[error("Failed to find attribute '{1}' in element '{0}'")]
    AttributeNotFound(String, String),
    /// Returned when no children can be matched with the predicate.
    /// First parameter is the element's name.
    #[error("No children matching predicate found in Element '{0}'")]
    NoChildrenFound(String),
    /// Returned when no children can be matched with the expected child's name.
    /// First parameter is the element's name, second parameter is the child's
    /// name.
    #[error("No children with name '{1}' in Element '{0}'")]
    NoChildren(String, String),
    /// Returned when multiple children can be matched with the predicate.
    /// First parameter is the element's name, second parameter is the number of
    /// matching children found.
    #[error("Multiple children matching predicate found in Element '{0}' (found {1} elements)")]
    MultipleChildrenFound(String, usize),
    /// Returned when multiple children can be matched with the expected child's
    /// name.
    /// First parameter is the element's name, second parameter is the child's
    /// name, third parameter is the number of matching children found.
    #[error("Multiple children with name '{1}' in Element '{0}' (found {2} elements)")]
    MultipleChildren(String, String, usize),
    /// Returned when the attribute cannot be parsed or convert into the
    /// expected type.
    #[error("Failed to parse and convert the value '{value}' of attribute '{attribute_name}' in element '{element_name}'")]
    ParseError {
        /// Element's name
        element_name: String,
        /// Attribute's name
        attribute_name: String,
        /// Value of the attribute
        value: String,
        /// Original parsing error. The specific type depends on what type the
        /// value is parsed into.
        /// For example, if parsing into a [`i64`] fails, the returned error
        /// would be of type [`ParseIntError`].
        ///
        /// [`i64`]: https://doc.rust-lang.org/std/primitive.i64.html
        /// [`ParseIntError`]: https://doc.rust-lang.org/std/num/struct.ParseIntError.html
        #[source]
        source: anyhow::Error,
    },
}
