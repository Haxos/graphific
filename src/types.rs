use std::hash::Hash;

pub trait Key: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash {}
// impl<T: Clone + Copy + Eq + PartialOrd + Ord + Hash> Key for T {}

pub trait Value: Clone + Copy + Default {}
// impl<T: Clone + Copy + Default> Value for T {}

/// Structure describing a vertex with a [`Key`] and a [`Value`]
pub struct Vertex<K, V>
where
    K: Key,
    V: Value,
{
    key: K,
    value: V,
}

impl<K, V> Clone for Vertex<K, V>
where
    K: Key,
    V: Value,
{
    fn clone(&self) -> Self {
        Vertex {
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }
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
