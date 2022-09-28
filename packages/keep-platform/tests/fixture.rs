use std::path::PathBuf;
use swc_core::{
  ecma::transforms::testing::{test, test_fixture},
};
use swc_plugin_keep_platform::{keep_platform, KeepPlatformConfig};

#[testing::fixture("tests/fixture/empty/input.js")]
fn fixture_empty(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      keep_platform(KeepPlatformConfig::KeepPlatform(String::from("web")))
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/kraken/input.js")]
fn fixture_kraken(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      keep_platform(KeepPlatformConfig::KeepPlatform(String::from("kraken")))
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/namespace/kraken/input.js")]
fn fixture_namespace_kraken(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      keep_platform(KeepPlatformConfig::KeepPlatform(String::from("kraken")))
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/namespace/web/input.js")]
fn fixture_namespace_web(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      keep_platform(KeepPlatformConfig::KeepPlatform(String::from("web")))
    },
    &input,
    &output,
  );
}


#[testing::fixture("tests/fixture/web/input.js")]
fn fixture_web(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      keep_platform(KeepPlatformConfig::KeepPlatform(String::from("web")))
    },
    &input,
    &output,
  );
}


#[testing::fixture("tests/fixture/namedexport/input.js")]
fn fixture_named_export(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      keep_platform(KeepPlatformConfig::KeepPlatform(String::from("web")))
    },
    &input,
    &output,
  );
}

#[testing::fixture("tests/fixture/defaultexport/input.js")]
fn fixture_default_export(input: PathBuf) {
  let parent = input.parent().unwrap();
  let output = parent.join("output.js");

  test_fixture(
    Default::default(),
    &|_t| {
      keep_platform(KeepPlatformConfig::KeepPlatform(String::from("web")))
    },
    &input,
    &output,
  );
}
