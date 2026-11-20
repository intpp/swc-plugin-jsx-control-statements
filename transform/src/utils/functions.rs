use std::collections::HashMap;

use swc_core::{
    atoms::Atom,
    common::{Mark, SyntaxContext, DUMMY_SP},
    ecma::{
        ast::*,
        utils::ts_bindings::TsBindings,
        visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
    },
};

struct ParameterReferences {
    parameters: HashMap<Atom, SyntaxContext>,
    locals: TsBindings,
}

impl VisitMut for ParameterReferences {
    noop_visit_mut_type!();

    fn visit_mut_ident(&mut self, ident: &mut Ident) {
        if let Some(context) = self.parameters.get(&ident.sym) {
            if self.locals.ident_declaration(ident).is_none() {
                ident.ctxt = *context;
            }
        }
    }
}

pub fn returning_function(mut parameters: Vec<Ident>, mut result: Expr) -> Expr {
    // SWC has already resolved the input. Bind free references in the children
    // to our new parameters, preserving bindings declared inside the children.
    let mut contexts = HashMap::new();
    for parameter in &mut parameters {
        // An index-only <For> has a private placeholder for its first argument.
        if parameter.ctxt == SyntaxContext::empty() {
            parameter.ctxt = SyntaxContext::empty().apply_mark(Mark::new());
            contexts.insert(parameter.sym.clone(), parameter.ctxt);
        }
    }
    if !contexts.is_empty() {
        let locals = TsBindings::collect(&result);
        result.visit_mut_with(&mut ParameterReferences {
            parameters: contexts,
            locals,
        });
    }

    Expr::Fn(FnExpr {
        ident: None,
        function: Box::new(Function {
            params: parameters
                .into_iter()
                .map(|id| Param::from(Pat::from(id)))
                .collect(),
            body: Some(FunctionBody {
                span: DUMMY_SP,
                stmts: vec![Stmt::Return(ReturnStmt {
                    span: DUMMY_SP,
                    arg: Some(Box::new(result)),
                })],
            }),
            ..Default::default()
        }),
    })
}
