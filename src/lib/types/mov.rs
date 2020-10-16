use std::fmt;
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

impl Move {
    fn revert(&self) -> Move {
	match self {
	    Move::Left => { Move::Right },
	    Move::Right => { Move::Left },
	    Move::Up => { Move::Down },
	    Move::Down => { Move::Up },
	    Move::None => { Move::None }
	}
    }
}
