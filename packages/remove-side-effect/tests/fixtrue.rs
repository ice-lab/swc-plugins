use std::path::PathBuf;
use swc_core::ecma::parser::{EsConfig, Syntax};
use swc_core::ecma::transforms::testing::test_fixture;
use swc_core::ecma::visit::as_folder;
use swc_plugin_remove_side_effect::TransformVisitor;
use testing::fixture;

#[fixture("tests/fixture/**/input.js")]
fn fixture_test(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        Syntax::Es(EsConfig {
            jsx: true,
            decorators: true,
            ..Default::default()
        }),
        &|_| as_folder(TransformVisitor::new()),
        &input,
        &output,
        Default::default(),
    );
}
