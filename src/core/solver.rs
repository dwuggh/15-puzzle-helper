use std::fmt::Debug;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::collections::BinaryHeap;
use std::hash::Hash;
use super::types::{Board, Wrapper, GameBoard, Dist};
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug)]
pub struct SearchGraph<T, W>
{
    nodes: Box<HashSet<T>>,
    priority_queue: Box<BinaryHeap<W>>,
    target: T
}

impl<T, W> SearchGraph<T, W>
where
    T: Hash + Eq + Clone + Debug,
    W: IntoIterator<Item=W> + Wrapper<T> + Ord
{
    pub fn new(target: T) -> SearchGraph<T, W> {
        SearchGraph {
            nodes: Box::new(HashSet::new()),
	    priority_queue: Box::new(BinaryHeap::new()),
	    target: target
        }
    }


    #[allow(non_snake_case)]
    fn _search_A_star(&mut self) -> W {
	loop {
	    let timer = Instant::now();
	    let pq: &mut BinaryHeap<W> = &mut *self.priority_queue;
	    let nodes = &mut *self.nodes;
	    let node = pq.pop().unwrap();
	    let data: &T = node.get_data();
	    println!("time 1: {:?}", timer.elapsed().as_nanos());
	    if data.eq(&self.target) {
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
pub fn search_A_star<T, W>(start: W, target: T) -> W
where
    T: Hash + Eq + Clone + Debug,
    W: IntoIterator<Item=W> + Wrapper<T> + Ord
{
    let mut graph: SearchGraph<T, W> = SearchGraph::new(target);
    (*graph.priority_queue).borrow_mut().push(start);
    graph._search_A_star()
    // graph._search_A_star(start)
}
