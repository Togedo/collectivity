#[cfg(feature = "std")]
use core::hash::Hash;

pub trait Get {
  type Key<'a>
  where
    Self: 'a;
  type Value<'a>
  where
    Self: 'a;
  fn get<'a>(&'a self, k: Self::Key<'a>) -> Option<Self::Value<'a>>;
  fn contains<'a>(&'a self, k: Self::Key<'a>) -> bool {
    self.get(k).is_some()
  }
}

impl<'g, G: Get> Get for &'g G {
  type Key<'a>
  where
    Self: 'a,
  = <G as Get>::Key<'a>;
  type Value<'a>
  where
    Self: 'a,
  = <G as Get>::Value<'a>;

  fn get<'a>(&'a self, k: Self::Key<'a>) -> Option<Self::Value<'a>> {
    <G as Get>::get(self, k)
  }
}

impl<V> Get for [V] {
  type Key<'a>
  where
    Self: 'a,
  = usize;
  type Value<'a>
  where
    Self: 'a,
  = &'a V;

  fn get<'a>(&'a self, k: Self::Key<'a>) -> Option<Self::Value<'a>> {
    <[V]>::get(self, k)
  }
}

impl<V> Get for &[V] {
  type Key<'a>
  where
    Self: 'a,
  = usize;
  type Value<'a>
  where
    Self: 'a,
  = &'a V;

  fn get<'a>(&'a self, k: Self::Key<'a>) -> Option<Self::Value<'a>> {
    <[V]>::get(self, k)
  }
}

impl<V, const N: usize> Get for [V; N] {
  type Key<'a>
  where
    Self: 'a,
  = usize;
  type Value<'a>
  where
    Self: 'a,
  = &'a V;

  fn get<'a>(&'a self, k: Self::Key<'a>) -> Option<Self::Value<'a>> {
    <[V]>::get(self, k)
  }
}

impl<V> Get for Vec<V> {
  type Key<'a>
  where
    Self: 'a,
  = usize;
  type Value<'a>
  where
    Self: 'a,
  = &'a V;

  fn get<'a>(&'a self, k: Self::Key<'a>) -> Option<Self::Value<'a>> {
    <[V]>::get(self, k)
  }
}

#[cfg(feature = "std")]
impl<V> Get for std::collections::VecDeque<V> {
  type Key<'a>
  where
    Self: 'a,
  = usize;
  type Value<'a>
  where
    Self: 'a,
  = &'a V;

  fn get<'a>(&'a self, k: Self::Key<'a>) -> Option<Self::Value<'a>> {
    std::collections::VecDeque::get(self, k)
  }
}

#[cfg(feature = "std")]
impl<V> Get for std::collections::LinkedList<V> {
  type Key<'a>
  where
    Self: 'a,
  = usize;
  type Value<'a>
  where
    Self: 'a,
  = &'a V;

  fn get<'a>(&'a self, k: Self::Key<'a>) -> Option<Self::Value<'a>> {
    self.iter().skip(k).next()
  }
}

#[cfg(feature = "std")]
impl<K: Ord, V> Get for std::collections::BTreeMap<K, V> {
  type Key<'a>
  where
    Self: 'a,
  = &'a K;
  type Value<'a>
  where
    Self: 'a,
  = &'a V;

  fn get<'a>(&'a self, k: Self::Key<'a>) -> Option<Self::Value<'a>> {
    std::collections::BTreeMap::get(self, &k)
  }
}

#[cfg(feature = "std")]
impl<K: Ord> Get for std::collections::BTreeSet<K> {
  type Key<'a>
  where
    Self: 'a,
  = &'a K;
  type Value<'a>
  where
    Self: 'a,
  = &'a K;

