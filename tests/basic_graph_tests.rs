#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod utils;

#[cfg(test)]
mod basic_directed_graph_tests {
    use crate::utils::assert_sorted_vec_eq;
    use graphific::{AnyGraph, BasicDirectedGraph, Edge, Kinship, Vertex};

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

        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        graph = graph.add_vertex(v1.clone()).unwrap();
        vertices.push(v1);
        assert_sorted_vec_eq(&vertices, &graph.vertices());

        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
        graph = graph.add_vertex(v2.clone()).unwrap();
        vertices.push(v2);
        assert_sorted_vec_eq(&vertices, &graph.vertices());

        let v3: Vertex<i32, i32> = Vertex::with_value(2, 9);
        let should_be_none = graph.add_vertex(v3.clone());
        assert_eq!(true, should_be_none.is_none());
    }

    #[test]
    fn remove_vertex() {
        let mut graph: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        let mut vertices: Vec<Vertex<i32, i32>> = vec![];
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
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
    fn remove_all_vertices() {
        let mut graph: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
        let v3: Vertex<i32, i32> = Vertex::with_value(3, 9);
        let e1: Edge<i32> = Edge::new(v1.key().clone(), v2.key().clone());
        let e2: Edge<i32> = Edge::new(v2.key().clone(), v1.key().clone());
        let e3: Edge<i32> = Edge::new(v2.key().clone(), v2.key().clone());
        let e4: Edge<i32> = Edge::new(v1.key().clone(), v3.key().clone());

        // init
        let expected_vertices = vec![v1.clone(), v2.clone(), v3.clone()];
        let expected_edges = vec![e1.clone(), e2.clone(), e3.clone(), e4.clone()];
        graph = graph
            .add_vertex(v1.clone())
            .unwrap()
            .add_vertex(v2.clone())
            .unwrap()
            .add_vertex(v3.clone())
            .unwrap()
            .add_edge(e1.clone())
            .unwrap()
            .add_edge(e2.clone())
            .unwrap()
            .add_edge(e3.clone())
            .unwrap()
            .add_edge(e4.clone())
            .unwrap();

        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());
        assert_sorted_vec_eq(&expected_edges, &graph.edges());

        // remove all vertices
        let (result_graph, removed_vertices, removed_edges) = graph.remove_all_vertices().unwrap();
        let expected_removed_vertices = vec![v1.clone(), v2.clone(), v3.clone()];
        let expected_removed_edges = vec![e1.clone(), e2.clone(), e3.clone(), e4.clone()];
        let expected_remaining_vertices: Vec<Vertex<i32, i32>> = vec![];
        let expected_remaining_edges: Vec<Edge<i32>> = vec![];

        assert_sorted_vec_eq(&expected_removed_vertices, &removed_vertices);
        assert_sorted_vec_eq(&expected_removed_edges, &removed_edges);
        assert_sorted_vec_eq(&expected_remaining_vertices, &result_graph.vertices());
        assert_sorted_vec_eq(&expected_remaining_edges, &result_graph.edges());
    }

    #[test]
    fn add_edge() {
        let mut graph: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
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
        let mut graph: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
        let e1: Edge<i32> = Edge::new(v1.key().clone(), v2.key().clone());
        let e2: Edge<i32> = Edge::new(v2.key().clone(), v1.key().clone());
        let e3: Edge<i32> = Edge::new(v2.key().clone(), v2.key().clone());

        // init
        let expected_vertices = vec![v1.clone(), v2.clone()];
        graph = graph.add_vertex(v1.clone()).unwrap();
        graph = graph.add_vertex(v2.clone()).unwrap();
        graph = graph.add_edge(e1.clone()).unwrap();
        graph = graph.add_edge(e2.clone()).unwrap();
        graph = graph.add_edge(e3.clone()).unwrap();

        let expected_edges = vec![e1.clone(), e2.clone(), e3.clone()];
        assert_sorted_vec_eq(&expected_edges, &graph.edges());

        // remove e1
        let (graph, removed_edge) = graph.remove_edge(&e1).unwrap();
        assert_eq!(e1, removed_edge);
        let expected_edges = vec![e2.clone(), e3.clone()];
        assert_sorted_vec_eq(&expected_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());

        // try to remove e1 but fail because this edge doesn't exists
        let should_be_none = graph.remove_edge(&e1);
        assert_eq!(true, should_be_none.is_none());
        assert_sorted_vec_eq(&expected_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());

        // remove v2 and v3
        let (graph, removed_e2) = graph.remove_edge(&e2).unwrap();
        let (graph, removed_e3) = graph.remove_edge(&e3).unwrap();
        let expected_edges = vec![];
        assert_eq!(e2, removed_e2);
        assert_eq!(e3, removed_e3);
        assert_sorted_vec_eq(&expected_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());
    }

    #[test]
    fn remove_all_edges() {
        let mut graph: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
        let v3: Vertex<i32, i32> = Vertex::with_value(3, 9);
        let e1: Edge<i32> = Edge::new(v1.key().clone(), v2.key().clone());
        let e2: Edge<i32> = Edge::new(v2.key().clone(), v1.key().clone());
        let e3: Edge<i32> = Edge::new(v2.key().clone(), v2.key().clone());
        let e4: Edge<i32> = Edge::new(v1.key().clone(), v3.key().clone());

        // init
        let expected_vertices = vec![v1.clone(), v2.clone(), v3.clone()];
        let expected_edges = vec![e1.clone(), e2.clone(), e3.clone(), e4.clone()];
        graph = graph
            .add_vertex(v1.clone())
            .unwrap()
            .add_vertex(v2.clone())
            .unwrap()
            .add_vertex(v3.clone())
            .unwrap()
            .add_edge(e1.clone())
            .unwrap()
            .add_edge(e2.clone())
            .unwrap()
            .add_edge(e3.clone())
            .unwrap()
            .add_edge(e4.clone())
            .unwrap();

        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());
        assert_sorted_vec_eq(&expected_edges, &graph.edges());

        // remove all vertices
        let (result_graph, removed_edges) = graph.remove_all_edges().unwrap();
        let expected_removed_edges = vec![e1.clone(), e2.clone(), e3.clone(), e4.clone()];
        let expected_remaining_vertices: Vec<Vertex<i32, i32>> =
            vec![v1.clone(), v2.clone(), v3.clone()];
        let expected_remaining_edges: Vec<Edge<i32>> = vec![];

        assert_sorted_vec_eq(&expected_removed_edges, &removed_edges);
        assert_sorted_vec_eq(&expected_remaining_vertices, &result_graph.vertices());
        assert_sorted_vec_eq(&expected_remaining_edges, &result_graph.edges());
    }

    #[test]
    fn remove_all_edges_where_vertex() {
        let mut graph: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
        let v3: Vertex<i32, i32> = Vertex::with_value(3, 9);
        let v4: Vertex<i32, i32> = Vertex::with_value(-1, -1);
        let e1: Edge<i32> = Edge::new(v1.key().clone(), v2.key().clone());
        let e2: Edge<i32> = Edge::new(v2.key().clone(), v1.key().clone());
        let e3: Edge<i32> = Edge::new(v2.key().clone(), v2.key().clone());
        let e4: Edge<i32> = Edge::new(v1.key().clone(), v3.key().clone());

        // init
        let expected_vertices = vec![v1.clone(), v2.clone(), v3.clone()];
        graph = graph.add_vertex(v1.clone()).unwrap();
        graph = graph.add_vertex(v2.clone()).unwrap();
        graph = graph.add_vertex(v3.clone()).unwrap();
        graph = graph.add_edge(e1.clone()).unwrap();
        graph = graph.add_edge(e2.clone()).unwrap();
        graph = graph.add_edge(e3.clone()).unwrap();
        graph = graph.add_edge(e4.clone()).unwrap();

        let expected_edges = vec![e1.clone(), e2.clone(), e3.clone(), e4.clone()];
        assert_sorted_vec_eq(&expected_edges, &graph.edges());

        // remove all from v1
        let (graph, removed_edges) = graph.remove_all_edges_where_vertex(&v1).unwrap();
        let expected_removed_edges = vec![e1.clone(), e2.clone(), e4.clone()];
        let expected_remaining_edges = vec![e3.clone()];
        assert_sorted_vec_eq(&expected_removed_edges, &removed_edges);
        assert_sorted_vec_eq(&expected_remaining_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());

        // try to remove v4 but fail because v4 doesn't exists in graph
        let should_be_none = graph.remove_all_edges_where_vertex(&v4);
        assert_eq!(true, should_be_none.is_none());
        assert_sorted_vec_eq(&expected_remaining_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());

        // remove all from v3
        let (graph, removed_edges) = graph.remove_all_edges_where_vertex(&v3).unwrap();
        let expected_removed_edges = vec![];
        let expected_remaining_edges = vec![e3.clone()];
        assert_sorted_vec_eq(&expected_removed_edges, &removed_edges);
        assert_sorted_vec_eq(&expected_remaining_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());

        // remove all from v2
        let (graph, removed_edges) = graph.remove_all_edges_where_vertex(&v2).unwrap();
        let expected_removed_edges = vec![e3.clone()];
        let expected_remaining_edges = vec![];
        assert_sorted_vec_eq(&expected_removed_edges, &removed_edges);
        assert_sorted_vec_eq(&expected_remaining_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());
    }

    #[test]
    fn remove_all_edges_from() {
        let mut graph: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
        let v3: Vertex<i32, i32> = Vertex::with_value(3, 9);
        let v4: Vertex<i32, i32> = Vertex::with_value(-1, -1);
        let e1: Edge<i32> = Edge::new(v1.key().clone(), v2.key().clone());
        let e2: Edge<i32> = Edge::new(v2.key().clone(), v1.key().clone());
        let e3: Edge<i32> = Edge::new(v2.key().clone(), v2.key().clone());
        let e4: Edge<i32> = Edge::new(v1.key().clone(), v3.key().clone());

        // init
        let expected_vertices = vec![v1.clone(), v2.clone(), v3.clone()];
        graph = graph.add_vertex(v1.clone()).unwrap();
        graph = graph.add_vertex(v2.clone()).unwrap();
        graph = graph.add_vertex(v3.clone()).unwrap();
        graph = graph.add_edge(e1.clone()).unwrap();
        graph = graph.add_edge(e2.clone()).unwrap();
        graph = graph.add_edge(e3.clone()).unwrap();
        graph = graph.add_edge(e4.clone()).unwrap();

        let expected_edges = vec![e1.clone(), e2.clone(), e3.clone(), e4.clone()];
        assert_sorted_vec_eq(&expected_edges, &graph.edges());

        // remove all from v1
        let (graph, removed_edges) = graph.remove_all_edges_from_vertex(&v1).unwrap();
        let expected_removed_edges = vec![e1.clone(), e4.clone()];
        let expected_remaining_edges = vec![e2.clone(), e3.clone()];
        assert_sorted_vec_eq(&expected_removed_edges, &removed_edges);
        assert_sorted_vec_eq(&expected_remaining_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());

        // try to remove v4 but fail because v4 doesn't exists in graph
        let should_be_none = graph.remove_all_edges_from_vertex(&v4);
        assert_eq!(true, should_be_none.is_none());
        assert_sorted_vec_eq(&expected_remaining_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());

        // remove all from v3
        let (graph, removed_edges) = graph.remove_all_edges_from_vertex(&v3).unwrap();
        let expected_removed_edges = vec![];
        let expected_remaining_edges = vec![e2.clone(), e3.clone()];
        assert_sorted_vec_eq(&expected_removed_edges, &removed_edges);
        assert_sorted_vec_eq(&expected_remaining_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());

        // remove all from v2
        let (graph, removed_edges) = graph.remove_all_edges_from_vertex(&v2).unwrap();
        let expected_removed_edges = vec![e2.clone(), e3.clone()];
        let expected_remaining_edges = vec![];
        assert_sorted_vec_eq(&expected_removed_edges, &removed_edges);
        assert_sorted_vec_eq(&expected_remaining_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());
    }

    #[test]
    fn successors() {
        let mut graph: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
        let v3: Vertex<i32, i32> = Vertex::with_value(3, 9);
        let e1: Edge<i32> = Edge::new(v1.key().clone(), v2.key().clone());
        let e2: Edge<i32> = Edge::new(v2.key().clone(), v1.key().clone());
        let e3: Edge<i32> = Edge::new(v2.key().clone(), v2.key().clone());
        let e4: Edge<i32> = Edge::new(v1.key().clone(), v3.key().clone());

        // init
        graph = graph
            .add_vertex(v1.clone())
            .unwrap()
            .add_vertex(v2.clone())
            .unwrap()
            .add_vertex(v3.clone())
            .unwrap()
            .add_edge(e1.clone())
            .unwrap()
            .add_edge(e2.clone())
            .unwrap()
            .add_edge(e3.clone())
            .unwrap()
            .add_edge(e4.clone())
            .unwrap();

        let expected_vertices = vec![v1.clone(), v2.clone(), v3.clone()];
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());
        let expected_edges = vec![e1.clone(), e2.clone(), e3.clone(), e4.clone()];
        assert_sorted_vec_eq(&expected_edges, &graph.edges());

        // tests all vertices and its corresponding edges
        let successors = graph.successors();
        let expected_edges_v1 = vec![e1.clone(), e4.clone()];
        let expected_edges_v2 = vec![e2.clone(), e3.clone()];
        let expected_edges_v3 = vec![];

        assert_sorted_vec_eq(&expected_edges_v1, &successors.get(&v1).unwrap());
        assert_sorted_vec_eq(&expected_edges_v2, &successors.get(&v2).unwrap());
        assert_sorted_vec_eq(&expected_edges_v3, &successors.get(&v3).unwrap());

        // tests all keys and its corresponding edges
        let successors = graph.successors_as_key_and_edges();

        assert_sorted_vec_eq(&expected_edges_v1, &successors.get(v1.key()).unwrap());
        assert_sorted_vec_eq(&expected_edges_v2, &successors.get(v2.key()).unwrap());
        assert_sorted_vec_eq(&expected_edges_v3, &successors.get(v3.key()).unwrap());
    }

    #[test]
    fn predecessors() {
        let mut graph: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
        let v3: Vertex<i32, i32> = Vertex::with_value(3, -1);
        let e1: Edge<i32> = Edge::new(v1.key().clone(), v2.key().clone());
        let e2: Edge<i32> = Edge::new(v2.key().clone(), v1.key().clone());
        let e3: Edge<i32> = Edge::new(v2.key().clone(), v2.key().clone());
        let e4: Edge<i32> = Edge::new(v1.key().clone(), v3.key().clone());

        // init
        graph = graph.add_vertex(v1.clone()).unwrap();
        graph = graph.add_vertex(v2.clone()).unwrap();
        graph = graph.add_vertex(v3.clone()).unwrap();
        graph = graph.add_edge(e1.clone()).unwrap();
        graph = graph.add_edge(e2.clone()).unwrap();
        graph = graph.add_edge(e3.clone()).unwrap();
        graph = graph.add_edge(e4.clone()).unwrap();

        let expected_vertices = vec![v1.clone(), v2.clone(), v3.clone()];
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());
        let expected_edges = vec![e1.clone(), e2.clone(), e3.clone(), e4.clone()];
        assert_sorted_vec_eq(&expected_edges, &graph.edges());

        let predecessors = graph.predecessors();

        // tests all vertices and its corresponding edges
        let expected_edges_v1 = vec![e2.clone()];
        let expected_edges_v2 = vec![e1.clone(), e3.clone()];
        let expected_edges_v3 = vec![e4.clone()];

        assert_sorted_vec_eq(&expected_edges_v1, &predecessors.get(&v1).unwrap());
        assert_sorted_vec_eq(&expected_edges_v2, &predecessors.get(&v2).unwrap());
        assert_sorted_vec_eq(&expected_edges_v3, &predecessors.get(&v3).unwrap());

        // tests all keys and its corresponding edges
        let predecessors = graph.predecessors_as_key_and_edges();

        assert_sorted_vec_eq(&expected_edges_v1, &predecessors.get(v1.key()).unwrap());
        assert_sorted_vec_eq(&expected_edges_v2, &predecessors.get(v2.key()).unwrap());
        assert_sorted_vec_eq(&expected_edges_v3, &predecessors.get(v3.key()).unwrap());
    }

    #[test]
    fn eq() {
        let mut graph1: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        let mut graph2: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
        let v3: Vertex<i32, i32> = Vertex::with_value(3, 9);
        let e1: Edge<i32> = Edge::new(v1.key().clone(), v2.key().clone());
        let e2: Edge<i32> = Edge::new(v2.key().clone(), v1.key().clone());
        let e3: Edge<i32> = Edge::new(v2.key().clone(), v2.key().clone());
        let e4: Edge<i32> = Edge::new(v1.key().clone(), v3.key().clone());

        // init
        graph1 = graph1
            .add_vertex(v1.clone())
            .unwrap()
            .add_vertex(v2.clone())
            .unwrap()
            .add_vertex(v3.clone())
            .unwrap()
            .add_edge(e1.clone())
            .unwrap()
            .add_edge(e2.clone())
            .unwrap()
            .add_edge(e3.clone())
            .unwrap()
            .add_edge(e4.clone())
            .unwrap();

        graph2 = graph2
            .add_vertex(v1.clone())
            .unwrap()
            .add_vertex(v2.clone())
            .unwrap()
            .add_edge(e1.clone())
            .unwrap()
            .add_edge(e2.clone())
            .unwrap()
            .add_edge(e3.clone())
            .unwrap();

        assert_eq!(false, graph1.eq(&graph2));
        assert_eq!(true, graph1.ne(&graph2));

        graph2 = graph2.add_vertex(v3.clone()).unwrap();
        assert_eq!(false, graph1.eq(&graph2));
        assert_eq!(true, graph1.ne(&graph2));

        graph2 = graph2.add_edge(e4.clone()).unwrap();
        assert_eq!(true, graph1.eq(&graph2));
        assert_eq!(false, graph1.ne(&graph2));
    }
}

