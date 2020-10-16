use super::types::{Board, GameBoard, Move, Wrapper};
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::time::Instant;


/// specialization of `Searchgraph`
struct SearchData {
    nodes: HashSet<Board>,
    priority_queue: BinaryHeap<GameBoard>,
}

impl SearchData {
    pub fn new() -> SearchData {
        SearchData {
            nodes: HashSet::new(),
            priority_queue: BinaryHeap::new(),
        }
    }

    fn _search_reduce(start: GameBoard) {}

    fn _search_A_star(&mut self) -> GameBoard {
        let nodes = &mut self.nodes;
        let pq = &mut self.priority_queue;
        loop {
            let timer = Instant::now();
            let node = pq.pop().unwrap();
            let data: &Board = node.get_data();
	    // dbg!(data);
	    println!("{}", data);
            // println!("time 1: {}", timer.elapsed().as_nanos());
            if node.can_reduce() {
                return node;
            }
            // println!("time 2: {}", timer.elapsed().as_nanos());
            for next in node.into_iter() {
                if nodes.insert(next.get_data_copy()) {
                    pq.push(next);
                }
            }
            // println!("time 3: {}", timer.elapsed().as_nanos());

            dbg!(nodes.len());
        }
    }
}

pub fn search_reduced(start: GameBoard) -> Vec<Move> {
    _search_reduced(start, Vec::new())
}

pub fn _search_reduced(start: GameBoard, mut moves: Vec<Move>) -> Vec<Move> {
    if (start.board.rank == 1) {
        return moves;
    }

    let mut data = SearchData::new();
    let start_sites = start.board.sites.clone();
    data.priority_queue.push(start);

    let res = data._search_A_star();
    // dbg!(res);
    println!("reduced! {:?}", res);


    moves.append(&mut res.moves.clone());

    let res_sites = res.board.sites;
    let mut next_sites: Vec<u64> = Vec::new();
    let mut i: u64 = 0;
    let rank = res.board.rank;
    let next_rank = rank - 1;
    while i < next_rank {
        let mut j: u64 = 0;
        while j < next_rank {
            // next_sites[(i * next_rank + j) as usize] =
            //     start_sites[((i + 1) * rank + (j + 1)) as usize];
	    let index = (i + 1) * rank + (j + 1);
	    let val = res_sites[index as usize];
	    let row = val / rank;
	    let col = val % rank;
	    // dbg!(val, row, col);
	    let next_val = (row - 1) * next_rank + (col - 1);
	    next_sites.push(next_val);
            j = j + 1;
        }
        i = i + 1;
    }

    let next: GameBoard = GameBoard::from_vec(next_sites, rank - 1);

    _search_reduced(next, moves)
}
