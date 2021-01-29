use crate::any_graph::AnyGraph;
use crate::types::{Key, Value, Vertex};
use std::collections::HashMap;

pub struct SimpleDirectedGraph<K, V>
where
    K: Key,
    V: Value,
{
    vertices: HashMap<K, Vertex<K, V>>,
}

impl<K, V> AnyGraph<K, V> for SimpleDirectedGraph<K, V>
where
    K: Key,
    V: Value,
{
    fn vertices(&self) -> Vec<Vertex<K, V>> {
        self.vertices.values().cloned().collect()
    }

    fn edges(&self) -> Vec<(Vertex<K, V>, Vertex<K, V>)> {
        unimplemented!()
    }

    fn add_vertex(&self, vertex: Vertex<K, V>) -> Self {
        unimplemented!()
    }

    fn remove_vertex_where_key(&self, key: K) -> Option<Vertex<K, V>> {
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
        }
    }
}
