#[derive(serde::Deserialize, Debug)]
struct CommitInfo {
    hash: String,
    date: String,
    message: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd_output = std::process::Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--pretty=format:hash=\"%h\"\ndate=\"%ad\"\nmessage=\"%s\"")
        .env("DATABASE_URL", "null")
        .output()?;
    let last_commit_str = String::from_utf8(cmd_output.stdout)?;
    let last_commit: CommitInfo = toml::de::from_str(&last_commit_str)?;
    dbg!(&last_commit);
    // if in build.rs, emit git_last_commit_info to compile time and retired use env! as const &st
    // println!("cargo:rustc-env={}={}", "LAST_COMMIT_HASH", last_commit.hash);
    // println!("cargo:rustc-env={}={}", "LAST_COMMIT_DATE", last_commit.date);
    // println!("cargo:rustc-env={}={}", "LAST_COMMIT_MESSAGE", last_commit.message);
    Ok(())
}

#[test]
fn process_pipe() -> Result<(), Box<dyn std::error::Error>> {
    let process = std::process::Command::new("ls")
        .arg("-l")
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    let pipe_grep_res = std::process::Command::new("grep")
        .arg("README.md")
        .stdin(process.stdout.unwrap())
        .output()?;
    println!("{}", String::from_utf8(pipe_grep_res.stdout)?);
    Ok(())
}
