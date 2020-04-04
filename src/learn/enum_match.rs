enum Gender {
  Male,
  #[allow(dead_code)]
  Female
}

#[allow(dead_code)]
impl Gender {
  fn is_male(&self) -> bool {
    match self {
      &Gender::Male => true,
      &Gender::Female => false,
    }
  }
}

#[allow(dead_code)]
pub fn run() {
  let my_gender: Gender = Gender::Male;
  match my_gender {
    Gender::Male => println!("male"),
    Gender::Female => println!("female")
  }
}