use std::path::PathBuf;
use testing::fixture;
use swc_core::{
  common::FileName,
  ecma::transforms::testing::{ test_fixture, FixtureTestConfig }
};
use swc_plugin_react_server_component::react_server_component;

#[fixture("tests/fixture/server/**/input.js")]
fn fixture(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      react_server_component(FileName::Real("file_path.js".into()), true)
    },
    &input,
    &output,
    FixtureTestConfig {
      ..Default::default()
    },
  );
}
