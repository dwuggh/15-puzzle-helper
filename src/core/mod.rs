
pub mod types;

mod scrambler;
mod parity;
mod solver;

pub use scrambler::scramble_random;
pub use parity::parity_check;

pub use solver::*;
// pub use solver::Dist;
