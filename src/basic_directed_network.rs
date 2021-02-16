use crate::{AnyGraph, AnyNetwork, Edge, Key, Value, Vertex, Weight, WeightedEdge};
use std::collections::HashSet;

/// A basic implementation of a directed network.
/// It doesn't allow multiple edges but allow loops.
#[derive(Clone, PartialEq)]
pub struct BasicDirectedNetwork<K, V, W>
where
    K: Key,
    V: Value,
    W: Weight,
{
    vertices: HashSet<Vertex<K, V>>,
    edges: HashSet<WeightedEdge<K, W>>,
}

impl<K, V, W> AnyGraph<K, V> for BasicDirectedNetwork<K, V, W>
where
    K: Key,
    V: Value,
    W: Weight,
{
    fn vertices(&self) -> Vec<Vertex<K, V>> {
        unimplemented!()
    }

    fn edges(&self) -> Vec<Edge<K>> {
        unimplemented!()
    }

    fn add_vertex(&self, vertex: Vertex<K, V>) -> Option<Self> {
        unimplemented!()
    }

    fn remove_vertex(&self, vertex: &Vertex<K, V>) -> Option<(Self, Vertex<K, V>, Vec<Edge<K>>)> {
        unimplemented!()
    }

    fn remove_all_vertices(&self) -> Option<(Self, Vec<Vertex<K, V>>, Vec<Edge<K>>)> {
        unimplemented!()
    }

    fn remove_vertex_where_key(&self, key: K) -> Option<(Self, Vertex<K, V>, Vec<Edge<K>>)> {
        unimplemented!()
    }

    fn add_edge(&self, edge: Edge<K>) -> Option<Self> {
        unimplemented!()
    }

    fn add_edge_between_keys(&self, key_from: K, key_to: K) -> Option<Self> {
        unimplemented!()
    }

    fn remove_edge(&self, edge: &Edge<K>) -> Option<(Self, Edge<K>)> {
        unimplemented!()
    }

    fn remove_edge_where_keys(&self, key_from: K, key_to: K) -> Option<(Self, Edge<K>)> {
        unimplemented!()
    }

    fn remove_all_edges(&self) -> Option<(Self, Vec<Edge<K>>)> {
        unimplemented!()
    }

    fn remove_all_edges_where_vertex(&self, vertex: &Vertex<K, V>) -> Option<(Self, Vec<Edge<K>>)> {
        unimplemented!()
    }

    fn remove_all_edges_where_key(&self, key_from: K) -> Option<(Self, Vec<Edge<K>>)> {
        unimplemented!()
    }

    fn remove_all_edges_from_vertex(&self, vertex: &Vertex<K, V>) -> Option<(Self, Vec<Edge<K>>)> {
        unimplemented!()
    }

    fn remove_all_edges_from_key(&self, key_from: K) -> Option<(Self, Vec<Edge<K>>)> {
        unimplemented!()
    }
}

impl<K, V, W> AnyNetwork<K, V, W> for BasicDirectedNetwork<K, V, W>
where
    K: Key,
    V: Value,
    W: Weight,
{
    fn weighted_edges(&self) -> Vec<WeightedEdge<K, W>> {
        unimplemented!()
    }

    fn remove_vertex(
        &self,
        vertex: &Vertex<K, V>,
    ) -> Option<(Self, Vertex<K, V>, Vec<WeightedEdge<K, W>>)> {
        unimplemented!()
    }

    fn remove_all_vertices(&self) -> Option<(Self, Vec<Vertex<K, V>>, Vec<WeightedEdge<K, W>>)> {
        unimplemented!()
    }

    fn add_weighted_edge(&self, weighted_edge: WeightedEdge<K, W>) -> Option<Self> {
        unimplemented!()
    }

    fn remove_weighted_edge(&self) -> Option<(Self, WeightedEdge<K, W>)> {
        unimplemented!()
    }

    fn remove_all_edges(&self) -> Option<(Self, Vec<WeightedEdge<K, W>>)> {
        unimplemented!()
    }

    fn remove_all_edges_where_vertex(
        &self,
        vertex: &Vertex<K, V>,
    ) -> Option<(Self, Vec<WeightedEdge<K, W>>)> {
        unimplemented!()
    }

    fn remove_all_edges_where_key(&self, key_from: K) -> Option<(Self, Vec<WeightedEdge<K, W>>)> {
        unimplemented!()
    }

    fn remove_all_edges_from_vertex(
        &self,
        vertex: &Vertex<K, V>,
    ) -> Option<(Self, Vec<WeightedEdge<K, W>>)> {
        unimplemented!()
    }

    fn remove_all_edges_from_key(&self, key_from: K) -> Option<(Self, Vec<WeightedEdge<K, W>>)> {
        unimplemented!()
    }
}
