use std::cmp::Ordering;

use super::board::{Board, Move};
use super::traits::{Dist, Wrapper};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct GameBoard {
    pub board: Board,
    pub last_move: Move,
    pub moves: Vec<Move>,
    dist: u64,
}

#[allow(dead_code)]
impl GameBoard {
    pub fn new(rank: u64) -> GameBoard {
        GameBoard {
            board: Board::new(rank),
            last_move: Move::None,
            moves: Vec::new(),
            dist: 0,
        }
    }

    pub fn from_vec(vec: Vec<u64>, rank: u64) -> GameBoard {
        let mut g = GameBoard {
            board: Board::from_vec(vec, rank),
            last_move: Move::None,
            moves: Vec::new(),
            dist: 0,
        };
        g.dist = g.dist_to_target();
        g
    }

    // perform a move
    fn mov(&mut self, m: Move) {
        self.moves.push(m.clone());
        self.last_move = m.clone();
        let n = self.board.empty_site_pos;
        let rank = self.board.rank;
        match m {
            Move::Left => {
                self.board.swap_with(n - 1);
            }
            Move::Right => {
                self.board.swap_with(n + 1);
            }
            Move::Up => {
                self.board.swap_with(n - rank);
            }
            Move::Down => {
                self.board.swap_with(n + rank);
            }
            Move::None => {}
        }
        // update dist
        // there exists more effcient way: only compute the swapped site's
        // dist difference.
        self.dist = self.dist_to_target();
    }

    fn neighbors(&self) -> Vec<GameBoard> {
        let mut neighbors: Vec<GameBoard> = Vec::new();
        let pos = self.board.empty_site_pos;
        let rank = self.board.rank;
        let row = pos / rank;
        let col = pos % rank;
        if row > 0 && self.last_move != Move::Down {
            let mut b: GameBoard = self.clone();
            b.mov(Move::Up);
            neighbors.push(b);
        };
        if row < rank - 1 && self.last_move != Move::Up {
            let mut b: GameBoard = self.clone();
            b.mov(Move::Down);
            neighbors.push(b);
        };
        if col > 0 && self.last_move != Move::Right {
            let mut b: GameBoard = self.clone();
            b.mov(Move::Left);
            neighbors.push(b);
        }
        if col < rank - 1 && self.last_move != Move::Left {
            let mut b: GameBoard = self.clone();
            b.mov(Move::Right);
            neighbors.push(b);
        };
        neighbors
    }

    fn canonical(&self) -> GameBoard {
        let mut result: GameBoard = self.clone();
        let rank = self.board.rank;
        let col = result.board.empty_site_pos % rank;
        while col != rank - 1 {
            result.mov(Move::Right);
        }
        let row = result.board.empty_site_pos / rank;
        while row != rank - 1 {
            result.mov(Move::Down);
        }
        result
    }

    fn to_canonical(&mut self) {
        let rank = self.board.rank;
        let col = self.board.empty_site_pos % rank;
        while col != rank - 1 {
            self.mov(Move::Right);
        }
        let row = self.board.empty_site_pos / rank;
        while row != rank - 1 {
            self.mov(Move::Down);
        }
    }

    pub fn can_reduce(&self) -> bool {
        let board = &self.board;
        let mut i: u64 = 0;
        while i < board.rank {
            if board.sites[i as usize] != i {
                return false;
            }
            if board.sites[(i * board.rank) as usize] != i * board.rank {
                return false;
            }
            i = i + 1;
        }
        return true;
    }
}

// traits implementation

impl IntoIterator for GameBoard {
    type Item = GameBoard;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.neighbors().into_iter()
    }
}

impl Wrapper<Board> for GameBoard {
    fn get_data(&self) -> &Board {
        &self.board
    }
    fn is_target(&self) -> bool {
        self.dist == self.moves.len() as u64
    }
}

impl Dist for GameBoard {
    type D = u64;
    fn dist_to_target(&self) -> Self::D {
        self.board.a_dist() + self.moves.len() as u64
    }
}

impl Ord for GameBoard {
    fn cmp(&self, other: &Self) -> Ordering {
        let da = &self.dist;
        let db = other.dist;
        db.cmp(da)
    }
}

impl PartialOrd for GameBoard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
