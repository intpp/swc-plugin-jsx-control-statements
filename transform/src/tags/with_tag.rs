use swc_core::common::{Spanned, DUMMY_SP};
use swc_core::ecma::ast::*;

use crate::utils::attributes::get_key_attribute;
use crate::utils::elements::convert_children_to_expression;
use crate::utils::functions::returning_function;
use crate::utils::playthings::display_error;

const CALL: &str = "call";

pub fn convert_jsx_element(element: &mut JSXElement) -> Expr {
    let mut params = Vec::new();
    let mut args = vec![ExprOrSpread::from(Box::new(Expr::This(ThisExpr {
        span: DUMMY_SP,
    })))];

    for attribute in &element.opening.attrs {
        let JSXAttrOrSpread::JSXAttr(attr) = attribute else {
            display_error(
                attribute.span(),
                "Unsupported: Spread operator disallowed for JSX control statement tags!",
            );
            continue;
        };
        let JSXAttrName::Ident(name) = &attr.name else {
            display_error(
                attr.name.span(),
                "Unsupported: Namespaced name for JSX control statement tag's prop!",
            );
            continue;
        };

        let value = match &attr.value {
            None => Expr::Lit(Lit::Bool(true.into())),
            Some(JSXAttrValue::Str(value)) => Expr::Lit(Lit::Str(value.clone())),
            Some(JSXAttrValue::JSXElement(value)) => Expr::JSXElement(value.clone()),
            Some(JSXAttrValue::JSXFragment(value)) => Expr::JSXFragment(value.clone()),
            Some(JSXAttrValue::JSXExprContainer(container)) => match &container.expr {
                JSXExpr::Expr(expr) => *expr.clone(),
                JSXExpr::JSXEmptyExpr(empty) => Expr::JSXEmpty(empty.clone()),
                #[cfg(swc_ast_unknown)]
                _ => panic!("Unsupported JSX expression in <With>"),
            },
            #[cfg(swc_ast_unknown)]
            _ => panic!("Unsupported JSX attribute value in <With>"),
        };
        params.push(Ident::from(name.sym.clone()));
        args.push(ExprOrSpread::from(Box::new(value)));
    }

    let group_key = get_key_attribute(element);
    let result = convert_children_to_expression(&mut element.children, group_key);
    Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: Box::new(returning_function(params, result)),
            prop: MemberProp::Ident(CALL.into()),
        }))),
        args,
        ..Default::default()
    })
}
