use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

use crate::graph::Graph;
use crate::graph_vertex::GraphVertex;
use crate::graph_visitor::GraphVisitor;

pub struct DfsVisitor<'a, T: FromStr + Display> {
    visited: HashSet<usize>,
    graph: &'a Graph<T>,
}

impl<'a, T: FromStr + Display> DfsVisitor<'a, T> {
    pub fn new(graph: &'a Graph<T>) -> Self {
        Self {
            visited: Default::default(),
            graph,
        }
    }

    fn dfs_impl<F: FnMut(&GraphVertex<T>) -> ()>(&mut self, v: usize, f: &mut F) {
        if self.visited.contains(&v) {
            return;
        }
        self.visited.insert(v);
        f(self.graph.get_vertex(v).unwrap());
        if let Some(neighbours) = self.graph.get_neighbours(v) {
            for nx in neighbours {
                self.dfs_impl(nx, f);
            }
        }
    }
}

impl<'a, T: FromStr + Display> GraphVisitor<T> for DfsVisitor<'a, T> {
    fn visit<F: FnMut(&GraphVertex<T>) -> ()>(&mut self, vertex: usize, f: F) {
        let mut f = f;
        self.dfs_impl(vertex, &mut f);
    }

    fn clear(&mut self) {
        self.visited.clear();
    }

    fn get_graph(&self) -> &Graph<T> {
        &self.graph
    }
}
