use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;
use trivial_graph::{BfsVisitor, Graph, GraphVertex, GraphVisitor, VisitOrder};

#[derive(Debug, StructOpt)]
#[structopt(name = "graph-viewer")]
struct Opt {
    /// Path to graph
    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    let opt: Opt = Opt::from_args();
    let mut file = File::open(opt.file).expect("failed to open graph file");
    let graph: Graph<String> = Graph::from_reader(&mut file).unwrap();
    let vertex_printer = |v: &GraphVertex<String>| {
        println!("Vertex: {}", v.id);
        let neighbours: Vec<_> = graph.get_neighbours(v.id).unwrap().into_iter().map(|id| id.to_string()).collect();
        println!("Neighbours: {}", neighbours.join(" "));
        println!("Value: {}", v.value);
    };
    let mut visitor = BfsVisitor::new(&graph);
    visitor.visit_all(VisitOrder::TopologicalSort, vertex_printer);
}
