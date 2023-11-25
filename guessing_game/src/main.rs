use guessing_game::*;
use rand::Rng;
use std::io::{stdin, stdout};

fn main() -> Result<(), std::io::Error> {
    let num: i32 = rand::thread_rng().gen_range(1..=100);
    do_guess_random(stdin().lock(), stdout().lock(), num)
}
