#[cfg(feature = "std")]
use core::hash::Hash;

pub trait Len {
  fn len(&self) -> usize;
}

impl<L: Len> Len for &L {
  fn len(&self) -> usize {
    <L as Len>::len(self)
  }
}

impl<V> Len for [V] {
  fn len(&self) -> usize {
    self.len()
  }
}

impl<V, const N: usize> Len for [V; N] {
  fn len(&self) -> usize {
    <[V]>::len(self)
  }
}

impl<V> Len for Vec<V> {
  fn len(&self) -> usize {
    self.len()
  }
}

#[cfg(feature = "std")]
impl<V> Len for std::collections::VecDeque<V> {
  fn len(&self) -> usize {
    self.len()
  }
}

#[cfg(feature = "std")]
impl<V> Len for std::collections::LinkedList<V> {
  fn len(&self) -> usize {
    self.len()
  }
}

#[cfg(feature = "std")]
impl<V> Len for std::collections::BinaryHeap<V> {
  fn len(&self) -> usize {
    Self::len(self)
  }
}

#[cfg(feature = "std")]
impl<K: Ord, V> Len for std::collections::BTreeMap<K, V> {
  fn len(&self) -> usize {
    self.len()
  }
}

#[cfg(feature = "std")]
impl<K: Ord> Len for std::collections::BTreeSet<K> {
  fn len(&self) -> usize {
    self.len()
  }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, V> Len for std::collections::HashMap<K, V> {
  fn len(&self) -> usize {
    self.len()
  }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash> Len for std::collections::HashSet<K> {
  fn len(&self) -> usize {
    self.len()
  }
}

#[cfg(feature = "dashmap")]
impl<K: Eq + Hash, V> Len for dashmap::DashMap<K, V> {
  fn len(&self) -> usize {
    self.len()
  }
}

#[cfg(feature = "dashmap")]
impl<K: Eq + Hash> Len for dashmap::DashSet<K> {
  fn len(&self) -> usize {
    self.len()
  }
}

#[cfg(feature = "serde_json")]
impl Len for serde_json::Value {
  fn len(&self) -> usize {
    match self {
      serde_json::Value::Array(a) => a.len(),
      serde_json::Value::Object(o) => o.len(),
      _ => 0,
    }
  }
}

#[cfg(feature = "simd-json")]
impl Len for simd_json::BorrowedValue<'_> {
  fn len(&self) -> usize {
    match self {
      simd_json::BorrowedValue::Array(a) => a.len(),
      simd_json::BorrowedValue::Object(o) => o.len(),
      _ => 0,
    }
  }
}

#[cfg(feature = "simd-json")]
impl Len for simd_json::OwnedValue {
  fn len(&self) -> usize {
    match self {
      simd_json::OwnedValue::Array(a) => a.len(),
      simd_json::OwnedValue::Object(o) => o.len(),
      _ => 0,
    }
  }
}

#[cfg(feature = "slab")]
impl<V> Len for slab::Slab<V> {
  fn len(&self) -> usize {
    self.len()
  }
}

#[cfg(feature = "smallvec")]
impl<V: smallvec::Array> Len for smallvec::SmallVec<V> {
  fn len(&self) -> usize {
    self.len()
  }
}
