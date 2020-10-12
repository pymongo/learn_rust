#[derive(serde::Deserialize, Debug)]
struct CommitInfo {
    hash: String,
    date: String,
    message: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd_res = std::process::Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--pretty=format:hash=\"%h\"\ndate=\"%ad\"\nmessage=\"%s\"")
        .env("DATABASE_URL", "null")
        .output()?;
    let last_commit_str = String::from_utf8(cmd_res.stdout)?;
    let last_commit: CommitInfo = toml::de::from_str(&last_commit_str)?;
    dbg!(&last_commit);
    // if in build.rs, emit git_last_commit_info to compile time and retired use env! as const &st
    // println!("cargo:rustc-env={}={}", "LAST_COMMIT_HASH", last_commit.hash);
    // println!("cargo:rustc-env={}={}", "LAST_COMMIT_DATE", last_commit.date);
    // println!("cargo:rustc-env={}={}", "LAST_COMMIT_MESSAGE", last_commit.message);
    Ok(())
}
