extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);


    loop {

        let mut guess = String::new();

        println!("Please enter your guess.");


        io::stdin().read_line(&mut guess)
            .expect("Failed to read file");

        println!("You guessed {}.", guess.trim() );

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too kleine"),
            Ordering::Greater => println!("Too groß"),
            Ordering::Equal => {
                println!("You made it!");
                break;
            },
        }
    }
}
