use crate::{AnyGraph, Edge, Key, Value, Vertex};

use std::collections::HashMap;

/// An interface for getting the successors and predecessors of each [`Vertex`].
pub trait Kinship<K, V>: AnyGraph<K, V>
where
    K: Key,
    V: Value,
{
    /// Get the successors of each vertex.
    fn successors(&self) -> HashMap<Vertex<K, V>, Vec<Edge<K>>>;

    /// Get the predecessors of each vertex.
    fn predecessors(&self) -> HashMap<Vertex<K, V>, Vec<Edge<K>>>;

    /// Get the successors of each vertex where the key is a [`Key`].
    fn successors_as_key_and_edges(&self) -> HashMap<K, Vec<Edge<K>>> {
        self.successors()
            .iter()
            .fold(HashMap::new(), |mut acc, (vertex, edges)| {
                acc.insert(vertex.key().clone(), edges.iter().cloned().collect());
                acc
            })
    }

    /// Get the predecessors of each vertex where the key is a [`Key`].
    fn predecessors_as_key_and_edges(&self) -> HashMap<K, Vec<Edge<K>>> {
        self.predecessors()
            .iter()
            .fold(HashMap::new(), |mut acc, (vertex, edges)| {
                acc.insert(vertex.key().clone(), edges.iter().cloned().collect());
                acc
            })
    }

    /// Get the map of key and vertex.
    fn key_vertex_map(&self) -> HashMap<K, Vertex<K, V>> {
        self.vertices()
            .iter()
            .fold(HashMap::new(), |mut acc, vertex| {
                acc.insert(vertex.key().clone(), vertex.clone());
                acc
            })
    }
}
