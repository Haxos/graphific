extern crate graphific;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use std::fmt::Debug;

#[cfg(test)]
pub fn assert_sorted_vec_eq<T>(a: &Vec<T>, b: &Vec<T>)
where
    T: PartialEq + Ord + Clone + Debug,
{
    let mut a: Vec<T> = (*a).clone();
    let mut b: Vec<T> = (*b).clone();

    a.sort();
    b.sort();
    assert_eq!(a, b)
}

#[cfg(test)]
mod basic_directed_graph_tests {
    use crate::assert_sorted_vec_eq;
    use graphific::{AnyGraph, BasicDirectedGraph, Edge, Vertex};

    #[test]
    fn new_basic_directed_graph() {
        let graph: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        let vertices: Vec<Vertex<i32, i32>> = vec![];
        let edges: Vec<Edge<i32>> = vec![];
        assert_sorted_vec_eq(&vertices, &graph.vertices());
        assert_sorted_vec_eq(&edges, &graph.edges());
    }

    #[test]
    fn add_vertex() {
        let mut graph: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        let mut vertices: Vec<Vertex<i32, i32>> = vec![];

        assert_sorted_vec_eq(&vertices, &graph.vertices());

        let v1: Vertex<i32, i32> = Vertex::with_value(0, 1);
        graph = graph.add_vertex(v1.clone()).unwrap();
        vertices.push(v1);
        assert_sorted_vec_eq(&vertices, &graph.vertices());

        let v2: Vertex<i32, i32> = Vertex::with_value(1, 2);
        graph = graph.add_vertex(v2.clone()).unwrap();
        vertices.push(v2);
        assert_sorted_vec_eq(&vertices, &graph.vertices());

        let v3: Vertex<i32, i32> = Vertex::with_value(1, 3);
        let should_be_none = graph.add_vertex(v3.clone());
        assert_eq!(true, should_be_none.is_none());
    }

    #[test]
    fn remove_vertex() {
        let mut graph: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        let mut vertices: Vec<Vertex<i32, i32>> = vec![];
        let v1: Vertex<i32, i32> = Vertex::with_value(0, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(1, 2);
        let e1: Edge<i32> = Edge::new(v1.key().clone(), v2.key().clone());

        graph = graph.add_vertex(v1.clone()).unwrap();
        graph = graph.add_vertex(v2.clone()).unwrap();
        graph = graph.add_edge(e1.clone()).unwrap();
        vertices.push(v1.clone());
        vertices.push(v2.clone());

        // initial check
        assert_sorted_vec_eq(&vertices, &graph.vertices());

        // remove v1
        let (graph, removed_v1, removed_edges) = graph.remove_vertex(&v1).unwrap();
        let excepted_removed_edges: Vec<Edge<i32>> = vec![e1];
        let excepted_remaining_edges: Vec<Edge<i32>> = vec![];
        vertices = vec![v2.clone()];

        assert_sorted_vec_eq(&vertices, &graph.vertices());
        assert_sorted_vec_eq(&excepted_remaining_edges, &graph.edges());
        assert_eq!(v1, removed_v1);
        assert_sorted_vec_eq(&excepted_removed_edges, &removed_edges);

        // try to remove v1 but fail
        let should_be_none = graph.remove_vertex(&v1);
        assert_eq!(true, should_be_none.is_none());

        // remove v2
        let (graph, removed_v2, removed_edges) = graph.remove_vertex(&v2).unwrap();
        let excepted_removed_edges: Vec<Edge<i32>> = vec![];
        let excepted_remaining_edges: Vec<Edge<i32>> = vec![];
        vertices = vec![];

        assert_sorted_vec_eq(&vertices, &graph.vertices());
        assert_sorted_vec_eq(&excepted_remaining_edges, &graph.edges());
        assert_eq!(v2, removed_v2);
        assert_sorted_vec_eq(&excepted_removed_edges, &removed_edges);
    }

    #[test]
    fn add_edge() {
        let mut graph: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        let v1: Vertex<i32, i32> = Vertex::with_value(0, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(1, 2);
        let e1: Edge<i32> = Edge::new(v1.key().clone(), v2.key().clone());
        let e2: Edge<i32> = Edge::new(v2.key().clone(), v1.key().clone());
        let e3: Edge<i32> = Edge::new(v2.key().clone(), v2.key().clone());

        // cannot add if their is no vertices
        let should_be_none = graph.add_edge(e1.clone());
        assert_eq!(true, should_be_none.is_none());

        graph = graph.add_vertex(v1.clone()).unwrap();
        graph = graph.add_vertex(v2.clone()).unwrap();

        // add an edge
        graph = graph.add_edge(e1.clone()).unwrap();
        let expected_edges: Vec<Edge<i32>> = vec![e1.clone()];
        assert_sorted_vec_eq(&expected_edges, &graph.edges());

        // add more edges
        graph = graph.add_edge(e2.clone()).unwrap();
        graph = graph.add_edge(e3.clone()).unwrap();
        let expected_edges: Vec<Edge<i32>> = vec![e1.clone(), e2.clone(), e3.clone()];
        assert_sorted_vec_eq(&expected_edges, &graph.edges());
    }

    #[test]
    fn remove_edge() {
        unimplemented!()
    }

    #[test]
    fn remove_all_edges() {
        unimplemented!()
    }

    #[test]
    fn successors() {
        unimplemented!()
    }

    #[test]
    fn predecessors() {
        unimplemented!()
    }
}

#[cfg(test)]
mod basic_undirected_graph_tests {

    #[test]
    fn new_basic_directed_graph() {
        unimplemented!()
    }

    #[test]
    fn add_vertex() {
        unimplemented!()
    }

    #[test]
    fn remove_vertex() {
        unimplemented!()
    }

    #[test]
    fn add_edge() {
        unimplemented!()
    }

    #[test]
    fn remove_edge() {
        unimplemented!()
    }

    #[test]
    fn remove_all_edges() {
        unimplemented!()
    }

    #[test]
    fn successors() {
        unimplemented!()
    }

    #[test]
    fn predecessors() {
        unimplemented!()
    }
}
