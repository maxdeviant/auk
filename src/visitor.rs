//! Constructs for traversing and manipulating trees of [`HtmlElement`]s.

use crate::HtmlElement;

/// A visitor for [`HtmlElement`]s.
pub trait Visitor: Sized {
    /// The type of error this visitor returns.
    type Error;

    /// Visits the given [`HtmlElement`].
    fn visit(&mut self, element: &HtmlElement) -> Result<(), Self::Error> {
        walk_element(self, element)
    }

    /// Visits the given attribute.
    fn visit_attr(&mut self, _name: &str, _value: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit the children of an [`HtmlElement`].
    fn visit_children(&mut self, children: &[HtmlElement]) -> Result<(), Self::Error> {
        walk_children(self, children)
    }
}

/// Walks the given [`HtmlElement`].
pub fn walk_element<V: Visitor>(visitor: &mut V, element: &HtmlElement) -> Result<(), V::Error> {
    for (name, value) in &element.attrs {
        visitor.visit_attr(name, value)?;
    }

    visitor.visit_children(&element.children)?;

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

/// A mutating visitor for [`HtmlElement`]s.
pub trait MutVisitor: Sized {
    /// The type of error this visitor returns.
    type Error;

    /// Visits the given [`HtmlElement`].
    fn visit(&mut self, element: &mut HtmlElement) -> Result<(), Self::Error> {
        noop_visit_element(self, element)
    }

    /// Visits the given attribute.
    fn visit_attr(&mut self, _name: &str, _value: &mut String) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit the children of an [`HtmlElement`].
    fn visit_children(&mut self, children: &mut [HtmlElement]) -> Result<(), Self::Error> {
        noop_visit_children(self, children)
    }
}

/// Walks the given [`HtmlElement`] without mutating it.
pub fn noop_visit_element<V: MutVisitor>(
    visitor: &mut V,
    element: &mut HtmlElement,
) -> Result<(), V::Error> {
    for (name, value) in &mut element.attrs {
        visitor.visit_attr(name, value)?;
    }

    visitor.visit_children(&mut element.children)?;

    Ok(())
}

/// Walks the given children without mutating it..
pub fn noop_visit_children<V: MutVisitor>(
    visitor: &mut V,
    children: &mut [HtmlElement],
) -> Result<(), V::Error> {
    for child in children {
        visitor.visit(child)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use insta::assert_yaml_snapshot;

    use crate::*;

    use super::*;

    #[test]
    fn test_mut_visitor_attribute_replacement() {
        struct ReplaceAttr;

        impl MutVisitor for ReplaceAttr {
            type Error = ();

            fn visit_attr(&mut self, name: &str, value: &mut String) -> Result<(), Self::Error> {
                if name == "href" {
                    *value = format!("https://example.com/{value}");
                }

                Ok(())
            }
        }

        let mut element = html().child(head().child(title())).child(
            body().child(
                ul().child(li().child(a().href("home")))
                    .child(li().child(a().href("features")))
                    .child(li().child(a().href("about"))),
            ),
        );

        let mut visitor = ReplaceAttr;

        visitor.visit(&mut element).unwrap();

        assert_yaml_snapshot!(HtmlElementRenderer::new()
            .render_to_string(&element)
            .unwrap());
    }
}
