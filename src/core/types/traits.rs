
// use crate::core::types::board::{Board, GameBoard};
use crate::core::types::{Board, GameBoard};

pub trait Wrapper<T: Clone> {
    fn get_data(&self) -> &T;
    fn get_data_copy(&self) -> T {
	self.get_data().clone()
    }
    fn is_target(&self) -> bool;
}

pub trait Dist {
    type D: Ord;
    fn dist_to_target(&self) -> Self::D;
}

