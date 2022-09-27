use std::io;
use std::io::Write; //so I can use flush()
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess which number I'm thinking!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        print!("Input your guess:");
        io::stdout().flush().expect("flush failed!");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}