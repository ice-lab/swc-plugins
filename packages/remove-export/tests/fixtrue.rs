use std::path::PathBuf;
use swc_ecma_transforms_testing::{test, test_fixture};
use swc_plugin_remove_export::{remove_export_exprs};

#[testing::fixture("tests/fixture/base/input.js")]
fn fixture_base(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      remove_export_exprs([String::from("getData")].to_vec())
    },
    &input,
    &output,
  );
}
