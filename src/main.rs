use std::io::{self, Write};

fn main() {
    let x = 5;
    print!("Input: ");
    io::stdout().flush().unwrap();
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: i32 = guess.trim().parse()
        .expect("Please type a number");
    if guess == x {
        println!("Right")
    } else {
        println!("Wrong")
    }
}
