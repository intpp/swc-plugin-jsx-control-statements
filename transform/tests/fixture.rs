use std::path::PathBuf;

use swc_core::ecma::{
        parser::{EsSyntax, Syntax},
        transforms::testing::{test_fixture, FixtureTestConfig},
    };

use jsx_control_statements::visitor::transform_jsx_control_statements;

fn syntax() -> Syntax {
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    })
}

#[testing::fixture("tests/fixture/**/input.js")]
fn jsx_control_statements_fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        syntax(),
        &|_| transform_jsx_control_statements(),
        &input,
        &output,
        FixtureTestConfig {
            allow_error: true,
            ..Default::default()
        },
    );
}
