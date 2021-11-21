use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

use crate::{Graph, GraphVertex, GraphVisitor};

/// Helper for storing state between dfs runs in graph.
///
/// ```
/// use std::collections::HashSet;
/// use trivial_graph::{DfsVisitor, Graph, GraphVertex, GraphVisitor, VisitOrder};
/// let mut graph = Graph::new();
/// graph.add_vertex(1, "node".to_string());
/// graph.add_vertex(2, "node2".to_string());
/// graph.add_vertex(3, "node3".to_string());
/// graph.add_vertex(4, "node4".to_string());
/// graph.add_vertex(5, "node5".to_string());
/// assert!(graph.add_edge(1, 2).is_ok());
/// assert!(graph.add_edge(1, 3).is_ok());
/// assert!(graph.add_edge(2, 4).is_ok());
/// assert!(graph.add_edge(3, 4).is_ok());
/// assert!(graph.add_edge(5, 1).is_ok());
/// let mut visited_vertices = Vec::new();
/// let mut callback = |v: &GraphVertex<String>| {
///     visited_vertices.push(v.id);
/// };
/// let mut visitor = DfsVisitor::new(&graph);
/// visitor.visit(2, &mut callback);
/// visitor.visit(1, &mut callback);
/// visitor.visit(5, &mut callback);
/// assert_eq!(visited_vertices, vec![2, 4, 1, 3, 5]);
///
/// visited_vertices.clear();
/// visitor.clear();
/// let mut callback = |v: &GraphVertex<String>| {
///     visited_vertices.push(v.id);
/// };
///
/// visitor.visit_all(VisitOrder::TopologicalSort, &mut callback);
/// assert_eq!(visited_vertices[3], 4);
///
/// visited_vertices.clear();
/// visitor.clear();
///
/// let mut callback = |v: &GraphVertex<String>| {
///     visited_vertices.push(v.id);
/// };
/// visitor.visit_all(VisitOrder::NumbersAscending, &mut callback);
/// assert_eq!(visited_vertices[4], 5);
/// ```
pub struct DfsVisitor<'a, T: FromStr + Display> {
    visited: HashSet<usize>,
    graph: &'a Graph<T>,
}

impl<'a, T: FromStr + Display> DfsVisitor<'a, T> {
    /// Creates new visitor for given graph
    pub fn new(graph: &'a Graph<T>) -> Self {
        Self {
            visited: Default::default(),
            graph,
        }
    }

    fn dfs_impl<F: FnMut(&GraphVertex<T>)>(&mut self, v: usize, f: &mut F) {
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
    fn visit<F: FnMut(&GraphVertex<T>)>(&mut self, vertex: usize, f: F) {
        let mut f = f;
        self.dfs_impl(vertex, &mut f);
    }

    fn clear(&mut self) {
        self.visited.clear();
    }

    fn get_graph(&self) -> &Graph<T> {
        self.graph
    }
}
