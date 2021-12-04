#[cfg(feature = "std")]
use core::hash::Hash;

/// Provides safe access to a value at a specified key.
///
/// ## Examples
/// ```
/// use std::collections::BTreeMap;
/// use collectivity::Get;
///
/// fn get<'a, K, V>(
///   col: &'a impl Get<K, Value<'a> = V>,
///   pos: K,
/// ) -> Option<V> {
///   col.get(pos)
/// }
///
/// let v = vec![0];
/// assert_eq!(get(&v, 0), Some(&0));
/// let m = BTreeMap::from([("A", 1)]);
/// assert_eq!(get(&m, &"A"), Some(&1));
/// ```
pub trait Get<K> {
  /// The type of the value returned by `get`.
  type Value<'a>
  where
    Self: 'a;
  /// Returns the `Option`-wrapped value, or `None` if `k` is missing. Both the argument and the return value can be either borrowed or owned, depending on the implementer.
  fn get<'a>(&'a self, k: K) -> Option<Self::Value<'a>>;
  /// Returns `true` if the `k` exists in the collection and `false` otherwise.
  fn contains<'a>(&'a self, k: K) -> bool {
    self.get(k).is_some()
  }
}

// impl<'g, K, G: Get<K>> Get<K> for &'g G {
//   type Value<'a>
//   where
//     Self: 'a,
//   = <G as Get<K>>::Value<'a>;

//   fn get<'a>(&'a self, k: K) -> Option<Self::Value<'a>> {
//     <G as Get<K>>::get(self, k)
//   }
// }

impl<'k, G: Get<usize>> Get<&'k usize> for G {
  type Value<'a>
  where
    Self: 'a,
  = <G as Get<usize>>::Value<'a>;

  fn get<'a>(&'a self, k: &'k usize) -> Option<Self::Value<'a>> {
    <G as Get<usize>>::get(self, *k)
  }
}

impl<V> Get<usize> for [V] {
  type Value<'a>
  where
    Self: 'a,
  = &'a V;

  fn get<'a>(&'a self, k: usize) -> Option<&'a V> {
    <[V]>::get(self, k)
  }
}

impl<V> Get<usize> for &[V] {
  type Value<'a>
  where
    Self: 'a,
  = &'a V;

  fn get<'a>(&'a self, k: usize) -> Option<&'a V> {
    <[V]>::get(self, k)
  }
}

impl<V, const N: usize> Get<usize> for [V; N] {
  type Value<'a>
  where
    Self: 'a,
  = &'a V;

  fn get<'a>(&'a self, k: usize) -> Option<&'a V> {
    <[V]>::get(self, k)
  }
}

impl<V> Get<usize> for Vec<V> {
  type Value<'a>
  where
    Self: 'a,
  = &'a V;

  fn get<'a>(&'a self, k: usize) -> Option<&'a V> {
    <[V]>::get(self, k)
  }
}

#[cfg(feature = "std")]
impl<V> Get<usize> for std::collections::VecDeque<V> {
  type Value<'a>
  where
    Self: 'a,
  = &'a V;

  fn get<'a>(&'a self, k: usize) -> Option<&'a V> {
    std::collections::VecDeque::get(self, k)
  }
}

#[cfg(feature = "std")]
impl<V> Get<usize> for std::collections::LinkedList<V> {
  type Value<'a>
  where
    Self: 'a,
  = &'a V;

  fn get<'a>(&'a self, k: usize) -> Option<&'a V> {
    self.iter().skip(k).next()
  }
}

#[cfg(feature = "std")]
impl<'k, K: Ord, V> Get<&'k K> for std::collections::BTreeMap<K, V> {
  type Value<'a>
  where
    Self: 'a,
  = &'a V;

  fn get<'a>(&'a self, k: &'k K) -> Option<&'a V> {
    std::collections::BTreeMap::get(self, &k)
  }
}

#[cfg(feature = "std")]
impl<'k, K: Ord> Get<&'k K> for std::collections::BTreeSet<K> {
  type Value<'a>
  where
    Self: 'a,
  = &'a K;

  fn get<'a>(&'a self, k: &'k K) -> Option<&'a K> {
    std::collections::BTreeSet::get(self, &k)
  }
}

#[cfg(feature = "std")]
impl<'k, K: Eq + Hash, V> Get<&'k K> for std::collections::HashMap<K, V> {
  type Value<'a>
  where
    Self: 'a,
  = &'a V;

