use std::hash::Hash;

pub trait Key: Clone + Eq + Hash {}
impl<T: Clone + Eq + Hash> Key for T {}

pub trait Value: Clone + Default {}
impl<T: Clone + Default> Value for T {}

pub struct Vertex<K, V>
where
    K: Key,
    V: Value,
{
    key: K,
    value: V,
}

impl<K, V> Vertex<K, V>
where
    K: Key,
    V: Value,
{
    fn new(key: K, value: V) -> Self {
        Vertex { key, value }
    }

    fn key(&self) -> &K {
        &self.key
    }

    fn value(&self) -> &V {
        &self.value
    }

    fn set_value(&mut self, value: V) {
        self.value = value
    }
}
