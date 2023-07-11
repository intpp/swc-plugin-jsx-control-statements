use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::Expr;
use swc_core::ecma::ast::{CondExpr, JSXElement, JSXElementChild, JSXText};

use crate::utils::attributes::{
    get_condition_expression, get_jsx_element_name, get_key_attribute,
    validate_jsx_control_statement_attributes,
};
use crate::utils::elements::convert_children_to_expression;
use crate::utils::playthings::display_error;

pub fn convert_jsx_element(jsx_element: &mut JSXElement) -> Expr {
    validate_jsx_control_statement_attributes(jsx_element);

    let (cons, alt) = parse_if_jsx_element(jsx_element);

    Expr::Cond(CondExpr {
        test: Box::new(get_condition_expression(jsx_element)),
        cons: Box::new(cons),
        alt: Box::new(alt),
        span: DUMMY_SP,
    })
}

fn parse_if_jsx_element(jsx_element: &mut JSXElement) -> (Expr, Expr) {
    let mut else_found = false;
    let initial = (&mut Vec::new(), &mut Vec::new());

    let (left_chilren, right_children): (&mut Vec<JSXElementChild>, &mut Vec<JSXElementChild>) = jsx_element.children
        .iter()
        .fold(initial, |(cons, alts), child| {
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

    let cons_group_key = get_key_attribute(jsx_element);
    let alt_group_key = cons_group_key.clone();

    (
        convert_children_to_expression(left_chilren, cons_group_key),
        convert_children_to_expression(right_children, alt_group_key),
    )
}
