use std::process::Command;
use std::fs::File;
use std::io::Write;
// use std::env;

fn main() {
    let mut file = File::create("build_debug.log").expect("Failed to create log file");
    let profile = std::env::var("PROFILE").unwrap();
    println!("cargo:warning=Running build script for profile: {}", profile);
    writeln!(file, "Running build script for profile: {}", profile).expect("Failed to write to log file");    // Embedding the build date
    let output = Command::new("date")
                        .arg("+%Y-%m-%d %H:%M:%S")
                        .output()
                        .expect("Failed to execute date command");
    let build_date = String::from_utf8(output.stdout).expect("Output was not UTF-8").trim().to_string();
    println!("cargo:rustc-env=BUILD_DATE={}", build_date);
    writeln!(file, "Running build date: {}", build_date).expect("Failed to write to log file");    // Embedding the build date


    // Embedding the last git commit hash
    let output = Command::new("git")
                        .args(["rev-parse", "HEAD"])
                        .output()
                        .expect("Failed to execute git command");
    let git_hash = String::from_utf8(output.stdout).expect("Output was not UTF-8").trim().to_string();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    writeln!(file, "GIT_HASH: {}", git_hash).expect("Failed to write to log file");    // Embedding the build date


    // Additional setup could go here, such as configuring paths or setting other environment variables
}
