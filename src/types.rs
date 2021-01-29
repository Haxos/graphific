use std::hash::Hash;

pub trait Key: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash {}
// impl<T: Clone + Copy + Eq + PartialOrd + Ord + Hash> Key for T {}

pub trait Value: Clone + Copy + Default {}
// impl<T: Clone + Copy + Default> Value for T {}

/// Structure describing a vertex with a [`Key`] and a [`Value`].
#[derive(Clone, Copy)]
pub struct Vertex<K, V>
where
    K: Key,
    V: Value,
{
    key: K,
    value: V,
}

/// Structure describing an edge with an origin [`Key`] and destination [`Key`].
#[derive(Clone, Copy)]
pub struct Edge<K>
where
    K: Key,
{
    from: K,
    to: K,
}

impl<K, V> Vertex<K, V>
where
    K: Key,
    V: Value,
{
    /// Create a new vertex with the default value.
    fn new(key: K) -> Self {
        Vertex {
            key,
            value: V::default(),
        }
    }

    /// Create a new vertex with a value.
    fn with_value(key: K, value: V) -> Self {
        Vertex { key, value }
    }

    /// Get the key of the vertex.
    fn key(&self) -> &K {
        &self.key
    }

    /// Get the value of the vertex.
    fn value(&self) -> &V {
        &self.value
    }

    /// Set the value of the vertex.
    fn set_value(&mut self, value: V) {
        self.value = value
    }
}

impl<K> Edge<K>
where
    K: Key,
{
    /// Create a new edge.
    fn new(from: K, to: K) -> Self {
        Edge { from, to }
    }

    /// Get the origin.
    fn from(&self) -> &K {
        &self.from
    }

    /// Get the destination
    fn to(&self) -> &K {
        &self.to
    }
}
