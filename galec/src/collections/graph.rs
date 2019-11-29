use std::collections::HashMap;
use std::collections::HashSet;
use crate::collections::graph;

pub type Vertex = u32;
pub type Edge = (u32, u32);

#[derive(Debug)]
pub struct Graph {
    pub vertices: HashSet<Vertex>,
    pub edges: HashSet<Edge>,

    next_vertex: Vertex,
}

pub struct GraphVertexProperty<T> {
    pub property_map: HashMap<Vertex, T>,
}

pub struct GraphEdgeProperty<T> {
    pub property_map: HashMap<Edge, T>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph { vertices: HashSet::new(), edges: HashSet::new(), next_vertex: 0 }
    }

    pub fn new_vertex(&mut self) -> Vertex {
        let v = self.next_vertex;
        self.vertices.insert(v);
        self.next_vertex += 1;
        v
    }

    pub fn make_vertex(&mut self, v: Vertex) -> bool {
        if v > self.next_vertex {
            self.next_vertex = v + 1;
        };
        self.vertices.insert(v)
    }

    pub fn delete_vertex(&mut self, v: Vertex) {
        self.vertices.remove(&v);
    }

    pub fn new_edge(&mut self, a: Vertex, b: Vertex) -> Edge {
        self.edges.insert((a, b));
        (a, b)
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

impl<T> GraphVertexProperty<T> {
    pub fn new() -> GraphVertexProperty<T> {
        GraphVertexProperty { property_map: HashMap::new() }
    }

    pub fn get(&self, v: Vertex) -> Option<&T> {
        self.property_map.get(&v)
    }

    pub fn insert(&mut self, v: Vertex, t: T) {
        self.property_map.insert(v, t);
    }

    pub fn delete(&mut self, v: Vertex) {
        self.property_map.remove(&v);
    }
}

impl<T> GraphEdgeProperty<T> {
    pub fn new() -> GraphEdgeProperty<T> {
        GraphEdgeProperty { property_map: HashMap::new() }
    }

    pub fn get(&self, e: Edge) -> Option<&T> {
        self.property_map.get(&e)
    }

    pub fn insert(&mut self, e: Edge, t: T) {
        self.property_map.insert(e, t);
    }

    pub fn delete(&mut self, e: Edge) {
        self.property_map.remove(&e);
    }
}

pub struct VertexIterator<'a> {
    vertex_it: std::collections::hash_set::Iter<'a, Vertex>
}

impl<'a> VertexIterator<'a> {
    pub fn new(g: &'a Graph) -> VertexIterator {
        VertexIterator { vertex_it: g.vertices.iter() }
    }
}

impl<'a> Iterator for VertexIterator<'a> {
    type Item = Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        match self.vertex_it.next() {
            None => None,
            Some(&u) => Some(u)
        }
    }
}


pub struct EdgeIterator<'a> {
    edge_it: std::collections::hash_set::Iter<'a, Edge>
}

impl<'a> EdgeIterator<'a> {
    pub fn new(g: &'a Graph) -> EdgeIterator {
        EdgeIterator { edge_it: g.edges.iter() }
    }
}

impl<'a> Iterator for EdgeIterator<'a> {
    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        match self.edge_it.next() {
            None => None,
            Some(&u) => Some(u)
        }
    }
}

#[test]
fn test_edge_iterator_single() {
    let mut g: Graph = Graph::new();
    let v1 = g.new_vertex();
    let v2 = g.new_vertex();
    let e1 = g.new_edge(v1, v2);

    assert_eq!(EdgeIterator::new(&g).next(), Some(e1));
}


#[test]
fn test_edge_iterator_multiple() {
    let mut g: Graph = Graph::new();
    let v1 = g.new_vertex();
    let v2 = g.new_vertex();
    let v3 = g.new_vertex();
    let e1 = g.new_edge(v1, v2);
    let e2 = g.new_edge(v2, v3);

    let mut it = EdgeIterator::new(&g); 
    assert_eq!(it.next(), Some(e1));
    assert_eq!(it.next(), Some(e2));
}