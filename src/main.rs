// use crate::core::Board;

mod core;

use std::time::Instant;

use crate::core::{scramble_random, search_reduced};
// use crate::core::types::{GameBoard};

fn main() {
    let b = scramble_random(4);
    let b_clone = b.clone();
    let timer = Instant::now();
    // let re: GameBoard = search_A_star(b);
    let re = search_reduced(b);
    // dbg!(re);
    println!("{:?}", re);
    println!("{}", b_clone.board);
    println!("time: {}", timer.elapsed().as_secs_f64());
}
