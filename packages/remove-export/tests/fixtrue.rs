use std::path::PathBuf;
use swc_core::{
  ecma::parser::{EsConfig, Syntax},
  ecma::transforms::testing::{test_fixture, FixtureTestConfig},
};
use testing::fixture;
use swc_plugin_remove_export::{remove_export_exprs};

#[fixture("tests/fixture/base/input.js")]
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
    FixtureTestConfig {
      ..Default::default()
    },
  );
}

#[fixture("tests/fixture/preserveConfig/**/input.js")]
fn fixture_preserve_config(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Syntax::Es(EsConfig {
      decorators: true,
      jsx: true,
      ..Default::default()
    }),
    &|_t| {
      remove_export_exprs([String::from("getData"), String::from("default")].to_vec())
    },
    &input,
    &output,
    FixtureTestConfig {
      ..Default::default()
    },
  );
}

#[fixture("tests/fixture/preserveConfigAndDefault/**/input.js")]
fn fixture_preserve_config_and_config(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Syntax::Es(EsConfig {
      decorators: true,
      jsx: true,
      ..Default::default()
    }),
    &|_t| {
      remove_export_exprs([String::from("getData")].to_vec())
    },
    &input,
    &output,
    FixtureTestConfig {
      ..Default::default()
    },
  );
}

#[fixture("tests/fixture/preserveData/**/input.js")]
fn fixture_preserve_data(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Syntax::Es(EsConfig {
      decorators: true,
      jsx: true,
      ..Default::default()
    }),
    &|_t| {
      remove_export_exprs([String::from("getConfig"), String::from("default")].to_vec())
    },
    &input,
    &output,
    FixtureTestConfig {
      ..Default::default()
    },
  );
}

#[fixture("tests/fixture/preserveDefault/**/input.js")]
fn fixture_preserve_default(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Syntax::Es(EsConfig {
      decorators: true,
      jsx: true,
      ..Default::default()
    }),
    &|_t| {
      remove_export_exprs([String::from("getConfig"), String::from("getData")].to_vec())
    },
    &input,
    &output,
    FixtureTestConfig {
      ..Default::default()
    },
  );
}

#[fixture("tests/fixture/removeData/**/input.js")]
fn fixture_remove_data(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Syntax::Es(EsConfig {
      decorators: true,
      jsx: true,
      ..Default::default()
    }),
    &|_t| {
      remove_export_exprs([String::from("getData")].to_vec())
    },
    &input,
    &output,
    FixtureTestConfig {
      ..Default::default()
    },
  );
}