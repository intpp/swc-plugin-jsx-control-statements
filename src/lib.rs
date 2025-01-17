use jsx_control_statements::visitor;
use swc_core::{
    ecma::{ast::*, visit::*},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.apply(visit_mut_pass(visitor::JSXControlStatements))
}
