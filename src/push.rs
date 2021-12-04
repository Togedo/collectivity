#[cfg(feature = "std")]
use std::collections::{BinaryHeap, LinkedList, VecDeque};

pub trait Push<V> {
  fn push(&mut self, v: V);
}

impl<'p, V, P: Push<V>> Push<V> for &'p mut P {
  fn push<'a>(&'a mut self, v: V) {
    <P as Push<V>>::push(self, v)
  }
}
impl<V> Push<V> for Vec<V> {
  fn push(&mut self, v: V) {
    self.push(v)
  }
}

#[cfg(feature = "std")]
impl<V> Push<V> for VecDeque<V> {
  fn push(&mut self, v: V) {
    VecDeque::push_back(self, v)
  }
}

#[cfg(feature = "std")]
impl<V> Push<V> for LinkedList<V> {
  fn push(&mut self, v: V) {
    self.push_back(v)
  }
}

#[cfg(feature = "serde_json")]
use serde_json::Value as SeV;

#[cfg(feature = "serde_json")]
impl Push<SeV> for SeV {
  fn push(&mut self, v: SeV) {
    match self {
      SeV::Array(o) => {
        o.push(v);
      }
      _ => panic!("Value is not an array"),
    }
  }
}

#[cfg(feature = "simd-json")]
use simd_json::{BorrowedValue as SBV, OwnedValue as SOV};

#[cfg(feature = "simd-json")]
impl<'a> Push<SBV<'a>> for SBV<'a> {
  fn push(&mut self, v: SBV<'a>) {
    match self {
      SBV::Array(a) => {
        a.push(v);
      }
      _ => panic!("Value is not an array"),
    }
  }
}

#[cfg(feature = "simd-json")]
impl Push<SOV> for SOV {
  fn push(&mut self, v: SOV) {
    match self {
      SOV::Array(a) => {
        a.push(v);
      }
      _ => panic!("Value is not an array"),
    }
  }
}

#[cfg(feature = "slab")]
use slab::Slab;

#[cfg(feature = "slab")]
impl<V> Push<V> for Slab<V> {
  fn push(&mut self, v: V) {
    self.insert(v);
  }
}

#[cfg(feature = "smallvec")]
use smallvec::{Array, SmallVec};

#[cfg(feature = "smallvec")]
impl<V, A: Array<Item = V>> Push<V> for SmallVec<A> {
  fn push(&mut self, v: V) {
    self.push(v)
  }
}
