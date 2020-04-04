pub fn unwarp_and_option() {
  // unwrap raise panic! when receive a None
  println!("{}", get_gender("Male").unwrap())
}

fn get_gender(gender: &str) -> Option<&str> {
  match gender {
    "Male" => Some("return male"),
    "Female" => Some("return female"),
    _ => None
  }
}
