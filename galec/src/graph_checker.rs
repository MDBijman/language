use crate::data::mlr_ast::*;
use crate::data::types::*;
use crate::collections::graph::*;
use crate::collections::flat_tree::{ RandomFlatTreeIterator, PreOrderTreeIterator };

pub fn gen_node_graph(t: &Tree) -> Graph {
    let mut g = Graph::new();
    let mut it = RandomFlatTreeIterator::new(t);

    while let Some(&n) = it.next().as_ref() {
        g.make_vertex(n.0 as u32);
        for child in t.get_children(n.0).iter() {
            g.new_edge(n.0 as u32, *child as u32);
        };
    };

    g
}

#[derive(Debug)]
pub enum DependencyType {
    Equal,
    In
}

impl std::fmt::Display for DependencyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DependencyType::Equal => write!(f, "="),
            DependencyType::In => write!(f, "∈")
        }
    }
}

pub fn gen_type_dependencies(g: &Graph, t: &Tree) -> (Graph, GraphEdgeProperty<DependencyType>) {
    let mut type_dependencies: Graph = Graph::new();
    let mut dependency_types: GraphEdgeProperty<DependencyType> = GraphEdgeProperty::new();

    // Populate dependency graph with same vertices
    for v in VertexIterator::new(g) {
        type_dependencies.make_vertex(v);
    }

    // Make edges
    let mut it = VertexIterator::new(g);
    while let Some(v) = it.next() {
        let n = t.get_node_value(v as usize).unwrap();

        match n {
            Node::Let(n) => { 
                let e = type_dependencies.new_edge(n.exp as u32, n.exp_type as u32);
                dependency_types.insert(e, DependencyType::Equal);
                let e2 = type_dependencies.new_edge(n.id as u32, n.exp_type as u32);
                dependency_types.insert(e2, DependencyType::Equal);
            },
            Node::ArrayType(n) => {
                let e = type_dependencies.new_edge(v, n.value_type as u32);
                dependency_types.insert(e, DependencyType::In);
            },
            Node::Array(n) => { 
                for c in n.elements.iter() {
                    let e = type_dependencies.new_edge(*c as u32, v);
                    dependency_types.insert(e, DependencyType::In);
                };
            },
            Node::Apply(n) => {
                let e = type_dependencies.new_edge(v, n.fn_name as u32);
                dependency_types.insert(e, DependencyType::Equal);
            },
            Node::BinOp(n) => {
                match n.op_type {
                    BinOpType::ArrIndex => { type_dependencies.new_edge(n.rhs as u32, n.lhs as u32); },
                    _ => {
                        type_dependencies.new_edge(n.rhs as u32, n.lhs as u32);
                        type_dependencies.new_edge(n.lhs as u32, n.rhs as u32);
                    }
                }
            },
            _ => {}
        };
    };


    (type_dependencies, dependency_types)
}

type Scope = u32;

pub fn gen_scope_graph(t: &Tree) -> (Graph, GraphVertexProperty<Scope>) {
    let mut scope_graph = Graph::new();
    let mut node_scopes = GraphVertexProperty::new();

    let mut it = PreOrderTreeIterator::new(t);

    while let Some((id, n)) = it.next() {
        match n {
            Node::Function(f) => {
                let fun_scope = scope_graph.new_vertex();
                // Edge to scope of parent
                scope_graph.new_edge(fun_scope, *node_scopes.get(t.get_parent(id) as u32).unwrap());

                node_scopes.insert(id as u32, fun_scope);
            },
            Node::Seq(s) => {
                let seq_scope = scope_graph.new_vertex();
                // Edge to scope of parent
                scope_graph.new_edge(seq_scope, *node_scopes.get(t.get_parent(id) as u32).unwrap());

                node_scopes.insert(id as u32, seq_scope);
            },
            Node::File(f) => {
                let file_scope = scope_graph.new_vertex();
                node_scopes.insert(id as u32, file_scope);
            },
            _ => {
                node_scopes.insert(id as u32, *node_scopes.get(t.get_parent(id) as u32).unwrap());
            }
        }
    }

    (scope_graph, node_scopes)
}

pub fn solve_type_dependencies(g: &Graph, e: &GraphEdgeProperty<DependencyType>) -> GraphVertexProperty<Type> {
    let mut solved: GraphVertexProperty<Type> = GraphVertexProperty::new();
    solved
}
