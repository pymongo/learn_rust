extern crate learn_rust;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Guess the number(1-100)!");
    let secret_number = learn_rust::random() as u32 % 100;
    loop {
        println!("\nPlease input your guess:");
        let mut guess_stdin_buf = String::new();
        std::io::stdin().lock().read_line(&mut guess_stdin_buf)?;
        let guess = guess_stdin_buf.trim().parse::<u32>()?;
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => break,
        }
    }
    println!("You guess is correct. You win!");
    Ok(())
}
