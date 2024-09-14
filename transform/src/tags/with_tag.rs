use swc_core::common::Spanned;
use swc_common::{SyntaxContext, DUMMY_SP};
use swc_core::atoms::Atom;
use swc_core::ecma::ast::JSXElement;
use swc_core::ecma::ast::*;

use crate::utils::attributes::get_key_attribute;
use crate::utils::elements::{convert_children_to_expression, get_jsx_element_child_ident_ctxt};

use crate::utils::playthings::display_error;


pub fn convert_jsx_element(jsx_element: &mut JSXElement) -> Expr {
    let (params, values, global_ctxt) = parse_with_jsx_element(jsx_element);

    let group_key = get_key_attribute(jsx_element);

    let children_expr= convert_children_to_expression(
        &mut jsx_element.children,
        group_key.clone(),
    );

    Expr::Call(CallExpr {
        span: DUMMY_SP,
        ctxt: global_ctxt,
        callee: Callee::Expr({
            Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: Box::new(
                    Expr::Fn(FnExpr {
                        ident: None,
                        function: Box::new(Function {
                            span: DUMMY_SP,
                            params, // With the key of all attributes on the JSX element
                            decorators: Default::default(),
                            body: Some(BlockStmt {
                                span: DUMMY_SP,
                                ctxt: global_ctxt,
                                stmts: vec![Stmt::Return(ReturnStmt {
                                    span: DUMMY_SP,
                                    arg: Some(Box::new(
                                        children_expr 
                                    )), // Child element With JSX element
                                })],
                            }),
                            ctxt: global_ctxt,
                            is_generator: Default::default(),
                            is_async: Default::default(),
                            type_params: Default::default(),
                            return_type: Default::default(),
                        })
                    })
                ),
                prop: MemberProp::Ident(IdentName {
                    span: DUMMY_SP,
                    sym: Atom::from("call"),
                }),
            }))
        }),
        args: values, // The first element is this, and the remaining elements are the key values of all the attributes on the With JSX element
        type_args: None,
    })
}


pub fn parse_with_jsx_element(jsx_element: &mut JSXElement) -> (Vec<Param>, Vec<ExprOrSpread>, SyntaxContext) { 
    let mut params = Vec::new();
    let mut values = Vec::new();

    let mut global_ctxt = SyntaxContext::empty();

    jsx_element
            .opening
            .attrs
            .iter().for_each(|attribute| {
                match attribute {
                    JSXAttrOrSpread::JSXAttr(JSXAttr {
                        name,
                        value,
                        ..
                    }) => {
                        if let JSXAttrName::Ident(IdentName{sym, ..}) = name {
                            global_ctxt = get_jsx_element_child_ident_ctxt(&jsx_element.children, sym.as_str());
                            params.push(Param::from(Pat::Ident(BindingIdent{ id: Ident {
                                span: DUMMY_SP,
                                sym: sym.clone(),
                                ctxt: global_ctxt,
                                optional: false,
                            }, type_ann: None})));
                        }

                        if let JSXAttrName::JSXNamespacedName(JSXNamespacedName {
                            ns: IdentName { span, .. },
                            ..
                        }) = name {
                            display_error(*span, "Unsupported: Namespaced name for JSX control statement tag's prop!");
                        }

                        if let Some(jsx_attr_value) = value {
                            match jsx_attr_value {
                                JSXAttrValue::JSXExprContainer(JSXExprContainer{expr,..}) => {
                                    let expr_or_spread;
                                    match expr {
                                        JSXExpr::JSXEmptyExpr(JSXEmptyExpr{span}) => {
                                            expr_or_spread = ExprOrSpread::from(Box::new(Expr::JSXEmpty(JSXEmptyExpr{
                                                span: *span,
                                            })));
                                        },
                                        JSXExpr::Expr(expr) => {
                                            expr_or_spread = ExprOrSpread::from(expr.clone());
                                        }
                                    }
                                    values.push(expr_or_spread);
                                },
                                JSXAttrValue::Lit(lit) => {
                                    values.push(ExprOrSpread::from(Box::new(Expr::Lit(lit.clone()))));
                                },
                                _ =>{}
                            }
                        }
                    },
                    JSXAttrOrSpread::SpreadElement(SpreadElement { dot3_token, .. }) => {
                        display_error(
                            dot3_token.span(),
                            "Unsupported: Spread operator disallowed for JSX control statement tags!",
                        );
                    }
                }
            });
    values.insert(0, ExprOrSpread::from(Box::new(
        Expr::This(ThisExpr {
            span: DUMMY_SP,
        })
    )));
    (params, values, global_ctxt)
}