use swc_core::ecma::ast::JSXElement;

use crate::utils::playthings::display_error;

pub fn convert_with_jsx_element(jsx_element: &JSXElement) {
    display_error(
        jsx_element.opening.span,
        "<With /> tag not implemented, and there are no plans for it!",
    );
}
