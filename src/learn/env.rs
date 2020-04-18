pub fn run() {
  // 下面三个应该是在Cargo.toml里面定义的
  // println!("HOST = {}", env!("HOST"));
  // println!("TARGET = {}", env!("TARGET"));
  // println!("PROFILE = {}", env!("PROFILE"));
  println!("CARGO = {}", env!("CARGO"));
  println!("CARGO_MANIFEST_DIR = {}", env!("CARGO_MANIFEST_DIR"));
}
