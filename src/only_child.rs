use crate::Error;
use minidom::Element;

/// Get the one and only child of an element.
///
/// If no children or more than two children are found, it is considered an error.
pub trait OnlyChildElementExt {
    /// Try to get the unique child of an element.
    ///
    /// To select this element, a predicate is specified taking the element as
    /// an input and returning a boolean.
    ///
    /// The function returns a [`Result`] with an error if there is none
    /// [`NoChildrenFound`] or more than one [`MultipleChildrenFound`] selected elements by the
    /// predicate above.
    ///
    /// [`NoChildrenFound`]: enum.Error.html#variant.NoChildrenFound
    /// [`MultipleChildrenFound`]: enum.Error.html#variant.MultipleChildrenFound
    /// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
    fn try_find_only_child<'a, P>(&'a self, predicate: P) -> Result<&'a Self, Error>
    where
        P: Fn(&'a Self) -> bool;

    /// Get the unique child of an element.
    ///
    /// To select this element, a predicate is specified taking the element as
    /// an input and returning a boolean.
    ///
    /// The function returns an [`Option`] with the child if there is one and
    /// only one child corresponding to the predicate.
    ///
    /// [`Option`]:  https://doc.rust-lang.org/std/option/enum.Option.html
    fn find_only_child<'a, P>(&'a self, predicate: P) -> Option<&'a Self>
    where
        P: Fn(&'a Self) -> bool,
    {
        self.try_find_only_child(predicate).ok()
    }

    /// Try to get an unique child from its name and return a [`Result`].
    ///
    /// Returns an [`Error`] if the child can't be found ([`NoChildren`])
    /// or if the child is not unique ([`MultipleChildren`])
    ///
    /// [`Error`]: enum.Error.html
    /// [`NoChildren`]: enum.Error.html#variant.NoChildren
    /// [`MultipleChildren`]: enum.Error.html#variant.MultipleChildren
    /// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
    fn try_only_child<'a>(&'a self, child_name: &str) -> Result<&'a Self, Error>;

    /// Get a unique child from its name and return an [`Option`].
    ///
    /// Returns [`None`] if the child can't be found or if the child is not
    /// unique.
    ///
    /// [`None`]:  https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    /// [`Option`]:  https://doc.rust-lang.org/std/option/enum.Option.html
    fn only_child<'a>(&'a self, child_name: &str) -> Option<&'a Self> {
        self.try_only_child(child_name).ok()
    }
}

impl OnlyChildElementExt for Element {
    /// Implementation of [`OnlyChildElementExt`] for [`Element`] gives you the ability to
    /// select one and only one child of an XML tag depending on a predicate. If
    /// none or more than two children are found with the predicate, an error is
    /// returned.
    ///
    /// ```
    /// use minidom::Element;
    /// use minidom_ext::OnlyChildElementExt;
    ///
    /// let xml: &'static str = r#"<root xmlns="ns">
    ///         <child type="ugly" />
    ///         <child />
    ///     </root>"#;
    /// let root: Element = xml.parse().unwrap();
    /// let child = root
    ///     .try_find_only_child(|e| {
    ///         e.name() == "child" && e.attr("type").map(|id| id == "ugly").unwrap_or(false)
    ///     })
    ///     .unwrap();
    /// assert_eq!("child", child.name());
    /// ```
    ///
    /// [`OnlyChildElementExt`]: trait.OnlyChildElementExt.html
    /// [`Element`]: ../minidom/element/struct.Element.html
    fn try_find_only_child<'a, P>(&'a self, predicate: P) -> Result<&'a Self, Error>
    where
        P: Fn(&'a Self) -> bool,
    {
        let mut child_iterator = self.children().filter(|child| predicate(*child));
        if let Some(child) = child_iterator.next() {
            if child_iterator.next().is_none() {
                Ok(child)
            } else {
                Err(Error::MultipleChildrenFound(
                    self.name().to_owned(),
                    2 + child_iterator.count(),
                ))
            }
        } else {
            Err(Error::NoChildrenFound(self.name().to_owned()))
        }
    }

    /// Implementation of [`OnlyChildElementExt`] for [`Element`] gives you the ability to
    /// select one and only one child of an XML tag depending on its name. If
    /// none or more than two are found with the name, an error is returned.
    ///
    /// ```
    /// use minidom::Element;
    /// use minidom_ext::OnlyChildElementExt;
    ///
    /// let xml: &'static str = r#"<root xmlns="ns">
    ///         <child />
    ///     </root>"#;
    /// let root: Element = xml.parse().unwrap();
    /// let child = root
    ///     .try_only_child("child")
    ///     .unwrap();
    /// assert_eq!("child", child.name());
    /// ```
    ///
    /// [`OnlyChildElementExt`]: trait.OnlyChildElementExt.html
    /// [`Element`]: ../minidom/element/struct.Element.html
    fn try_only_child<'a>(&'a self, child_name: &str) -> Result<&'a Self, Error> {
        self.try_find_only_child(|element| element.name() == child_name)
            .map_err(|e| match e {
                Error::MultipleChildrenFound(element_name, count) => {
                    Error::MultipleChildren(element_name, child_name.to_owned(), count)
                }
                Error::NoChildrenFound(element_name) => {
                    Error::NoChildren(element_name, child_name.to_owned())
                }
                e => e,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn only_one_child() {
        let xml: &'static str = r#"<root xmlns="ns">
                <child type="ugly" />
                <child />
            </root>"#;
        let root: Element = xml.parse().unwrap();
        let child = root
            .try_find_only_child(|e| {
                e.name() == "child" && e.attr("type").map(|id| id == "ugly").unwrap_or(false)
            })
            .unwrap();
        assert_eq!("child", child.name());
    }

    #[test]
    fn no_children() {
        let xml: &'static str = r#"<root xmlns="ns" />"#;
        let root: Element = xml.parse().unwrap();
        let error = root
            .try_find_only_child(|e| e.name() == "child")
            .unwrap_err();
        assert_eq!(
            "No children matching predicate found in Element \'root\'",
            format!("{}", error)
        );
    }

    #[test]
    fn multiple_child() {
        let xml: &'static str = r#"<root xmlns="ns">
                <child />
                <child />
            </root>"#;
        let root: Element = xml.parse().unwrap();
        let error = root
            .try_find_only_child(|e| e.name() == "child")
            .unwrap_err();
        assert_eq!(
            "Multiple children matching predicate found in Element \'root\' (found 2 elements)",
            format!("{}", error)
        );
    }
}
