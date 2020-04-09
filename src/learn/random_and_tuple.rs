#[cfg(feature = "unused")]
pub fn run() {
  extern crate rand;
  use rand::Rng;
  let tuple = (0, 1.1, (false, "1"));
  println!("{}", (tuple.2).1);
  let (a, b) = tuple.2;
  println!("a={}, b={}", a, b);
  let random_number: u8 = rand::thread_rng().gen_range(1, 101);
// thread_rng() function to get a copy of the random number generator, which is local to the particular thread of execution we’re in
// Because we use rand::Rng’d above, it has a gen_range() method available
  println!("{}", random_number);
  match 50.cmp(&random_number) {
    std::cmp::Ordering::Less => println!("Too small!"),
    std::cmp::Ordering::Greater => println!("Too big!"),
    std::cmp::Ordering::Equal => println!("You win!"),
  }
}