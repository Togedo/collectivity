use crate::{Safe, SafetyMarker, Unsafe};
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
pub trait Remove<K, V> {
  /// Indicates whether the `remove` method may panic in a particular implementation.
  type Safety: SafetyMarker;
  /// If there's a value at a specified key, the method removes it and returns it wrapped in an `Option`. Otherwise, it returns `None` without affecting the collection.
  ///
  /// # Panics
  ///
  /// May panic, e.g. when the index is out of bounds.
  fn remove(&mut self, k: K) -> Option<V>;
}

impl<'r, K, V, R: Remove<K, V>> Remove<K, V> for &'r mut R {
  type Safety = <R as Remove<K, V>>::Safety;
  fn remove<'a>(&'a mut self, k: K) -> Option<V> {
    <R as Remove<K, V>>::remove(self, k)
  }
}

impl<V> Remove<usize, V> for Vec<V> {
  type Safety = Unsafe;
  fn remove(&mut self, k: usize) -> Option<V> {
    Some(self.remove(k))
  }
}

#[cfg(feature = "std")]
impl<V> Remove<usize, V> for VecDeque<V> {
  type Safety = Safe;
  fn remove(&mut self, k: usize) -> Option<V> {
    VecDeque::remove(self, k)
  }
}

#[cfg(feature = "std")]
impl<V> Remove<usize, V> for LinkedList<V> {
  type Safety = Unsafe;
  fn remove(&mut self, k: usize) -> Option<V> {
    let mut rest = self.split_off(k);
    let v = self.pop_front();
    self.append(&mut rest);
    v
  }
}

#[cfg(feature = "std")]
impl<'k, K: Ord, V> Remove<&'k K, V> for BTreeMap<K, V> {
  type Safety = Safe;
  fn remove(&mut self, k: &'k K) -> Option<V> {
    self.remove(k)
  }
}

#[cfg(feature = "std")]
impl<K: Ord, V> Remove<K, V> for BTreeMap<K, V> {
  type Safety = Safe;
  fn remove(&mut self, k: K) -> Option<V> {
    self.remove(&k)
  }
}

#[cfg(feature = "std")]
impl<'k, K: Ord> Remove<&'k K, ()> for BTreeSet<K> {
  type Safety = Safe;
  fn remove(&mut self, k: &'k K) -> Option<()> {
    if self.remove(k) {
      Some(())
    } else {
      None
    }
  }
}

#[cfg(feature = "std")]
impl<K: Ord> Remove<K, ()> for BTreeSet<K> {
  type Safety = Safe;
  fn remove(&mut self, k: K) -> Option<()> {
    if self.remove(&k) {
      Some(())
    } else {
      None
    }
  }
}

#[cfg(feature = "std")]
impl<'k, K: Eq + Hash, V> Remove<&'k K, V> for HashMap<K, V> {
  type Safety = Safe;
  fn remove(&mut self, k: &'k K) -> Option<V> {
    self.remove(k)
  }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, V> Remove<K, V> for HashMap<K, V> {
  type Safety = Safe;
  fn remove(&mut self, k: K) -> Option<V> {
    self.remove(&k)
  }
}

#[cfg(feature = "std")]
impl<'k, K: Eq + Hash> Remove<&'k K, ()> for HashSet<K> {
  type Safety = Safe;
  fn remove(&mut self, k: &'k K) -> Option<()> {
    if self.remove(k) {
      Some(())
    } else {
      None
    }
  }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash> Remove<K, ()> for HashSet<K> {
  type Safety = Safe;
  fn remove(&mut self, k: K) -> Option<()> {
    if self.remove(&k) {
      Some(())
    } else {
      None
    }
  }
}

#[cfg(feature = "dashmap")]
use dashmap::{DashMap, DashSet};

#[cfg(feature = "dashmap")]
impl<'k, K: Eq + Hash, V> Remove<&'k K, V> for DashMap<K, V> {
  type Safety = Safe;
  fn remove(&mut self, k: &'k K) -> Option<V> {
    DashMap::remove(self, k).map(|v| v.1)
  }
}

#[cfg(feature = "dashmap")]
impl<K: Eq + Hash, V> Remove<K, V> for DashMap<K, V> {
  type Safety = Safe;
  fn remove(&mut self, k: K) -> Option<V> {
    DashMap::remove(self, &k).map(|v| v.1)
  }
}

#[cfg(feature = "dashmap")]
impl<'k, K: Eq + Hash> Remove<&'k K, K> for DashSet<K> {
  type Safety = Safe;
  fn remove(&mut self, k: &'k K) -> Option<K> {
    DashSet::remove(self, k)
  }
}

#[cfg(feature = "dashmap")]
impl<K: Eq + Hash> Remove<K, K> for DashSet<K> {
  type Safety = Safe;
  fn remove(&mut self, k: K) -> Option<K> {
    DashSet::remove(self, &k)
  }
}

#[cfg(feature = "serde_json")]
use serde_json::Value as SeV;

#[cfg(feature = "serde_json")]
impl<'k> Remove<&'k str, SeV> for SeV {
  type Safety = Unsafe;
  fn remove(&mut self, k: &'k str) -> Option<SeV> {
    match self {
      SeV::Object(o) => o.remove(k),
      _ => panic!("Value is not an object"),
    }
  }
}

#[cfg(feature = "serde_json")]
impl Remove<usize, SeV> for SeV {
  type Safety = Unsafe;
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
impl<'k, 'a> Remove<&'k str, SBV<'a>> for SBV<'a> {
  type Safety = Unsafe;
  fn remove(&mut self, k: &'k str) -> Option<SBV<'a>> {
    match self {
      SBV::Object(o) => o.remove(k),
      _ => panic!("Value is not an object"),
    }
  }
}

#[cfg(feature = "simd-json")]
impl<'a> Remove<usize, SBV<'a>> for SBV<'a> {
  type Safety = Unsafe;
  fn remove(&mut self, k: usize) -> Option<SBV<'a>> {
    match self {
      SBV::Array(a) => Some(a.remove(k)),
      _ => panic!("Value is not an array"),
    }
  }
}

#[cfg(feature = "simd-json")]
impl<'k> Remove<&'k str, SOV> for SOV {
  type Safety = Unsafe;
  fn remove(&mut self, k: &'k str) -> Option<SOV> {
    match self {
      SOV::Object(o) => o.remove(k),
      _ => panic!("Value is not an object"),
    }
  }
}

#[cfg(feature = "simd-json")]
impl Remove<usize, SOV> for SOV {
  type Safety = Unsafe;
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
impl<V> Remove<usize, V> for Slab<V> {
  type Safety = Unsafe;
  fn remove(&mut self, k: usize) -> Option<V> {
    Some(self.remove(k))
  }
}

#[cfg(feature = "smallvec")]
use smallvec::{Array, SmallVec};

#[cfg(feature = "smallvec")]
impl<V, A: Array<Item = V>> Remove<usize, V> for SmallVec<A> {
  type Safety = Unsafe;
  fn remove(&mut self, k: usize) -> Option<V> {
    Some(self.remove(k))
  }
}
