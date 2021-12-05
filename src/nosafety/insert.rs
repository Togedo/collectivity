use crate::Insert as InsertWithSafety;

/// `Insert` without safety information
pub trait Insert<K, V> {
  /// `insert` without safety information
  fn insert(&mut self, k: K, v: V);
}

impl<K, V, I: InsertWithSafety<K, V>> Insert<K, V> for I {
  fn insert(&mut self, k: K, v: V) {
    I::insert(self, k, v)
  }
}