#[cfg(test)]
mod basic_undirected_graph_tests {
    use crate::utils::assert_sorted_vec_eq;
    use graphific::{AnyGraph, BasicUndirectedGraph, Edge, Kinship, Vertex};

    #[test]
    fn new_basic_directed_graph() {
        let graph: BasicUndirectedGraph<i32, i32> = BasicUndirectedGraph::new();
        let vertices: Vec<Vertex<i32, i32>> = vec![];
        let edges: Vec<Edge<i32>> = vec![];
        assert_sorted_vec_eq(&vertices, &graph.vertices());
        assert_sorted_vec_eq(&edges, &graph.edges());
    }

    #[test]
    fn add_vertex() {
        let mut graph: BasicUndirectedGraph<i32, i32> = BasicUndirectedGraph::new();
        let mut vertices: Vec<Vertex<i32, i32>> = vec![];

        assert_sorted_vec_eq(&vertices, &graph.vertices());

        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        graph = graph.add_vertex(v1.clone()).unwrap();
        vertices.push(v1);
        assert_sorted_vec_eq(&vertices, &graph.vertices());

        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
        graph = graph.add_vertex(v2.clone()).unwrap();
        vertices.push(v2);
        assert_sorted_vec_eq(&vertices, &graph.vertices());

        let v3: Vertex<i32, i32> = Vertex::with_value(2, -1);
        let should_be_none = graph.add_vertex(v3.clone());
        assert_eq!(true, should_be_none.is_none());
    }

