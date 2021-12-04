#[cfg(feature = "std")]
use std::{
  collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
  hash::Hash,
};

macro_rules! insert {
  ([$($l:lifetime, )*$($x:ident$(: $bound:tt $(+ $others:tt )*)?),*], $t:ty, $k:ty, $v:ty, $b:item) => {
    impl<$($l, )*$($x$(: $bound $(+ $others)*)?),*> Insert for $t {
      type Key<'a>
      where
        Self: 'a,
      = $k;
      type Value<'a>
      where
        Self: 'a,
      = $v;
      $b
    }
  };
}

macro_rules! basic_insert {
  ($t:ty, $k:ty) => {
    insert!(
      [V],
      $t,
      $k,
      V,
      fn insert<'a>(&'a mut self, k: $k, v: V) {
        <$t>::insert(self, k, v)
      }
    );
  };
}

pub trait Insert {
  type Key<'a>
  where
    Self: 'a;
  type Value<'a>
  where
    Self: 'a;
  fn insert<'a>(&'a mut self, k: Self::Key<'a>, v: Self::Value<'a>);
}

impl<'i, I: Insert> Insert for &'i mut I {
  type Key<'a>
  where
    Self: 'a,
  = <I as Insert>::Key<'a>;
  type Value<'a>
  where
    Self: 'a,
  = <I as Insert>::Value<'a>;

  fn insert<'a>(&'a mut self, k: Self::Key<'a>, v: Self::Value<'a>) {
    <I as Insert>::insert(self, k, v)
  }
}

impl<V> Insert for [V] {
  type Key<'a>
  where
    Self: 'a,
  = usize;
  type Value<'a>
  where
    Self: 'a,
  = V;

  fn insert<'a>(&'a mut self, k: usize, v: V) {
    self[k] = v
  }
}

basic_insert!(Vec<V>, usize);

#[cfg(feature = "std")]
basic_insert!(VecDeque<V>, usize);

#[cfg(feature = "std")]
insert!(
  [V],
  LinkedList<V>,
  usize,
  V,
  fn insert<'a>(&'a mut self, k: usize, v: V) {
    let mut rest = self.split_off(k);
    self.push_back(v);
    self.append(&mut rest);
  }
);

#[cfg(feature = "std")]
insert!(
  [K: Ord, V],
  BTreeMap<K, V>,
  K,
  V,
  fn insert<'a>(&'a mut self, k: K, v: V) {
    BTreeMap::insert(self, k, v);
  }
);

#[cfg(feature = "std")]
insert!(
  [K: Ord],
  BTreeSet<K>,
  K,
  (),
  fn insert<'a>(&'a mut self, k: K, _v: ()) {
    BTreeSet::insert(self, k);
  }
);

#[cfg(feature = "std")]
insert!(
  [K: Eq + Hash, V],
  HashMap<K, V>,
  K,
  V,
  fn insert<'a>(&'a mut self, k: K, v: V) {
    HashMap::insert(self, k, v);
  }
);

#[cfg(feature = "std")]
insert!(
  [K: Eq + Hash],
  HashSet<K>,
  K,
  (),
  fn insert<'a>(&'a mut self, k: K, _v: ()) {
    HashSet::insert(self, k);
  }
);

#[cfg(feature = "dashmap")]
use dashmap::{DashMap, DashSet};

#[cfg(feature = "dashmap")]
insert!(
  [K: Eq + Hash, V],
  DashMap<K, V>,
  K,
  V,
  fn insert<'a>(&'a mut self, k: K, v: V) {
    DashMap::insert(self, k, v);
  }
);

#[cfg(feature = "dashmap")]
insert!(
  [K: Eq + Hash],
  DashSet<K>,
  K,
  (),
  fn insert<'a>(&'a mut self, k: K, _v: ()) {
    DashSet::insert(self, k);
  }
);

#[cfg(feature = "serde_json")]
insert!(
  [],
  serde_json::Value,
  String,
  serde_json::Value,
  fn insert<'a>(&'a mut self, k: String, v: serde_json::Value) {
    match self {
      serde_json::Value::Object(o) => {
        o.insert(k, v);
      }
      _ => panic!("Value is not an object"),
    }
  }
);

