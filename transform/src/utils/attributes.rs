use swc_core::atoms::{wtf8::Wtf8Buf, Wtf8Atom};
use swc_core::common::{Spanned, DUMMY_SP};
use swc_core::ecma::ast::{
    ArrayLit, CondExpr, Expr, ExprOrSpread, Ident, IdentName, JSXAttr, JSXAttrName,
    JSXAttrOrSpread, JSXAttrValue, JSXElement, JSXElementChild, JSXElementName, JSXExpr,
    JSXExprContainer, JSXNamespacedName, Lit, SpreadElement, Str,
};
use tracing::debug;

use crate::utils::playthings::display_error;

const CONDITION: &str = "condition";
const KEY: &str = "key";

const KEY_SEPARATOR: &str = "-";

pub fn build_key_attribute_value(group: &Wtf8Atom, index: usize) -> Wtf8Atom {
    let mut key = Wtf8Buf::from(group);
    if !key.is_empty() {
        key.push_str(KEY_SEPARATOR);
    }
    key.push_str(&index.to_string());
    key.into()
}

pub fn get_jsx_element_name(jsx_element_name: &JSXElementName) -> &str {
    match jsx_element_name {
        JSXElementName::Ident(Ident { sym, .. }) => sym,
        _ => "unknown",
    }
}

fn get_jsx_element_attribute<'a>(element: &'a JSXElement, name: &str) -> Option<&'a JSXAttr> {
    element.opening.attrs.iter().find_map(|attr| match attr {
        JSXAttrOrSpread::JSXAttr(attr) if matches!(&attr.name, JSXAttrName::Ident(id) if id.sym == name) => Some(attr),
        _ => None,
    })
}

fn attribute_expression(attr: &JSXAttr) -> Option<&Expr> {
    match &attr.value {
        Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
            expr: JSXExpr::Expr(expr),
            ..
        })) => Some(expr),
        _ => None,
    }
}

pub fn get_condition_expression(element: &JSXElement) -> Expr {
    if let Some(attr) = get_jsx_element_attribute(element, CONDITION) {
        if let Some(expr) = attribute_expression(attr) {
            return expr.clone();
        }
    } else {
        let name = get_jsx_element_name(&element.opening.name);
        display_error(
            element.opening.span,
            &format!("Attribute \"condition\" is required for the <{name}> tag!"),
        );
    }
    Expr::Lit(Lit::Bool(false.into()))
}

pub fn get_key_attribute(element: &JSXElement) -> Option<Wtf8Atom> {
    match &get_jsx_element_attribute(element, KEY)?.value {
        Some(JSXAttrValue::Str(value)) => Some(value.value.clone()),
        _ => None,
    }
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
                    JSXAttrName::Ident(IdentName { sym, span, .. }) => {
                        if sym != CONDITION && sym != KEY {
                            display_error(*span, format!("Unsupported: \"{}\" for <{}>, valid props: \"condition\" and \"key\"!", sym, element_name).as_str());
                        }
                    },
                    JSXAttrName::JSXNamespacedName(JSXNamespacedName {
                                                       ns: IdentName { span, .. },
                                                       ..
                                                   }) => {
                        display_error(*span, "Unsupported: Namespaced name for JSX control statement tag's prop!");
                    },
                    #[cfg(swc_ast_unknown)]
                    _ => panic!("Unsupported JSX attribute name"),
                },
                JSXAttrOrSpread::SpreadElement(SpreadElement { dot3_token, .. }) => {
                    display_error(
                        dot3_token.span(),
                        "Unsupported: Spread operator disallowed for JSX control statement tags!",
                    );
                }
                #[cfg(swc_ast_unknown)]
                _ => panic!("Unsupported JSX attribute"),
            }
        });
}

pub fn set_jsx_element_attribute(
    jsx_element: &mut JSXElement,
    name: &str,
    value: Wtf8Atom,
    rewrite: bool,
) {
    let attr_value = JSXAttrValue::Str(Str {
        span: DUMMY_SP,
        value,
        raw: None,
    });
    let mut found = false;
    for attr in &mut jsx_element.opening.attrs {
        if let JSXAttrOrSpread::JSXAttr(attr) = attr {
            if matches!(&attr.name, JSXAttrName::Ident(id) if id.sym == name) {
                if !rewrite {
                    return;
                }
                attr.value = Some(attr_value.clone());
                found = true;
            }
        }
    }
    if found {
        return;
    }
    jsx_element
        .opening
        .attrs
        .push(JSXAttrOrSpread::JSXAttr(JSXAttr {
            span: DUMMY_SP,
            name: JSXAttrName::Ident(name.into()),
            value: Some(attr_value),
        }));
}

pub fn set_key_attribute_for_expr(
    expr: &mut Expr,
    key_attribute_value: Wtf8Atom,
    rewrite_value: bool,
) {
    match expr {
        Expr::JSXElement(jsx_element) => {
            set_jsx_element_attribute(jsx_element, KEY, key_attribute_value, rewrite_value);
        }
        Expr::Array(ArrayLit { elems, .. }) => {
            for (index, elem) in elems.iter_mut().enumerate() {
                if let Some(ExprOrSpread { expr, .. }) = elem {
                    if let Expr::JSXElement(jsx_element) = &mut **expr {
                        debug!("Trying to set key for an item");

                        set_jsx_element_attribute(
                            jsx_element,
                            KEY,
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
    key_attribute_value: Wtf8Atom,
) {
    if key_attribute_value.is_empty() {
        return;
    }

    match jsx_element_child {
        JSXElementChild::JSXElement(value) => {
            set_jsx_element_attribute(value, KEY, key_attribute_value.clone(), false);
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

pub fn throw_not_string_type(jsx_element: &JSXElement, attr_name: &str) {
    let element_name = get_jsx_element_name(&jsx_element.opening.name);
    display_error(
        jsx_element.opening.span,
        format!(
            "Attribute \"{1}\" of <{}> tag must be of type String, e.g. {1}=\"...\"!",
            element_name, attr_name,
        )
        .as_str(),
    );
}

pub fn throw_not_expression_type(jsx_element: &JSXElement, attr_name: &str) {
    let element_name: &str = get_jsx_element_name(&jsx_element.opening.name);
    display_error(
        jsx_element.opening.span,
        format!(
            "Attribute \"{1}\" of <{}> tag must be an expression, e.g. \"{1}={{...}}\"!",
            element_name, attr_name,
        )
        .as_str(),
    );
}

pub fn get_for_jsx_element_attributes_expr(element: &JSXElement, name: &str) -> Option<Expr> {
    let attr = get_jsx_element_attribute(element, name)?;
    let value = attribute_expression(attr);
    if value.is_none() {
        throw_not_expression_type(element, name);
    }
    value.cloned()
}

pub fn get_for_jsx_element_attributes_ident(element: &JSXElement, name: &str) -> Option<Ident> {
    let attr = get_jsx_element_attribute(element, name)?;
    if let Some(JSXAttrValue::Str(value)) = &attr.value {
        if let Some(sym) = value.value.as_atom() {
            return Some(Ident::from(sym.clone()));
        }
    }
    throw_not_string_type(element, name);
    None
}
