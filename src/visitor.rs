//! Constructs for traversing and manipulating trees of [`HtmlElement`]s.

use crate::HtmlElement;

/// A visitor for [`HtmlElement`]s.
pub trait Visitor: Sized {
    /// The type of error this visitor returns.
    type Error;

    /// Visits the given [`HtmlElement`].
    fn visit(&mut self, element: &HtmlElement) -> Result<(), Self::Error>;

    /// Visits the given attribute list.
    fn visit_attrs(&mut self, attrs: &[(&str, &str)]) -> Result<(), Self::Error> {
        walk_attrs(self, attrs)
    }

    /// Visits the given attribute.
    fn visit_attr(&mut self, name: &str, value: &str) -> Result<(), Self::Error> {
        let _ = name;
        let _ = value;

        Ok(())
    }

    /// Visit the children of an [`HtmlElement`].
    fn visit_children(&mut self, children: &[HtmlElement]) -> Result<(), Self::Error> {
        walk_children(self, children)
    }
}

/// Walks the given attribute list.
pub fn walk_attrs<V: Visitor>(visitor: &mut V, attrs: &[(&str, &str)]) -> Result<(), V::Error> {
    for (name, value) in attrs {
        visitor.visit_attr(name, value)?;
    }

    Ok(())
}

/// Walks the given children.
pub fn walk_children<V: Visitor>(
    visitor: &mut V,
    children: &[HtmlElement],
) -> Result<(), V::Error> {
    for child in children {
        visitor.visit(child)?;
    }

    Ok(())
}
