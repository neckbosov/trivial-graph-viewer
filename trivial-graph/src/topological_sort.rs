use std::collections::HashSet;
use std::str::FromStr;

use crate::graph::Graph;

pub(crate) struct TopologicalSort<'a, T: FromStr> {
    graph: &'a Graph<T>,
    visited: HashSet<usize>,
    order: Vec<usize>,
}

impl<'a, T: FromStr> TopologicalSort<'a, T> {
    pub(crate) fn new(graph: &'a Graph<T>) -> Self {
        Self {
            graph,
            visited: Default::default(),
            order: vec![],
        }
    }
    fn dfs(&mut self, v: usize) {
        if self.visited.contains(&v) {
            return;
        }
        self.visited.insert(v);
        if let Some(neighbours) = self.graph.get_neighbours(v) {
            for nx in neighbours {
                self.dfs(*nx);
            }
        }

        self.order.push(v);
    }
    pub(crate) fn create_order(mut self) -> Vec<usize> {
        for v in &self.graph.get_vertices_ids() {
            self.dfs(*v);
        }
        let mut order = self.order;
        order.reverse();
        order
    }
}
