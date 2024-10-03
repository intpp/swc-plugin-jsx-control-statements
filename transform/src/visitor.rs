use swc_core::ecma::{ast::*, visit::*};
use tracing::debug;

use crate::tags::{choose_tag, for_tag, if_tag, with_tag};
use crate::utils::attributes::get_jsx_element_name;
use crate::utils::elements::wrap_by_child_jsx_expr_container;

pub struct JSXControlStatements;

impl VisitMut for JSXControlStatements {
    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        if let Expr::JSXElement(jsx_element) = expr {
            let element_name = get_jsx_element_name(&jsx_element.opening.name);

            debug!("fold_expr::Expr::JSXElement::tag_name = {}", element_name);

            match &*element_name {
                "If" => {
                    *expr = if_tag::convert_jsx_element(jsx_element);
                }
                "Choose" => {
                    *expr = choose_tag::convert_jsx_element(jsx_element);
                }
                "With" => {
                    *expr = with_tag::convert_jsx_element(jsx_element);
                }
                "For" => {
                    *expr = for_tag::convert_jsx_element(jsx_element);
                }
                _ => {}
            }
        }
    }

    fn visit_mut_jsx_element_child(&mut self, element: &mut JSXElementChild) {
        element.visit_mut_children_with(self);

        if let JSXElementChild::JSXElement(jsx_element) = element {
            let element_name = get_jsx_element_name(&jsx_element.opening.name);

            debug!(
                "fold_jsx_element_child::JSXElementChild::JSXElement::tag_name = {}",
                element_name
            );

            match element_name {
                "If" => {
                    *element =
                        wrap_by_child_jsx_expr_container(if_tag::convert_jsx_element(jsx_element));
                }
                "Choose" => {
                    *element = wrap_by_child_jsx_expr_container(choose_tag::convert_jsx_element(
                        jsx_element,
                    ));
                }
                "With" => {
                    *element = wrap_by_child_jsx_expr_container(with_tag::convert_jsx_element(
                        jsx_element,
                    ));
                }
                "For" => {
                    *element =
                        wrap_by_child_jsx_expr_container(for_tag::convert_jsx_element(jsx_element));
                }
                _ => {}
            }
        }
    }
}
