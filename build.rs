use std::process::Command;
use std::env;

fn main() {
    // Embedding the build date
    let output = Command::new("date")
                        .arg("+%Y-%m-%d %H:%M:%S")
                        .output()
                        .expect("Failed to execute date command");
    let build_date = String::from_utf8(output.stdout).expect("Output was not UTF-8").trim().to_string();
    println!("cargo:rustc-env=BUILD_DATE={}", build_date);

    // Embedding the last git commit hash
    let output = Command::new("git")
                        .args(["rev-parse", "HEAD"])
                        .output()
                        .expect("Failed to execute git command");
    let git_hash = String::from_utf8(output.stdout).expect("Output was not UTF-8").trim().to_string();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    // Additional setup could go here, such as configuring paths or setting other environment variables
}
