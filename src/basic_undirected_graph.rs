use crate::algo::Algorithms;
use crate::kinship::Kinship;
use crate::{AnyGraph, Edge, Key, Value, Vertex};
use std::borrow::BorrowMut;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};

/// A basic implementation of an undirected graph.
/// It doesn't allow multiple edges but allow loops.
#[derive(Clone, PartialEq)]
pub struct BasicUndirectedGraph<K, V>
where
    K: Key,
    V: Value,
{
    vertices: HashSet<Vertex<K, V>>,
    edges: HashSet<Edge<K>>,
}

impl<K, V> AnyGraph<K, V> for BasicUndirectedGraph<K, V>
where
    K: Key,
    V: Value,
{
    /// Get the vertices of the graph.
    /// Complexity: O(1*).
    fn vertices(&self) -> Vec<Vertex<K, V>> {
        self.vertices.iter().cloned().collect()
    }

    /// Get the edges of the graph.
    /// Complexity: O(1*).
    fn edges(&self) -> Vec<Edge<K>> {
        self.edges.iter().cloned().collect()
    }

    /// Add a new vertex then return the graph.
    /// Complexity: O(1*).
    fn add_vertex(&self, vertex: Vertex<K, V>) -> Option<Self> {
        let mut new_graph = self.clone();
        return if new_graph.vertices.borrow_mut().insert(vertex) {
            Some(new_graph)
        } else {
            None
        };
    }

    /// Remove a vertex then return the new graph, the deleted vertex and its edges.
    /// Complexity: O(E).
    fn remove_vertex(&self, vertex: &Vertex<K, V>) -> Option<(Self, Vertex<K, V>, Vec<Edge<K>>)> {
        let mut new_graph = self.clone();
        return if let Some(removed_vertex) = self.vertices.get(&vertex) {
            if new_graph.vertices.remove(removed_vertex) {
                if let Some((new_graph, removed_edges)) =
                    new_graph.internal_remove_all_edges_where_vertex(removed_vertex)
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

    /// Remove all vertices then return the new graph, the deleted vertices and all the edges.
    /// Complexity: O(1*).
    fn remove_all_vertices(&self) -> Option<(Self, Vec<Vertex<K, V>>, Vec<Edge<K>>)> {
        let new_graph = BasicUndirectedGraph::new();
        let vertices = self.vertices();
        let edges = self.edges();

        Some((new_graph, vertices, edges))
    }

    /// Remove a vertex by its key then return the new graph, the deleted vertex and its edges.
    /// Complexity: O(E).
    fn remove_vertex_where_key(&self, key: K) -> Option<(Self, Vertex<K, V>, Vec<Edge<K>>)> {
        let vertex: Vertex<K, V> = Vertex::new(key);
        self.remove_vertex(&vertex)
    }

    /// Add a new edge then return the new graph.
    /// Complexity: O(1*).
    fn add_edge(&self, edge: Edge<K>) -> Option<Self> {
        let key_from: Vertex<K, V> = Vertex::new(edge.from().clone());
        let key_to: Vertex<K, V> = Vertex::new(edge.to().clone());
        if !self.vertices.contains(&key_from) || !self.vertices.contains(&key_to) {
            return None;
        }

        let mut new_graph = self.clone();
        return if new_graph.edges.insert(edge) {
            Some(new_graph)
        } else {
            None
        };
    }

    /// Add a new edge between 2 keys then return the new graph.
    /// Complexity: O(1*).
    fn add_edge_between_keys(&self, key_from: K, key_to: K) -> Option<Self> {
        self.add_edge(Edge::new(key_from, key_to))
    }

    /// Remove an existing edge then return the new graph and the deleted edge.
    /// Complexity: O(1*).
    fn remove_edge(&self, edge: &Edge<K>) -> Option<(Self, Edge<K>)> {
        let mut new_graph = self.clone();
        return if let Some(removed_edge) = self.edges.get(edge) {
            if new_graph.edges.remove(removed_edge) {
                Some((new_graph, *removed_edge))
            } else {
                None
            }
        } else {
            None
        };
    }

    /// Remove an existing edge by their keys, then return the new graph and the deleted edge.
    /// Complexity: O(1*).
    fn remove_edge_where_keys(&self, key_from: K, key_to: K) -> Option<(Self, Edge<K>)> {
        let edge = Edge::new(key_from, key_to);
        self.remove_edge(&edge)
    }

    /// Remove all the edges then return the new graph and all the deleted edges.
    /// Complexity: O(1*).
    fn remove_all_edges(&self) -> Option<(Self, Vec<Edge<K>>)> {
        let new_graph = BasicUndirectedGraph {
            vertices: self.vertices.clone(),
            edges: HashSet::new(),
        };
        let edges = self.edges();

        Some((new_graph, edges))
    }

    /// Remove all existing edges from or to a given vertex, then return the new graph and the deleted edges.
    /// Complexity: O(1*).
    fn remove_all_edges_where_vertex(&self, vertex: &Vertex<K, V>) -> Option<(Self, Vec<Edge<K>>)> {
        if !self.vertices.contains(vertex) {
            return None;
        }
        self.internal_remove_all_edges_where_vertex(vertex)
    }

    /// Remove all existing edges from or to a given key, then return the new graph and the deleted edges.
    /// Complexity: O(1*).
    fn remove_all_edges_where_key(&self, key_from: K) -> Option<(Self, Vec<Edge<K>>)> {
        let vertex = Vertex::new(key_from);
        self.remove_all_edges_where_vertex(&vertex)
    }

    /// Remove all existing edges from a given vertex, then return the new graph and the deleted edges.
    /// Complexity: O(1*).
    fn remove_all_edges_from_vertex(&self, vertex: &Vertex<K, V>) -> Option<(Self, Vec<Edge<K>>)> {
        self.remove_all_edges_where_vertex(vertex)
    }

    /// Remove all existing edges from a given key, then return the new graph and the deleted edges.
    /// Complexity: O(1*).
    fn remove_all_edges_from_key(&self, key_from: K) -> Option<(Self, Vec<Edge<K>>)> {
        let vertex = Vertex::new(key_from);
        self.remove_all_edges_from_vertex(&vertex)
    }
}

impl<K, V> Kinship<K, V> for BasicUndirectedGraph<K, V>
where
    K: Key,
    V: Value,
{
    /// Get the successors of each vertex.
    /// Complexity: O(V + E).
    fn successors(&self) -> HashMap<Vertex<K, V>, Vec<Edge<K>>, RandomState> {
        let init_hashmap = self
            .vertices
            .iter()
            .fold(HashMap::new(), |mut acc, vertex| {
                let vec: Vec<Edge<K>> = vec![];
                acc.insert(vertex.clone(), vec);
                acc
            });
        self.edges.iter().fold(init_hashmap, |mut acc, edge| {
            let tmp_from_vertex = Vertex::new(edge.from().clone());
            let tmp_to_vertex = Vertex::new(edge.to().clone());
            if let Some(vector) = acc.get_mut(&tmp_from_vertex) {
                vector.push(edge.clone());
            }
            if edge.from().ne(edge.to()) {
                if let Some(vector) = acc.get_mut(&tmp_to_vertex) {
                    vector.push(edge.clone());
                }
            }
            acc
        })
    }

    /// Get the predecessors of each vertex.
    /// Complexity: O(V + E).
    fn predecessors(&self) -> HashMap<Vertex<K, V>, Vec<Edge<K>>, RandomState> {
        self.successors()
    }
}

impl<K, V> Algorithms<K, V> for BasicUndirectedGraph<K, V>
where
    K: Key,
    V: Value,
{
}

impl<K, V> BasicUndirectedGraph<K, V>
where
    K: Key,
    V: Value,
{
    /// Create a new undirected graph.
    /// Complexity: O(1)
    pub fn new() -> Self {
        BasicUndirectedGraph {
            vertices: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    fn internal_remove_all_edges_where_vertex(
        &self,
        vertex: &Vertex<K, V>,
    ) -> Option<(Self, Vec<Edge<K>>)> {
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

        Some((new_graph, removed_edges))
    }
}
