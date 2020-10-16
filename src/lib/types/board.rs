use std::fmt;

use super::traits::{Dist};


#[derive(Clone, Hash)]
pub struct Board {
    pub rank: u64,
    pub empty_site_pos: u64,
    pub sites: Vec<u64>,
}

impl Board {
    pub fn new(rank: u64) -> Board {
        let sites: Vec<u64> = (0..rank * rank).collect();
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
    pub fn swap(&mut self, n1: u64, n2: u64) {
        let temp = self.sites[n1 as usize];
        self.sites[n1 as usize] = self.sites[n2 as usize];
        self.sites[n2 as usize] = temp;
    }

    #[inline]
    pub fn swap_with(&mut self, n: u64) {
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

    #[inline]
    pub fn rank_reduction_dist(&self) -> u64 {
        let mut i: usize = 0;
        let mut manhattan_dist: u64 = 0;
        let mut hamming_dist: u64 = 0;
        let mut other_dist: u64 = 0;
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
                } else {
                    f.write_fmt(format_args!("{:5}", num))?;
                }
                j = j + 1;
            }
            i = i + 1;
        }
        f.write_str("\n")
    }
}


impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
            && self
                .sites
                .iter()
                .zip(other.sites.iter())
                .all(|(a, b)| *a == *b)
    }
}

impl Eq for Board {}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	self.fmt(f)
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	self.fmt(f)
    }
}

impl Dist for Board {
    
    type D = u64;
    fn dist_to_target(&self) -> Self::D {
	self.rank_reduction_dist()
    }
}
