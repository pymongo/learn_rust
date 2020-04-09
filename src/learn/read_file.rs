#[cfg(feature = "unused")]
use std::fs::File;
#[cfg(feature = "unused")]
use std::io::prelude::*;

#[cfg(feature = "unused")]
pub fn read_file() {
  let mut file = File::open("./src/enum_match.rs").expect("Open Failed");

  let mut content = String::new();
  file.read_to_string(&mut content).expect("Can't read file");

  println!("{}", content);
}

#[cfg(feature = "unused")]
fn write_a_file() {
  /* Write to file */
  let mut write_file = File::create("output.txt").expect("Error");
  // b"" means byte slice
  write_file.write_all(b"Haha").expect("Error");
}
#[allow(dead_code)]
fn string_to_char() {
  let string_to_char = String::from("abc");
  match string_to_char.chars().nth(4) {
    Some(c) => println!("{}", c),
    None => println!("Index out of range")
  }
}