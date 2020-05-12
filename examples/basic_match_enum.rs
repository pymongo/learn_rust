enum Gender {
    Male,
    #[allow(dead_code)]
    Female,
}

impl Gender {
    fn is_male(&self) -> bool {
        match self {
            &Gender::Male => true,
            &Gender::Female => false,
        }
    }
}

pub fn main() {
    let gender: Gender = Gender::Male;
    println!("gender.is_male() = {}", gender.is_male());
}
