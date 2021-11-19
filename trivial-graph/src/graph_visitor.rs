use std::str::FromStr;

use crate::graph::Graph;
use crate::graph_vertex::GraphVertex;
use crate::topological_sort::TopologicalSort;
use crate::visit_order::VisitOrder;

pub trait GraphVisitor<T: FromStr> {
    fn visit<F: FnMut(&GraphVertex<T>) -> ()>(&mut self, vertex: usize, f: F);
    fn clear(&mut self);
    fn get_graph(&self) -> &Graph<T>;
    fn visit_all<F: FnMut(&GraphVertex<T>) -> ()>(&mut self, visit_order: VisitOrder, mut f: F) {
        self.clear();
        let vertices: Vec<_> = match visit_order {
            VisitOrder::Undefined => self.get_graph().get_vertices_ids().into_iter().collect(),
            VisitOrder::NumbersAscending => {
                let mut v: Vec<_> = self.get_graph().get_vertices_ids().into_iter().collect();
                v.sort_unstable();
                v
            }
            VisitOrder::TopologicalSort => TopologicalSort::new(self.get_graph()).create_order(),
        };
        for v in vertices {
            self.visit(v, &mut f);
        }
    }
}