    #[test]
    fn remove_vertex() {
        let mut graph: BasicUndirectedGraph<i32, i32> = BasicUndirectedGraph::new();
        let mut vertices: Vec<Vertex<i32, i32>> = vec![];
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
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
    fn remove_all_vertices() {
        let mut graph: BasicUndirectedGraph<i32, i32> = BasicUndirectedGraph::new();
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
        let v3: Vertex<i32, i32> = Vertex::with_value(3, 9);
        let e1: Edge<i32> = Edge::new(v1.key().clone(), v2.key().clone());
        let e2: Edge<i32> = Edge::new(v2.key().clone(), v2.key().clone());
        let e3: Edge<i32> = Edge::new(v1.key().clone(), v3.key().clone());
        let e4: Edge<i32> = Edge::new(v2.key().clone(), v3.key().clone());

        // init
        let expected_vertices = vec![v1.clone(), v2.clone(), v3.clone()];
        let expected_edges = vec![e1.clone(), e2.clone(), e3.clone(), e4.clone()];
        graph = graph
            .add_vertex(v1.clone())
            .unwrap()
            .add_vertex(v2.clone())
            .unwrap()
            .add_vertex(v3.clone())
            .unwrap()
            .add_edge(e1.clone())
            .unwrap()
            .add_edge(e2.clone())
            .unwrap()
            .add_edge(e3.clone())
            .unwrap()
            .add_edge(e4.clone())
            .unwrap();

        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());
        assert_sorted_vec_eq(&expected_edges, &graph.edges());

        // remove all vertices
        let (result_graph, removed_vertices, removed_edges) = graph.remove_all_vertices().unwrap();
        let expected_removed_vertices = vec![v1.clone(), v2.clone(), v3.clone()];
        let expected_removed_edges = vec![e1.clone(), e2.clone(), e3.clone(), e4.clone()];
        let expected_remaining_vertices: Vec<Vertex<i32, i32>> = vec![];
        let expected_remaining_edges: Vec<Edge<i32>> = vec![];

        assert_sorted_vec_eq(&expected_removed_vertices, &removed_vertices);
        assert_sorted_vec_eq(&expected_removed_edges, &removed_edges);
        assert_sorted_vec_eq(&expected_remaining_vertices, &result_graph.vertices());
        assert_sorted_vec_eq(&expected_remaining_edges, &result_graph.edges());
    }

