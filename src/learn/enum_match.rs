enum Gender {
  Male,
  #[allow(dead_code)]
  Female
}

// const MY_SCHOOL: &str = "Switch";

#[allow(dead_code)]
pub fn run() {
  let my_gender: Gender = Gender::Male;
  match my_gender {
    Gender::Male => println!("male"),
    Gender::Female => println!("female")
  }
}