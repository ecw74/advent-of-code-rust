use std::env;
use std::process::Command;

fn main() {
    let build_date = env::var("BUILD_DATE").unwrap_or_else(|_| {
        let output = Command::new("date")
            .arg("+%Y-%m-%dT%H:%M:%S")
            .output()
            .expect("Failed to execute date command");
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    });

    let commit_hash = env::var("COMMIT_HASH").unwrap_or_else(|_| {
        let output = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .output()
            .expect("Failed to execute git command");
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    });

    let commit_hash_short = env::var("COMMIT_HASH_SHORT").unwrap_or_else(|_| {
        let output = Command::new("git")
            .args(&["rev-parse", "--short", "HEAD"])
            .output()
            .expect("Failed to execute git command");
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    });

    println!("cargo:rustc-env=BUILD_DATE={}", build_date);
    println!("cargo:rustc-env=COMMIT_HASH={}", commit_hash);
    println!("cargo:rustc-env=COMMIT_HASH_SHORT={}", commit_hash_short);
}
