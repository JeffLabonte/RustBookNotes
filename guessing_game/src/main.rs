extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::{ thread_rng, Rng };

fn main() {
    println!("Guess the number !");
    let mut rng = thread_rng();
    let secret_number = rng.gen_range(0, 101);

    println!("The secret number is {}", secret_number);
    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "quit"{ break; }
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number");

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

