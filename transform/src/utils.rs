use swc_core::common::errors::HANDLER;
use swc_core::common::DUMMY_SP;
use swc_core::common::{Span, Spanned};
use swc_core::ecma::ast::Expr;
use swc_core::ecma::ast::Ident;
use swc_core::ecma::ast::JSXAttr;
use swc_core::ecma::ast::JSXAttrName;
use swc_core::ecma::ast::JSXAttrOrSpread;
use swc_core::ecma::ast::JSXAttrValue;
use swc_core::ecma::ast::JSXClosingFragment;
use swc_core::ecma::ast::JSXElement;
use swc_core::ecma::ast::JSXElementChild;
use swc_core::ecma::ast::JSXElementName;
use swc_core::ecma::ast::JSXExpr;
use swc_core::ecma::ast::JSXExprContainer;
use swc_core::ecma::ast::JSXFragment;
use swc_core::ecma::ast::JSXOpeningFragment;
use swc_core::ecma::ast::JSXText;
use swc_core::ecma::ast::Lit;
use swc_core::ecma::ast::Null;
use swc_core::ecma::ast::Str;
use tracing::error;

pub fn clone_children(children: &Vec<JSXElementChild>) -> Vec<JSXElementChild> {
    let mut copy: Vec<JSXElementChild> = Vec::new();

    for child in children {
        match child {
            JSXElementChild::JSXText(JSXText { value, .. }) => {
                let mut value = value.to_string();

                value = value.replace('\n', "");

                if value.trim() != "" {
                    copy.push((*child).clone());
                }
            }
            _ => {
                copy.push((*child).clone());
            }
        }
    }

    copy
}

pub fn convert_children_to_expression(children: Vec<JSXElementChild>) -> Expr {
    match children.len() {
        0 => Expr::Lit(Lit::Null(Null { span: DUMMY_SP })),
        1 => {
            let sub_element = &children[0];

            match sub_element {
                JSXElementChild::JSXElement(value) => Expr::JSXElement(Box::new((**value).clone())),
                JSXElementChild::JSXExprContainer(JSXExprContainer {
                    expr: JSXExpr::Expr(expr),
                    ..
                }) => match &**expr {
                    Expr::Cond(expr) => Expr::Cond((*expr).clone()),
                    _ => Expr::Lit(Lit::Null(Null { span: DUMMY_SP })),
                },
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
        _ => Expr::JSXFragment(JSXFragment {
            span: DUMMY_SP,
            children,
            opening: JSXOpeningFragment { span: DUMMY_SP },
            closing: JSXClosingFragment { span: DUMMY_SP },
        }),
    }
}

pub fn display_error(span: Span, message: &str) {
    HANDLER.with(|handler| handler.struct_span_err(span, message).emit());

    error!(message);
}

pub fn get_condition_expression(jsx_element: &JSXElement) -> Expr {
    jsx_element
        .opening
        .attrs
        .iter()
        .find(|attr| {
            if let JSXAttrOrSpread::JSXAttr(JSXAttr {
                name: JSXAttrName::Ident(Ident { sym, span, .. }),
                ..
            }) = attr
            {
                if sym == "condition" {
                    return true;
                } else {
                    display_error(
                        *span,
                        format!("Only \"condition\" attribute allowed, got: \"{}\".", sym).as_str(),
                    );
                }
            }

            false
        })
        .map(|attr| match attr {
            JSXAttrOrSpread::JSXAttr(JSXAttr { value, .. }) => match value {
                Some(JSXAttrValue::JSXExprContainer(value)) => {
                    let JSXExprContainer { expr, .. } = value;

                    match expr {
                        JSXExpr::Expr(value) => (**value).clone(),
                        _ => Expr::Lit(Lit::Bool(false.into())),
                    }
                }
                _ => Expr::Lit(Lit::Bool(false.into())),
            },
            JSXAttrOrSpread::SpreadElement(value) => {
                display_error(
                    value.dot3_token.span(),
                    "Spread is invalid for the value of a condition!",
                );

                Expr::Lit(Lit::Bool(false.into()))
            }
        })
        .unwrap_or_else(|| {
            let element_name = get_jsx_element_name(&jsx_element.opening.name);

            display_error(
                jsx_element.opening.span,
                format!(
                    "Attribute \"condition\" is required for the <{}> tag!",
                    element_name
                )
                .as_str(),
            );

            Expr::Lit(Lit::Bool(false.into()))
        })
}

pub fn get_jsx_element_name(jsx_element_name: &JSXElementName) -> &str {
    match jsx_element_name {
        JSXElementName::Ident(Ident { sym, .. }) => sym,
        _ => "unknown",
    }
}

pub fn wrap_by_child_jsx_expr_container(expr: Expr) -> JSXElementChild {
    JSXElementChild::JSXExprContainer(JSXExprContainer {
        span: DUMMY_SP,
        expr: JSXExpr::Expr(Box::new(expr)),
    })
}
