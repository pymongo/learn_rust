enum Gender {
  Male,
  Female,
}

const MY_SCHOOL: &str = "Switch";

pub fn run() {
  let my_gender: Gender = Gender::Male;
  match my_gender {
    Gender::Male => println!("male"),
    Gender::Female => println!("female")
  }
}