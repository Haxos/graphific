//! `graphific` is a graph data structure library.

mod algo;
mod any_graph;
mod any_network;
mod basic_directed_graph;
mod basic_directed_network;
mod basic_undirected_graph;
mod kinship;
mod types;

pub use self::algo::Algorithms;

pub use self::any_graph::AnyGraph;
pub use self::any_network::AnyNetwork;

pub use self::kinship::Kinship;

pub use self::types::Edge;
pub use self::types::Key;
pub use self::types::Value;
pub use self::types::Vertex;
pub use self::types::Weight;
pub use self::types::WeightedEdge;

pub use self::basic_directed_graph::BasicDirectedGraph;
pub use self::basic_directed_network::BasicDirectedNetwork;
pub use self::basic_undirected_graph::BasicUndirectedGraph;
