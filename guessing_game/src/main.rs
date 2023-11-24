use std::cmp::Ordering;
use std::io::stdin;

use rand::Rng;

fn main() {
    let num: i32 = rand::thread_rng().gen_range(1..=100);

    println!("The random number is: {num}");
    println!("Enter a number from 1-100: ");

    loop {
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("failed to read stdin");
        let guess: i32 = guess.trim().parse().expect("You must enter a number!");

        match num.cmp(&guess) {
            Ordering::Equal => {
                println!("You guessed correctly!");
                break
            }
            Ordering::Greater => println!("The number is greater"),
            Ordering::Less => println!("The number is lower"),
        }
    }
}
