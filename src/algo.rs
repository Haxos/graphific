use crate::{AnyGraph, Edge, Key, Kinship, Value, Vertex, Weight};
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};

#[derive(Copy, Clone)]
pub struct Functions<K, V, W>
where
    K: Key,
    V: Value,
    W: Weight,
{
    edges_comparator: Option<fn(&Edge<K, W>, &Edge<K, W>) -> Ordering>,
    vertex_fn: Option<fn(&Vertex<K, V>)>,
    pre_fn: Option<fn(&Edge<K, W>)>,
    sym_fn: Option<fn(&Edge<K, W>)>,
    post_fn: Option<fn(&Edge<K, W>)>,
}

impl<K, V, W> Functions<K, V, W>
where
    K: Key,
    V: Value,
    W: Weight,
{
    pub fn empty() -> Self {
        Functions {
            edges_comparator: None,
            vertex_fn: None,
            pre_fn: None,
            sym_fn: None,
            post_fn: None,
        }
    }

    pub fn edges_comparator(&self) -> &Option<fn(&Edge<K, W>, &Edge<K, W>) -> Ordering> {
        &self.edges_comparator
    }

    pub fn set_edges_comparator(&mut self, f: fn(&Edge<K, W>, &Edge<K, W>) -> Ordering) {
        self.edges_comparator = Some(f);
    }

    pub fn vertex_fn(&self) -> &Option<fn(&Vertex<K, V>)> {
        &self.vertex_fn
    }

    pub fn set_vertex_fn(&mut self, f: fn(&Vertex<K, V>)) {
        self.vertex_fn = Some(f);
    }

    pub fn pre_fn(&self) -> &Option<fn(&Edge<K, W>)> {
        &self.pre_fn
    }

    pub fn set_pre_fn(&mut self, f: fn(&Edge<K, W>)) {
        self.pre_fn = Some(f);
    }

    pub fn sym_fn(&self) -> &Option<fn(&Edge<K, W>)> {
        &self.sym_fn
    }

    pub fn set_sym_fn(&mut self, f: fn(&Edge<K, W>)) {
        self.sym_fn = Some(f);
    }

    pub fn post_fn(&self) -> &Option<fn(&Edge<K, W>)> {
        &self.post_fn
    }

    pub fn set_post_fn(&mut self, f: fn(&Edge<K, W>)) {
        self.post_fn = Some(f);
    }
}

/// An interface describing all the algorithms that can be used on any kind of graphs.
pub trait Algorithms<K, V, W>: AnyGraph<K, V, W> + Kinship<K, V, W>
where
    K: Key,
    V: Value,
    W: Weight,
{
    /// Execute a Broad Search First the return the discovered graph.
    /// There is no order in which the edges are treated.
    fn simple_bfs(&self) -> Option<Self> {
        let mut fns = Functions::empty();
        fns.set_edges_comparator(|a, b| a.partial_cmp(b).unwrap());
        self.bfs(fns)
    }

    fn bfs(&self, fns: Functions<K, V, W>) -> Option<Self> {
        return if self.vertices().is_empty() {
            None
        } else {
            let first = self.vertices().first().unwrap().clone();
            self.bfs_with_starting_vertex(&first, fns)
        };
    }

    /// Execute a Broad Search First with a starting vertex the return the discovered graph.
    /// There is no order in which the edges are treated.
    fn bfs_with_starting_vertex(
        &self,
        starting_vertex: &Vertex<K, V>,
        fns: Functions<K, V, W>,
    ) -> Option<Self> {
        if !self.vertices().contains(starting_vertex) {
            return None;
        }
        let cloned_graph = self.clone();
        let mut queue: VecDeque<K> = VecDeque::new();
        queue.push_back(starting_vertex.key().clone());

        return if let Some((mut new_graph, _)) = cloned_graph.remove_all_edges() {
            let successors = cloned_graph.successors_as_key_and_edges();
            let mut flagged: HashSet<K> = HashSet::new();

            flagged.insert(starting_vertex.key().clone());

            while !queue.is_empty() {
                let current = queue.pop_front().unwrap();
                let mut neighbours: Vec<Edge<K, W>> = successors
                    .get(&current)
                    .unwrap()
                    .into_iter()
                    .cloned()
                    .collect();

                if let Some(edges_comparator) = fns.edges_comparator() {
                    neighbours.sort_by(edges_comparator);
                }

                for neighbour in neighbours {
                    if !flagged.contains(neighbour.to()) {
                        new_graph = new_graph.add_edge(neighbour).unwrap();
                        flagged.insert(neighbour.to().clone());
                        queue.push_back(neighbour.to().clone());
                    }
                }
            }
            Some(new_graph)
        } else {
            return None;
        };
    }

    /// Execute a Deep Search First the return the discovered graph.
    /// There is no order in which the edges are treated.
    fn simple_dfs(&self) -> Option<Self> {
        return if self.vertices().is_empty() {
            None
        } else {
            let first = self.vertices().first().unwrap().clone();
            self.dfs_with_starting_vertex(&first)
        };
    }

    /// Execute a Deep Search First with a starting vertex the return the discovered graph.
    /// There is no order in which the edges are treated.
    fn dfs_with_starting_vertex(&self, starting_vertex: &Vertex<K, V>) -> Option<Self> {
        if !self.vertices().contains(starting_vertex) {
            return None;
        }
        let cloned_graph = self.clone();
        let mut stack: VecDeque<K> = VecDeque::new();
        stack.push_back(starting_vertex.key().clone());

        return if let Some((mut new_graph, _)) = cloned_graph.remove_all_edges() {
            let successors = cloned_graph.successors_as_key_and_edges();
            let mut flagged: HashSet<K> = HashSet::new();

            flagged.insert(starting_vertex.key().clone());

            while !stack.is_empty() {
                let current = stack.pop_back().unwrap();
                let neighbours = successors.get(&current).unwrap();

                for neighbour in neighbours {
                    if !flagged.contains(neighbour.to()) {
                        new_graph = new_graph.add_edge(*neighbour).unwrap();
                        flagged.insert(neighbour.to().clone());
                        stack.push_back(neighbour.to().clone());
                    }
                }
            }
            Some(new_graph)
        } else {
            return None;
        };
    }
}
