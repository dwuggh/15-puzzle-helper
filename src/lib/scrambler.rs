use rand::seq::SliceRandom;
use rand::thread_rng;

pub use super::parity::parity_check;

// mod types;
use super::types::{Board, GameBoard};

pub fn random_board(rank: u64) -> Vec<u64> {
    let mut vec: Vec<u64> = (0..rank * rank).collect();
    vec.shuffle(&mut thread_rng());
    vec
}

pub fn scramble_random(rank: u64) -> Board {
    loop {
        let vec = random_board(rank);
        let board = Board::from_vec(vec, rank);
        if !parity_check(&board) {
            return board;
        }
    }
}