    #[test]
    fn add_edge() {
        let mut graph: BasicUndirectedGraph<i32, i32> = BasicUndirectedGraph::new();
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
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

        // cannot add the same edge in reverse order
        let should_be_none = graph.add_edge(e2.clone());
        assert_eq!(true, should_be_none.is_none());

        // add more edges
        graph = graph.add_edge(e3.clone()).unwrap();
        let expected_edges: Vec<Edge<i32>> = vec![e1.clone(), e3.clone()];
        assert_sorted_vec_eq(&expected_edges, &graph.edges());
    }

    #[test]
    fn remove_edge() {
        let mut graph: BasicUndirectedGraph<i32, i32> = BasicUndirectedGraph::new();
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
        let e1: Edge<i32> = Edge::new(v1.key().clone(), v2.key().clone());
        let e2: Edge<i32> = Edge::new(v2.key().clone(), v2.key().clone());

        // init
        let expected_vertices = vec![v1.clone(), v2.clone()];
        graph = graph.add_vertex(v1.clone()).unwrap();
        graph = graph.add_vertex(v2.clone()).unwrap();
        graph = graph.add_edge(e1.clone()).unwrap();
        graph = graph.add_edge(e2.clone()).unwrap();

        let expected_edges = vec![e1.clone(), e2.clone()];
        assert_sorted_vec_eq(&expected_edges, &graph.edges());

        // remove e1
        let (graph, removed_edge) = graph.remove_edge(&e1).unwrap();
        assert_eq!(e1, removed_edge);
        let expected_edges = vec![e2.clone()];
        assert_sorted_vec_eq(&expected_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());

        // try to remove e1 but fail because this edge doesn't exists
        let should_be_none = graph.remove_edge(&e1);
        assert_eq!(true, should_be_none.is_none());
        assert_sorted_vec_eq(&expected_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());

        // remove v2
        let (graph, removed_e2) = graph.remove_edge(&e2).unwrap();
        let expected_edges = vec![];
        assert_eq!(e2, removed_e2);
        assert_sorted_vec_eq(&expected_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());
    }

