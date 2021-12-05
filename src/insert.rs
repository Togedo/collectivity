use crate::{Safe, SafetyMarker, Unsafe};
#[cfg(feature = "std")]
use std::{
  collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
  hash::Hash,
};

/// Provides the ability to insert a provided value at a specified index, possibly overwriting the previous value.
///
/// ## Examples
/// ```
/// use std::borrow::Cow;
/// use collectivity::Insert;
///
/// fn insert<'a, V>(
///   col: &mut impl Insert<usize, V>,
///   pos: usize,
///   val: V
/// ) {
///   col.insert(pos, val);
/// }
///
/// let mut v = vec![];
/// let s = Cow::Borrowed("abc");
/// insert(&mut v, 0, s);
/// assert_eq!(v[0], Cow::Borrowed("abc"));
///
/// let mut v = [0, 1];
/// insert(&mut v, 0, 1);
/// assert_eq!(v[0], 1);
/// ```
pub trait Insert<K, V> {
  /// Indicates whether the `insert` method may panic in a particular implementation.
  type Safety: SafetyMarker;
  /// Inserts value `v` at key `k`.
  ///
  /// # Panics
  ///
  /// May panic, e.g. when the index is out of bounds.
  fn insert(&mut self, k: K, v: V);
}

impl<'i, K, V, I: Insert<K, V>> Insert<K, V> for &'i mut I {
  type Safety = <I as Insert<K, V>>::Safety;
  fn insert<'a>(&'a mut self, k: K, v: V) {
    <I as Insert<K, V>>::insert(self, k, v)
  }
}

impl<V, const N: usize> Insert<usize, V> for [V; N] {
  type Safety = Unsafe;
  fn insert(&mut self, k: usize, v: V) {
    self[k] = v
  }
}

impl<V> Insert<usize, V> for [V] {
  type Safety = Unsafe;
  fn insert(&mut self, k: usize, v: V) {
    self[k] = v
  }
}

impl<V> Insert<usize, V> for Vec<V> {
  type Safety = Unsafe;
  fn insert(&mut self, k: usize, v: V) {
    self.insert(k, v)
  }
}

#[cfg(feature = "std")]
impl<V> Insert<usize, V> for VecDeque<V> {
  type Safety = Unsafe;
  fn insert(&mut self, k: usize, v: V) {
    self.insert(k, v)
  }
}

#[cfg(feature = "std")]
impl<V> Insert<usize, V> for LinkedList<V> {
  type Safety = Unsafe;
  fn insert(&mut self, k: usize, v: V) {
    let mut rest = self.split_off(k);
    self.push_back(v);
    self.append(&mut rest);
  }
}

#[cfg(feature = "std")]
impl<K: Ord, V> Insert<K, V> for BTreeMap<K, V> {
  type Safety = Safe;
  fn insert(&mut self, k: K, v: V) {
    self.insert(k, v);
  }
}

