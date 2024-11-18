use std::path::PathBuf;
use std::sync::Arc;

use swc_core::common::SourceMap;
use swc_core::ecma::codegen::text_writer::JsWriter;
use swc_core::ecma::codegen::Emitter;
use swc_core::ecma::transforms::testing::{test_fixture, FixtureTestConfig};
use swc_core::ecma::visit::as_folder;
use swc_plugin_remove_side_effect::TransformVisitor;
use testing::fixture;

#[fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let parent = input.parent().unwrap();
    let output = parent.join("output.js");

    println!("Processing file: {:?}", input);

    test_fixture(
        Default::default(),
        &|t| {
            let program = t.try_get_program().expect("failed to get program");

            let srcmap = Arc::new(SourceMap::default());
            let mut buf = vec![];
            let writer = JsWriter::new(srcmap.clone(), "\n", &mut buf, None);
            let mut emitter = Emitter {
                cfg: swc_core::ecma::codegen::Config::default(),
                cm: srcmap,
                comments: None,
                wr: writer,
            };

            emitter.emit_program(&program).unwrap();
            println!("Transform result:\n{}", String::from_utf8_lossy(&buf));

            as_folder(TransformVisitor)
        },
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}
