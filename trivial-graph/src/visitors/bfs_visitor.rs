use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
use std::str::FromStr;

use crate::{Graph, GraphVertex, GraphVisitor};

/// Helper for storing state between bfs runs in graph.
///
/// ```
/// use std::collections::HashSet;
/// use trivial_graph::{BfsVisitor, Graph, GraphVertex, GraphVisitor, VisitOrder};
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
/// let mut visitor = BfsVisitor::new(&graph);
/// visitor.visit(2, &mut callback);
/// visitor.visit(1, &mut callback);
/// visitor.visit(5, &mut callback);
/// assert_eq!(visited_vertices, vec![2, 4, 1, 3, 5]);
///
/// visited_vertices.clear();
/// visitor.clear();
///
/// let mut callback = |v: &GraphVertex<String>| {
///     visited_vertices.push(v.id);
/// };
///
/// visitor.visit_all(VisitOrder::TopologicalSort, &mut callback);
/// assert_eq!(visited_vertices[4], 4);
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
pub struct BfsVisitor<'a, T: FromStr + Display> {
    visited: HashSet<usize>,
    graph: &'a Graph<T>,
}

impl<'a, T: FromStr + Display> BfsVisitor<'a, T> {
    /// Creates new visitor for given graph
    pub fn new(graph: &'a Graph<T>) -> Self {
        Self {
            visited: Default::default(),
            graph,
        }
    }

    fn bfs_impl<F: FnMut(&GraphVertex<T>)>(&mut self, v: usize, f: &mut F) {
        let mut vertex_queue = VecDeque::new();
        if !self.visited.contains(&v) {
            vertex_queue.push_back(v);
            self.visited.insert(v);
        }

        while let Some(v) = vertex_queue.pop_front() {
            f(self.graph.get_vertex(v).unwrap());
            if let Some(neighbours) = self.graph.get_neighbours(v) {
                for nx in neighbours {
                    if !self.visited.contains(&nx) {
                        self.visited.insert(nx);
                        vertex_queue.push_back(nx);
                    }
                }
            }
        }
    }
}

impl<'a, T: FromStr + Display> GraphVisitor<T> for BfsVisitor<'a, T> {
    fn visit<F: FnMut(&GraphVertex<T>)>(&mut self, vertex: usize, f: F) {
        let mut f = f;
        self.bfs_impl(vertex, &mut f);
    }

    fn clear(&mut self) {
        self.visited.clear();
    }

    fn get_graph(&self) -> &Graph<T> {
        self.graph
    }
}
