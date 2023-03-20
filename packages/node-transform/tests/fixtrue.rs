use std::path::PathBuf;
use swc_core::{
  ecma::transforms::testing::{test_fixture, FixtureTestConfig},
};
use testing::fixture;
use swc_plugin_node_transform::{node_transform};

#[fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      node_transform()
    },
    &input,
    &output,
    FixtureTestConfig {
      ..Default::default()
    },
  );
}