#[cfg(feature = "std")]
impl<K: Ord> Insert<K, ()> for BTreeSet<K> {
  type Safety = Safe;
  fn insert(&mut self, k: K, _v: ()) {
    self.insert(k);
  }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, V> Insert<K, V> for HashMap<K, V> {
  type Safety = Safe;
  fn insert(&mut self, k: K, v: V) {
    self.insert(k, v);
  }
}

#[cfg(feature = "std")]
impl<K: Eq + Hash> Insert<K, ()> for HashSet<K> {
  type Safety = Safe;
  fn insert(&mut self, k: K, _v: ()) {
    self.insert(k);
  }
}

#[cfg(feature = "dashmap")]
use dashmap::{DashMap, DashSet};

#[cfg(feature = "dashmap")]
impl<K: Eq + Hash, V> Insert<K, V> for DashMap<K, V> {
  type Safety = Safe;
  fn insert(&mut self, k: K, v: V) {
    DashMap::insert(self, k, v);
  }
}

#[cfg(feature = "dashmap")]
impl<K: Eq + Hash> Insert<K, ()> for DashSet<K> {
  type Safety = Safe;
  fn insert(&mut self, k: K, _v: ()) {
    DashSet::insert(self, k);
  }
}

#[cfg(feature = "serde_json")]
use serde_json::Value as SeV;

#[cfg(feature = "serde_json")]
impl Insert<usize, SeV> for SeV {
  type Safety = Unsafe;
  fn insert(&mut self, k: usize, v: SeV) {
    match self {
      SeV::Array(a) => {
        a.insert(k, v);
      }
      _ => panic!("Value is not an array"),
    }
  }
}

#[cfg(feature = "serde_json")]
impl Insert<String, SeV> for SeV {
  type Safety = Unsafe;
  fn insert(&mut self, k: String, v: SeV) {
    match self {
      SeV::Object(o) => {
        o.insert(k, v);
      }
      _ => panic!("Value is not an object"),
    }
  }
}

#[cfg(feature = "simd-json")]
use simd_json::{cow::Cow, BorrowedValue as SBV, OwnedValue as SOV};

#[cfg(feature = "simd-json")]
impl<'a> Insert<usize, SBV<'a>> for SBV<'a> {
  type Safety = Unsafe;
  fn insert(&mut self, k: usize, v: SBV<'a>) {
    match self {
      SBV::Array(a) => {
        a.insert(k, v);
      }
      _ => panic!("Value is not an array"),
    }
  }
}

#[cfg(feature = "simd-json")]
impl<'a> Insert<Cow<'a, str>, SBV<'a>> for SBV<'a> {
  type Safety = Unsafe;
  fn insert(&mut self, k: Cow<'a, str>, v: SBV<'a>) {
    match self {
      SBV::Object(o) => {
        o.insert(k, v);
      }
      _ => panic!("Value is not an object"),
    }
  }
}

#[cfg(feature = "simd-json")]
impl Insert<usize, SOV> for SOV {
  type Safety = Unsafe;
  fn insert(&mut self, k: usize, v: SOV) {
    match self {
      SOV::Array(a) => {
        a.insert(k, v);
      }
      _ => panic!("Value is not an array"),
    }
  }
}

#[cfg(feature = "simd-json")]
impl Insert<String, SOV> for SOV {
  type Safety = Safe;
  fn insert(&mut self, k: String, v: SOV) {
    match self {
      SOV::Object(o) => {
        o.insert(k, v);
      }
      _ => panic!("Value is not an object"),
    }
  }
}

#[cfg(feature = "smallvec")]
use smallvec::{Array, SmallVec};

