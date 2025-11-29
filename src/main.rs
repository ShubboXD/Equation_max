#![allow(unused)]
use std::io::{self, Write};
use std::cmp::Ordering;
use std::os::linux::fs::MetadataExt;
use rand::Rng;

fn main () {
    // max values
    let mut maxl = String::new();
    let mut maxt = String::new();
    // taking input for max values
    println!();
    println!("Lives mean the amount of times you can retry after a failed equation, while tries is the same but for a single equation,");
    print!("How many lives do you prefer to play with?: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut maxl).expect("Failed to read line");
    print!("How many tries per eq do you prefer to play with?: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut maxt).expect("Failed to read line");
    // conversion for max values
    let maxl: u32 = maxl.trim().parse().expect("Need a +ve number input");
    let maxt: i32 = maxt.trim().parse().expect("Need a +ve number input");
    // somethingss
    let mut ca = 0;
    println!();
    println!("You have {maxl} Lives overall + {maxt} tries per eq in this game, Let's begin!");
    // while-for loop logic
    let mut lives = maxl;
    while lives != 0 { 
        // display lives
        if lives == maxl {
            println!();
        } else {
            println!("You have {lives} life/lives remaining!");
            println!("You have answered {ca} questions correctly till now on {maxl} lives and {maxt} tries overall!\n");
        }
    for tries in 1..=maxt {
    // variables to be used
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..101);
    let y = rng.gen_range(0..101);
    let z = rng.gen_range(0..101);
    let ans = x + y;
    let mut guess = String::new();
    // displaying question
    println!("x - y = {} and y + {z} = {}",x - y, y + z);
    // getting input
    print!("What is the value of x + y ({} tries left): ", (maxt +1) - tries);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
    // conversion 
    let guess: i32 = guess.trim().parse()
    .expect("Need a number input");
    // if-else based on the answer
    if guess == ans {
        println!("You win!");
        ca += 1;
    } else if tries == maxt {
        println!("You lost!, the answer is {x}(x) + {y}(y) = {}", ans);
        lives -= 1;
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