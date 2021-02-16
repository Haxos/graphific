use crate::{AnyGraph, Key, Value, Vertex, Weight, WeightedEdge};

/// An interface used for describing any type of network.
pub trait AnyNetwork<K, V, W>: AnyGraph<K, V>
where
    K: Key,
    V: Value,
    W: Weight,
{
    /// Get the weighted edges of the graph.
    fn weighted_edges(&self) -> Vec<WeightedEdge<K, W>>;

    /// Remove a vertex then return the new graph, the deleted vertex and its edges.
    fn remove_vertex(
        &self,
        vertex: &Vertex<K, V>,
    ) -> Option<(Self, Vertex<K, V>, Vec<WeightedEdge<K, W>>)>;

    /// Remove all vertices then return the new graph, the deleted vertices and all the weighted edges.
    fn remove_all_vertices(&self) -> Option<(Self, Vec<Vertex<K, V>>, Vec<WeightedEdge<K, W>>)>;

    /// Add a new weighted edge.
    fn add_weighted_edge(&self, weighted_edge: WeightedEdge<K, W>) -> Option<Self>;

    /// Remove a weighted edge.
    fn remove_weighted_edge(&self) -> Option<(Self, WeightedEdge<K, W>)>;

    /// Remove all the edges then return the new graph and all the deleted edges.
    fn remove_all_edges(&self) -> Option<(Self, Vec<WeightedEdge<K, W>>)>;

    /// Remove all existing edges from or to a given vertex, then return the new graph and the deleted edges.
    fn remove_all_edges_where_vertex(
        &self,
        vertex: &Vertex<K, V>,
    ) -> Option<(Self, Vec<WeightedEdge<K, W>>)>;

    /// Remove all existing edges from or to a given key, then return the new graph and the deleted edges.
    fn remove_all_edges_where_key(&self, key_from: K) -> Option<(Self, Vec<WeightedEdge<K, W>>)>;

    /// Remove all existing edges from a given vertex, then return the new graph and the deleted edges.
    fn remove_all_edges_from_vertex(
        &self,
        vertex: &Vertex<K, V>,
    ) -> Option<(Self, Vec<WeightedEdge<K, W>>)>;

    /// Remove all existing edges from a given key, then return the new graph and the deleted edges.
    fn remove_all_edges_from_key(&self, key_from: K) -> Option<(Self, Vec<WeightedEdge<K, W>>)>;
}
