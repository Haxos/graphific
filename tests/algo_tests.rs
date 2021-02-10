#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod utils;

#[cfg(test)]
mod algo_tests {
    use crate::utils::assert_sorted_vec_eq;
    use graphific::{Algorithms, AnyGraph, BasicDirectedGraph, BasicUndirectedGraph, Edge, Vertex};

    fn init_bdg() -> BasicDirectedGraph<i32, i32> {
        let bdg: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        bdg.add_vertex(Vertex::new(1))
            .unwrap()
            .add_vertex(Vertex::new(2))
            .unwrap()
            .add_vertex(Vertex::new(3))
            .unwrap()
            .add_vertex(Vertex::new(4))
            .unwrap()
            .add_edge(Edge::new(1, 2))
            .unwrap()
            .add_edge(Edge::new(1, 3))
            .unwrap()
            .add_edge(Edge::new(2, 3))
            .unwrap()
            .add_edge(Edge::new(3, 4))
            .unwrap()
            .add_edge(Edge::new(4, 1))
            .unwrap()
    }

    fn init_bug() -> BasicUndirectedGraph<i32, i32> {
        let bdg: BasicUndirectedGraph<i32, i32> = BasicUndirectedGraph::new();
        bdg.add_vertex(Vertex::new(1))
            .unwrap()
            .add_vertex(Vertex::new(2))
            .unwrap()
            .add_vertex(Vertex::new(3))
            .unwrap()
            .add_vertex(Vertex::new(4))
            .unwrap()
            .add_edge(Edge::new(1, 2))
            .unwrap()
            .add_edge(Edge::new(1, 3))
            .unwrap()
            .add_edge(Edge::new(2, 3))
            .unwrap()
            .add_edge(Edge::new(3, 4))
            .unwrap()
            .add_edge(Edge::new(4, 1))
            .unwrap()
    }

    #[test]
    fn bfs() {
        let bdg: BasicDirectedGraph<i32, i32> = init_bdg();
        let bug: BasicUndirectedGraph<i32, i32> = init_bug();

        // test bfs_with_starting_vertex and bfs
        let expected_dg = bdg
            .bfs_with_starting_vertex(bdg.vertices().first().unwrap())
            .unwrap();
        let result_dg = bdg.bfs().unwrap();
        assert_eq!(true, result_dg.eq(&expected_dg));

        let expected_ug = bug
            .bfs_with_starting_vertex(bug.vertices().first().unwrap())
            .unwrap();
        let result_ug = bug.bfs().unwrap();
        assert_eq!(true, result_ug.eq(&expected_ug));

        // test if empty
        let empty_dg: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        let empty_ug: BasicUndirectedGraph<i32, i32> = BasicUndirectedGraph::new();

        let should_be_none = empty_dg.bfs();
        assert_eq!(true, should_be_none.is_none());
        let should_be_none = empty_ug.bfs();
        assert_eq!(true, should_be_none.is_none());

        // test bfs_with_starting_vertex and bfs
        let start_vertex: Vertex<i32, i32> = Vertex::new(1);
        let expected_dg: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new()
            .add_vertex(Vertex::new(1))
            .unwrap()
            .add_vertex(Vertex::new(2))
            .unwrap()
            .add_vertex(Vertex::new(3))
            .unwrap()
            .add_vertex(Vertex::new(4))
            .unwrap()
            .add_edge(Edge::new(1, 2))
            .unwrap()
            .add_edge(Edge::new(1, 3))
            .unwrap()
            .add_edge(Edge::new(3, 4))
            .unwrap();

        let result_dg = bdg.bfs_with_starting_vertex(&start_vertex).unwrap();
        assert_sorted_vec_eq(&expected_dg.edges(), &result_dg.edges());
        assert_eq!(true, result_dg.eq(&expected_dg));

        let expected_ug: BasicUndirectedGraph<i32, i32> = BasicUndirectedGraph::new()
            .add_vertex(Vertex::new(1))
            .unwrap()
            .add_vertex(Vertex::new(2))
            .unwrap()
            .add_vertex(Vertex::new(3))
            .unwrap()
            .add_vertex(Vertex::new(4))
            .unwrap()
            .add_edge(Edge::new(1, 2))
            .unwrap()
            .add_edge(Edge::new(1, 3))
            .unwrap()
            .add_edge(Edge::new(3, 4))
            .unwrap();
        let result_ug = bug.bfs_with_starting_vertex(&start_vertex).unwrap();
        assert_eq!(true, result_ug.eq(&expected_ug));
    }

    #[test]
    fn dfs() {
        let bdg: BasicDirectedGraph<i32, i32> = init_bdg();
        let bug: BasicUndirectedGraph<i32, i32> = init_bug();

        // test dfs_with_starting_vertex and dfs
        let expected_dg = bdg
            .dfs_with_starting_vertex(bdg.vertices().first().unwrap())
            .unwrap();
        let result_dg = bdg.dfs().unwrap();
        assert_eq!(true, result_dg.eq(&expected_dg));

        let expected_ug = bug
            .dfs_with_starting_vertex(bug.vertices().first().unwrap())
            .unwrap();
        let result_ug = bug.dfs().unwrap();
        assert_eq!(true, result_ug.eq(&expected_ug));

        // test if empty
        let empty_dg: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new();
        let empty_ug: BasicUndirectedGraph<i32, i32> = BasicUndirectedGraph::new();

        let should_be_none = empty_dg.dfs();
        assert_eq!(true, should_be_none.is_none());
        let should_be_none = empty_ug.dfs();
        assert_eq!(true, should_be_none.is_none());

        // test dfs_with_starting_vertex and dfs
        let start_vertex: Vertex<i32, i32> = Vertex::new(1);
        let expected_dg: BasicDirectedGraph<i32, i32> = BasicDirectedGraph::new()
            .add_vertex(Vertex::new(1))
            .unwrap()
            .add_vertex(Vertex::new(2))
            .unwrap()
            .add_vertex(Vertex::new(3))
            .unwrap()
            .add_vertex(Vertex::new(4))
            .unwrap()
            .add_edge(Edge::new(1, 2))
            .unwrap()
            .add_edge(Edge::new(1, 3))
            .unwrap()
            .add_edge(Edge::new(3, 4))
            .unwrap();

        let result_dg = bdg.dfs_with_starting_vertex(&start_vertex).unwrap();
        assert_sorted_vec_eq(&expected_dg.edges(), &result_dg.edges());
        assert_eq!(true, result_dg.eq(&expected_dg));

        let expected_ug: BasicUndirectedGraph<i32, i32> = BasicUndirectedGraph::new()
            .add_vertex(Vertex::new(1))
            .unwrap()
            .add_vertex(Vertex::new(2))
            .unwrap()
            .add_vertex(Vertex::new(3))
            .unwrap()
            .add_vertex(Vertex::new(4))
            .unwrap()
            .add_edge(Edge::new(1, 2))
            .unwrap()
            .add_edge(Edge::new(1, 3))
            .unwrap()
            .add_edge(Edge::new(3, 4))
            .unwrap();
        let result_ug = bug.dfs_with_starting_vertex(&start_vertex).unwrap();
        assert_eq!(true, result_ug.eq(&expected_ug));
    }
}
