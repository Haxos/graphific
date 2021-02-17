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
        self.vertices.iter().cloned().collect()
    }

    fn edges(&self) -> Vec<Edge<K>> {
        self.edges.iter().map(|e| e.to_edge()).collect()
    }

    fn add_vertex(&self, vertex: Vertex<K, V>) -> Option<Self> {
        let mut new_graph = self.clone();
        return if new_graph.vertices.insert(vertex) {
            Some(new_graph)
        } else {
            None
        };
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
        let vertex_from: Vertex<K, V> = Vertex::new(edge.from().clone());
        let vertex_to: Vertex<K, V> = Vertex::new(edge.to().clone());
        if !self.vertices.contains(&vertex_from) || !self.vertices.contains(&vertex_to) {
            return None;
        }

        let mut new_graph = self.clone();
        let weighted_edge = WeightedEdge::new(edge.from().clone(), edge.to().clone());
        return if new_graph.edges.insert(weighted_edge) {
            Some(new_graph)
        } else {
            None
        };
    }

    fn remove_edge(&self, edge: &Edge<K>) -> Option<(Self, Edge<K>)> {
        unimplemented!()
    }

    fn remove_all_edges(&self) -> Option<(Self, Vec<Edge<K>>)> {
        unimplemented!()
    }

    fn remove_all_edges_where_vertex(&self, vertex: &Vertex<K, V>) -> Option<(Self, Vec<Edge<K>>)> {
        unimplemented!()
    }

    fn remove_all_edges_from_vertex(&self, vertex: &Vertex<K, V>) -> Option<(Self, Vec<Edge<K>>)> {
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
        self.edges.iter().cloned().collect()
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
        let vertex_from: Vertex<K, V> = Vertex::new(weighted_edge.from().clone());
        let vertex_to: Vertex<K, V> = Vertex::new(weighted_edge.to().clone());
        if !self.vertices.contains(&vertex_from) || !self.vertices.contains(&vertex_to) {
            return None;
        }

        let mut new_graph = self.clone();
        return if new_graph.edges.insert(weighted_edge) {
            Some(new_graph)
        } else {
            None
        };
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
