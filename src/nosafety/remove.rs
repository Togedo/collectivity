use crate::Remove as RemoveWithSafety;

/// `Remove` without safety information
pub trait Remove<K, V> {
  /// `remove` without safety information
  fn remove(&mut self, k: K) -> Option<V>;
}

impl<K, V, R: RemoveWithSafety<K, V>> Remove<K, V> for R {
  fn remove(&mut self, k: K) -> Option<V> {
    R::remove(self, k)
  }
}
