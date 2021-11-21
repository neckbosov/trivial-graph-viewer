use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::io::{BufRead, BufReader, Read};
use std::num::ParseIntError;
use std::option::Option::Some;
use std::str::FromStr;

use thiserror::Error;

use crate::bfs_visitor::BfsVisitor;
use crate::graph_vertex::GraphVertex;
use crate::graph_visitor::GraphVisitor;

#[derive(Error, Debug)]
pub struct VertexValueParseError<E>(#[from] E);

#[derive(Error, Debug)]
#[error("{message}")]
pub struct VertexNotExistsError {
    message: String,
}

#[derive(Error, Debug)]
pub enum GraphParseError<E> {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("Incorrect data, {0} items expected, {1} got")]
    DataError(usize, usize),
    #[error("Fail to parse vertex number")]
    VertexParseError(#[from] ParseIntError),
    #[error(transparent)]
    ValueParseError(#[from] VertexValueParseError<E>),
    #[error(transparent)]
    VertexNotExists(#[from] VertexNotExistsError),
}

#[derive(Debug)]
pub struct Graph<T: FromStr + Display> {
    vertices: HashMap<usize, GraphVertex<T>>,
    edges: HashMap<usize, HashSet<usize>>,
}

impl<T: FromStr + Display> Graph<T> {
    /// Creates empty graph.
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    /// Add new vertex to graph with given value
    ///
    /// If vertex with such identifier exists, replace value with new one.
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use trivial_graph::graph::Graph;
    /// let mut graph = Graph::new();
    /// graph.add_vertex(1, "node".to_string());
    /// assert_eq!(graph.get_vertices_ids().len(), 1);
    /// let neighbours = graph.get_neighbours(1);
    /// assert_eq!(neighbours, Some(HashSet::new()));
    /// ```
    pub fn add_vertex(&mut self, vertex: usize, value: T) {
        self.vertices
            .insert(vertex, GraphVertex::new(vertex, value));
    }

    /// Remove vertex from graph.
    ///
    /// If vertex does not exists, nothing happens.
    ///
    /// ```
    /// use trivial_graph::graph::Graph;
    /// let mut graph = Graph::new();
    /// graph.add_vertex(1, "node".to_string());
    /// graph.remove_vertex(2);
    /// assert_eq!(graph.get_vertices_ids().len(), 1);
    /// graph.remove_vertex(1);
    /// assert_eq!(graph.get_vertices_ids().len(), 0);
    /// ```
    pub fn remove_vertex(&mut self, vertex: usize) {
        let neighbours = self.edges.remove(&vertex);
        if let Some(neighbours) = neighbours {
            for neighbour in neighbours {
                self.edges.get_mut(&neighbour).unwrap().remove(&vertex);
            }
        }
        self.vertices.remove(&vertex);
    }

    /// Add edge to current graph, both start and end of edge must exist in graph.
    ///
    /// # Errors
    /// Returns [`VertexNotExistsError`] if one of vertices not in graph.
    ///
    /// ```
    /// use trivial_graph::graph::Graph;
    /// let mut graph = Graph::new();
    /// graph.add_vertex(1, "node".to_string());
    /// graph.add_vertex(2, "node2".to_string());
    /// assert!(graph.add_edge(1, 2).is_ok());
    /// assert!(graph.add_edge(1, 3).is_err());
    /// assert!(graph.add_edge(3, 2).is_err());
    /// ```
    pub fn add_edge(
        &mut self,
        vertex_from: usize,
        vertex_to: usize,
    ) -> Result<(), VertexNotExistsError> {
        if !self.vertices.contains_key(&vertex_from) {
            return Err(VertexNotExistsError {
                message: format!("Vertex {} not exists in graph", vertex_from),
            });
        }
        if !self.vertices.contains_key(&vertex_to) {
            return Err(VertexNotExistsError {
                message: format!("Vertex {} not exists in graph", vertex_to),
            });
        }
        self.edges.entry(vertex_from).or_default().insert(vertex_to);
        Ok(())
    }

    /// Remove edge from graph.
    ///
    /// If edge not presented in graph, nothing happens.
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use trivial_graph::graph::Graph;
    /// let mut graph = Graph::new();
    /// graph.add_vertex(1, "node".to_string());
    /// graph.add_vertex(2, "node2".to_string());
    /// assert!(graph.add_edge(1, 2).is_ok());
    /// graph.remove_edge(1, 2);
    /// assert_eq!(graph.get_neighbours(1), Some(HashSet::new()));
    /// ```
    pub fn remove_edge(&mut self, vertex_from: usize, vertex_to: usize) {
        if let Some(from_neighbours) = self.edges.get_mut(&vertex_from) {
            from_neighbours.remove(&vertex_to);
            if from_neighbours.is_empty() {
                self.edges.remove(&vertex_from);
            }
        }
    }

    /// Get vertex from graph.
    ///
    /// If vertex not presented in graph, returns `None`.
    ///
    /// ```
    /// use trivial_graph::graph::Graph;
    /// let mut graph = Graph::new();
    /// graph.add_vertex(1, "node".to_string());
    /// let vertex = graph.get_vertex(1);
    /// assert!(vertex.is_some());
    /// let vertex = vertex.unwrap();
    /// assert_eq!(vertex.id, 1);
    /// assert_eq!(vertex.value, "node".to_string());
    ///
    /// assert!(graph.get_vertex(10).is_none());
    /// ```
    pub fn get_vertex(&self, vertex_id: usize) -> Option<&GraphVertex<T>> {
        self.vertices.get(&vertex_id)
    }

    /// Get set of neighbours of vertex in graph.
    ///
    /// If vertex not presented in graph, returns `None`.
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use trivial_graph::graph::Graph;
    /// let mut graph = Graph::new();
    /// graph.add_vertex(1, "node".to_string());
    /// graph.add_vertex(2, "node2".to_string());
    /// assert!(graph.add_edge(1, 2).is_ok());
    /// graph.remove_edge(1, 2);
    /// assert_eq!(graph.get_neighbours(1), Some(HashSet::new()));
    /// ```
    pub fn get_neighbours(&self, vertex: usize) -> Option<HashSet<usize>> {
        if self.vertices.contains_key(&vertex) {
            Some(
                self.edges
                    .get(&vertex)
                    .map(Clone::clone)
                    .unwrap_or(HashSet::new()),
            )
        } else {
            None
        }
    }
    /// Get set of vertices of graph.
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use trivial_graph::graph::Graph;
    /// let mut graph = Graph::new();
    /// graph.add_vertex(1, "node".to_string());
    /// graph.add_vertex(2, "node2".to_string());
    /// assert_eq!(graph.get_vertices_ids(), HashSet::from([1, 2]));
    /// ```
    pub fn get_vertices_ids(&self) -> HashSet<usize> {
        self.vertices.keys().map(usize::clone).collect()
    }

    /// Visit vertices in graph with `bfs` algorithm starting from `start_vertex` and apply `f` to them.
    ///
    /// In you want to visit all vertices in graph, see [`BfsVisitor`] for more details.
    /// ```
    /// use std::collections::HashSet;
    /// use trivial_graph::graph::Graph;
    /// let mut graph = Graph::new();
    /// graph.add_vertex(1, "node".to_string());
    /// graph.add_vertex(2, "node2".to_string());
    /// graph.add_vertex(3, "node3".to_string());
    /// graph.add_vertex(4, "node4".to_string());
    /// assert!(graph.add_edge(1, 2).is_ok());
    /// assert!(graph.add_edge(1, 3).is_ok());
    /// assert!(graph.add_edge(2, 4).is_ok());
    /// let mut visited_vertices = Vec::new();
    /// graph.bfs(1, |v| {visited_vertices.push(v.id)});
    /// assert_eq!(visited_vertices.len(), 4);
    /// assert_eq!(visited_vertices[0], 1);
    /// assert_eq!(visited_vertices[3], 4);
    /// ```
    pub fn bfs<F: FnMut(&GraphVertex<T>) -> ()>(&self, start_vertex: usize, f: F) {
        BfsVisitor::new(&self).visit(start_vertex, f);
    }

    /// Reads graph from given reader and return `Graph` structure.
    /// Requires value type to implement [`FromStr`] trait.
    ///
    ///
    /// # Errors
    /// Return `GraphParseError` in case of some I/O or parsing problems.
    /// See [`GraphParseError`] documentation for more details.
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use trivial_graph::graph::Graph;
    /// let mut graph_string = concat!(
    /// "1 1\n",
    /// "2 2\n",
    /// "#\n",
    /// "1 2\n"
    /// ).as_bytes();
    /// let res = Graph::<i32>::read_from(&mut graph_string);
    /// assert!(res.is_ok());
    /// let graph = res.unwrap();
    /// assert_eq!(graph.get_vertices_ids(), HashSet::from([1, 2]));
    /// assert_eq!(graph.get_neighbours(1), Some(HashSet::from([2])));
    /// assert_eq!(graph.get_neighbours(2), Some(HashSet::new()));
    /// ```
    ///
    /// ```
    /// use std::num::ParseIntError;
    /// use trivial_graph::graph::{Graph, GraphParseError};
    /// let mut graph_string = concat!(
    /// "1 1\n",
    /// "2 kek\n",
    /// "#\n",
    /// "1 2\n"
    /// ).as_bytes();
    /// let res = Graph::<i32>::read_from(&mut graph_string);
    /// assert!(res.is_err());
    /// let err = res.unwrap_err();
    /// if let GraphParseError::<ParseIntError>::ValueParseError(e) = err {
    ///     assert!(true);
    /// } else {
    ///     assert!(false, "Incorrect error type");
    /// }
    /// ```
    ///
    /// ```
    /// use std::num::ParseIntError;
    /// use trivial_graph::graph::{Graph, GraphParseError};
    /// let mut graph_string = concat!(
    /// "1 1\n",
    /// "2 2\n",
    /// "#\n",
    /// "1 w\n"
    /// ).as_bytes();
    /// let res = Graph::<i32>::read_from(&mut graph_string);
    /// assert!(res.is_err());
    /// let err = res.unwrap_err();
    /// if let GraphParseError::<ParseIntError>::VertexParseError(e) = err {
    ///     assert!(true);
    /// } else {
    ///     assert!(false, "Incorrect error type");
    /// }
    /// ```
    ///
    /// ```
    /// use std::num::ParseIntError;
    /// use trivial_graph::graph::{Graph, GraphParseError};
    /// let mut graph_string = concat!(
    /// "1 1\n",
    /// "2 2\n",
    /// "#\n",
    /// "1\n"
    /// ).as_bytes();
    /// let res = Graph::<i32>::read_from(&mut graph_string);
    /// assert!(res.is_err());
    /// let err = res.unwrap_err();
    /// if let GraphParseError::<ParseIntError>::DataError(_, _) = err {
    ///     assert!(true);
    /// } else {
    ///     assert!(false, "Incorrect error type");
    /// }
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
            graph.add_edge(vertex_from_id, vertex_to_id)?;
        }
        Ok(graph)
    }
}

impl<T: FromStr + Display> Display for Graph<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for v in self.vertices.values() {
            writeln!(f, "{} {}", v.id, v.value)?;
        }
        writeln!(f, "#")?;
        for (v, neighbours) in &self.edges {
            for u in neighbours {
                writeln!(f, "{} {}", v, u)?;
            }
        }
        Ok(())
    }
}
