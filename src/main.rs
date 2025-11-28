#![allow(unused)]
use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main () {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0..101);
    let y = rng.gen_range(0..101);
    let z = rng.gen_range(0..101);
    let ans = x + y;
    println!("x - y = {} and y + {z} = {}",x - y, y + z);
    for mut i in 1..=3 { 
    let mut guess = String::new();
    print!("What is the value of x + y ({} tries left): ", 4 - i);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
    let guess: i32 = guess.trim().parse()
    .expect("Need a number input");
    if guess == ans {
        println!("You win!");
        break;
    } else if i == 3 {
        println!("You lost!, the answer is {x}(x) + {y}(y) = {}", ans);
    } else {
        match guess.cmp(&ans) {
            Ordering::Less => println!("That's less!"),
            Ordering::Equal => println!(),
            Ordering::Greater => println!("That's more!")
        }
    }
}
}