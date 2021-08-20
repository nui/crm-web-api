use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

use protobuf_codegen_pure::{Codegen, Customize};

fn get_rustc_version() -> String {
    let Output { stdout, .. } = Command::new("rustc").arg("--version").output().unwrap();
    String::from_utf8(stdout).unwrap()
}

/// Pass value to Rust compiler via environment variable.
/// Our code can access during compile time.
macro_rules! rustc_env {
    ($name:expr, $value:expr) => {
        println!("cargo:rustc-env={}={}", $name, $value);
    };
}

fn main() {
    let seconds_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    rustc_env!("BUILD_EPOCHSECONDS", seconds_since_epoch);
    if let Ok(id) = std::env::var("BUILD_VCS_NUMBER").or_else(|_| std::env::var("BUILD_COMMIT_ID"))
    {
        rustc_env!("BUILD_COMMIT_ID", id);
    }
    rustc_env!("BUILD_TARGET", std::env::var("TARGET").unwrap());
    rustc_env!("BUILD_RUSTC_VERSION", get_rustc_version());
    let profile = std::env::var("PROFILE").unwrap();
    rustc_env!("BUILD_PROFILE", profile);
    // combine! macro rely on this
    println!(r#"cargo:rustc-cfg=build_profile="{}""#, profile);

    // compile protobuf
    if let Err(e) = Codegen::new()
        .out_dir("src/protos")
        .inputs(&["protos/token.proto"])
        .includes(&["protos"])
        .customize(Customize {
            ..Default::default()
        })
        .run()
    {
        println!(
            "cargo:warning=fail to generate protobuf file from source: {:?}",
            e
        );
    }
}
