use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};
use std::io::{BufRead, BufReader, Read};
use std::num::ParseIntError;
use std::ops::IndexMut;
use std::option::Option::Some;
use std::str::FromStr;

use thiserror::Error;

use crate::bfs_visitor::BfsVisitor;
use crate::dfs_visitor::DfsVisitor;
use crate::graph_vertex::GraphVertex;
use crate::graph_visitor::GraphVisitor;
use crate::visit_order::VisitOrder;

#[derive(Error, Debug)]
pub struct VertexValueParseError<E>(#[from] E);

#[derive(Error, Debug)]
pub enum GraphParseError<E> {
    #[error("Input/output error")]
    IO(#[from] std::io::Error),
    #[error("Incorrect data, {0} items expected, {1} got")]
    DataError(usize, usize),
    #[error("Fail to parse vertex number")]
    VertexParseError(#[from] ParseIntError),
    #[error("kek")]
    ValueParseError(#[from] VertexValueParseError<E>),
}

pub struct Graph<T: FromStr> {
    vertices: HashMap<usize, GraphVertex<T>>,
    edges: HashMap<usize, HashSet<usize>>,
}

impl<T: FromStr> Graph<T> {
    pub fn add_vertex(&mut self, vertex: usize, value: T) {
        self.vertices
            .insert(vertex, GraphVertex::new(vertex, value));
    }
    pub fn remove_vertex(&mut self, vertex: usize) {
        let neighbours = self.edges.remove(&vertex);
        if let Some(neighbours) = neighbours {
            for neighbour in neighbours {
                self.edges.get_mut(&neighbour).unwrap().remove(&vertex);
            }
        }
    }
    pub fn add_edge(&mut self, vertex_from: usize, vertex_to: usize) -> bool {
        if !self.vertices.contains_key(&vertex_from) || !self.vertices.contains_key(&vertex_to) {
            return false;
        }
        self.edges.entry(vertex_from).or_default().insert(vertex_to);
        true
    }
    pub fn remove_edge(&mut self, vertex_from: usize, vertex_to: usize) {
        if let Some(from_neighbours) = self.edges.get_mut(&vertex_from) {
            from_neighbours.remove(&vertex_to);
        }
    }
    pub fn get_vertex(&self, vertex_id: usize) -> &GraphVertex<T> {
        &self.vertices[&vertex_id]
    }

    pub fn get_neighbours(&self, vertex: usize) -> Option<&HashSet<usize>> {
        self.edges.get(&vertex)
    }
    pub fn get_vertices_ids(&self) -> HashSet<usize> {
        self.vertices.keys().map(usize::clone).collect()
    }

    pub fn bfs<F: FnMut(&GraphVertex<T>) -> ()>(&self, f: F) {
        BfsVisitor::new(&self).visit_all(VisitOrder::TopologicalSort, f);
    }
    /// Reads graph from given reader and return `Graph` structure.
    /// Requires value type to implement [`FromStr`] trait.
    ///
    /// # Errors
    /// Return `GraphParseError` in case of some I/O or parsing problems.
    /// See [`GraphParseError`] documentation for more details.
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use crate::trivial_graph::graph::Graph;
    /// let mut graph_bytes = concat!(
    /// "1 1\n",
    /// "2 2\n",
    /// "#\n",
    /// "1 2\n"
    /// ).as_bytes();
    /// let res = Graph::<i32>::read_from(&mut graph_bytes);
    /// assert!(res.is_ok());
    /// let graph = res.unwrap();
    /// assert_eq!(graph.get_vertices_ids(), HashSet::from([1, 2]));
    /// assert_eq!(graph.get_neighbours(1), Some(&HashSet::from([2])));
    /// assert_eq!(graph.get_neighbours(2), None);
    /// ```
    pub fn read_from<R: Read>(reader: &mut R) -> Result<Self, GraphParseError<T::Err>> {
        let mut graph = Self {
            vertices: Default::default(),
            edges: Default::default(),
        };
        let mut buf_reader = BufReader::new(reader);
        let mut buf = String::new();
        loop {
            buf.clear();
            buf_reader.read_line(&mut buf)?;
            let line = buf.trim();
            if line == "#" || line.is_empty() {
                break;
            }
            let parts: Vec<_> = line.splitn(2, ' ').collect();
            if parts.len() < 2 {
                return Err(GraphParseError::DataError(2, parts.len()));
            }

            let vertex_id: usize = parts[0].parse()?;
            let value: T = parts[1]
                .parse()
                .map_err(|err| VertexValueParseError::from(err))?;
            graph.add_vertex(vertex_id, value);
        }
        loop {
            buf.clear();
            buf_reader.read_line(&mut buf)?;
            let line = buf.trim();
            if line.is_empty() {
                break;
            }
            let parts: Vec<_> = line.splitn(3, ' ').collect();
            if parts.len() < 2 {
                return Err(GraphParseError::DataError(2, parts.len()));
            }
            let vertex_from_id: usize = parts[0].parse()?;
            let vertex_to_id: usize = parts[1].parse()?;
            graph.add_edge(vertex_from_id, vertex_to_id);
        }
        Ok(graph)
    }
}
