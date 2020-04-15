use crate::Error;
use minidom::Element;
use std::str::FromStr;

/// Get an attribute from an element.
pub trait AttributeElementExt {
    /// Try to get an attribute from its name and return a [`Result`].
    ///
    /// The type of the return value is chosen by the caller, as long as this
    /// type can be parsed from `&str`.
    ///
    /// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
    fn try_attribute<F>(&self, attr_name: &str) -> Result<F, Error>
    where
        F: FromStr,
        F::Err: std::error::Error + Send + Sync + 'static;

    /// Get an attribute from its name if present and return a [`Option`].
    ///
    /// The type of the return value is chosen by the caller, as long as this
    /// type can be parsed from `&str`.
    ///
    /// [`Option`]:  https://doc.rust-lang.org/std/option/enum.Option.html
    fn attribute<F>(&self, attr_name: &str) -> Option<F>
    where
        F: FromStr,
        F::Err: std::error::Error + Send + Sync + 'static,
    {
        self.try_attribute(attr_name).ok()
    }
}

impl AttributeElementExt for Element {
    /// Implementation of [`AttributeElementExt`] for [`Element`] gives you
    /// access to the attribute's value of an XML element. For example, the
    /// `id`'s value of this XML element `<tag id="value" />`.
    ///
    /// ```
    /// use minidom::Element;
    /// use minidom_ext::AttributeElementExt;
    ///
    /// let xml: &'static str = r#"<root id="42" />"#;
    /// let root: Element = xml.parse().unwrap();
    /// let id: u64 = root.try_attribute("id").unwrap();
    /// assert_eq!(42, id);
    /// ```
    ///
    /// [`AttributeElementExt`]: trait.AttributeElementExt.html
    /// [`Element`]: ../minidom/element/struct.Element.html
    fn try_attribute<F>(&self, attr_name: &str) -> Result<F, Error>
    where
        F: FromStr,
        F::Err: std::error::Error + Send + Sync + 'static,
    {
        let value = self.attr(attr_name).ok_or_else(|| {
            Error::AttributeNotFound(self.name().to_owned(), attr_name.to_owned())
        })?;
        value.parse().map_err(|e: F::Err| Error::ParseError {
            element_name: self.name().to_owned(),
            attribute_name: attr_name.to_owned(),
            value: value.to_owned(),
            source: e.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn no_attribute() {
        let xml: &'static str = r#"<root />"#;
        let root: Element = xml.parse().unwrap();
        let error = root.try_attribute::<String>("id").unwrap_err();
        assert_eq!(
            "Failed to find attribute \'id\' in element \'root\'",
            format!("{}", error)
        );
    }

    #[test]
    fn no_children() {
        let xml: &'static str = r#"<root id="root:1" />"#;
        let root: Element = xml.parse().unwrap();
        let error = root.try_attribute::<f64>("id").unwrap_err();
        assert_eq!("Failed to parse and convert the value \'root:1\' of attribute \'id\' in element \'root\'", format!("{}", error));
    }
}
