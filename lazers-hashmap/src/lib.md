# A hashmap backend for lazers


```rust
extern crate lazers_traits;
use std::collections::HashMap;
use std::hash::Hash;
use std::borrow::Borrow;
use lazers_traits::Backend;

pub struct HashMapBackend<K,V> {
    map: HashMap<K,V>
}

impl<K: Eq + Hash, V> HashMapBackend<K, V> {
    pub fn new() -> HashMapBackend<K, V> {
        HashMapBackend { map: HashMap::new() }
    }
}

impl<K: Eq + Hash, V> Backend for HashMapBackend<K, V> {
    type K = K;
    type V = V;

    fn get<Q: ?Sized>(&self, k: &Q) -> Option<&Self::V> where Self::K: Borrow<Q>, Q: Hash + Eq, Self : Sized {
        self.map.get(k)
    }
    fn set(&mut self, k: Self::K, v: Self::V) -> Option<Self::V> {
        self.map.insert(k, v)
    }
    fn delete<Q: ?Sized>(&mut self, k: &Q) -> Option<Self::V> where Self::K: Borrow<Q>, Q: Hash + Eq, Self : Sized {
        self.map.remove(k)
    }

}
```
