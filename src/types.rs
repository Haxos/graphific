use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

/// An interface used as an helper to implement a key.
pub trait Key: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash + Display {}

impl<T: Copy + Hash + Ord + Display> Key for T {}

/// An interface used as an helper to implement a value contained in a vertex.
pub trait Value: Clone + Copy + Default + PartialEq + Display {}

impl<T: Copy + Default + PartialEq + Display> Value for T {}

/// An interface used as an helper to implement a weight for an edge.
pub trait Weight: Clone + Copy + Default + PartialEq + PartialOrd + Display {}

impl<T: Copy + Default + PartialEq + PartialOrd + Display> Weight for T {}

/// A structure describing a vertex with a [`Key`] and a [`Value`].
#[derive(Clone, Copy, Debug)]
pub struct Vertex<K, V>
where
    K: Key,
    V: Value,
{
    key: K,
    value: V,
}

/// A structure describing an edge with an origin [`Key`] and destination [`Key`] and a potentially [`Weight`].
#[derive(Clone, Copy, Debug)]
pub struct Edge<K, W = i8>
where
    K: Key,
    W: Weight,
{
    from: K,
    to: K,
    weight: W,
}

impl<K, V> Hash for Vertex<K, V>
where
    K: Key,
    V: Value,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl<K, V> PartialEq for Vertex<K, V>
where
    K: Key,
    V: Value,
{
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(other.key())
    }
}

impl<K, V> Eq for Vertex<K, V>
where
    K: Key,
    V: Value,
{
}

impl<K, V> PartialOrd for Vertex<K, V>
where
    K: Key,
    V: Value,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(other.key())
    }
}

impl<K, V> Ord for Vertex<K, V>
where
    K: Key,
    V: Value,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(other.key())
    }
}

impl<K, V> Display for Vertex<K, V>
where
    K: Key,
    V: Value,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.key, self.value)
    }
}

impl<K, V> Vertex<K, V>
where
    K: Key,
    V: Value,
{
    /// Create a new vertex with the default value.
    pub fn new(key: K) -> Self {
        Vertex {
            key,
            value: V::default(),
        }
    }

    /// Create a new vertex with a value.
    pub fn with_value(key: K, value: V) -> Self {
        Vertex { key, value }
    }

    /// Get the key of the vertex.
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Get the value of the vertex.
    pub fn value(&self) -> &V {
        &self.value
    }

    /// Set the value of the vertex.
    pub fn set_value(&mut self, value: V) {
        self.value = value
    }
}

impl<K, W> Hash for Edge<K, W>
where
    K: Key,
    W: Weight,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.from.hash(state);
        self.to.hash(state);
    }
}

impl<K, W> PartialEq for Edge<K, W>
where
    K: Key,
    W: Weight,
{
    fn eq(&self, other: &Self) -> bool {
        self.from.eq(&other.from) && self.to.eq(&other.to)
    }
}

impl<K, W> Eq for Edge<K, W>
where
    K: Key,
    W: Weight,
{
}

impl<K, W> PartialOrd for Edge<K, W>
where
    K: Key,
    W: Weight,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.from.lt(&other.from) || self.from.eq(&other.from) && self.to.lt(&other.to) {
            Some(Ordering::Less)
        } else if self.from.eq(&other.from) && self.to.eq(&other.to) {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl<K, W> Ord for Edge<K, W>
where
    K: Key,
    W: Weight,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.from.lt(&other.from) || self.from.eq(&other.from) && self.to.lt(&other.to) {
            Ordering::Less
        } else if self.from.eq(&other.from) && self.to.eq(&other.to) {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl<K, W> Display for Edge<K, W>
where
    K: Key,
    W: Weight,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}--{}->{}", self.from, self.weight, self.to)
    }
}

impl<K, W> Edge<K, W>
where
    K: Key,
    W: Weight,
{
    /// Create a new edge.
    pub fn new(from: K, to: K) -> Self {
        Edge {
            from,
            to,
            weight: W::default(),
        }
    }

    /// Create a new edge with a weight.
    pub fn with_weight(from: K, to: K, weight: W) -> Self {
        Edge { from, to, weight }
    }

    /// Get the origin.
    pub fn from(&self) -> &K {
        &self.from
    }

    /// Get the destination.
    pub fn to(&self) -> &K {
        &self.to
    }

    /// Get the weight.
    pub fn weight(&self) -> &W {
        &self.weight
    }
}
