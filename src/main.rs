use std::io::{self, Write};

fn main () {
    let mut guess = String::new();
    print!("Enter your guess: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut guess).unwrap();
    let guess: i32 = guess.trim().parse().unwrap();
    let x = 1;
    if guess == x {
        println!("You won")
    } else {
        println!("You lost")
    }
}