#[cfg(feature = "simd-json")]
insert!(
  [],
  simd_json::OwnedValue,
  String,
  simd_json::OwnedValue,
  fn insert<'a>(&'a mut self, k: String, v: simd_json::OwnedValue) {
    match self {
      simd_json::OwnedValue::Object(o) => {
        o.insert(k, v);
      }
      _ => panic!("Value is not an object"),
    }
  }
);

#[cfg(feature = "smallvec")]
use smallvec::{Array, SmallVec};

#[cfg(feature = "smallvec")]
impl<V, A: Array<Item = V>> Insert for SmallVec<A> {
  type Key<'a>
  where
    Self: 'a,
  = usize;
  type Value<'a>
  where
    Self: 'a,
  = V;

  fn insert<'a>(&'a mut self, k: usize, v: V) {
    SmallVec::insert(self, k, v)
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
    <Vec<i32> as Insert>::insert(&mut v, 0, 1);
    assert_eq!(v[0], 1);
    let mut v = VecDeque::new();
    <VecDeque<i32> as Insert>::insert(&mut v, 0, 1);
    assert_eq!(v[0], 1);
    let mut v = LinkedList::new();
    <LinkedList<i32> as Insert>::insert(&mut v, 0, 1);
    <LinkedList<i32> as Insert>::insert(&mut v, 1, 2);
    <LinkedList<i32> as Insert>::insert(&mut v, 1, 3);
    assert_eq!(v.get(0), Some(&1));
    assert_eq!(v.get(1), Some(&3));
    assert_eq!(v.get(2), Some(&2));
    let mut m = BTreeMap::new();
    <BTreeMap<i32, i32> as Insert>::insert(&mut m, 0, 1);
    assert_eq!(m[&0], 1);
    let mut s = BTreeSet::new();
    <BTreeSet<i32> as Insert>::insert(&mut s, 0, ());
    assert_eq!(s.get(&0), Some(&0));
    let mut m = HashMap::new();
    <HashMap<i32, i32> as Insert>::insert(&mut m, 0, 1);
    assert_eq!(m[&0], 1);
    let mut s = HashSet::new();
    <HashSet<i32> as Insert>::insert(&mut s, 0, ());
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
    <DashMap<i32, i32> as Insert>::insert(&mut m, 0, 1);
    assert_eq!(m.get(&0).map(|v| v.clone()), Some(1));
    let mut s = DashSet::new();
    <DashSet<i32> as Insert>::insert(&mut s, 0, ());
    assert_eq!(s.get(&0).map(|v| v.clone()), Some(0));
  }
}

#[cfg(test)]
#[cfg(feature = "serde_json")]
mod serde_json_tests {
  use super::*;
  #[test]
  fn serde_json() {
    let mut o = serde_json::Value::Object(Default::default());
    <serde_json::Value as Insert>::insert(&mut o, "a".into(), serde_json::Value::Null);
    assert_eq!(o.get("a"), Some(&serde_json::Value::Null));
    // let mut n = serde_json::Value::Null;
    // <serde_json::Value as Insert>::insert(&mut n, "a".into(), serde_json::Value::Null);
    // assert_eq!(n.get("a"), None);
  }
}

#[cfg(test)]
#[cfg(feature = "simd-json")]
mod simd_json_tests {
  use super::*;
  use crate::Get;
  #[test]
  fn simd_json() {
    let mut o = simd_json::OwnedValue::Object(Default::default());
    <simd_json::OwnedValue as Insert>::insert(
      &mut o,
      "a".into(),
      simd_json::OwnedValue::Static(simd_json::StaticNode::Null),
    );
    assert_eq!(
      o.get("a"),
      Some(&simd_json::OwnedValue::Static(simd_json::StaticNode::Null))
    );
  }
}

#[cfg(test)]
#[cfg(feature = "smallvec")]
mod smallvec_tests {
  use super::*;
  #[test]
  fn smallvec() {
    let mut sv = smallvec::SmallVec::<[i32; 2]>::new();
    <SmallVec<[i32; 2]> as Insert>::insert(&mut sv, 0, 1);
    assert_eq!(sv[0], 1);
  }
}
