use crate::collections::graph;
use crate::data::mlr_ast;

pub fn print_as_graphviz<F, G>(g: &graph::Graph, f: F, e: G) -> String 
    where F: Fn(graph::Vertex) -> String, G: Fn(graph::Edge) -> String
{
    let mut edges: Vec<&graph::Edge> = g.edges.iter().collect::<Vec<&graph::Edge>>();
    edges.sort_unstable();

    let mut r = String::new();

    r += "digraph G {\n";
    for v in edges.into_iter() {
        r.push_str(format!("  \"{} {}\" -> \"{} {}\" [ label=\"{}\" ];\n", v.0, f(v.0), v.1, f(v.1), e(*v)).as_str());
    }
    r += "}\n";

    r
}