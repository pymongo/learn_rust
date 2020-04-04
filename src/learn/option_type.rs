pub fn get_gender(gender: &str) -> Option<&str> {
  match gender {
    "Male" => Some("return male"),
    "Female" => Some("return female"),
    _ => None
  }
}