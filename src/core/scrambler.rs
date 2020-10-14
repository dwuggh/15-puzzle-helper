
use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::core::{parity_check};

// mod types;
use super::types::{Board, GameBoard};

pub fn random_board(rank: u64) -> Vec<u64> {
    let mut vec: Vec<u64> = (0 .. rank * rank).collect();
    vec.shuffle(&mut thread_rng());
    vec
}

pub fn scramble_random(rank: u64) -> GameBoard {
    let mut vec: Vec<u64> = random_board(rank);
    while parity_check(&vec, rank) {
	vec = random_board(rank);
    }
    GameBoard::from_vec(vec, rank)
}

