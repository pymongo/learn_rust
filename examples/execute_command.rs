use std::process::Command;

fn main() {
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