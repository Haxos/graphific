//! `graphific` is a graph data structure library.

mod algorithms;
mod any_graph;
mod basic_directed_graph;
mod basic_undirected_graph;
mod kinship;
mod types;

pub use self::algorithms::Algorithms;

pub use self::any_graph::AnyGraph;
pub use self::any_graph::GraphError;

pub use self::kinship::Kinship;

pub use self::types::Edge;
pub use self::types::Key;
pub use self::types::Value;
pub use self::types::Vertex;
pub use self::types::Weight;

pub use self::basic_directed_graph::BasicDirectedGraph;
pub use self::basic_undirected_graph::BasicUndirectedGraph;
