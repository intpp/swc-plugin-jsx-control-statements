use std::path::PathBuf;

use swc_core::common::Mark;
use swc_core::ecma::{
    ast::Pass,
    parser::{EsSyntax, Syntax},
    transforms::base::resolver,
    transforms::testing::{test_fixture, FixtureTestConfig},
    visit::visit_mut_pass,
};

use jsx_control_statements::visitor::JSXControlStatements;

fn syntax() -> Syntax {
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    })
}

// https://swc.rs/docs/plugin/ecmascript/cheatsheet#apply-resolver-while-testing
fn tr() -> impl Pass {
    (
        resolver(Mark::new(), Mark::new(), false),
        // Most of transform does not care about globals so it does not need `SyntaxContext`
        visit_mut_pass(JSXControlStatements),
    )
}

#[testing::fixture("tests/fixture/**/input.js")]
fn jsx_control_statements_fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        syntax(),
        &|_| tr(),
        &input,
        &output,
        FixtureTestConfig {
            allow_error: true,
            ..Default::default()
        },
    );
}
