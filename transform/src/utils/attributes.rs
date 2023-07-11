use swc_core::common::Spanned;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::ArrayLit;
use swc_core::ecma::ast::CondExpr;
use swc_core::ecma::ast::Expr;
use swc_core::ecma::ast::ExprOrSpread;
use swc_core::ecma::ast::Ident;
use swc_core::ecma::ast::JSXAttr;
use swc_core::ecma::ast::JSXAttrName;
use swc_core::ecma::ast::JSXAttrOrSpread;
use swc_core::ecma::ast::JSXAttrValue;
use swc_core::ecma::ast::JSXElement;
use swc_core::ecma::ast::JSXElementChild;
use swc_core::ecma::ast::JSXElementName;
use swc_core::ecma::ast::JSXExpr;
use swc_core::ecma::ast::JSXExprContainer;
use swc_core::ecma::ast::JSXNamespacedName;
use swc_core::ecma::ast::Lit;
use swc_core::ecma::ast::SpreadElement;
use swc_core::ecma::ast::Str;
use tracing::debug;

use crate::utils::playthings::display_error;

pub fn build_key_attribute_value(group: &String, index: usize) -> String {
    if !group.is_empty() {
        format!("{}-{}", group, index)
    } else {
        format!("{}", index)
    }
}

pub fn get_jsx_element_name(jsx_element_name: &JSXElementName) -> &str {
    match jsx_element_name {
        JSXElementName::Ident(Ident { sym, .. }) => sym,
        _ => "unknown",
    }
}

pub fn get_jsx_element_attribute(
    jsx_element: &JSXElement,
    attribute_name: &str,
) -> Option<JSXAttrOrSpread> {
    jsx_element
        .opening
        .attrs
        .iter()
        .find(|attribute| match attribute {
            JSXAttrOrSpread::JSXAttr(JSXAttr {
                name: JSXAttrName::Ident(Ident { sym, .. }),
                ..
            }) => sym == attribute_name,
            _ => false,
        })
        .cloned()
}

pub fn get_condition_expression(jsx_element: &JSXElement) -> Expr {
    get_jsx_element_attribute(jsx_element, "condition")
        .map(|attr| match attr {
            JSXAttrOrSpread::JSXAttr(JSXAttr { value, .. }) => match value {
                Some(JSXAttrValue::JSXExprContainer(value)) => {
                    let JSXExprContainer { expr, .. } = value;

                    match expr {
                        JSXExpr::Expr(value) => (*value).clone(),
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

pub fn get_key_attribute(jsx_element: &JSXElement) -> Option<String> {
    let attribute = get_jsx_element_attribute(jsx_element, "key");

    if let Some(JSXAttrOrSpread::JSXAttr(JSXAttr {
        value: Some(JSXAttrValue::Lit(Lit::Str(Str { value, .. }))),
        ..
    })) = attribute
    {
        return Some(value.to_string());
    }

    None
}

pub fn validate_jsx_control_statement_attributes(jsx_element: &JSXElement) {
    jsx_element
        .opening
        .attrs
        .iter()
        .for_each(|attribute| {
            let element_name = get_jsx_element_name(&jsx_element.opening.name);

            match attribute {
                JSXAttrOrSpread::JSXAttr(JSXAttr { name, .. }) => match name {
                    JSXAttrName::Ident(Ident { sym, span, .. }) => {
                        if sym != "condition" && sym != "key" {
                            display_error(*span, format!("Unsupported: \"{}\" for <{}>, valid props: \"condition\" and \"key\"!", sym, element_name).as_str());
                        }
                    },
                    JSXAttrName::JSXNamespacedName(JSXNamespacedName {
                                                       ns: Ident { span, .. },
                                                       ..
                                                   }) => {
                        display_error(*span, "Unsupported: Namespaced name for JSX control statement tag's prop!");
                    },
                },
                JSXAttrOrSpread::SpreadElement(SpreadElement { dot3_token, .. }) => {
                    display_error(
                        dot3_token.span(),
                        "Unsupported: Spread operator disallowed for JSX control statement tags!",
                    );
                }
            }
        });
}

pub fn set_jsx_element_attribute(
    jsx_element: &mut JSXElement,
    name: &str,
    value: String,
    rewrite: bool,
) {
    let mut has_attribute = false;

    for attr in jsx_element.opening.attrs.iter_mut() {
        if let JSXAttrOrSpread::JSXAttr(jsx_attribute) = attr {
            if let JSXAttr {
                name: JSXAttrName::Ident(Ident { sym, .. }),
                ..
            } = jsx_attribute
            {
                if sym == name {
                    has_attribute = true;

                    if rewrite {
                        jsx_attribute.value.replace(JSXAttrValue::Lit(Lit::Str(Str {
                            value: value.clone().into(),
                            raw: None,
                            span: DUMMY_SP,
                        })));
                    }
                }
            }
        }
    }

    if !has_attribute {
        jsx_element
            .opening
            .attrs
            .push(JSXAttrOrSpread::JSXAttr(JSXAttr {
                name: JSXAttrName::Ident(Ident {
                    span: DUMMY_SP,
                    sym: name.into(),
                    optional: false,
                }),
                value: Some(JSXAttrValue::Lit(Lit::Str(Str {
                    value: value.into(),
                    raw: None,
                    span: DUMMY_SP,
                }))),
                span: DUMMY_SP,
            }));
    }
}

pub fn set_key_attribute_for_expr(
    expr: &mut Expr,
    key_attribute_value: String,
    rewrite_value: bool,
) {
    match expr {
        Expr::JSXElement(jsx_element) => {
            set_jsx_element_attribute(jsx_element, "key", key_attribute_value, rewrite_value);
        }
        Expr::Array(ArrayLit { elems, .. }) => {
            for (index, elem) in elems.iter_mut().enumerate() {
                if let Some(ExprOrSpread { expr, .. }) = elem {
                    if let Expr::JSXElement(jsx_element) = &mut **expr {
                        debug!("Trying to set key for an item");

                        set_jsx_element_attribute(
                            jsx_element,
                            "key",
                            build_key_attribute_value(&key_attribute_value, index),
                            rewrite_value,
                        );
                    }
                }
            }
        }
        Expr::Cond(CondExpr { cons, alt, .. }) => {
            set_key_attribute_for_expr(cons, key_attribute_value.clone(), rewrite_value);
            set_key_attribute_for_expr(alt, key_attribute_value.clone(), rewrite_value);
        }
        _ => {}
    }
}

pub fn set_jsx_child_element_key_attribute(
    jsx_element_child: &mut JSXElementChild,
    key_attribute_value: String,
) {
    if key_attribute_value.is_empty() {
        return;
    }

    match jsx_element_child {
        JSXElementChild::JSXElement(value) => {
            set_jsx_element_attribute(value, "key", key_attribute_value.clone(), false);
        }
        JSXElementChild::JSXExprContainer(JSXExprContainer {
            expr: JSXExpr::Expr(expr),
            ..
        }) => {
            if let Expr::Cond(CondExpr { cons, alt, .. }) = &mut **expr {
                debug!("Checking branches");

                set_key_attribute_for_expr(cons, key_attribute_value.clone(), true);
                set_key_attribute_for_expr(alt, key_attribute_value.clone(), true);
            };
        }
        _ => {}
    }
}
