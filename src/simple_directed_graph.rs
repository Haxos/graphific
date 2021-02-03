use crate::any_graph::AnyGraph;
use crate::kinship::Kinship;
use crate::types::{Key, Value, Vertex};
use crate::Edge;
use std::borrow::BorrowMut;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};

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

    fn remove_vertex(
        &self,
        vertex: &Vertex<K, V>,
    ) -> Option<(Box<Self>, Vertex<K, V>, Vec<Edge<K>>)> {
        let mut new_graph = self.clone();
        return if let Some(removed_vertex) = self.vertices.get(&vertex) {
            if new_graph.vertices.remove(removed_vertex) {
                if let Some((new_graph, removed_edges)) =
                    new_graph.remove_all_edges_where_vertex(removed_vertex)
                {
                    Some((new_graph, *removed_vertex, removed_edges))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
    }

    fn remove_vertex_where_key(&self, key: K) -> Option<(Box<Self>, Vertex<K, V>, Vec<Edge<K>>)> {
        let vertex: Vertex<K, V> = Vertex::new(key);
        self.remove_vertex(&vertex)
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
        self.add_edge(Edge::new(key_from, key_to))
    }

    fn remove_edge(&self, edge: &Edge<K>) -> Option<(Box<Self>, Edge<K>)> {
        let mut new_graph = self.clone();
        return if let Some(removed_edge) = self.edges.get(edge) {
            if new_graph.edges.remove(removed_edge) {
                Some((Box::new(new_graph), *removed_edge))
            } else {
                None
            }
        } else {
            None
        };
    }

    fn remove_edge_where_keys(&self, key_from: K, key_to: K) -> Option<(Box<Self>, Edge<K>)> {
        let edge = Edge::new(key_from, key_to);
        self.remove_edge(&edge)
    }

    fn remove_all_edges_where_vertex(
        &self,
        vertex: &Vertex<K, V>,
    ) -> Option<(Box<Self>, Vec<Edge<K>>)> {
        let mut new_graph = self.clone();
        let removed_edges: Vec<Edge<K>> = new_graph
            .edges
            .iter()
            .cloned()
            .filter(|edge| edge.from().eq(vertex.key()) || edge.to().eq(vertex.key()))
            .collect();
        new_graph.edges = new_graph
            .edges
            .into_iter()
            .filter(|edge| !(edge.from().eq(vertex.key()) || edge.to().eq(vertex.key())))
            .collect();

        Some((Box::new(new_graph), removed_edges))
    }

    fn remove_all_edges_where_key(&self, key_from: K) -> Option<(Box<Self>, Vec<Edge<K>>)> {
        let vertex = Vertex::new(key_from);
        self.remove_all_edges_where_vertex(&vertex)
    }
}

impl<K, V> Kinship<K, V> for SimpleDirectedGraph<K, V>
where
    K: Key,
    V: Value,
{
    fn successors(&self) -> HashMap<Vertex<K, V>, Edge<K>, RandomState> {
        unimplemented!()
    }

    fn predecessors(&self) -> HashMap<Vertex<K, V>, Edge<K>, RandomState> {
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
