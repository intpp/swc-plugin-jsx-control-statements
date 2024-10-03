use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::{
    ArrayLit, Expr, ExprOrSpread, JSXElementChild, JSXExpr, JSXExprContainer, JSXText, Lit, Null,
    Str,
};
use tracing::debug;

use crate::utils::attributes::{build_key_attribute_value, set_jsx_child_element_key_attribute};

pub fn convert_child_to_expression(
    jsx_element_child: &mut JSXElementChild,
    key_attribute: String,
) -> Expr {
    set_jsx_child_element_key_attribute(jsx_element_child, key_attribute.clone());

    match jsx_element_child {
        JSXElementChild::JSXFragment(value) => Expr::JSXFragment((*value).clone()),
        JSXElementChild::JSXElement(value) => Expr::JSXElement(Box::new((**value).clone())),
        JSXElementChild::JSXExprContainer(JSXExprContainer {
            expr: JSXExpr::Expr(expr),
            ..
        }) => (**expr).clone(),
        JSXElementChild::JSXText(JSXText { value, .. }) => {
            let mut content = value.to_string();

            content = content.replace('\n', "");

            Expr::Lit(Lit::Str(Str {
                span: DUMMY_SP,
                value: content.trim().into(),
                raw: None,
            }))
        }
        _ => Expr::Lit(Lit::Null(Null { span: DUMMY_SP })),
    }
}

pub fn convert_children_to_expression(
    children: &mut Vec<JSXElementChild>,
    group_key: Option<String>,
) -> Expr {
    let group_key = group_key.unwrap_or("".to_string());

    debug!(
        "convert_children_to_expression (length before filter is {})",
        children.len()
    );

    children.retain(|child| match child {
        JSXElementChild::JSXText(JSXText { value, .. }) => {
            let mut value = value.to_string();

            value = value.replace('\n', "");

            value.trim() != ""
        }
        _ => true,
    });

    debug!(
        "convert_children_to_expression (length after filter is {})",
        children.len()
    );

    match children.len() {
        0 => Expr::Lit(Lit::Null(Null { span: DUMMY_SP })),
        1 => convert_child_to_expression(&mut children[0], group_key.clone()), // TODO: add group key if it present and key set manually
        _ => Expr::Array(ArrayLit {
            span: DUMMY_SP,
            elems: children
                .iter_mut()
                .enumerate()
                .map(|(index, child)| {
                    let key = build_key_attribute_value(&group_key, index);

                    debug!("convert_children_to_expression (len > 1), key = {}", key);

                    Some(ExprOrSpread {
                        spread: None,
                        expr: Box::new(convert_child_to_expression(child, key)),
                    })
                })
                .collect(),
        }),
    }
}

pub fn wrap_by_child_jsx_expr_container(expr: Expr) -> JSXElementChild {
    JSXElementChild::JSXExprContainer(JSXExprContainer {
        span: DUMMY_SP,
        expr: JSXExpr::Expr(Box::new(expr)),
    })
}
