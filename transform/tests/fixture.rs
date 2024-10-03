use std::path::PathBuf;

use swc_core::ecma::{
    parser::{EsSyntax, Syntax},
    transforms::{
        // base::resolver,
        testing::{test_fixture, FixtureTestConfig},
    },
    visit::as_folder,
};

use jsx_control_statements::visitor::JSXControlStatements;

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
        &|_| as_folder(JSXControlStatements),
        &input,
        &output,
        FixtureTestConfig {
            allow_error: true,
            ..Default::default()
        },
    );
}
