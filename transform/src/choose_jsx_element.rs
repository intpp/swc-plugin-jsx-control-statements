use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::Expr;
use swc_core::ecma::ast::{CondExpr, JSXElement, JSXElementChild, JSXText, Lit, Null};

use crate::utils::{
    clone_children, convert_children_to_expression, display_error, get_condition_expression,
    get_jsx_element_name,
};

pub fn convert_choose_jsx_element(jsx_element: &JSXElement) -> Expr {
    let (cons, alt) = parse_choose_jsx_element(jsx_element);

    let mut condition_expression = alt;

    for (condition, cons) in cons {
        condition_expression = Expr::Cond(CondExpr {
            test: Box::new(condition),
            cons: Box::new(cons),
            alt: Box::new(condition_expression),
            span: DUMMY_SP,
        })
    }

    condition_expression
}

fn parse_choose_jsx_element(jsx_element: &JSXElement) -> (Vec<(Expr, Expr)>, Expr) {
    let mut cons: Vec<(Expr, Expr)> = Vec::new();
    let mut alt = Expr::Lit(Lit::Null(Null { span: DUMMY_SP }));

    let mut otherwise_found = false;

    if jsx_element.children.is_empty() {
        display_error(
            jsx_element.opening.span,
            "<Condition /> tag should contain at least one <When /> tag.",
        );

        return (cons, alt);
    }

    for child in jsx_element.children.iter().rev() {
        match child {
            JSXElementChild::JSXText(JSXText { value, .. }) => {
                let mut value = value.to_string();

                value = value.replace('\n', "");

                if value.trim() != "" {
                    display_error(
                        jsx_element.opening.span,
                        "<Condition /> tag should contain at least one <When /> tag.",
                    );
                }
            }
            JSXElementChild::JSXElement(jsx_element) => {
                let element_name = get_jsx_element_name(&jsx_element.opening.name);

                if element_name == "Otherwise" {
                    if otherwise_found {
                        display_error(
                            jsx_element.opening.span,
                            "<Choose /> can contain only one <Otherwise /> tag.",
                        );
                    } else if cons.is_empty() {
                        otherwise_found = true;

                        if jsx_element.children.is_empty() {
                            display_error(
                                jsx_element.opening.span,
                                "<Otherwise /> tag should contain children.",
                            );
                        } else {
                            alt = convert_children_to_expression(clone_children(
                                &jsx_element.children,
                            ));
                        }
                    } else {
                        display_error(
                            jsx_element.opening.span,
                            "<Otherwise /> tag should be last in the conditions.",
                        );
                    }
                } else if element_name == "When" {
                    cons.push((
                        get_condition_expression(jsx_element),
                        convert_children_to_expression(clone_children(&jsx_element.children)),
                    ));
                } else {
                    display_error(
                        jsx_element.opening.span,
                        format!("<Condition /> tag can contain only <When /> and <Otherwise /> tags, got: <{}>.", element_name).as_str(),
                    );
                }
            }
            _ => {
                display_error(
                    jsx_element.opening.span,
                    "<Condition /> tag can contain only <When /> and <Otherwise /> tags.",
                );
            }
        }
    }

    if cons.is_empty() {
        display_error(
            jsx_element.opening.span,
            "<Condition /> tag should contain at least one <When /> tag.",
        );
    }

    (cons, alt)
}
