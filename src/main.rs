// use crate::core::Board;

mod core;

use crate::core::{scramble_random, search_A_star};
use crate::core::types::{GameBoard, Board};

fn main() {
    let b = scramble_random(4);
    println!("{:?}", b);
    let re: GameBoard = search_A_star(b, Board::new(4));
    // dbg!(re);
    println!("{:?}", re.moves);
}
