use swc_common::{SyntaxContext, DUMMY_SP};
use swc_core::atoms::Atom;
use swc_core::ecma::ast::JSXElement;
use swc_core::ecma::ast::*;

use crate::utils::attributes::{get_key_attribute, get_for_jsx_element_attributes_expression};
use crate::utils::elements::convert_children_to_expression;

pub fn convert_jsx_element(jsx_element: &mut JSXElement) -> Expr {
  
  let (of_expr, params) = parse_for_jsx_element(jsx_element);
  
  let group_key = get_key_attribute(jsx_element);
  let children_expr= convert_children_to_expression(
    &mut jsx_element.children,
    group_key.clone(),
  );

  Expr::Call(CallExpr{
    span: DUMMY_SP,
    ctxt: SyntaxContext::empty(),
    callee: Callee::Expr(
      Box::new(
        Expr::Member(MemberExpr {
          span: DUMMY_SP,
          obj: Box::new(
            of_expr
          ),
          prop: MemberProp::Ident(Ident::from(Atom::from("map")).into())
        })
      )
    ),
    args: vec![
        ExprOrSpread {
          spread: None,
          expr: Box::new(
            Expr::Fn(FnExpr {
              ident: Default::default(),
              function: Box::new(Function {
                params,
                decorators: Default::default(),
                span: DUMMY_SP,
                ctxt: SyntaxContext::empty(),
                body: Some(BlockStmt {
                  span: DUMMY_SP,
                  ctxt: SyntaxContext::empty(),
                  stmts: vec![
                    Stmt::Return(ReturnStmt {
                      span: DUMMY_SP,
                      arg: Some(Box::new(
                        children_expr
                      )),
                    })
                  ],
                }),
                is_generator: Default::default(),
                is_async: Default::default(),
                type_params: Default::default(),
                return_type: Default::default(),
              })
            })
          )
        },
        ExprOrSpread {
          spread: None,
          expr: Box::new(Expr::This(ThisExpr {
            span: DUMMY_SP,
          }))
        }
      ],
    type_args: Default::default(),
  })
}

pub fn parse_for_jsx_element(jsx_element: &mut JSXElement) -> (Expr, Vec<Param>) { 
  let mut params = Vec::new();
  
  let each_expr = get_for_jsx_element_attributes_expression(&jsx_element, "each");
  let index_expr = get_for_jsx_element_attributes_expression(&jsx_element, "index");
  let of_expr = get_for_jsx_element_attributes_expression(&jsx_element, "of");
  
  params.push(
    Param {
      span: DUMMY_SP,
      decorators: Default::default(),
      pat: Pat::Ident(
        BindingIdent{
          id: match each_expr {
              Expr::Ident(Ident{sym, ..}) => {
                Ident::new(sym, DUMMY_SP, SyntaxContext::empty())
              },
              _ => {
                Ident::new("_".into(), DUMMY_SP, SyntaxContext::empty())
              },
          },
          type_ann: Default::default(),
        }
      )
    }
  );
  params.push(
    Param {
      span: DUMMY_SP,
      decorators: Default::default(),
      pat: Pat::Ident(
        BindingIdent{
          id: match index_expr {
              Expr::Ident(Ident{sym, ..}) => {
                Ident::new(sym, DUMMY_SP, SyntaxContext::empty())
              },
              _ => {
                Ident::new("_".into(), DUMMY_SP, SyntaxContext::empty())
              },
          },
          type_ann: Default::default(),
        }
      )
    }
  );
  
  (of_expr, params)
}