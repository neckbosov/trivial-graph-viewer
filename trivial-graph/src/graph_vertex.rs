use std::fmt::Display;
use std::str::FromStr;

/// Vertex of a graph.
#[derive(Debug)]
pub struct GraphVertex<T: FromStr + Display> {
    pub id: usize,
    pub value: T,
}

impl<T: FromStr + Display> GraphVertex<T> {
    /// Create new vertex with given id and value
    pub fn new(id: usize, value: T) -> Self {
        Self { id, value }
    }
}
