struct Permutation<'a> {
    mark: Vec<usize>,
    perm: &'a Vec<u64>,
}

impl<'a> Permutation<'a> {
    fn new(perm: &'a Vec<u64>) -> Permutation<'a> {
        Permutation {
            mark: vec![0; perm.len()],
            perm: perm,
        }
    }

    fn count_loops(&mut self) -> usize {
        let mut loops: Vec<usize> = Vec::new();
        let mut i: usize = 0;
        while i < self.perm.len() {
            // println!("{:?}", self.mark);
            match self.mark[i] {
                0 => {
                    let l = self._count_loops(i);
                    if l > 1 {
                        loops.push(l);
                    }
                }
                _ => {}
            }
            i = i + 1;
        }
        // println!("{:?}", loops);
        let even_loops: Vec<usize> = loops.into_iter().filter(|l| l % 2 == 0).collect();
        return even_loops.len();
    }

    fn _count_loops(&mut self, i: usize) -> usize {
        // println!("{:?}", self.mark);
        let mark = &mut self.mark;
        let next: usize = self.perm[i] as usize;
        match mark[next] {
            0 => {
                mark[next] = mark[i] + 1;
                self._count_loops(next)
            }
            _ => mark[i],
        }
    }
}

fn empty_site_parity_check(board: &Vec<u64>, rank: u64) -> bool {
    let mut i: usize = 0;
    while i < board.len() {
        if board[i] == rank * rank - 1 {
            let delta = rank * rank - 1 - (i as u64);
            let row = delta / rank;
            let col = delta % rank;
            // dbg!(delta, row, col);
            // odd number means parity
            return (row + col) % 2 == 1;
        }
        i = i + 1;
    }
    panic!("invalid board: {:?}", board)
}

#[inline]
fn permutation_parity_check(board: &Vec<u64>) -> bool {
    Permutation::new(board).count_loops() % 2 == 1
}

/// Check parity of a 15-puzzle with given `rank`.
/// Parity can be reduced to 2 parts: empty site parity(to canonical state), and
/// loop parity:
/// - odd number moves of empty site means parity
/// - odd number of even-length loops means parity
pub fn parity_check(board: &Vec<u64>, rank: u64) -> bool {
    permutation_parity_check(board) ^ empty_site_parity_check(board, rank)
}

#[cfg(test)]
mod parity_check_test {

    // use crate::core::random_board;
    use crate::core::parity::empty_site_parity_check;
    use crate::core::parity::parity_check;
    use crate::core::parity::permutation_parity_check;

    #[test]
    fn initial_state() {
        let vec: Vec<u64> = (0..9).collect();
        assert_eq!(parity_check(&vec, 3), false);
    }

    #[test]
    fn move_1_empty_site() {
        let mut vec: Vec<u64> = (0..9).collect();
        vec[7] = 8;
        vec[8] = 7;
        assert_eq!(empty_site_parity_check(&vec, 3), true);
    }

    #[test]
    fn swap_2_perm() {
        let mut vec: Vec<u64> = (0..16).collect();
        vec[7] = 8;
        vec[8] = 7;
        assert_eq!(permutation_parity_check(&vec), true);
    }

    #[test]
    fn real_case_rank_4_empty_site() {
        let vec: Vec<u64> = vec![1, 12, 14, 15, 9, 11, 7, 2, 5, 10, 8, 3, 0, 13, 4, 6];
        assert_eq!(empty_site_parity_check(&vec, 4), true);
    }

    #[test]
    fn real_case_rank_4_permutation() {
        let vec: Vec<u64> = vec![1, 12, 14, 15, 9, 11, 7, 2, 5, 10, 8, 3, 0, 13, 4, 6];
        assert_eq!(permutation_parity_check(&vec), true);
    }

    #[test]
    fn real_case_rank_4_2() {
        let vec: Vec<u64> = vec![6, 1, 0, 15, 11, 5, 13, 3, 8, 9, 4, 10, 2, 12, 7, 14];
        assert_eq!(parity_check(&vec, 4), false);
    }

    #[test]
    fn real_case_rank_5() {
        let vec: Vec<u64> = vec![
            4, 13, 5, 17, 18, 11, 16, 2, 7, 19, 10, 12, 23, 1, 15, 15, 24, 20, 6, 8, 21, 0, 3, 9,
            22,
        ];
        assert_eq!(parity_check(&vec, 4), false);
    }
}
