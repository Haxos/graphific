use crate::types::{Key, Value, Vertex};
use crate::Edge;

/// An interface used to describe any kind of graph.
///
/// # Generic implementations
/// K describe a type of [`Key`] to use.
/// V describe a type of [`Value`] to store.
pub trait AnyGraph<K, V>: Sized + Clone + PartialEq
where
    K: Key,
    V: Value,
{
    /// Get the vertices of the graph.
    fn vertices(&self) -> Vec<Vertex<K, V>>;

    /// Get the edges of the graph.
    fn edges(&self) -> Vec<Edge<K>>;

    /// Add a new vertex then return the graph.
    fn add_vertex(&self, vertex: Vertex<K, V>) -> Option<Self>;

    /// Remove a vertex then return the new graph, the deleted vertex and its edges.
    fn remove_vertex(&self, vertex: &Vertex<K, V>) -> Option<(Self, Vertex<K, V>, Vec<Edge<K>>)>;

    /// Remove all vertices then return the new graph, the deleted vertices and all the edges.
    fn remove_all_vertices(&self) -> Option<(Self, Vec<Vertex<K, V>>, Vec<Edge<K>>)>;

    /// Remove a vertex by its key then return the new graph, the deleted vertex and its edges.
    fn remove_vertex_where_key(&self, key: K) -> Option<(Self, Vertex<K, V>, Vec<Edge<K>>)>;

    /// Add a new edge then return the new graph.
    fn add_edge(&self, edge: Edge<K>) -> Option<Self>;

    /// Add a new edge between 2 keys then return the new graph.
    fn add_edge_between_keys(&self, key_from: K, key_to: K) -> Option<Self>;

    /// Remove an existing edge then return the new graph and the deleted edge.
    fn remove_edge(&self, edge: &Edge<K>) -> Option<(Self, Edge<K>)>;

    /// Remove an existing edge by their keys, then return the new graph and the deleted edge.
    fn remove_edge_where_keys(&self, key_from: K, key_to: K) -> Option<(Self, Edge<K>)>;

    /// Remove all the edges then return the new graph and all the deleted edges.
    fn remove_all_edges(&self) -> Option<(Self, Vec<Edge<K>>)>;

    /// Remove all existing edges from or to a given vertex, then return the new graph and the deleted edges.
    fn remove_all_edges_where_vertex(&self, vertex: &Vertex<K, V>) -> Option<(Self, Vec<Edge<K>>)>;

    /// Remove all existing edges from or to a given key, then return the new graph and the deleted edges.
    fn remove_all_edges_where_key(&self, key_from: K) -> Option<(Self, Vec<Edge<K>>)>;

    /// Remove all existing edges from a given vertex, then return the new graph and the deleted edges.
    fn remove_all_edges_from_vertex(&self, vertex: &Vertex<K, V>) -> Option<(Self, Vec<Edge<K>>)>;

    /// Remove all existing edges from a given key, then return the new graph and the deleted edges.
    fn remove_all_edges_from_key(&self, key_from: K) -> Option<(Self, Vec<Edge<K>>)>;
}
