use std::path::PathBuf;
use swc_core::{
  ecma::transforms::testing::{test, test_fixture},
};
use swc_plugin_node_transform::{node_transform};

#[testing::fixture("tests/fixture/base/input.js")]
fn fixture_base(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      node_transform()
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/namespace/input.js")]
fn fixture_namespace(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      node_transform()
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/multi/input.js")]
fn fixture_multi(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      node_transform()
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/function/input.js")]
fn fixture_export_function(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      node_transform()
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/class/input.js")]
fn fixture_export_class(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      node_transform()
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/var/input.js")]
fn fixture_export_var(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      node_transform()
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/named/input.js")]
fn fixture_export_named(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      node_transform()
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/named-from/input.js")]
fn fixture_export_named_from(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      node_transform()
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/named-export/input.js")]
fn fixture_export_named_export(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      node_transform()
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/export-all/input.js")]
fn fixture_export_all(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      node_transform()
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/export-all-as/input.js")]
fn fixture_export_all_as(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      node_transform()
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/export-default/input.js")]
fn fixture_export_default(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      node_transform()
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/mix-import-export/input.js")]
fn fixture_mix_import_export(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      node_transform()
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/export-decl/input.js")]
fn fixture_export_default_decl(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      node_transform()
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/export-class/input.js")]
fn fixture_export_default_class(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      node_transform()
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/export-anonymous/input.js")]
fn fixture_export_default_anonymous(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      node_transform()
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/dynamic-import/input.js")]
fn fixture_dynamic_import(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      node_transform()
    },
    &input,
    &output,
  );
}


#[testing::fixture("tests/fixture/meta/input.js")]
fn fixture_meta(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      node_transform()
    },
    &input,
    &output,
  );
}