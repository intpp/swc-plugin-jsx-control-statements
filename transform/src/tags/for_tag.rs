use swc_core::common::{SyntaxContext, DUMMY_SP};
use swc_core::ecma::ast::*;

use crate::utils::attributes::{
    get_for_jsx_element_attributes_expr, get_for_jsx_element_attributes_ident, get_key_attribute,
};
use crate::utils::elements::convert_children_to_expression;
use crate::utils::functions::returning_function;

const EACH: &str = "each";
const INDEX: &str = "index";
const OF: &str = "of";
const BODY: &str = "body";
const MAP: &str = "map";
const UNUSED_ITEM: &str = "_";

pub fn convert_jsx_element(jsx_element: &mut JSXElement) -> Expr {
    let (of_expr, body_expr, params) = parse_for_jsx_element(jsx_element);

    let group_key = get_key_attribute(jsx_element);
    let children_expr = convert_children_to_expression(&mut jsx_element.children, group_key);

    let callback = if let Some(body) = body_expr {
        body
    } else if matches!(children_expr, Expr::Lit(Lit::Null(_))) {
        return children_expr;
    } else {
        returning_function(params, children_expr)
    };

    map_expression(of_expr, callback)
}

fn parse_for_jsx_element(element: &JSXElement) -> (Expr, Option<Expr>, Vec<Ident>) {
    let each = get_for_jsx_element_attributes_ident(element, EACH);
    let index = get_for_jsx_element_attributes_ident(element, INDEX);
    let of = get_for_jsx_element_attributes_expr(element, OF)
        .unwrap_or_else(|| Expr::Array(ArrayLit::default()));
    let body = get_for_jsx_element_attributes_expr(element, BODY);

    let mut params = Vec::new();
    if each.is_some() || index.is_some() {
        let item = each.unwrap_or_else(|| Ident::new_private(UNUSED_ITEM.into(), DUMMY_SP));
        params.push(item);
    }
    if let Some(index) = index {
        params.push(index);
    }
    (of, body, params)
}

fn map_expression(of_expr: Expr, callback: Expr) -> Expr {
    Expr::Call(CallExpr {
        span: DUMMY_SP,
        ctxt: SyntaxContext::empty(),
        callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(of_expr),
            prop: MemberProp::Ident(MAP.into()),
        }))),
        args: vec![
            ExprOrSpread::from(Box::new(callback)),
            ExprOrSpread {
                spread: None,
                expr: Box::new(Expr::This(ThisExpr { span: DUMMY_SP })),
            },
        ],
        type_args: Default::default(),
    })
}
