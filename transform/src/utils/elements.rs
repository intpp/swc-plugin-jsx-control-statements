use swc_common::SyntaxContext;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::BinExpr;
use swc_core::ecma::ast::BlockStmt;
use swc_core::ecma::ast::CallExpr;
use swc_core::ecma::ast::Callee;
use swc_core::ecma::ast::ExprOrSpread;
use swc_core::ecma::ast::Ident;
use swc_core::ecma::ast::JSXElementChild;
use swc_core::ecma::ast::JSXExpr;
use swc_core::ecma::ast::JSXExprContainer;
use swc_core::ecma::ast::JSXSpreadChild;
use swc_core::ecma::ast::JSXText;
use swc_core::ecma::ast::Lit;
use swc_core::ecma::ast::MemberExpr;
use swc_core::ecma::ast::Null;
use swc_core::ecma::ast::ReturnStmt;
use swc_core::ecma::ast::Stmt;
use swc_core::ecma::ast::Str;
use swc_core::ecma::ast::FnExpr;
use swc_core::ecma::ast::{ArrayLit, Expr};
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

pub fn get_jsx_element_child_ident_ctxt_by_attr(
    children: &Vec<JSXElementChild>,
    attr: &str,
) -> SyntaxContext {
    let mut global_ctxt = SyntaxContext::empty();
    children.iter().for_each(|jsx_element_child| {
        match jsx_element_child {
            JSXElementChild::JSXElement(ref value) => {
                global_ctxt = get_jsx_element_child_ident_ctxt_by_attr(&value.children, &attr);
            },
            JSXElementChild::JSXExprContainer(JSXExprContainer{expr,..}) => {
                match expr {
                    JSXExpr::Expr(expr) => {
                        match &**expr {
                            Expr::Ident(Ident{ctxt, sym, ..}) => {
                                if sym.clone() == attr {
                                    global_ctxt = ctxt.clone();
                                }
                            },
                            Expr::Bin(BinExpr{left, right, ..}) => {
                                if let Expr::Ident(Ident{ctxt, sym, ..}) = (&**left).clone() {
                                    if sym.clone() == attr {
                                        global_ctxt = ctxt.clone();
                                    }
                                } else if let Expr::Bin(BinExpr{left, right, ..}) = (&**left).clone() {
                                    if let Expr::Bin(BinExpr{left,..}) = (*left).clone() {
                                        match *left {
                                            Expr::Ident(Ident{ctxt, ..}) => {
                                                global_ctxt = ctxt.clone();
                                            },
                                            _ => {}
                                        }
                                    } 
                                    if let Expr::Ident(Ident{ctxt, sym, ..}) = (*left).clone() {
                                        if sym.clone() == attr {
                                            global_ctxt = ctxt.clone();
                                        }
                                    } 
                                    if let Expr::Ident(Ident{ctxt, sym, ..}) = (*right).clone() {
                                        if sym.clone() == attr {
                                            global_ctxt = ctxt.clone();
                                        }
                                    }
                                }
                                if let Expr::Ident(Ident{ctxt, sym, ..}) = (&**right).clone() {
                                    if sym.clone() == attr {
                                        global_ctxt = ctxt.clone();
                                    }
                                }
                            },
                            Expr::Call(CallExpr{ctxt, callee, args, ..}) => {
                                for arg in args {
                                    match &*arg.expr {
                                        Expr::Fn(FnExpr{function, ..}) => {
                                            if let Some(BlockStmt{
                                                stmts,
                                                ..
                                            }) = &(*function).body {
                                                stmts.into_iter().for_each(|stmt| {
                                                    match stmt {
                                                        Stmt::Return(ReturnStmt{arg, ..}) => {
                                                            match arg {
                                                                Some(expr) => {
                                                                    match &**expr {
                                                                        Expr::Ident(Ident{ctxt,..}) => {
                                                                            global_ctxt = ctxt.clone();
                                                                        },
                                                                        Expr::JSXElement(jsx_element) => {
                                                                            global_ctxt = get_jsx_element_child_ident_ctxt_by_attr(&jsx_element.children, &attr);
                                                                        },
                                                                        _ => {},
                                                                    }
                                                                },
                                                                _ => {},
                                                            }
                                                        },
                                                        _ => {}
                                                    }
                                                })   
                                            }
                                        },
                                        _ => {},
                                    }
                                }
                                if let Callee::Expr(expr) = callee{
                                    match &**expr {
                                        Expr::Ident(Ident{ctxt, ..}) => {
                                            global_ctxt = ctxt.clone();
                                        },
                                        Expr::Member(MemberExpr{obj, ..}) => {
                                            match &**obj {
                                                Expr::Fn(FnExpr{function, ..}) => {
                                                    global_ctxt = (**function).ctxt.clone();
                                                },
                                                _ => {},
                                            }
                                        }
                                        _ => {},
                                    }
                                } else {
                                    global_ctxt = ctxt.clone();
                                }
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                }
            },
            JSXElementChild::JSXSpreadChild(value) => {
                match value {
                    JSXSpreadChild { expr, ..} => {
                        match &**expr {
                            Expr::Ident(Ident{ctxt, sym, ..}) => {
                                if sym.clone() == attr {
                                    global_ctxt = ctxt.clone();
                                }
                            },
                            Expr::Bin(BinExpr{left, right, ..}) => {
                                if let Expr::Ident(Ident{ctxt, sym, ..}) = (**left).clone() {
                                    if sym.clone() == attr {
                                        global_ctxt = ctxt.clone();
                                    }
                                }
                                if let Expr::Ident(Ident{ctxt, sym, ..}) = (**right).clone() {
                                    if sym.clone() == attr {
                                        global_ctxt =ctxt.clone();
                                    }
                                }
                            },
                            _ => {},
                        }
                    },
                }
            },
            
            _ => {},
        }
    });
    global_ctxt
}