//! 类似功能的crate: rustc_tools_util，rustc_tools_util的例子看clippy源码
use std::process::Command;

/*
/// compile-time env emmit from build.rs
const GIT_COMMIT_HASH: &str = env!("GIT_COMMIT_HASH");
const GIT_COMMIT_MESSAGE: &str = env!("GIT_COMMIT_MESSAGE");
const GIT_COMMIT_DATE: &str = env!("GIT_COMMIT_DATE");
const COMPILE_AT: &str = env!("COMPILE_AT");
const RUST_VERSION: &str = env!("RUST_VERSION");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Debug)]
pub struct ServerInfo {
    git_commit_hash: String,
    git_commit_message: String,
    git_commit_date: String,
    compile_at: String,
    rust_version: String,
    cargo_pkg_version: String,
}
*/
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd_output = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--pretty=format:%H,%ad,%s")
        .output()?;
    let last_commit_str = String::from_utf8(cmd_output.stdout)?;
    let commit_info = last_commit_str.split(',').collect::<Vec<&str>>();
    let (hash, date, message) = (commit_info[0], commit_info[1], commit_info[2]);
    println!("cargo:rustc-env=GIT_COMMIT_HASH={}", hash);
    println!("cargo:rustc-env=GIT_COMMIT_DATE={}", date);
    println!("cargo:rustc-env=GIT_COMMIT_MESSAGE={}", message);

    let date_output = String::from_utf8(Command::new("date").output()?.stdout)?;
    println!("cargo:rustc-env=COMPILE_AT={}", date_output);

    let rustc_output = String::from_utf8(Command::new("rustc").arg("--version").output()?.stdout)?;
    println!("cargo:rustc-env=RUST_VERSION={}", rustc_output);
    Ok(())
}

#[test]
fn process_pipe() -> Result<(), Box<dyn std::error::Error>> {
    let process = Command::new("ls")
        .arg("-l")
        .stdout(std::process::Stdio::piped())
        .env("DATABASE_URL", "null")
        .spawn()?;
    let pipe_grep_res = Command::new("grep")
        .arg("README.md")
        .stdin(process.stdout.unwrap())
        .output()?;
    println!("{}", String::from_utf8(pipe_grep_res.stdout)?);
    Ok(())
}
