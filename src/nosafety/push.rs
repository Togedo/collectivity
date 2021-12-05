use crate::Push as PushWithSafety;

/// `Push` without safety information
pub trait Push<V> {
  /// `push` without safety information
  fn push(&mut self, v: V);
}

impl<V, P: PushWithSafety<V>> Push<V> for P {
  fn push(&mut self, v: V) {
    P::push(self, v)
  }
}