  fn get<'a>(&'a self, k: Self::Key<'a>) -> Option<Self::Value<'a>> {
    std::collections::BTreeSet::get(self, &k)
  }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, V> Get for std::collections::HashMap<K, V> {
  type Key<'a>
  where
    Self: 'a,
  = &'a K;
  type Value<'a>
  where
    Self: 'a,
  = &'a V;

  fn get<'a>(&'a self, k: Self::Key<'a>) -> Option<Self::Value<'a>> {
    std::collections::HashMap::get(self, &k)
  }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash> Get for std::collections::HashSet<K> {
  type Key<'a>
  where
    Self: 'a,
  = &'a K;
  type Value<'a>
  where
    Self: 'a,
  = &'a K;

  fn get<'a>(&'a self, k: Self::Key<'a>) -> Option<Self::Value<'a>> {
    std::collections::HashSet::get(self, &k)
  }
}

#[cfg(feature = "dashmap")]
impl<K: Eq + Hash, V> Get for dashmap::DashMap<K, V> {
  type Key<'a>
  where
    Self: 'a,
  = &'a K;
  type Value<'a>
  where
    Self: 'a,
  = dashmap::mapref::one::Ref<'a, K, V>;

  fn get<'a>(&'a self, k: Self::Key<'a>) -> Option<Self::Value<'a>> {
    dashmap::DashMap::get(self, &k)
  }
}

#[cfg(feature = "dashmap")]
impl<K: Eq + Hash> Get for dashmap::DashSet<K> {
  type Key<'a>
  where
    Self: 'a,
  = &'a K;
  type Value<'a>
  where
    Self: 'a,
  = dashmap::setref::one::Ref<'a, K>;

  fn get<'a>(&'a self, k: Self::Key<'a>) -> Option<Self::Value<'a>> {
    dashmap::DashSet::get(self, &k)
  }
}

#[cfg(feature = "serde_json")]
impl Get for serde_json::Value {
  type Key<'a>
  where
    Self: 'a,
  = &'a str;
  type Value<'a>
  where
    Self: 'a,
  = &'a serde_json::Value;

  fn get<'a>(&'a self, k: Self::Key<'a>) -> Option<Self::Value<'a>> {
    match self {
      serde_json::Value::Object(o) => o.get(k),
      _ => None,
    }
  }
}

#[cfg(feature = "simd-json")]
impl Get for simd_json::BorrowedValue<'_> {
  type Key<'a>
  where
    Self: 'a,
  = &'a str;
  type Value<'a>
  where
    Self: 'a,
  = &'a simd_json::BorrowedValue<'a>;

  fn get<'a>(&'a self, k: Self::Key<'a>) -> Option<Self::Value<'a>> {
    match self {
      simd_json::BorrowedValue::Object(o) => o.get(k),
      _ => None,
    }
  }
}

#[cfg(feature = "simd-json")]
impl Get for simd_json::OwnedValue {
  type Key<'a>
  where
    Self: 'a,
  = &'a str;
  type Value<'a>
  where
    Self: 'a,
  = &'a simd_json::OwnedValue;

  fn get<'a>(&'a self, k: Self::Key<'a>) -> Option<Self::Value<'a>> {
    match self {
      simd_json::OwnedValue::Object(o) => o.get(k),
      _ => None,
    }
  }
}

#[cfg(feature = "slab")]
impl<V> Get for slab::Slab<V> {
  type Key<'a>
  where
    Self: 'a,
  = usize;
  type Value<'a>
  where
    Self: 'a,
  = &'a V;

  fn get<'a>(&'a self, k: Self::Key<'a>) -> Option<Self::Value<'a>> {
    slab::Slab::get(self, k)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn calls() {
    assert_eq!(<[()] as Get>::get(&[], 0), None);
    assert_eq!(<[usize; 0] as Get>::get(&[0; 0], 0), None);
    assert_eq!(<Vec<()> as Get>::get(&vec![], 0), None);
    assert_eq!(
      <std::collections::VecDeque<()> as Get>::get(&Default::default(), 0),
      None
    );
    assert_eq!(
      <std::collections::LinkedList<()> as Get>::get(&Default::default(), 0),
      None
    );
    assert_eq!(
      <std::collections::BTreeMap<(), ()> as Get>::get(&Default::default(), &()),
      None
    );
    assert_eq!(
      <std::collections::BTreeSet<()> as Get>::get(&Default::default(), &()),
      None
    );
    assert_eq!(
      <std::collections::HashMap<(), ()> as Get>::get(&Default::default(), &()),
      None
    );
    assert_eq!(
      <std::collections::HashSet<()> as Get>::get(&Default::default(), &()),
      None
    );
  }
}

#[cfg(test)]
#[cfg(feature = "dashmap")]
mod dashmap_tests {
  use super::*;
  #[test]
  fn dashmap_calls() {
    <dashmap::DashMap<(), ()> as Get>::get(&Default::default(), &());
    <dashmap::DashSet<()> as Get>::get(&Default::default(), &());
  }
}

#[cfg(test)]
#[cfg(feature = "serde_json")]
mod serde_json_tests {
  use super::*;
  #[test]
  fn serde_json_calls() {
    assert_eq!(
      <serde_json::Value as Get>::get(&serde_json::Value::Null, ""),
      None
    );
  }
}

#[cfg(test)]
#[cfg(feature = "simd-json")]
mod simd_json_tests {
  use super::*;
  #[test]
  fn simd_json_calls() {
    assert_eq!(
      <simd_json::BorrowedValue as Get>::get(
        &simd_json::BorrowedValue::Static(simd_json::StaticNode::Null),
        ""
      ),
      None
    );
    assert_eq!(
      <simd_json::OwnedValue as Get>::get(
        &simd_json::OwnedValue::Static(simd_json::StaticNode::Null),
        ""
      ),
      None
    );
  }
}

#[cfg(test)]
#[cfg(feature = "slab")]
mod slab_tests {
  use super::*;
  #[test]
  fn slab_calls() {
    assert_eq!(
      <slab::Slab<()> as Get>::get(&slab::Slab::<()>::new(), 0),
      None
    );
  }
}

#[cfg(test)]
#[cfg(feature = "smallvec")]
mod smallvec_tests {
  use super::*;
  #[test]
  fn smallvec_calls() {
    assert_eq!(
      <[()] as Get>::get(&smallvec::SmallVec::<[(); 0]>::new(), 0),
      None
    );
  }
}
