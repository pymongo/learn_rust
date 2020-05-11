
use std::io;
use std::cmp::Ordering;
use rand::Rng;

#[allow(dead_code)]
pub fn run() {
  println!("Guess the number!(1-100)");

  let secret_number = rand::thread_rng().gen_range(1, 101);

  loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
      .ok()
      .expect("failed to read line");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Please input a number!");
        continue;
      }
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less    => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal   => {
        println!("You win!");
        break;
      }
    }

    /*
    match的另一种写法
    match xxx {
      Ok(_) => do_some_thing,
      Err(e) => println!("{}". e)
    }
    */
  }
}