  fn get<'a>(&'a self, k: &'k K) -> Option<&'a V> {
    std::collections::HashMap::get(self, &k)
  }
}

#[cfg(feature = "std")]
impl<'k, K: Eq + Hash> Get<&'k K> for std::collections::HashSet<K> {
  type Value<'a>
  where
    Self: 'a,
  = &'a K;

  fn get<'a>(&'a self, k: &'k K) -> Option<&'a K> {
    std::collections::HashSet::get(self, &k)
  }
}

#[cfg(feature = "dashmap")]
impl<'k, K: Eq + Hash, V> Get<&'k K> for dashmap::DashMap<K, V> {
  type Value<'a>
  where
    Self: 'a,
  = dashmap::mapref::one::Ref<'a, K, V>;

  fn get<'a>(&'a self, k: &'k K) -> Option<Self::Value<'a>> {
    dashmap::DashMap::get(self, &k)
  }
}

#[cfg(feature = "dashmap")]
impl<'k, K: Eq + Hash> Get<&'k K> for dashmap::DashSet<K> {
  type Value<'a>
  where
    Self: 'a,
  = dashmap::setref::one::Ref<'a, K>;

  fn get<'a>(&'a self, k: &'k K) -> Option<Self::Value<'a>> {
    dashmap::DashSet::get(self, &k)
  }
}

#[cfg(feature = "serde_json")]
impl<'k> Get<&'k str> for serde_json::Value {
  type Value<'a>
  where
    Self: 'a,
  = &'a serde_json::Value;

  /// Returns `None` if the value is not an object or `k` is missing
  fn get<'a>(&'a self, k: &'k str) -> Option<Self::Value<'a>> {
    match self {
      serde_json::Value::Object(o) => o.get(k),
      _ => None,
    }
  }
}

#[cfg(feature = "serde_json")]
impl Get<usize> for serde_json::Value {
  type Value<'a>
  where
    Self: 'a,
  = &'a serde_json::Value;

  /// Returns `None` if the value is not an array or `k` is out of bounds
  fn get<'a>(&'a self, k: usize) -> Option<Self::Value<'a>> {
    match self {
      serde_json::Value::Array(a) => a.get(k),
      _ => None,
    }
  }
}

#[cfg(feature = "simd-json")]
impl<'k> Get<&'k str> for simd_json::BorrowedValue<'_> {
  type Value<'a>
  where
    Self: 'a,
  = &'a simd_json::BorrowedValue<'a>;

  /// Returns `None` if the value is not an object or `k` is missing
  fn get<'a>(&'a self, k: &'k str) -> Option<Self::Value<'a>> {
    match self {
      simd_json::BorrowedValue::Object(o) => o.get(k),
      _ => None,
    }
  }
}

#[cfg(feature = "simd-json")]
impl Get<usize> for simd_json::BorrowedValue<'_> {
  type Value<'a>
  where
    Self: 'a,
  = &'a simd_json::BorrowedValue<'a>;

  /// Returns `None` if the value is not an array or `k` is out of bounds
  fn get<'a>(&'a self, k: usize) -> Option<Self::Value<'a>> {
    match self {
      simd_json::BorrowedValue::Array(a) => a.get(k),
      _ => None,
    }
  }
}

#[cfg(feature = "simd-json")]
impl<'k> Get<&'k str> for simd_json::OwnedValue {
  type Value<'a>
  where
    Self: 'a,
  = &'a simd_json::OwnedValue;

  /// Returns `None` if the value is not an object or `k` is missing
  fn get<'a>(&'a self, k: &'k str) -> Option<Self::Value<'a>> {
    match self {
      simd_json::OwnedValue::Object(o) => o.get(k),
      _ => None,
    }
  }
}

#[cfg(feature = "simd-json")]
impl Get<usize> for simd_json::OwnedValue {
  type Value<'a>
  where
    Self: 'a,
  = &'a simd_json::OwnedValue;

  /// Returns `None` if the value is not an array or `k` is out of bounds
  fn get<'a>(&'a self, k: usize) -> Option<Self::Value<'a>> {
    match self {
      simd_json::OwnedValue::Array(a) => a.get(k),
      _ => None,
    }
  }
}

