#![allow(unused)]
use std::io::{self, Write};
use rand::Rng;

fn main () {
    let mut rng = rand::thread_rng();
    let mut guess = String::new();
    let x = rng.gen_range(0..=1);
    print!("Enter your guess: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
    let guess: i32 = guess.trim().parse()
    .expect("Need a number Input");
    
    if guess == x {
        println!("You won")
    } else {
        println!("You lost")
    }
}