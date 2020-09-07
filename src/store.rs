extern crate lru;

use lru::{ LruCache };
use std::hash::Hash;

pub struct Store<K, V> {
  cache: LruCache<K, V>
}

impl<'a, K: Clone + Hash + Eq, V> Store<K, V> {
  pub fn new() -> Self {
    Store {
      cache: LruCache::new(1000),
    }
  }

  pub fn get(&mut self, key: &K) -> Option<&V> {
    return self.cache.get(key);
  }

  // FIXME: Lifetime problem
  //  E0495 - cannot infer an appropriate lifetime for 
  //  autoref due to conflicting requirements
  pub fn get_multi(&mut self, keys: &Vec<&K>) -> Vec<Option<&V>> {
    return keys
      .iter()
      .map(|&key| self.get(key))
      .collect::<Vec<Option<&V>>>();
  }

  // TODO: Set multiple records
  // recordKey, keys, values
  pub fn set(&mut self, key: K, value: V) -> Option<V> {
    return self.cache.put(key, value);
   }

  pub fn clear(&mut self) {
    self.cache.clear();
  }

  pub fn size(&mut self) -> usize {
    return self.cache.len();
  }
}

#[cfg(test)]
mod tests {
  use super::Store;

  #[test]
  fn it_works() {
    let mut store = Store::new();

    store.set("apple", 3);
    store.set("banana", 2);
    assert_eq!(store.get(&"apple"), Some(&3));
    assert_eq!(store.get(&"banana"), Some(&2));
    assert!(store.get(&"pear").is_none());
    assert_eq!(store.set("banana", 4), Some(2));
    assert_eq!(store.set("pear", 5), None);
    assert_eq!(store.get(&"pear"), Some(&5));
    assert_eq!(store.get(&"banana"), Some(&4));
    assert_eq!(store.size(), 3);

    assert_eq!(store.get_multi(&vec![&"pear", &"banana"]), [Some(&5), Some(&4)]);

    store.clear();
    assert_eq!(store.size(), 0);
  }
}
