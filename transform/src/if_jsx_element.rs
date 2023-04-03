use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::Expr;
use swc_core::ecma::ast::{CondExpr, JSXElement, JSXElementChild, JSXText};

use crate::utils::{
    convert_children_to_expression, display_error, get_condition_expression, get_jsx_element_name,
};

pub fn convert_if_jsx_element(jsx_element: &JSXElement) -> Expr {
    let (cons, alt) = parse_if_jsx_element(&jsx_element);

    Expr::Cond(CondExpr {
        test: Box::new(get_condition_expression(&jsx_element.opening.attrs)),
        cons: Box::new(cons),
        alt: Box::new(alt),
        span: DUMMY_SP,
    })
}

fn parse_if_jsx_element(jsx_element: &JSXElement) -> (Expr, Expr) {
    let mut else_found = false;

    let (left_chilren, right_children): (Vec<JSXElementChild>, Vec<JSXElementChild>) = jsx_element.children
            .iter()
            .fold((Vec::new(), Vec::new()), |(mut cons, mut alts), child| {
                match child {
                    JSXElementChild::JSXText(JSXText { value, .. }) => {
                        let mut value = value.to_string();

                        value = value.replace('\n', "");

                        if value.trim() == "" {
                            return (cons, alts);
                        }
                    }
                    JSXElementChild::JSXElement(jsx_element) => {
                        let tag_name = get_jsx_element_name(&jsx_element.opening.name);

                        if tag_name == "Else" {
                            if jsx_element.closing.is_some() {
                                display_error(
                                    jsx_element.opening.span,
                                    "<Else /> should be self-closing!",
                                );
                            }

                            if !else_found {
                                else_found = true;
                            } else {
                                display_error(
                                    jsx_element.opening.span,
                                    "<Else /> can be used one per <If>, If you want multiple choises use Choose + When, Otherwise.",
                                );
                            }

                            return (cons, alts);
                        }
                    }
                    _ => {}
                }

                if !else_found {
                    cons.push((*child).clone());
                } else {
                    alts.push((*child).clone());
                }

                (cons, alts)
            });

    (
        convert_children_to_expression(left_chilren),
        convert_children_to_expression(right_children),
    )
}
