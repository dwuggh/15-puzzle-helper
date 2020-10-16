use std::cmp::Ordering;
use std::fmt;

use crate::core::types::traits::{Dist, Wrapper};


// the four move types(directions), used for scramble representation
// direction is the empty site moving direction
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum Move {
    Left,
    Right,
    Up,
    Down,
    None,
}


#[derive(Clone, Debug, Hash)]
pub struct Board {
    pub rank: u64,
    pub empty_site_pos: u64,
    pub sites: Vec<u64>
}


#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct GameBoard {
    pub board: Board,
    pub last_move: Move,
    pub moves: Vec<Move>,
    dist: u64,
}



impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match *self {
	    Move::Left => f.write_str("←"),
	    Move::Right => f.write_str("→"),
	    Move::Up => f.write_str("↑"),
	    Move::Down => f.write_str("↓"),
	    Move::None => f.write_str(" "),
	}
    }
}


impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
	self.rank == other.rank &&
	    self.sites.iter().zip(other.sites.iter()).all(|(a, b)| *a == *b)
    }
}

impl Eq for Board {}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	let mut i: usize = 0;
	let rank = self.rank as usize;
	while i < rank {
	    f.write_str("\n")?;
	    let mut j: usize = 0;
	    while j < rank {
		let num: usize = (self.sites[i * rank + j] + 1) as usize;
		if num == rank * rank {
		    f.write_str("     ")?;
		}
		else {
		    f.write_fmt(format_args!("{:5}", num))?;
		}
		j = j + 1;
	    }
	    i = i + 1;
	}
	f.write_str("\n")
    }
}

impl Board {
    pub fn new(rank: u64) -> Board {
        let sites: Vec<u64> = (0 .. rank * rank).collect();
	Board {
	    rank: rank,
	    sites: sites,
	    empty_site_pos: rank * rank - 1,
	}
    }

    pub fn from_vec(vec: Vec<u64>, rank: u64) -> Board {
	let mut empty_site_pos: u64 = 0;
	let mut i: usize = 0;
	while i < vec.len() {
	    if vec[i] == rank * rank - 1 {
		empty_site_pos = i as u64;
	    }
	    i = i + 1;
	}
        Board {
            rank: rank,
            sites: vec,
            empty_site_pos: empty_site_pos,
        }
    }

    // swap 2 sites, no update empty site.
    #[inline]
    fn swap(&mut self, n1: u64, n2: u64) {
        let temp = self.sites[n1 as usize];
        self.sites[n1 as usize] = self.sites[n2 as usize];
        self.sites[n2 as usize] = temp;
    }

    #[inline]
    fn swap_with(&mut self, n: u64) {
        self.swap(self.empty_site_pos, n);
        self.empty_site_pos = n;
    }

    pub fn manhattan_dist(&self) -> u64 {
	let mut i: usize = 0;
	let mut dist: u64 = 0;
	while i < self.sites.len() {
	    let s = self.sites[i];
	    let d = if s > i as u64 {
		s - i as u64
	    } else {
		i as u64 - s
	    };
	    let row = d / self.rank;
	    let col = d % self.rank;
	    dist = dist + row + col;
	    i = i + 1;
	}
	dist
    }
    pub fn a_dist(&self) -> u64 {
	let mut i: usize = 0;
	let mut manhattan_dist: u64 = 0;
	let mut hamming_dist: u64 = 0;
	let mut other_dist : u64 = 0;
	while i < self.sites.len() {
	    let s = self.sites[i];
	    let d = if s > i as u64 {
		s - i as u64
	    } else {
		i as u64 - s
	    };
	    let row = d / self.rank;
	    let col = d % self.rank;
	    manhattan_dist = manhattan_dist + row + col;

	    if i as u64 != s {
		hamming_dist = hamming_dist + 1;
	    }

	    let magic_a = 2;
	    let magic_b = 1;
	    if s / self.rank == 0 || s % self.rank == 0 {
		if s != i as u64 {
		    other_dist = other_dist + magic_a;
		    other_dist = other_dist + magic_b * (row + col);
		}
	    }

	    i = i + 1;
	}
	manhattan_dist + other_dist
    }
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

impl Dist for Board {
    type D = u64;
    fn dist_to_target(&self) -> Self::D {
	// self.manhattan_dist()
	todo!()
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
