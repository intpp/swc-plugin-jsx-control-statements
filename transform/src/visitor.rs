use swc_core::ecma::ast::JSXElementChild;
use swc_core::ecma::{
    ast::Expr,
    visit::{Fold, FoldWith, VisitMut},
};
use tracing::debug;

use crate::choose_jsx_element::convert_choose_jsx_element;
use crate::if_jsx_element::convert_if_jsx_element;
use crate::utils::{get_jsx_element_name, wrap_by_child_jsx_expr_container};
use crate::with_jsx_element::convert_with_jsx_element;

pub fn transform_jsx_control_statements() -> impl Fold {
    JSXControlStatements
}

pub struct JSXControlStatements;

impl VisitMut for JSXControlStatements {}

impl Fold for JSXControlStatements {
    fn fold_expr(&mut self, expr: Expr) -> Expr {
        let expr = expr.fold_children_with(self);

        match expr {
            Expr::JSXElement(jsx_element) => {
                let tag_name = get_jsx_element_name(&jsx_element.opening.name);

                debug!("fold_expr::Expr::JSXElement::tag_name = {}", tag_name);

                match tag_name {
                    "If" => convert_if_jsx_element(&jsx_element),
                    "Choose" => convert_choose_jsx_element(&jsx_element),
                    _ => {
                        if tag_name == "With" {
                            convert_with_jsx_element(&jsx_element);
                        }

                        Expr::JSXElement(jsx_element)
                    }
                }
            }
            _ => expr,
        }
    }

    fn fold_jsx_element_child(&mut self, element: JSXElementChild) -> JSXElementChild {
        let element = element.fold_children_with(self);

        match element {
            JSXElementChild::JSXElement(value) => {
                let jsx_element = *value;

                let tag_name = get_jsx_element_name(&jsx_element.opening.name);

                debug!(
                    "fold_jsx_element_child::JSXElementChild::JSXElement::tag_name = {}",
                    tag_name
                );

                match tag_name {
                    "If" => wrap_by_child_jsx_expr_container(convert_if_jsx_element(&jsx_element)),
                    "Choose" => {
                        wrap_by_child_jsx_expr_container(convert_choose_jsx_element(&jsx_element))
                    }
                    _ => {
                        if tag_name == "With" {
                            convert_with_jsx_element(&jsx_element);
                        }

                        JSXElementChild::JSXElement(Box::new(jsx_element))
                    }
                }
            }
            _ => element,
        }
    }
}
