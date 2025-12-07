#![allow(unused)]
use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main () {
    // max values
    let mut max_lives = String::new();
    let mut max_tries = String::new();
        // taking input for max values
        println!();
        println!("Lives mean the amount of times you can retry after a failed equation, while tries is the same but for a single equation,");
        print!("How many lives do you prefer to play with?: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut max_lives) .expect("Failed to read line");
        print!("How many tries per eq do you prefer to play with?: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut max_tries)
        .expect("Failed to read line");
    // conversion for max values
    let max_lives: u32 = max_lives.trim().parse()
    .expect("Need a +ve number input");
    let max_tries: u32 = max_tries.trim().parse()
    .expect("Need a +ve number input");
        // some variables
        let mut correct_anss = 0;
        let mut plays = 0;
        println!();
        println!("You have {max_lives} Lives overall + {max_tries} tries per eq in this game.\n");
    let mut lives = max_lives;
    // while loop
    while lives != 0 { 
    // variables to be used
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..101);
    let y = rng.gen_range(0..101);
    let z = rng.gen_range(0..101);
         // display lives
        if plays != 0 {
            println!();
        println!("You have {lives} life/lives remaining!");
        println!("You have answered {correct_anss}/{plays} questions correctly till now on {max_lives} lives and {max_tries} tries settings!\n");
    }
        // displaying question
        println!("x - y = {} and y + {z} = {}",x - y, y + z);
    let ans = x + y;
    // for loop
    for tries in 1..=max_tries {
   // getting input
    let mut guess = String::new();
    print!("What is the value of x + y ({} tries left): ", (max_tries +1) - tries);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
        // conversion 
        let guess: i32 = guess.trim().parse()
        .expect("Need a number input");
    // if-else based on the answer
    if guess == ans {
        println!("You win!");
        correct_anss += 1;
        plays += 1;
        let tries = max_tries;
        break;
    } else if tries == max_tries {
        println!("You lost!, the answer is {x}(x) + {y}(y) = {}", ans);
        lives -= 1;
        plays += 1;
    } else {
        match guess.cmp(&ans) {
            Ordering::Less => println!("That's less!"),
            Ordering::Equal => println!(),
            Ordering::Greater => println!("That's more!")
        }
    }
    println!();
}
}
}