#[cfg(feature = "smallvec")]
impl<V, A: Array<Item = V>> Insert<usize, V> for SmallVec<A> {
  type Safety = Unsafe;
  fn insert(&mut self, k: usize, v: V) {
    self.insert(k, v)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Get;
  #[test]
  fn std() {
    let mut v = [0, 1, 2];
    v.insert(0, 1);
    assert_eq!(v[0], 1);
    let v = &mut [0, 1, 2][..];
    v.insert(0, 1);
    assert_eq!(v[0], 1);
    let mut v = vec![];
    <Vec<i32> as Insert<usize, i32>>::insert(&mut v, 0, 1);
    assert_eq!(v[0], 1);
    let mut v = VecDeque::new();
    <VecDeque<i32> as Insert<usize, i32>>::insert(&mut v, 0, 1);
    assert_eq!(v[0], 1);
    let mut v = LinkedList::new();
    <LinkedList<i32> as Insert<usize, i32>>::insert(&mut v, 0, 1);
    <LinkedList<i32> as Insert<usize, i32>>::insert(&mut v, 1, 2);
    <LinkedList<i32> as Insert<usize, i32>>::insert(&mut v, 1, 3);
    assert_eq!(v.get(0), Some(&1));
    assert_eq!(v.get(1), Some(&3));
    assert_eq!(v.get(2), Some(&2));
    let mut m = BTreeMap::new();
    <BTreeMap<i32, i32> as Insert<i32, i32>>::insert(&mut m, 0, 1);
    assert_eq!(m[&0], 1);
    let mut s = BTreeSet::new();
    <BTreeSet<i32> as Insert<i32, ()>>::insert(&mut s, 0, ());
    assert_eq!(s.get(&0), Some(&0));
    let mut m = HashMap::new();
    <HashMap<i32, i32> as Insert<i32, i32>>::insert(&mut m, 0, 1);
    assert_eq!(m[&0], 1);
    let mut s = HashSet::new();
    <HashSet<i32> as Insert<i32, ()>>::insert(&mut s, 0, ());
    assert_eq!(s.get(&0), Some(&0));
  }
}

#[cfg(test)]
#[cfg(feature = "dashmap")]
mod dashmap_tests {
  use super::*;
  #[test]
  fn dashmap() {
    let mut m = DashMap::new();
    <DashMap<i32, i32> as Insert<i32, i32>>::insert(&mut m, 0, 1);
    assert_eq!(m.get(&0).map(|v| v.clone()), Some(1));
    let mut s = DashSet::new();
    <DashSet<i32> as Insert<i32, ()>>::insert(&mut s, 0, ());
    assert_eq!(s.get(&0).map(|v| v.clone()), Some(0));
  }
}

#[cfg(test)]
#[cfg(feature = "serde_json")]
mod serde_json_tests {
  use super::*;
  #[test]
  fn serde_json() {
    let mut a = SeV::Array(Default::default());
    <SeV as Insert<usize, SeV>>::insert(&mut a, 0, SeV::Null);
    assert_eq!(a.get(0), Some(&SeV::Null));
    let mut o = SeV::Object(Default::default());
    <SeV as Insert<String, SeV>>::insert(&mut o, "a".into(), SeV::Null);
    assert_eq!(o.get("a"), Some(&SeV::Null));
  }
}

#[cfg(test)]
#[cfg(feature = "simd-json")]
mod simd_json_tests {
  use super::*;
  use crate::Get;
  #[test]
  fn simd_json_borrowed() {
    let mut a = SBV::Array(Default::default());
    <SBV as Insert<usize, SBV>>::insert(&mut a, 0_usize, SBV::Static(simd_json::StaticNode::Null));
    assert_eq!(
      <simd_json::BorrowedValue as Get<usize>>::get(&a, 0),
      Some(&SBV::Static(simd_json::StaticNode::Null))
    );
    let mut o = SBV::Object(Default::default());
    <SBV as Insert<Cow<'_, str>, SBV>>::insert(
      &mut o,
      "a".into(),
      SBV::Static(simd_json::StaticNode::Null),
    );
    assert_eq!(o.get("a"), Some(&SBV::Static(simd_json::StaticNode::Null)));
  }
  #[test]
  fn simd_json_owned() {
    let mut a = SOV::Array(Default::default());
    <SOV as Insert<usize, SOV>>::insert(&mut a, 0, SOV::Static(simd_json::StaticNode::Null));
    assert_eq!(
      <simd_json::OwnedValue as Get<usize>>::get(&a, 0),
      Some(&SOV::Static(simd_json::StaticNode::Null))
    );
    let mut o = SOV::Object(Default::default());
    <SOV as Insert<String, SOV>>::insert(
      &mut o,
      "a".into(),
      SOV::Static(simd_json::StaticNode::Null),
    );
    assert_eq!(o.get("a"), Some(&SOV::Static(simd_json::StaticNode::Null)));
  }
}

#[cfg(test)]
#[cfg(feature = "smallvec")]
mod smallvec_tests {
  use super::*;
  #[test]
  fn smallvec() {
    let mut sv = smallvec::SmallVec::<[i32; 2]>::new();
    <SmallVec<[i32; 2]> as Insert<usize, i32>>::insert(&mut sv, 0, 1);
    assert_eq!(sv[0], 1);
  }
}