// #[cfg(feature = "slab")]
// impl<V> Get<usize> for slab::Slab<V> {
//   type Key = usize;
//   type Value<'a>
//   where
//     Self: 'a,
//   = &'a V;

//   fn get<'a>(&'a self, k: usize) -> Option<Self::Value<'a>> {
//     slab::Slab::get(self, k)
//   }
// }

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn array() {
    assert_eq!(<[i32] as Get<usize>>::get(&[], 1), None);
    assert_eq!(<[i32] as Get<usize>>::get(&[0, 1], 1), Some(&1));
    assert_eq!(<[usize; 0] as Get<usize>>::get(&[0; 0], 0), None);
    assert_eq!(<[usize; 1] as Get<usize>>::get(&[0; 1], 0), Some(&0));
  }
  #[test]
  fn vec() {
    assert_eq!(<Vec<i32> as Get<_>>::get(&vec![10], 0), Some(&10));
  }
  #[test]
  fn vec_deque() {
    assert_eq!(
      <std::collections::VecDeque<()> as Get<_>>::get(&Default::default(), 0),
      None
    );
  }
  #[test]
  fn linked_list() {
    assert_eq!(
      <std::collections::LinkedList<()> as Get<_>>::get(&Default::default(), 0),
      None
    );
  }
  #[test]
  fn b_tree_map() {
    assert_eq!(
      <std::collections::BTreeMap<(), ()> as Get<_>>::get(&Default::default(), &()),
      None
    );
  }
  #[test]
  fn b_tree_set() {
    assert_eq!(
      <std::collections::BTreeSet<()> as Get<_>>::get(&Default::default(), &()),
      None
    );
  }
  #[test]
  fn hash_map() {
    assert_eq!(
      <std::collections::HashMap<(), ()> as Get<_>>::get(&Default::default(), &()),
      None
    );
  }
  #[test]
  fn hash_set() {
    assert_eq!(
      <std::collections::HashSet<()> as Get<_>>::get(&Default::default(), &()),
      None
    );
  }
}

#[cfg(test)]
#[cfg(feature = "dashmap")]
mod dashmap_tests {
  use super::*;
  #[test]
  fn dash_map() {
    <dashmap::DashMap<(), ()> as Get<_>>::get(&Default::default(), &());
  }
  #[test]
  fn dash_set() {
    <dashmap::DashSet<()> as Get<_>>::get(&Default::default(), &());
  }
}

#[cfg(test)]
#[cfg(feature = "serde_json")]
mod serde_json_tests {
  #[test]
  fn serde_json_object() {
    assert_eq!(serde_json::Value::Null.get(""), None);
  }
  #[test]
  fn serde_json_array() {
    assert_eq!(
      serde_json::Value::Array(vec![serde_json::Value::Bool(true)]).get(0),
      Some(&serde_json::Value::Bool(true))
    );
  }
}

#[cfg(test)]
#[cfg(feature = "simd-json")]
mod simd_json_tests {
  use super::*;
  #[test]
  fn simd_json_borrowed_object() {
    assert_eq!(
      simd_json::BorrowedValue::Static(simd_json::StaticNode::Null).get(""),
      None
    );
  }
  #[test]
  fn simd_json_borrowed_array() {
    assert_eq!(simd_json::BorrowedValue::Array(vec![]).get(0), None);
  }
  #[test]
  fn simd_json_owned_object() {
    assert_eq!(
      simd_json::OwnedValue::Static(simd_json::StaticNode::Null).get(""),
      None
    );
  }
  #[test]
  fn simd_json_owned_array() {
    assert_eq!(
      simd_json::OwnedValue::Static(simd_json::StaticNode::Null).get(0),
      None
    );
  }
}

// #[cfg(test)]
// #[cfg(feature = "slab")]
// mod slab_tests {
//   use super::*;
//   #[test]
//   fn slab() {
//     assert_eq!(
//       <slab::Slab<()> as Get<_>>::get(&slab::Slab::<()>::new(), 0),
//       None
//     );
//   }
// }

// #[cfg(test)]
// #[cfg(feature = "smallvec")]
// mod smallvec_tests {
//   use super::*;
//   #[test]
//   fn small_vec() {
//     assert_eq!(
//       <[()] as Get<_>>::get(&smallvec::SmallVec::<[(); 0]>::new(), 0),
//       None
//     );
//   }
// }
