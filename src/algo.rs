use crate::{AnyGraph, Key, Kinship, Value, Vertex};
use std::collections::{HashSet, VecDeque};

/// An interface describing all the algorithms that can be used on any kind of graphs.
pub trait Algorithms<K, V>: AnyGraph<K, V> + Kinship<K, V>
where
    K: Key,
    V: Value,
{
    /// Execute a Broad Search First the return the discovered graph.
    /// There is no order in which the edges are treated.
    fn bfs(&self) -> Option<Self> {
        return if self.vertices().is_empty() {
            None
        } else {
            let first = self.vertices().first().unwrap().clone();
            self.bfs_with_starting_vertex(&first)
        };
    }

    /// Execute a Broad Search First with a starting vertex the return the discovered graph.
    /// There is no order in which the edges are treated.
    fn bfs_with_starting_vertex(&self, starting_vertex: &Vertex<K, V>) -> Option<Self> {
        if !self.vertices().contains(starting_vertex) {
            return None;
        }
        let cloned_graph = self.clone();
        let mut queue: VecDeque<K> = VecDeque::new();
        queue.push_back(starting_vertex.key().clone());

        return if let Some((mut new_graph, _)) = cloned_graph.remove_all_edges() {
            let successors = cloned_graph.successors_as_key_and_edges();
            let mut flagged: HashSet<K> = HashSet::new();

            flagged.insert(starting_vertex.key().clone());

            while !queue.is_empty() {
                let current = queue.pop_front().unwrap();
                let neighbours = successors.get(&current).unwrap();

                for neighbour in neighbours {
                    if !flagged.contains(neighbour.to()) {
                        new_graph = new_graph.add_edge(*neighbour).unwrap();
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
    fn dfs(&self) -> Option<Self> {
        return if self.vertices().is_empty() {
            None
        } else {
            let first = self.vertices().first().unwrap().clone();
            self.dfs_with_starting_vertex(&first)
        };
    }

    /// Execute a Deep Search First with a starting vertex the return the discovered graph.
    /// There is no order in which the edges are treated.
    fn dfs_with_starting_vertex(&self, starting_vertex: &Vertex<K, V>) -> Option<Self> {
        if !self.vertices().contains(starting_vertex) {
            return None;
        }
        let cloned_graph = self.clone();
        let mut stack: VecDeque<K> = VecDeque::new();
        stack.push_back(starting_vertex.key().clone());

        return if let Some((mut new_graph, _)) = cloned_graph.remove_all_edges() {
            let successors = cloned_graph.successors_as_key_and_edges();
            let mut flagged: HashSet<K> = HashSet::new();

            flagged.insert(starting_vertex.key().clone());

            while !stack.is_empty() {
                let current = stack.pop_back().unwrap();
                let neighbours = successors.get(&current).unwrap();

                for neighbour in neighbours {
                    if !flagged.contains(neighbour.to()) {
                        new_graph = new_graph.add_edge(*neighbour).unwrap();
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
