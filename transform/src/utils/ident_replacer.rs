use swc_core::{
    atoms::Atom,
    common::SyntaxContext,
    ecma::{
        ast::{
            ClassMethod, ClassProp, Ident, KeyValueProp, MemberExpr, MemberProp, PropName,
            VarDeclarator,
        },
        visit::{VisitMut, VisitMutWith},
    },
};

/// to keep ident same ctxt as `each`, `index` in `<For>` tag.
pub struct IdentReplacer {
    pub target_sym: Atom,
    pub target_ctxt: SyntaxContext,
}

impl VisitMut for IdentReplacer {
    // make same ctxt
    fn visit_mut_ident(&mut self, node: &mut Ident) {
        if node.sym == self.target_sym {
            node.ctxt = self.target_ctxt;
        }
        node.visit_mut_children_with(self);
    }

    // exclude MemberExpr (obj.item)
    fn visit_mut_member_expr(&mut self, node: &mut MemberExpr) {
        // visit object parts (e.g `this.props` in this.props.list)
        node.obj.visit_mut_with(self);

        // only key of Compuetd
        if let MemberProp::Computed(c) = &mut node.prop {
            c.visit_mut_with(self);
        }

        // no need to visit MemberProp::Ident
    }

    // exclude Object KeyValue ({ item: value })
    fn visit_mut_key_value_prop(&mut self, node: &mut KeyValueProp) {
        // visit value
        node.value.visit_mut_with(self);

        // only key of Compuetd
        if let PropName::Computed(c) = &mut node.key {
            c.visit_mut_with(self);
        }
        // no need to visit MemberProp::Ident
    }

    // exclude class { item() {} }
    fn visit_mut_class_method(&mut self, n: &mut ClassMethod) {
        // visit function body
        n.function.visit_mut_with(self);

        // like KeyValue, but only key of Compuetd
        if let PropName::Computed(c) = &mut n.key {
            c.visit_mut_with(self);
        }
        // no need to visit MemberProp::Ident
    }

    // exclude class { item = 1 }
    fn visit_mut_class_prop(&mut self, n: &mut ClassProp) {
        n.value.visit_mut_with(self);

        if let PropName::Computed(c) = &mut n.key {
            c.visit_mut_with(self);
        }
        // no need to visit MemberProp::Ident
    }

    // exclude const item = xxx
    fn visit_mut_var_declarator(&mut self, node: &mut VarDeclarator) {
        // ignore `id`, only init
        node.init.visit_mut_with(self);
    }
}

