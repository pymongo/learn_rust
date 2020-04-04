use std::process::Command;

#[allow(dead_code)]
pub fn run() {
  let mut cmd = Command::new("df");
  cmd.arg("-h");
  match cmd.output() {
    Ok(output) => {
      unsafe {
        println!("{}", String::from_utf8_unchecked(output.stdout));
      }
    },
    Err(e) => println!("{}", e)
  }
}