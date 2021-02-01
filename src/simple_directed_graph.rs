use crate::any_graph::AnyGraph;
use crate::types::{Key, Value, Vertex};
use crate::Edge;
use std::collections::HashMap;

pub struct SimpleDirectedGraph<K, V>
where
    K: Key,
    V: Value,
{
    vertices: HashMap<K, Vertex<K, V>>,
    edges: HashMap<K, HashMap<K, Edge<K>>>,
}

impl<K, V> AnyGraph<K, V> for SimpleDirectedGraph<K, V>
where
    K: Key,
    V: Value,
{
    fn vertices(&self) -> Vec<Vertex<K, V>> {
        self.vertices.values().cloned().collect()
    }

    fn edges(&self) -> Vec<Edge<K>> {
        self.edges
            .values()
            .flat_map(|ref_map| ref_map.values().cloned())
            .collect()
    }

    fn add_vertex(&self, vertex: Vertex<K, V>) -> Option<Box<Self>> {
        unimplemented!()
    }

    fn remove_vertex_where_key(&self, key: K) -> Option<(Box<Self>, Vertex<K, V>)> {
        unimplemented!()
    }

    fn add_edge(&self, edge: Edge<K>) -> Option<Box<Self>> {
        unimplemented!()
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
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }
}
