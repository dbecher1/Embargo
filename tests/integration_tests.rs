use embargo::{cli::Args, run};
use assert_fs::prelude::*;
use serial_test::serial;

use crate::common::setup;

mod common;

#[test]
#[serial]
fn test_new() {
    setup();
    let test_project_name = "test_project";
    let args = Args::debug_new(test_project_name);
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let temp_dir_path = temp_dir.path();

    assert!(run(args, Some(temp_dir_path)).is_ok());

    let project_dir = temp_dir.child(test_project_name);
    project_dir.assert(predicates::path::exists());
    project_dir.child("Embargo.toml").assert(predicates::path::exists());
    
    let src = project_dir.child("src");
    src.assert(predicates::path::is_dir());
    src.child("main.cpp").assert(predicates::path::exists());
    assert!(temp_dir.close().is_ok());
    
} 

#[test]
#[serial]
fn test_build() {
    setup();
    // Might consolidate this in the future
    // WIP
    /* let test_project_name = "test_project";
    let args_new = Args::debug_new(test_project_name);
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let temp_dir_path = temp_dir.path();
    assert!(run(args_new, Some(temp_dir_path)).is_ok());

    let project_dir = temp_dir.child(test_project_name);
    let src = project_dir.child("src");
    src.assert(predicates::path::is_dir());
    src.child("main.cpp").assert(predicates::path::exists());

    let args_build = Args::debug_build();
    let binding = temp_dir.child(test_project_name);
    let path = binding.path();
    assert!(run(args_build, Some(path)).is_ok());
    assert!(temp_dir.close().is_ok()); */
}