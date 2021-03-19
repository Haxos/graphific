use crate::{Algorithms, AnyGraph, Edge, GraphError, Key, Kinship, Value, Vertex, Weight};
use std::borrow::BorrowMut;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};

/// A basic implementation of an undirected graph.
/// It doesn't allow multiple edges but allow loops.
#[derive(Clone, PartialEq)]
pub struct BasicUndirectedGraph<K, V, W = i8>
where
    K: Key,
    V: Value,
    W: Weight,
{
    vertices: HashSet<Vertex<K, V>>,
    edges: HashSet<Edge<K, W>>,
}

impl<K, V, W> AnyGraph<K, V, W> for BasicUndirectedGraph<K, V, W>
where
    K: Key,
    V: Value,
    W: Weight,
{
    /// Get the vertices of the graph.
    /// Complexity: O(V).
    fn vertices(&self) -> Vec<Vertex<K, V>> {
        self.vertices.iter().cloned().collect()
    }

    /// Get the edges of the graph.
    /// Complexity: O(E).
    fn edges(&self) -> Vec<Edge<K, W>> {
        self.edges.iter().cloned().collect()
    }

    /// Add a new vertex then return the graph.
    /// Complexity: O(1*).
    fn add_vertex(&self, vertex: Vertex<K, V>) -> Result<Self, GraphError> {
        let mut new_graph = self.clone();
        return if new_graph.vertices.borrow_mut().insert(vertex) {
            Ok(new_graph)
        } else {
            Err(GraphError::VertexAlreadyExists)
        };
    }

    /// Remove a vertex then return the new graph, the deleted vertex and its edges.
    /// Complexity: O(E).
    fn remove_vertex(
        &self,
        vertex: &Vertex<K, V>,
    ) -> Result<(Self, Vertex<K, V>, Vec<Edge<K, W>>), GraphError> {
        let mut new_graph = self.clone();
        return if let Some(removed_vertex) = self.vertices.get(&vertex) {
            if new_graph.vertices.remove(removed_vertex) {
                if let Ok((new_graph, removed_edges)) =
                    new_graph.internal_remove_all_edges_where_vertex(removed_vertex)
                {
                    Ok((new_graph, *removed_vertex, removed_edges))
                } else {
                    Err(GraphError::Unknown)
                }
            } else {
                Err(GraphError::Unknown)
            }
        } else {
            Err(GraphError::VertexDoesNotExists)
        };
    }

    /// Remove all vertices then return the new graph, the deleted vertices and all the edges.
    /// Complexity: O(1*).
    fn remove_all_vertices(
        &self,
    ) -> Result<(Self, Vec<Vertex<K, V>>, Vec<Edge<K, W>>), GraphError> {
        let new_graph = BasicUndirectedGraph::new();
        let vertices = self.vertices();
        let edges = self.edges();

        Ok((new_graph, vertices, edges))
    }

    /// Add a new edge then return the new graph.
    /// Complexity: O(1*).
    fn add_edge(&self, edge: Edge<K, W>) -> Result<Self, GraphError> {
        let vertex_from: Vertex<K, V> = Vertex::new(edge.from().clone());
        let vertex_to: Vertex<K, V> = Vertex::new(edge.to().clone());
        if !self.vertices.contains(&vertex_from) || !self.vertices.contains(&vertex_to) {
            return Err(GraphError::VertexDoesNotExists);
        }

        let other_edge = Edge::new(edge.to().clone(), edge.from().clone());
        if self.edges.contains(&other_edge) {
            return Err(GraphError::EdgeAlreadyExists);
        }

        let mut new_graph = self.clone();
        return if new_graph.edges.insert(edge) {
            Ok(new_graph)
        } else {
            Err(GraphError::EdgeAlreadyExists)
        };
    }

    /// Remove an existing edge then return the new graph and the deleted edge.
    /// Complexity: O(1*).
    fn remove_edge(&self, edge: &Edge<K, W>) -> Result<(Self, Edge<K, W>), GraphError> {
        let mut new_graph = self.clone();

        let other_edge = Edge::new(edge.to().clone(), edge.from().clone());
        let mut edge_to_remove = edge;
        if self.edges.contains(&other_edge) {
            edge_to_remove = &other_edge;
        }

        return if let Some(removed_edge) = self.edges.get(edge_to_remove) {
            if new_graph.edges.remove(removed_edge) {
                Ok((new_graph, *removed_edge))
            } else {
                Err(GraphError::Unknown)
            }
        } else {
            Err(GraphError::EdgeDoesNotExists)
        };
    }

    /// Remove all the edges then return the new graph and all the deleted edges.
    /// Complexity: O(1*).
    fn remove_all_edges(&self) -> Result<(Self, Vec<Edge<K, W>>), GraphError> {
        let new_graph = BasicUndirectedGraph {
            vertices: self.vertices.clone(),
            edges: HashSet::new(),
        };
        let edges = self.edges();

        Ok((new_graph, edges))
    }

    /// Remove all existing edges from or to a given vertex, then return the new graph and the deleted edges.
    /// Complexity: O(E).
    fn remove_all_edges_where_vertex(
        &self,
        vertex: &Vertex<K, V>,
    ) -> Result<(Self, Vec<Edge<K, W>>), GraphError> {
        if !self.vertices.contains(vertex) {
            return Err(GraphError::VertexDoesNotExists);
        }
        self.internal_remove_all_edges_where_vertex(vertex)
    }

    /// Remove all existing edges from a given vertex, then return the new graph and the deleted edges.
    /// Complexity: O(E).
    fn remove_all_edges_from_vertex(
        &self,
        vertex: &Vertex<K, V>,
    ) -> Result<(Self, Vec<Edge<K, W>>), GraphError> {
        self.remove_all_edges_where_vertex(vertex)
    }
}

impl<K, V, W> Kinship<K, V, W> for BasicUndirectedGraph<K, V, W>
where
    K: Key,
    V: Value,
    W: Weight,
{
    /// Get the successors of each vertex.
    /// Complexity: O(V + E).
    fn successors(&self) -> HashMap<Vertex<K, V>, Vec<Edge<K, W>>, RandomState> {
        let init_hashmap = self
            .vertices
            .iter()
            .fold(HashMap::new(), |mut acc, vertex| {
                let vec: Vec<Edge<K, W>> = vec![];
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
    fn predecessors(&self) -> HashMap<Vertex<K, V>, Vec<Edge<K, W>>, RandomState> {
        self.successors()
    }
}

impl<K, V, W> Algorithms<K, V, W> for BasicUndirectedGraph<K, V, W>
where
    K: Key,
    V: Value,
    W: Weight,
{
}

impl<K, V, W> BasicUndirectedGraph<K, V, W>
where
    K: Key,
    V: Value,
    W: Weight,
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
    ) -> Result<(Self, Vec<Edge<K, W>>), GraphError> {
        let mut new_graph = self.clone();
        let removed_edges: Vec<Edge<K, W>> = new_graph
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

        Ok((new_graph, removed_edges))
    }
}
