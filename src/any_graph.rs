use crate::types::{Key, Value, Vertex};
use crate::Edge;

/// An interface used to describe any kind of graph.
///
/// # Generic implementations
/// K describe a type of [`Key`] to use.
/// V describe a type of [`Value`] to store.
pub trait AnyGraph<K, V>
where
    K: Key,
    V: Value,
{
    /// Get the vertices of the graph.
    fn vertices(&self) -> Vec<Vertex<K, V>>;

    /// Get the edges of the graph.
    fn edges(&self) -> Vec<Edge<K>>;

    /// Add a new vertex then return the graph.
    fn add_vertex(&self, vertex: Vertex<K, V>) -> Option<Box<Self>>;

    /// Remove a vertex by its key then return the given vertex if it exists.
    fn remove_vertex_where_key(&self, key: K) -> Option<(Box<Self>, Vertex<K, V>)>;

    /// Add a new edge then return the new graph.
    fn add_edge(&self, edge: Edge<K>) -> Option<Box<Self>>;

    /// Add a new edge between 2 keys then return the new graph.
    fn add_edge_between_keys(&self, key_from: K, key_to: K) -> Option<Box<Self>>;

    /// Remove an existing edge by their keys, then return the new graph and the deleted edge
    fn remove_edge_where_keys(&self, key_from: K, key_to: K) -> Option<(Box<Self>, Edge<K>)>;

    /// Remove all existing edges from a given key, then return the new graph and the deleted edges.
    fn remove_all_edges_where_key(&self, key_from: K) -> Option<(Box<Self>, Vec<Edge<K>>)>;
}
