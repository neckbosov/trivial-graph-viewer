use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

use crate::graph::Graph;
use crate::graph_vertex::GraphVertex;
use crate::graph_visitor::GraphVisitor;
use crate::visit_order::VisitOrder;

pub struct BfsVisitor<'a, T: FromStr> {
    visited: HashSet<usize>,
    graph: &'a Graph<T>,
}

impl<'a, T: FromStr> BfsVisitor<'a, T> {
    pub fn new(graph: &'a Graph<T>) -> Self {
        Self {
            visited: Default::default(),
            graph,
        }
    }

    fn bfs_impl<F: FnMut(&GraphVertex<T>) -> ()>(&mut self, v: usize, f: &mut F) {
        let mut vertex_queue = VecDeque::new();
        if !self.visited.contains(&v) {
            vertex_queue.push_back(v);
        }

        while let Some(v) = vertex_queue.pop_front() {
            f(self.graph.get_vertex(v));
            if let Some(neighbours) = self.graph.get_neighbours(v) {
                for nx in neighbours {
                    if !self.visited.contains(nx) {
                        self.visited.insert(*nx);
                        vertex_queue.push_back(*nx);
                    }
                }
            }
        }
    }
}

impl<'a, T: FromStr> GraphVisitor<T> for BfsVisitor<'a, T> {
    fn visit<F: FnMut(&GraphVertex<T>) -> ()>(&mut self, vertex: usize, f: F) {
        let mut f = f;
        self.bfs_impl(vertex, &mut f);
    }

    fn clear(&mut self) {
        self.visited.clear();
    }

    fn get_graph(&self) -> &Graph<T> {
        &self.graph
    }
}