    #[test]
    fn remove_all_edges() {
        let mut graph: BasicUndirectedGraph<i32, i32> = BasicUndirectedGraph::new();
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
        let v3: Vertex<i32, i32> = Vertex::with_value(3, 9);
        let e1: Edge<i32> = Edge::new(v1.key().clone(), v2.key().clone());
        let e2: Edge<i32> = Edge::new(v2.key().clone(), v2.key().clone());
        let e3: Edge<i32> = Edge::new(v1.key().clone(), v3.key().clone());

        // init
        let expected_vertices = vec![v1.clone(), v2.clone(), v3.clone()];
        let expected_edges = vec![e1.clone(), e2.clone(), e3.clone()];
        graph = graph
            .add_vertex(v1.clone())
            .unwrap()
            .add_vertex(v2.clone())
            .unwrap()
            .add_vertex(v3.clone())
            .unwrap()
            .add_edge(e1.clone())
            .unwrap()
            .add_edge(e2.clone())
            .unwrap()
            .add_edge(e3.clone())
            .unwrap();

        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());
        assert_sorted_vec_eq(&expected_edges, &graph.edges());

        // remove all vertices
        let (result_graph, removed_edges) = graph.remove_all_edges().unwrap();
        let expected_removed_edges = vec![e1.clone(), e2.clone(), e3.clone()];
        let expected_remaining_vertices: Vec<Vertex<i32, i32>> =
            vec![v1.clone(), v2.clone(), v3.clone()];
        let expected_remaining_edges: Vec<Edge<i32>> = vec![];

