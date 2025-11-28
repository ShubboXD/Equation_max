#![allow(unused)]
use std::io::{self, Write};
use rand::Rng;

fn main () {
    let mut rng = rand::thread_rng();
    let mut guess = String::new();
    let x = rng.gen_range(0..101);
    let y = rng.gen_range(0..101);
    let z = rng.gen_range(0..101);
    println!("x = {x} and y + {z} = {}", y + z);
    print!("What is the value of x + y: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
    let guess: i32 = guess.trim().parse()
    .expect("Need a number Input");
    
    if guess == x + y {
        println!("You won")
    } else {
        println!("You lost")
    }
}