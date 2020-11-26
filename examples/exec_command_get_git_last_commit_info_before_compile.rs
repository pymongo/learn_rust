fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd_output = std::process::Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--pretty=format:%H,%ad,%s")
        .output()?;
    let last_commit_str = String::from_utf8(cmd_output.stdout)?;
    let commit_info = last_commit_str.split(',').collect::<Vec<&str>>();
    let (hash, date, message) = (commit_info[0], commit_info[1], commit_info[2]);
    dbg!(hash, date, message);
    // if in build.rs, emit git_last_commit_info to compile time and retired use env! as const &st
    // println!("cargo:rustc-env={}={}", "LAST_COMMIT_HASH", hash);
    // println!("cargo:rustc-env={}={}", "LAST_COMMIT_DATE", date);
    // println!("cargo:rustc-env={}={}", "LAST_COMMIT_MESSAGE", message);
    Ok(())
}

#[test]
fn process_pipe() -> Result<(), Box<dyn std::error::Error>> {
    let process = std::process::Command::new("ls")
        .arg("-l")
        .stdout(std::process::Stdio::piped())
        .env("DATABASE_URL", "null")
        .spawn()?;
    let pipe_grep_res = std::process::Command::new("grep")
        .arg("README.md")
        .stdin(process.stdout.unwrap())
        .output()?;
    println!("{}", String::from_utf8(pipe_grep_res.stdout)?);
    Ok(())
}
