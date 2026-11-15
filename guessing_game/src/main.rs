use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = rand::random_range(1..=100);
    println!("Guess the number between 1 and 100!");

    loop {
        println!("Enter your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
