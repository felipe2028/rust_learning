use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game.");
    println!("Your goal is to correctly guess an integer between 1 and 100.");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read input from user.");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You need to enter an integer. Try again.");
                continue;
            }
        };

        match guess {
            1..=100 => {},
            _ => {
                println!("Your guess needs to be between 1 and 100. Try again.");
                continue;
            }
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}