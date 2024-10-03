use swc_common::{SyntaxContext, DUMMY_SP};
use swc_core::atoms::Atom;
use swc_core::ecma::ast::JSXElement;
use swc_core::ecma::ast::*;

use crate::utils::attributes::{get_key_attribute, get_for_jsx_element_attributes_expr, get_for_jsx_element_attributes_ident};
use crate::utils::elements::convert_children_to_expression;
pub fn convert_jsx_element(jsx_element: &mut JSXElement) -> Expr {

  let (of_expr, body_expr, params) = parse_for_jsx_element(jsx_element.clone());
  
  let group_key = get_key_attribute(jsx_element);
  let children_expr= convert_children_to_expression(
    &mut jsx_element.children,
    group_key.clone(),
  );

  let mut map_args_fn = ExprOrSpread {
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
                  children_expr.clone()
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
  };

  let mut expr = get_call_expr(map_args_fn, &of_expr);
  if body_expr != Expr::Invalid(Invalid{span: DUMMY_SP}) {
    map_args_fn = ExprOrSpread {
      spread: None,
      expr: Box::new(
        body_expr
      )
    };
    expr = get_call_expr(map_args_fn, &of_expr);
  } else if children_expr == Expr::Lit(Lit::Null(Null{span: DUMMY_SP})) {
    expr = Expr::Lit(Lit::Null(Null{span: DUMMY_SP}));
  }
  expr

}

pub fn parse_for_jsx_element(jsx_element: JSXElement) -> (Expr, Expr, Vec<Param>) { 
  let mut params = Vec::new();
  
  let each_ident = get_for_jsx_element_attributes_ident(&jsx_element, "each");
  let index_ident = get_for_jsx_element_attributes_ident(&jsx_element, "index");
  let mut of_expr = get_for_jsx_element_attributes_expr(&jsx_element, "of");
  let body_expr = get_for_jsx_element_attributes_expr(&jsx_element, "body");

  if of_expr == Expr::Invalid(Invalid{span: DUMMY_SP}) {
    of_expr = Expr::Ident(Ident::from(Atom::from("[]")).into());
  }

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
  (of_expr, body_expr, params)
}

fn get_call_expr(map_args_fn: ExprOrSpread, of_expr: &Expr) -> Expr {
  Expr::Call(CallExpr{
    span: DUMMY_SP,
    ctxt: SyntaxContext::empty(),
    callee: Callee::Expr(
      Box::new(
        Expr::Member(MemberExpr {
          span: DUMMY_SP,
          obj: Box::new(
            of_expr.clone()
          ),
          prop: MemberProp::Ident(Ident::from(Atom::from("map")).into())
        })
      )
    ),
    args: vec![
        map_args_fn.clone(),
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