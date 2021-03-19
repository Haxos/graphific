use crate::{AnyGraph, Edge, Key, Kinship, Value, Vertex, Weight};
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};

/// An interface describing all the algorithms that can be used on any kind of graphs.
pub trait Algorithms<K, V, W>: AnyGraph<K, V, W> + Kinship<K, V, W>
where
    K: Key,
    V: Value,
    W: Weight,
{
    /// Execute a Broad Search First the return the discovered graph.
    /// There is no order in which the edges are treated.
    fn simple_bfs(&self) -> Option<Self> {
        return if let Some(first) = self.vertices().first() {
            self.bfs(first, |a, b| a.partial_cmp(b).unwrap())
        } else {
            None
        };
    }

    /// Execute a Broad Search First with a starting vertex the return the discovered graph.
    /// There is no order in which the edges are treated.
    fn bfs(
        &self,
        starting_vertex: &Vertex<K, V>,
        sorting_edges_fn: fn(&Edge<K, W>, &Edge<K, W>) -> Ordering,
    ) -> Option<Self> {
        if !self.vertices().contains(starting_vertex) {
            return None;
        }
        let cloned_graph = self.clone();
        let mut queue: VecDeque<K> = VecDeque::new();
        queue.push_back(starting_vertex.key().clone());

        return if let Ok((mut new_graph, _)) = cloned_graph.remove_all_edges() {
            let successors = cloned_graph.successors_as_key_and_edges();
            let mut flagged: HashSet<K> = HashSet::new();

            flagged.insert(starting_vertex.key().clone());

            while !queue.is_empty() {
                let current = queue.pop_front().unwrap();
                let mut neighbours: Vec<Edge<K, W>> = successors
                    .get(&current)
                    .unwrap()
                    .into_iter()
                    .cloned()
                    .collect();

                neighbours.sort_by(sorting_edges_fn);

                for neighbour in neighbours {
                    if !flagged.contains(neighbour.to()) {
                        new_graph = new_graph.add_edge(neighbour).unwrap();
                        flagged.insert(neighbour.to().clone());
                        queue.push_back(neighbour.to().clone());
                    }
                }
            }
            Some(new_graph)
        } else {
            return None;
        };
    }

    /// Execute a Deep Search First the return the discovered graph.
    /// There is no order in which the edges are treated.
    fn simple_dfs(&self) -> Option<Self> {
        return if let Some(first) = self.vertices().first() {
            self.dfs(first, |a, b| a.partial_cmp(b).unwrap())
        } else {
            None
        };
    }

    /// Execute a Deep Search First with a starting vertex the return the discovered graph.
    /// There is no order in which the edges are treated.
    fn dfs(
        &self,
        starting_vertex: &Vertex<K, V>,
        sorting_edges_fn: fn(&Edge<K, W>, &Edge<K, W>) -> Ordering,
    ) -> Option<Self> {
        if !self.vertices().contains(starting_vertex) {
            return None;
        }
        let cloned_graph = self.clone();
        let mut stack: VecDeque<K> = VecDeque::new();
        stack.push_back(starting_vertex.key().clone());

        return if let Ok((mut new_graph, _)) = cloned_graph.remove_all_edges() {
            let successors = cloned_graph.successors_as_key_and_edges();
            let mut flagged: HashSet<K> = HashSet::new();

            flagged.insert(starting_vertex.key().clone());

            while !stack.is_empty() {
                let current = stack.pop_back().unwrap();
                let mut neighbours: Vec<Edge<K, W>> = successors
                    .get(&current)
                    .unwrap()
                    .into_iter()
                    .cloned()
                    .collect();

                neighbours.sort_by(sorting_edges_fn);

                for neighbour in neighbours {
                    if !flagged.contains(neighbour.to()) {
                        new_graph = new_graph.add_edge(neighbour).unwrap();
                        flagged.insert(neighbour.to().clone());
                        stack.push_back(neighbour.to().clone());
                    }
                }
            }
            Some(new_graph)
        } else {
            return None;
        };
    }
}
