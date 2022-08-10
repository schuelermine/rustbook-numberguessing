use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

const DEBUG: bool = false;

fn main() {
    println!("Welcome to the number guessing game");

    let secret_number: u16 = rand::thread_rng().gen_range(1..=100);
    if DEBUG {
        eprintln!("@debug secret_number = {secret_number}")
    }

    loop {
        print!("Please enter a guess: ");
        io::stdout().flush().expect("Failure to write to output");

        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Cannot read input");

        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}.");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
