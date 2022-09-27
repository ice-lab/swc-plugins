use std::path::PathBuf;
use swc_core::{
  ecma::transforms::testing::{test, test_fixture},
};
use swc_plugin_keep_export::{keep_exprs};

#[testing::fixture("tests/fixture/base/input.js")]
fn fixture_base(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      keep_exprs([String::from("getData")].to_vec())
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/remove-unused-code/input.js")]
fn fixture_remove_unused_code(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      keep_exprs([String::from("getData")].to_vec())
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/keep-referenced-code/input.js")]
fn fixture_keep_referenced_code(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      keep_exprs([String::from("getData")].to_vec())
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/keep-default-decl/input.js")]
fn fixture_default_decl(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      keep_exprs([String::from("default")].to_vec())
    },
    &input,
    &output,
  );
}
#[testing::fixture("tests/fixture/keep-default-expr/input.js")]
fn fixture_default_expr(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      keep_exprs([String::from("default")].to_vec())
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/remove-all/input.js")]
fn fixture_remove_all(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      keep_exprs([String::from("getServerData")].to_vec())
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/remove-top-expr/input.js")]
fn fixture_remove_top_expr(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      keep_exprs([String::from("getData")].to_vec())
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/remove-top-func/input.js")]
fn fixture_remove_top_func(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      keep_exprs([String::from("getData")].to_vec())
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/remove-side-effect-import/input.js")]
fn fixture_remove_side_effect_import(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      keep_exprs([String::from("getData")].to_vec())
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/remove-top-if-state/input.js")]
fn fixture_remove_top_if_state(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      keep_exprs([String::from("getData")].to_vec())
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/remove-top-do-while/input.js")]
fn fixture_remove_top_do_while(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      keep_exprs([String::from("getData")].to_vec())
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/remove-top-try-catch/input.js")]
fn fixture_remove_top_try_catch(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      keep_exprs([String::from("getData")].to_vec())
    },
    &input,
    &output,
  );
}