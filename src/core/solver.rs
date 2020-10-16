use super::types::{Board, Dist, GameBoard, Move, Wrapper};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::thread;
use std::time::Instant;

#[derive(Debug)]
pub struct SearchGraph<T, W> {
    nodes: Box<HashSet<T>>,
    priority_queue: Box<BinaryHeap<W>>,
}

impl<T, W> SearchGraph<T, W>
where
    T: Hash + Eq + Clone + Debug,
    W: IntoIterator<Item = W> + Wrapper<T> + Ord,
{
    pub fn new() -> SearchGraph<T, W> {
        SearchGraph {
            nodes: Box::new(HashSet::new()),
            priority_queue: Box::new(BinaryHeap::new()),
        }
    }

    fn worker(nodes: &mut HashSet<T>, pq: &mut BinaryHeap<W>) -> W {
        loop {
            let timer = Instant::now();
            let node = pq.pop().unwrap();

            if node.is_target() {
                return node;
            }

            let data: &T = node.get_data();
            println!("time 1: {:?}", timer.elapsed().as_nanos());
            println!("time 2: {}", timer.elapsed().as_nanos());
            for next in node.into_iter() {
                if nodes.insert(next.get_data_copy()) {
                    pq.push(next);
                }
            }
            println!("time 3: {}", timer.elapsed().as_nanos());

            dbg!(nodes.len());
        }
    }

    #[allow(non_snake_case)]
    fn _search_A_star(&mut self) -> W {
        let pq: &mut BinaryHeap<W> = &mut *self.priority_queue;
        let nodes = &mut *self.nodes;
        loop {
            let timer = Instant::now();
            let node = pq.pop().unwrap();
            let data: &T = node.get_data();
            println!("time 1: {:?}", timer.elapsed().as_nanos());
            if node.is_target() {
                return node;
            }
            println!("time 2: {}", timer.elapsed().as_nanos());
            for next in node.into_iter() {
                if nodes.insert(next.get_data_copy()) {
                    pq.push(next);
                }
            }
            println!("time 3: {}", timer.elapsed().as_nanos());

            dbg!(nodes.len());
        }
    }
}

#[allow(non_snake_case)]
pub fn search_A_star<T, W>(start: W) -> W
where
    T: Hash + Eq + Clone + Debug,
    W: IntoIterator<Item = W> + Wrapper<T> + Ord,
{
    // let graph: RefCell<SearchGraph<T, W>> = RefCell::new(SearchGraph::new(target));
    let mut graph: SearchGraph<T, W> = SearchGraph::new();
    (*graph.priority_queue).borrow_mut().push(start);
    // let pq: &mut BinaryHeap<W> = &mut *graph_ref.priority_queue;
    // pq.push(start);
    let multithread = false;
    let max_thread = 6;

    if multithread {
        let pq: &mut BinaryHeap<W> = &mut *graph.priority_queue;
        let nodes = &mut *graph.nodes;
        let handle = thread::spawn(|| {
            // SearchGraph::worker(nodes, pq);
        });
    }

    // graph.borrow()._search_A_star()
    graph._search_A_star()
}

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
