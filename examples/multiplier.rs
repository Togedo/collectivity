use collectivity::Get;
use core::marker::PhantomData;
use std::collections::HashSet;

pub struct Multiplier<'a, K, C: 'a + Get<K, Value<'a> = &'a usize>> {
  data: C,
  mul: usize,
  k: PhantomData<K>,
  l: PhantomData<&'a C>,
}

impl<'a, K, C: 'a + Get<K, Value<'a> = &'a usize>> Multiplier<'a, K, C> {
  pub fn new(data: C, mul: usize) -> Self {
    Self {
      data,
      mul,
      k: PhantomData,
      l: PhantomData,
    }
  }
  pub fn get(&'a self, k: K) -> Option<usize> {
    self.data.get(k).map(|v| v * self.mul)
  }
}

fn main() {
  let vec_mul = Multiplier::new(vec![1, 2, 3], 2);
  assert_eq!(vec_mul.get(0), Some(2));
  assert_eq!(vec_mul.get(2), Some(6));
  let vec_mul = Multiplier::new(vec![1, 2, 3], 2);
  assert_eq!(vec_mul.get(&0), Some(2));
  assert_eq!(vec_mul.get(&2), Some(6));
  let mut hset = HashSet::new();
  hset.insert(5);
  let hset_mul = Multiplier::new(hset, 2);
  assert_eq!(hset_mul.get(&5), Some(10));
  assert_eq!(hset_mul.get(&10), None);
}
