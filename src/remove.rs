use crate::{Safe, Unsafe};
#[cfg(feature = "std")]
use std::{
  collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
  hash::Hash,
};

/// Provides the ability to remove a value by key, potentially moving other items within the collection.
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use collectivity::Remove;
///
/// fn remove<K, V>(col: &mut impl Remove<K, V>, k: K) -> Option<V> {
///   col.remove(k)
/// }
///
/// let mut v = vec![1, 2, 3];
/// v.remove(1);
/// assert_eq!(v, vec![1, 3]);
/// let mut v = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
/// v.remove("B");
/// assert_eq!(v, HashMap::from([("A", 1), ("C", 3)]));
/// ```
pub trait Remove<K, V, Safety = Unsafe> {
  /// If there's a value at a specified key, the method removes it and returns it wrapped in an `Option`. Otherwise, it returns `None` without affecting the collection.
  ///
  /// # Panics
  ///
  /// May panic, e.g. when the index is out of bounds.
  fn remove(&mut self, k: K) -> Option<V>;
}

impl<'p, K, V, S, R: Remove<K, V, S>> Remove<K, V, S> for &'p mut R {
  fn remove<'a>(&'a mut self, k: K) -> Option<V> {
    <R as Remove<K, V, S>>::remove(self, k)
  }
}
impl<V> Remove<usize, V, Unsafe> for Vec<V> {
  fn remove(&mut self, k: usize) -> Option<V> {
    Some(self.remove(k))
  }
}

#[cfg(feature = "std")]
impl<V> Remove<usize, V, Safe> for VecDeque<V> {
  fn remove(&mut self, k: usize) -> Option<V> {
    VecDeque::remove(self, k)
  }
}

#[cfg(feature = "std")]
impl<V> Remove<usize, V, Unsafe> for LinkedList<V> {
  fn remove(&mut self, k: usize) -> Option<V> {
    let mut rest = self.split_off(k);
    let v = self.pop_front();
    self.append(&mut rest);
    v
  }
}

#[cfg(feature = "std")]
impl<'k, K: Ord, V> Remove<&'k K, V, Safe> for BTreeMap<K, V> {
  fn remove(&mut self, k: &'k K) -> Option<V> {
    self.remove(k)
  }
}

#[cfg(feature = "std")]
impl<'k, K: Ord> Remove<&'k K, (), Safe> for BTreeSet<K> {
  fn remove(&mut self, k: &'k K) -> Option<()> {
    if self.remove(k) {
      Some(())
    } else {
      None
    }
  }
}

#[cfg(feature = "std")]
impl<'k, K: Eq + Hash, V> Remove<&'k K, V, Safe> for HashMap<K, V> {
  fn remove(&mut self, k: &'k K) -> Option<V> {
    self.remove(k)
  }
}

#[cfg(feature = "std")]
impl<'k, K: Eq + Hash> Remove<&'k K, (), Safe> for HashSet<K> {
  fn remove(&mut self, k: &'k K) -> Option<()> {
    if self.remove(k) {
      Some(())
    } else {
      None
    }
  }
}

#[cfg(feature = "dashmap")]
use dashmap::{DashMap, DashSet};

#[cfg(feature = "dashmap")]
impl<'k, K: Eq + Hash, V> Remove<&'k K, V, Safe> for DashMap<K, V> {
  fn remove(&mut self, k: &'k K) -> Option<V> {
    DashMap::remove(self, k).map(|v| v.1)
  }
}

#[cfg(feature = "dashmap")]
impl<'k, K: Eq + Hash> Remove<&'k K, K, Safe> for DashSet<K> {
  fn remove(&mut self, k: &'k K) -> Option<K> {
    DashSet::remove(self, k)
  }
}

#[cfg(feature = "serde_json")]
use serde_json::Value as SeV;

#[cfg(feature = "serde_json")]
impl<'k> Remove<&'k str, SeV, Safe> for SeV {
  fn remove(&mut self, k: &'k str) -> Option<SeV> {
    match self {
      SeV::Object(o) => o.remove(k),
      _ => panic!("Value is not an object"),
    }
  }
}

#[cfg(feature = "serde_json")]
impl Remove<usize, SeV, Unsafe> for SeV {
  fn remove(&mut self, k: usize) -> Option<SeV> {
    match self {
      SeV::Array(a) => Some(a.remove(k)),
      _ => panic!("Value is not an array"),
    }
  }
}

#[cfg(feature = "simd-json")]
use simd_json::{BorrowedValue as SBV, OwnedValue as SOV};

#[cfg(feature = "simd-json")]
impl<'k, 'a> Remove<&'k str, SBV<'a>, Safe> for SBV<'a> {
  fn remove(&mut self, k: &'k str) -> Option<SBV<'a>> {
    match self {
      SBV::Object(o) => o.remove(k),
      _ => panic!("Value is not an object"),
    }
  }
}

#[cfg(feature = "simd-json")]
impl<'a> Remove<usize, SBV<'a>, Unsafe> for SBV<'a> {
  fn remove(&mut self, k: usize) -> Option<SBV<'a>> {
    match self {
      SBV::Array(a) => Some(a.remove(k)),
      _ => panic!("Value is not an array"),
    }
  }
}

#[cfg(feature = "simd-json")]
impl<'k> Remove<&'k str, SOV, Safe> for SOV {
  fn remove(&mut self, k: &'k str) -> Option<SOV> {
    match self {
      SOV::Object(o) => o.remove(k),
      _ => panic!("Value is not an object"),
    }
  }
}

#[cfg(feature = "simd-json")]
impl Remove<usize, SOV, Unsafe> for SOV {
  fn remove(&mut self, k: usize) -> Option<SOV> {
    match self {
      SOV::Array(a) => Some(a.remove(k)),
      _ => panic!("Value is not an array"),
    }
  }
}

#[cfg(feature = "slab")]
use slab::Slab;

#[cfg(feature = "slab")]
impl<V> Remove<usize, V, Unsafe> for Slab<V> {
  fn remove(&mut self, k: usize) -> Option<V> {
    Some(self.remove(k))
  }
}

#[cfg(feature = "smallvec")]
use smallvec::{Array, SmallVec};

#[cfg(feature = "smallvec")]
impl<V, A: Array<Item = V>> Remove<usize, V, Unsafe> for SmallVec<A> {
  fn remove(&mut self, k: usize) -> Option<V> {
    Some(self.remove(k))
  }
}
