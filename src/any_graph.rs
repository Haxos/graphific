use crate::types::{Key, Value, Vertex};
use crate::{Edge, Weight};
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum GraphError {
    Unknown,
    VertexAlreadyExists,
    EdgeAlreadyExists,
    VertexDoesNotExists,
    EdgeDoesNotExists,
    LoopCreated,
    CycleCreated,
}

impl std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphError::VertexAlreadyExists => write!(f, "the vertex already exists in the graph"),
            GraphError::VertexDoesNotExists => write!(f, "the vertex doesn't exists in the graph"),
            GraphError::EdgeAlreadyExists => write!(f, "the edge already exists in the graph"),
            GraphError::EdgeDoesNotExists => write!(f, "the edge doesn't exists in the graph"),
            GraphError::LoopCreated => write!(f, "a loop is created"),
            GraphError::CycleCreated => write!(f, "a cycle is created"),
            GraphError::Unknown => write!(f, "an unknown error has happened"),
        }
    }
}

impl Error for GraphError {}

/// An interface used for describing any kind of graph.
///
/// # Generic implementations
/// K describe a type of [`Key`] to use.
/// V describe a type of [`Value`] to store.
pub trait AnyGraph<K, V, W>: Sized + Clone + PartialEq
where
    K: Key,
    V: Value,
    W: Weight,
{
    /// Get the vertices of the graph.
    fn vertices(&self) -> Vec<Vertex<K, V>>;

    /// Get the edges of the graph.
    fn edges(&self) -> Vec<Edge<K, W>>;

    /// Add a new vertex then return the graph.
    fn add_vertex(&self, vertex: Vertex<K, V>) -> Result<Self, GraphError>;

    /// Remove a vertex then return the new graph, the deleted vertex and its edges.
    fn remove_vertex(
        &self,
        vertex: &Vertex<K, V>,
    ) -> Result<(Self, Vertex<K, V>, Vec<Edge<K, W>>), GraphError>;

    /// Remove a vertex by its key then return the new graph, the deleted vertex and its edges.
    fn remove_vertex_where_key(
        &self,
        key: K,
    ) -> Result<(Self, Vertex<K, V>, Vec<Edge<K, W>>), GraphError> {
        self.remove_vertex(&Vertex::new(key))
    }

    /// Remove all vertices then return the new graph, the deleted vertices and all the edges.
    fn remove_all_vertices(&self)
        -> Result<(Self, Vec<Vertex<K, V>>, Vec<Edge<K, W>>), GraphError>;

    /// Add a new edge then return the new graph.
    fn add_edge(&self, edge: Edge<K, W>) -> Result<Self, GraphError>;

    /// Add a new edge between 2 keys then return the new graph.
    fn add_edge_between_keys(&self, key_from: K, key_to: K) -> Result<Self, GraphError> {
        self.add_edge(Edge::new(key_from, key_to))
    }

    /// Remove an existing edge then return the new graph and the deleted edge.
    fn remove_edge(&self, edge: &Edge<K, W>) -> Result<(Self, Edge<K, W>), GraphError>;

    /// Remove an existing edge by their keys, then return the new graph and the deleted edge.
    fn remove_edge_where_keys(
        &self,
        key_from: K,
        key_to: K,
    ) -> Result<(Self, Edge<K, W>), GraphError> {
        self.remove_edge(&Edge::new(key_from, key_to))
    }

    /// Remove all the edges then return the new graph and all the deleted edges.
    fn remove_all_edges(&self) -> Result<(Self, Vec<Edge<K, W>>), GraphError>;

    /// Remove all existing edges from or to a given vertex, then return the new graph and the deleted edges.
    fn remove_all_edges_where_vertex(
        &self,
        vertex: &Vertex<K, V>,
    ) -> Result<(Self, Vec<Edge<K, W>>), GraphError>;

    /// Remove all existing edges from or to a given key, then return the new graph and the deleted edges.
    fn remove_all_edges_where_key(
        &self,
        key_from: K,
    ) -> Result<(Self, Vec<Edge<K, W>>), GraphError> {
        self.remove_all_edges_where_vertex(&Vertex::new(key_from))
    }

    /// Remove all existing edges from a given vertex, then return the new graph and the deleted edges.
    fn remove_all_edges_from_vertex(
        &self,
        vertex: &Vertex<K, V>,
    ) -> Result<(Self, Vec<Edge<K, W>>), GraphError>;

    /// Remove all existing edges from a given key, then return the new graph and the deleted edges.
    fn remove_all_edges_from_key(
        &self,
        key_from: K,
    ) -> Result<(Self, Vec<Edge<K, W>>), GraphError> {
        self.remove_all_edges_from_vertex(&Vertex::new(key_from))
    }
}

pub trait DirectedGraph<K, V, W>: AnyGraph<K, V, W>
where
    K: Key,
    V: Value,
    W: Weight,
{
}

pub trait UndirectedGraph<K, V, W>: AnyGraph<K, V, W>
where
    K: Key,
    V: Value,
    W: Weight,
{
}
