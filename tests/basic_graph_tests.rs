extern crate graphific;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(test)]
mod basic_directed_graph_tests {
    use graphific::{AnyGraph, BasicDirectedGraph, Edge, Vertex};
    use std::borrow::Borrow;

    #[test]
    fn new_basic_directed_graph() {
        let graph: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        let vertices: Vec<Vertex<i32, i32>> = vec![];
        let edges: Vec<Edge<i32>> = vec![];
        assert_eq!(vertices, graph.vertices());
        assert_eq!(edges, graph.edges());
    }

    #[test]
    fn add_vertex() {
        let mut graph: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        let mut vertices: Vec<Vertex<i32, i32>> = vec![];

        assert_eq!(vertices, graph.vertices());

        let v1: Vertex<i32, i32> = Vertex::with_value(0, 1);
        graph = *graph.add_vertex(v1.clone()).unwrap();
        vertices.push(v1);
        assert_eq!(vertices, graph.vertices());

        let v2: Vertex<i32, i32> = Vertex::with_value(1, 2);
        graph = *graph.add_vertex(v2.clone()).unwrap();
        vertices.push(v2);
        assert_eq!(vertices, graph.vertices());

        let v3: Vertex<i32, i32> = Vertex::with_value(1, 2);
        let should_be_none = graph.add_vertex(v3.clone());
        assert_eq!(true, should_be_none.is_none());
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
