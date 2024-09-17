use swc_common::{Span, SyntaxContext, DUMMY_SP};
use swc_core::atoms::Atom;
use swc_core::ecma::ast::JSXElement;
use swc_core::ecma::ast::*;

use crate::utils::attributes::{get_key_attribute, get_for_jsx_element_attributes_expr, get_for_jsx_element_attributes_ident};
use crate::utils::elements::convert_children_to_expression;
pub fn convert_jsx_element(jsx_element: &mut JSXElement) -> Expr {

  let (of_expr, params) = parse_for_jsx_element(jsx_element.clone());
  
  let group_key = get_key_attribute(jsx_element);
  let children_expr= convert_children_to_expression(
    &mut jsx_element.children,
    group_key.clone(),
  );

  match children_expr {
      Expr::Lit(Lit::Null(Null{..})) => {
        return Expr::Lit(Lit::Null(Null{span: DUMMY_SP}));
      },
      _ => {
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
  }
}

pub fn parse_for_jsx_element(jsx_element: JSXElement) -> (Expr, Vec<Param>) { 
  let mut params = Vec::new();
  
  let each_ident = get_for_jsx_element_attributes_ident(&jsx_element, "each");
  let index_ident = get_for_jsx_element_attributes_ident(&jsx_element, "index");
  let of_expr = get_for_jsx_element_attributes_expr(&jsx_element, "of");

  params.push(
    Param {
      span: DUMMY_SP,
      decorators: Default::default(),
      pat: Pat::Ident(
        BindingIdent{
          id: each_ident,
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
          id: index_ident,
          type_ann: Default::default(),
        }
      )
    }
  );  
  (of_expr, params)
}