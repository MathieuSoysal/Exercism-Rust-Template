#![feature(test)]
extern crate test;

use prusti_contracts::*;

use template_exercisme::play_game;

#[trusted]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let i = args
        .get(1)
        .expect("Give one argument")
        .parse::<u32>()
        .expect("Given argument should be a integer.");
    play_game(i);
}
