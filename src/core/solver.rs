use std::fmt::Debug;
use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::hash::Hash;
use super::types::{Board, Wrapper, GameBoard, Dist};
use std::collections::HashSet;

#[derive(Debug)]
pub struct SearchGraph<T, W>
{
    nodes: RefCell<HashSet<T>>,
    priority_queue: RefCell<BinaryHeap<W>>,
    target: T
}

impl<T, W> SearchGraph<T, W>
where
    T: Hash + Eq + Clone + Debug,
    W: IntoIterator<Item=W> + Wrapper<T> + Ord
{
    pub fn new(target: T) -> SearchGraph<T, W> {
        SearchGraph {
            nodes: RefCell::new(HashSet::new()),
	    priority_queue: RefCell::new(BinaryHeap::new()),
	    target: target
        }
    }


    #[allow(non_snake_case)]
    fn _search_A_star(&self) -> W {
	loop {
	    let pq = &mut self.priority_queue.borrow_mut();
	    let nodes = &mut self.nodes.borrow_mut();
	    let node = pq.pop().unwrap();
	    let data: &T = node.get_data();
	    if data.eq(&self.target) {
		return node;
	    }
	    for next in node.into_iter() {
		if nodes.insert(next.get_data_copy()) {
		    pq.push(next);
		}
	    }

	    dbg!(nodes.len());
	}
    }

    #[allow(non_snake_case)]
    fn _search_A_star_recursive(&self, node: W) -> W {

	let data: &T = node.get_data();

	// dbg!(data);
	// dbg!(self.nodes.borrow().len());

	if data.eq(&self.target) {
	    return node;
	}

	// to beat the borrow checker
	let n: W = {
	    let pq = &mut self.priority_queue.borrow_mut();
	    let nodes = &mut self.nodes.borrow_mut();

	    for next in node.into_iter() {
		if nodes.insert(next.get_data_copy()) {
		    pq.push(next);
		}
	    }
	    pq.pop().unwrap()
	};

	self._search_A_star_recursive(n)
    }
}


#[allow(non_snake_case)]
pub fn search_A_star<T, W>(start: W, target: T) -> W
where
    T: Hash + Eq + Clone + Debug,
    W: IntoIterator<Item=W> + Wrapper<T> + Ord
{
    let graph: SearchGraph<T, W> = SearchGraph::new(target);
    graph.priority_queue.borrow_mut().push(start);
    graph._search_A_star()
    // graph._search_A_star(start)
}
