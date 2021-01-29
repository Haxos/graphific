use crate::types::{Key, Value, Vertex};

pub trait AnyGraph<K, V>
where
    K: Key,
    V: Value,
{
    fn vertices(&self) -> Vec<Vertex<K, V>>;

    fn edges(&self) -> Vec<(Vertex<K, V>, Vertex<K, V>)>;
}
