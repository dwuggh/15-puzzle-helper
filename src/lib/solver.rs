use super::types::{Board, GameBoard, Move, Wrapper};
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::time::Instant;


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

    #[allow(non_snake_case)]
    fn _search_A_star(&mut self) -> GameBoard {
        let nodes = &mut self.nodes;
        let pq = &mut self.priority_queue;
        loop {
            let timer = Instant::now();
            let node = pq.pop().unwrap();

	    let time_0 = timer.elapsed().as_nanos();
            let data: &Board = node.get_data();

	    println!("{:?}", data);
	    let time_1 = timer.elapsed().as_nanos();
            println!("time 1: {:?}", time_0);

            if node.can_reduce() {
                return node;
            }

	    let time_2 = timer.elapsed().as_nanos();
            println!("time 2: {:?}", time_2 - time_1);

            for next in node.into_iter() {
		let timer_iter = Instant::now();
                if nodes.insert(next.get_data_copy()) {
		    let t_1 = timer_iter.elapsed().as_nanos();
		    println!("time for insert: {:?}", t_1);
                    pq.push(next);
		    let t_2 = timer_iter.elapsed().as_nanos();
		    println!("time for push: {:?}", t_2);
                }
            }

	    let time_3 = timer.elapsed().as_nanos();
            println!("time 3: {:?}", time_3 - time_2);

	    println!("searched nodes count: {:?}", nodes.len());
        }
    }
}

pub fn rank_reduction_search(start: GameBoard) -> Vec<Move> {
    _rank_reduction_search(start, Vec::new())
}

fn _rank_reduction_search(start: GameBoard, mut moves: Vec<Move>) -> Vec<Move> {
    if start.board.rank == 1 {
        return moves;
    }

    let mut data = SearchData::new();
    data.priority_queue.push(start);

    let res = data._search_A_star();

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
	    let index = (i + 1) * rank + (j + 1);
	    let val = res_sites[index as usize];
	    let row = val / rank;
	    let col = val % rank;
	    let next_val = (row - 1) * next_rank + (col - 1);
	    next_sites.push(next_val);
            j = j + 1;
        }
        i = i + 1;
    }

    let next: GameBoard = GameBoard::from_vec(next_sites, rank - 1);

    _rank_reduction_search(next, moves)
}
