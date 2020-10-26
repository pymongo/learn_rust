use rand::Rng;
use std::io::BufRead;

const LOWER: u32 = 1;
const UPPER: u32 = 100;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Guess the number({}-{})!", LOWER, UPPER);
    let secret_number = rand::thread_rng().gen_range(LOWER, UPPER);
    loop {
        println!("\nPlease input your guess:");
        let mut guess_stdin_buf = String::new();
        std::io::stdin().lock().read_line(&mut guess_stdin_buf)?;
        let guess = guess_stdin_buf.trim().parse::<u32>()?;
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => break
        }
    }
    println!("You guess is correct. You win!");
    Ok(())
}