use crate::types::{Key, Value, Vertex};

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

    /// Get the edges of the graph
    fn edges(&self) -> Vec<(Vertex<K, V>, Vertex<K, V>)>;

    /// Add a new vertex then return the graph
    fn add_vertex(&self, vertex: Vertex<K, V>) -> Self;

    /// Remove a vertex by its key then return the given vertex if it exists.
    fn remove_vertex_where_key(&self, key: K) -> Option<Vertex<K, V>>;
}
