use std::path::PathBuf;
use testing::fixture;
use swc_core::{
  common::FileName,
  ecma::transforms::testing::{ test_fixture, FixtureTestConfig }
};
use swc_plugin_react_server_component::react_server_component;

#[fixture("tests/fixture/server/**/input.js")]
fn fixture_server(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|t| {
      react_server_component(FileName::Real("file_path.js".into()), true, false,  t.comments.clone())
    },
    &input,
    &output,
    FixtureTestConfig {
      ..Default::default()
    },
  );
}

#[fixture("tests/fixture/client/**/input.js")]
fn fixture_client(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|t| {
      react_server_component(FileName::Real("file_path.js".into()), false, false, t.comments.clone())
    },
    &input,
    &output,
    FixtureTestConfig {
      ..Default::default()
    },
  );
}