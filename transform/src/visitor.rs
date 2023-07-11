use swc_core::ecma::ast::JSXElementChild;
use swc_core::ecma::{
    ast::Expr,
    visit::{Fold, FoldWith, VisitMut},
};
use tracing::debug;

use crate::tags::{choose_tag, if_tag, with_tag};
use crate::utils::attributes::get_jsx_element_name;
use crate::utils::elements::wrap_by_child_jsx_expr_container;

pub fn transform_jsx_control_statements() -> impl Fold {
    JSXControlStatements
}

pub struct JSXControlStatements;

impl VisitMut for JSXControlStatements {}

impl Fold for JSXControlStatements {
    fn fold_expr(&mut self, expr: Expr) -> Expr {
        let expr = expr.fold_children_with(self);

        match expr {
            Expr::JSXElement(mut jsx_element) => {
                let element_name = get_jsx_element_name(&jsx_element.opening.name);

                debug!("fold_expr::Expr::JSXElement::tag_name = {}", element_name);

                match element_name {
                    "If" => if_tag::convert_jsx_element(&mut jsx_element),
                    "Choose" => choose_tag::convert_jsx_element(&mut jsx_element),
                    _ => {
                        if element_name == "With" {
                            with_tag::convert_with_jsx_element(&jsx_element);
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
                let mut jsx_element = *value;

                let element_name = get_jsx_element_name(&jsx_element.opening.name);

                debug!(
                    "fold_jsx_element_child::JSXElementChild::JSXElement::tag_name = {}",
                    element_name
                );

                match element_name {
                    "If" => wrap_by_child_jsx_expr_container(if_tag::convert_jsx_element(
                        &mut jsx_element,
                    )),
                    "Choose" => wrap_by_child_jsx_expr_container(choose_tag::convert_jsx_element(
                        &mut jsx_element,
                    )),
                    _ => {
                        if element_name == "With" {
                            with_tag::convert_with_jsx_element(&jsx_element);
                        }

                        JSXElementChild::JSXElement(Box::new(jsx_element))
                    }
                }
            }
            _ => element,
        }
    }
}
