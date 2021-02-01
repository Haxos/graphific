use crate::any_graph::AnyGraph;
use crate::types::{Key, Value, Vertex};
use crate::Edge;
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashSet;

#[derive(Clone)]
pub struct SimpleDirectedGraph<K, V>
where
    K: Key,
    V: Value,
{
    vertices: HashSet<Vertex<K, V>>,
    edges: HashSet<Edge<K>>,
}

impl<K, V> AnyGraph<K, V> for SimpleDirectedGraph<K, V>
where
    K: Key,
    V: Value,
{
    fn vertices(&self) -> Vec<Vertex<K, V>> {
        self.vertices.iter().cloned().collect()
    }

    fn edges(&self) -> Vec<Edge<K>> {
        self.edges.iter().cloned().collect()
    }

    fn add_vertex(&self, vertex: Vertex<K, V>) -> Option<Box<Self>> {
        let mut new_graph = self.clone();
        return if new_graph.vertices.borrow_mut().insert(vertex) {
            Some(Box::new(new_graph))
        } else {
            None
        };
    }

    fn remove_vertex_where_key(&self, key: K) -> Option<(Box<Self>, Vertex<K, V>, Vec<Edge<K>>)> {
        let mut new_graph = self.clone();
        unimplemented!()
    }

    fn add_edge(&self, edge: Edge<K>) -> Option<Box<Self>> {
        let key_from: Vertex<K, V> = Vertex::new(edge.from().clone());
        let key_to: Vertex<K, V> = Vertex::new(edge.to().clone());
        if !self.vertices.contains(&key_from) || !self.vertices.contains(&key_to) {
            return None;
        }

        let mut new_graph = self.clone();
        return if new_graph.edges.insert(edge) {
            Some(Box::new(new_graph))
        } else {
            None
        };
    }

    fn add_edge_between_keys(&self, key_from: K, key_to: K) -> Option<Box<Self>> {
        unimplemented!()
    }

    fn remove_edge_where_keys(&self, key_from: K, key_to: K) -> Option<(Box<Self>, Edge<K>)> {
        unimplemented!()
    }

    fn remove_all_edges_where_key(&self, key_from: K) -> Option<(Box<Self>, Vec<Edge<K>>)> {
        unimplemented!()
    }
}

impl<K, V> SimpleDirectedGraph<K, V>
where
    K: Key,
    V: Value,
{
    pub fn new() -> Self {
        SimpleDirectedGraph {
            vertices: HashSet::new(),
            edges: HashSet::new(),
        }
    }
}
