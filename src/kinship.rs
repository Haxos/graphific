use crate::{Edge, Key, Value, Vertex};

use std::collections::HashMap;

/// An interface for getting the successors and predecessors of each [`Vertex`]
pub trait Kinship<K, V>
where
    K: Key,
    V: Value,
{
    /// Get the successors of each vertex.
    fn successors(&self) -> HashMap<Vertex<K, V>, Vec<Edge<K>>>;

    /// Get the predecessors of each vertex.
    fn predecessors(&self) -> HashMap<Vertex<K, V>, Vec<Edge<K>>>;
}