        assert_sorted_vec_eq(&expected_removed_edges, &removed_edges);
        assert_sorted_vec_eq(&expected_remaining_vertices, &result_graph.vertices());
        assert_sorted_vec_eq(&expected_remaining_edges, &result_graph.edges());
    }

    #[test]
    fn remove_all_edges_where_vertex() {
        let mut graph: BasicUndirectedGraph<i32, i32> = BasicUndirectedGraph::new();
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
        let v3: Vertex<i32, i32> = Vertex::with_value(3, 9);
        let v4: Vertex<i32, i32> = Vertex::with_value(-1, -1);
        let e1: Edge<i32> = Edge::new(v1.key().clone(), v2.key().clone());
        let e2: Edge<i32> = Edge::new(v2.key().clone(), v2.key().clone());
        let e3: Edge<i32> = Edge::new(v1.key().clone(), v3.key().clone());
        let e4: Edge<i32> = Edge::new(v2.key().clone(), v3.key().clone());

        // init
        let expected_vertices = vec![v1.clone(), v2.clone(), v3.clone()];
        graph = graph.add_vertex(v1.clone()).unwrap();
        graph = graph.add_vertex(v2.clone()).unwrap();
        graph = graph.add_vertex(v3.clone()).unwrap();
        graph = graph.add_edge(e1.clone()).unwrap();
        graph = graph.add_edge(e2.clone()).unwrap();
        graph = graph.add_edge(e3.clone()).unwrap();
        graph = graph.add_edge(e4.clone()).unwrap();

        let expected_edges = vec![e1.clone(), e2.clone(), e3.clone(), e4.clone()];
        assert_sorted_vec_eq(&expected_edges, &graph.edges());

        // remove all from v1
        let (graph, removed_edges) = graph.remove_all_edges_where_vertex(&v1).unwrap();
        let expected_removed_edges = vec![e1.clone(), e3.clone()];
        let expected_remaining_edges = vec![e2.clone(), e4.clone()];
        assert_sorted_vec_eq(&expected_removed_edges, &removed_edges);
        assert_sorted_vec_eq(&expected_remaining_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());

        // try to remove v4 but fail because v4 doesn't exists in graph
        let should_be_none = graph.remove_all_edges_where_vertex(&v4);
        assert_eq!(true, should_be_none.is_none());
        assert_sorted_vec_eq(&expected_remaining_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());

        // remove all from v3
        let (graph, removed_edges) = graph.remove_all_edges_where_vertex(&v3).unwrap();
        let expected_removed_edges = vec![e4.clone()];
        let expected_remaining_edges = vec![e2.clone()];
        assert_sorted_vec_eq(&expected_removed_edges, &removed_edges);
        assert_sorted_vec_eq(&expected_remaining_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());

        // remove all from v2
        let (graph, removed_edges) = graph.remove_all_edges_where_vertex(&v2).unwrap();
        let expected_removed_edges = vec![e2.clone()];
        let expected_remaining_edges = vec![];
        assert_sorted_vec_eq(&expected_removed_edges, &removed_edges);
        assert_sorted_vec_eq(&expected_remaining_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());
    }

    #[test]
    fn remove_all_edges_from() {
        let mut graph: BasicUndirectedGraph<i32, i32> = BasicUndirectedGraph::new();
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
        let v3: Vertex<i32, i32> = Vertex::with_value(3, 9);
        let v4: Vertex<i32, i32> = Vertex::with_value(-1, -1);
        let e1: Edge<i32> = Edge::new(v1.key().clone(), v2.key().clone());
        let e2: Edge<i32> = Edge::new(v2.key().clone(), v2.key().clone());
        let e3: Edge<i32> = Edge::new(v1.key().clone(), v3.key().clone());
        let e4: Edge<i32> = Edge::new(v2.key().clone(), v3.key().clone());

        // init
        let expected_vertices = vec![v1.clone(), v2.clone(), v3.clone()];
        graph = graph.add_vertex(v1.clone()).unwrap();
        graph = graph.add_vertex(v2.clone()).unwrap();
        graph = graph.add_vertex(v3.clone()).unwrap();
        graph = graph.add_edge(e1.clone()).unwrap();
        graph = graph.add_edge(e2.clone()).unwrap();
        graph = graph.add_edge(e3.clone()).unwrap();
        graph = graph.add_edge(e4.clone()).unwrap();

        let expected_edges = vec![e1.clone(), e2.clone(), e3.clone(), e4.clone()];
        assert_sorted_vec_eq(&expected_edges, &graph.edges());

        // remove all from v1
        let (graph, removed_edges) = graph.remove_all_edges_from_vertex(&v1).unwrap();
        let expected_removed_edges = vec![e1.clone(), e3.clone()];
        let expected_remaining_edges = vec![e2.clone(), e4.clone()];
        assert_sorted_vec_eq(&expected_removed_edges, &removed_edges);
        assert_sorted_vec_eq(&expected_remaining_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());

        // try to remove v4 but fail because v4 doesn't exists in graph
        let should_be_none = graph.remove_all_edges_from_vertex(&v4);
        assert_eq!(true, should_be_none.is_none());
        assert_sorted_vec_eq(&expected_remaining_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());

        // remove all from v3
        let (graph, removed_edges) = graph.remove_all_edges_from_vertex(&v3).unwrap();
        let expected_removed_edges = vec![e4.clone()];
        let expected_remaining_edges = vec![e2.clone()];
        assert_sorted_vec_eq(&expected_removed_edges, &removed_edges);
        assert_sorted_vec_eq(&expected_remaining_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());

        // remove all from v2
        let (graph, removed_edges) = graph.remove_all_edges_from_vertex(&v2).unwrap();
        let expected_removed_edges = vec![e2.clone()];
        let expected_remaining_edges = vec![];
        assert_sorted_vec_eq(&expected_removed_edges, &removed_edges);
        assert_sorted_vec_eq(&expected_remaining_edges, &graph.edges());
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());
    }

    #[test]
    fn successors() {
        let mut graph: BasicUndirectedGraph<i32, i32> = BasicUndirectedGraph::new();
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
        let v3: Vertex<i32, i32> = Vertex::with_value(3, 9);
        let e1: Edge<i32> = Edge::new(v1.key().clone(), v2.key().clone());
        let e2: Edge<i32> = Edge::new(v2.key().clone(), v2.key().clone());
        let e3: Edge<i32> = Edge::new(v1.key().clone(), v3.key().clone());
        let e4: Edge<i32> = Edge::new(v2.key().clone(), v3.key().clone());

        // init
        graph = graph.add_vertex(v1.clone()).unwrap();
        graph = graph.add_vertex(v2.clone()).unwrap();
        graph = graph.add_vertex(v3.clone()).unwrap();
        graph = graph.add_edge(e1.clone()).unwrap();
        graph = graph.add_edge(e2.clone()).unwrap();
        graph = graph.add_edge(e3.clone()).unwrap();
        graph = graph.add_edge(e4.clone()).unwrap();

        let expected_vertices = vec![v1.clone(), v2.clone(), v3.clone()];
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());
        let expected_edges = vec![e1.clone(), e2.clone(), e3.clone(), e4.clone()];
        assert_sorted_vec_eq(&expected_edges, &graph.edges());

        let successors = graph.successors();

        // tests all vertices and its corresponding edges
        let expected_edges_v1 = vec![e1.clone(), e3.clone()];
        let expected_edges_v2 = vec![e1.clone(), e2.clone(), e4.clone()];
        let expected_edges_v3 = vec![e3.clone(), e4.clone()];

        assert_sorted_vec_eq(&expected_edges_v1, &successors.get(&v1).unwrap());
        assert_sorted_vec_eq(&expected_edges_v2, &successors.get(&v2).unwrap());
        assert_sorted_vec_eq(&expected_edges_v3, &successors.get(&v3).unwrap());

        // tests all keys and its corresponding edges
        let successors = graph.successors_as_key_and_edges();

        assert_sorted_vec_eq(&expected_edges_v1, &successors.get(v1.key()).unwrap());
        assert_sorted_vec_eq(&expected_edges_v2, &successors.get(v2.key()).unwrap());
        assert_sorted_vec_eq(&expected_edges_v3, &successors.get(v3.key()).unwrap());
    }

    #[test]
    fn predecessors() {
        let mut graph: BasicUndirectedGraph<i32, i32> = BasicUndirectedGraph::new();
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
        let v3: Vertex<i32, i32> = Vertex::with_value(3, 9);
        let e1: Edge<i32> = Edge::new(v1.key().clone(), v2.key().clone());
        let e2: Edge<i32> = Edge::new(v2.key().clone(), v2.key().clone());
        let e3: Edge<i32> = Edge::new(v1.key().clone(), v3.key().clone());
        let e4: Edge<i32> = Edge::new(v2.key().clone(), v3.key().clone());

        // init
        graph = graph.add_vertex(v1.clone()).unwrap();
        graph = graph.add_vertex(v2.clone()).unwrap();
        graph = graph.add_vertex(v3.clone()).unwrap();
        graph = graph.add_edge(e1.clone()).unwrap();
        graph = graph.add_edge(e2.clone()).unwrap();
        graph = graph.add_edge(e3.clone()).unwrap();
        graph = graph.add_edge(e4.clone()).unwrap();

        let expected_vertices = vec![v1.clone(), v2.clone(), v3.clone()];
        assert_sorted_vec_eq(&expected_vertices, &graph.vertices());
        let expected_edges = vec![e1.clone(), e2.clone(), e3.clone(), e4.clone()];
        assert_sorted_vec_eq(&expected_edges, &graph.edges());

        let predecessors = graph.predecessors();

        // tests all vertices and its corresponding edges
        let expected_edges_v1 = vec![e1.clone(), e3.clone()];
        let expected_edges_v2 = vec![e1.clone(), e2.clone(), e4.clone()];
        let expected_edges_v3 = vec![e3.clone(), e4.clone()];

        assert_sorted_vec_eq(&expected_edges_v1, &predecessors.get(&v1).unwrap());
        assert_sorted_vec_eq(&expected_edges_v2, &predecessors.get(&v2).unwrap());
        assert_sorted_vec_eq(&expected_edges_v3, &predecessors.get(&v3).unwrap());

        // tests all keys and its corresponding edges
        let predecessors = graph.predecessors_as_key_and_edges();

        assert_sorted_vec_eq(&expected_edges_v1, &predecessors.get(v1.key()).unwrap());
        assert_sorted_vec_eq(&expected_edges_v2, &predecessors.get(v2.key()).unwrap());
        assert_sorted_vec_eq(&expected_edges_v3, &predecessors.get(v3.key()).unwrap());
    }

    #[test]
    fn eq() {
        let mut graph1: BasicUndirectedGraph<i32, i32> = BasicUndirectedGraph::new();
        let mut graph2: BasicUndirectedGraph<i32, i32> = BasicUndirectedGraph::new();
        let v1: Vertex<i32, i32> = Vertex::with_value(1, 1);
        let v2: Vertex<i32, i32> = Vertex::with_value(2, 4);
        let v3: Vertex<i32, i32> = Vertex::with_value(3, 9);
        let e1: Edge<i32> = Edge::new(v1.key().clone(), v2.key().clone());
        let e2: Edge<i32> = Edge::new(v2.key().clone(), v2.key().clone());
        let e3: Edge<i32> = Edge::new(v1.key().clone(), v3.key().clone());
        let e4: Edge<i32> = Edge::new(v2.key().clone(), v3.key().clone());

        // init
        graph1 = graph1
            .add_vertex(v1.clone())
            .unwrap()
            .add_vertex(v2.clone())
            .unwrap()
            .add_vertex(v3.clone())
            .unwrap()
            .add_edge(e1.clone())
            .unwrap()
            .add_edge(e2.clone())
            .unwrap()
            .add_edge(e3.clone())
            .unwrap()
            .add_edge(e4.clone())
            .unwrap();

        graph2 = graph2
            .add_vertex(v1.clone())
            .unwrap()
            .add_vertex(v2.clone())
            .unwrap()
            .add_edge(e1.clone())
            .unwrap()
            .add_edge(e2.clone())
            .unwrap();

        assert_eq!(false, graph1.eq(&graph2));
        assert_eq!(true, graph1.ne(&graph2));

        graph2 = graph2.add_vertex(v3.clone()).unwrap();
        assert_eq!(false, graph1.eq(&graph2));
        assert_eq!(true, graph1.ne(&graph2));

        graph2 = graph2
            .add_edge(e3.clone())
            .unwrap()
            .add_edge(e4.clone())
            .unwrap();
        assert_eq!(true, graph1.eq(&graph2));
        assert_eq!(false, graph1.ne(&graph2));
    }
}
