/// Specifies order of starting vertices for running visitor on all graph
pub enum VisitOrder {
    /// Order not specified, depends on graph and visitor implementation
    Undefined,
    /// Order starting vertices by ascending of their numbers
    ///
    /// Note:
    /// May require sorting and increase asymptotic of algorithm
    NumbersAscending,
    /// Order starting vertices according to possible topological sort
    /// of graph.
    ///
    /// Note: works on acyclic graphs. If cycle exists, order is undefined.
    TopologicalSort,
}
