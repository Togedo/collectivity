#![feature(box_syntax)]

use collectivity::{nosafety::Insert, Len};
use std::{
  collections::{BTreeMap, HashMap, VecDeque},
  time::Instant,
};

pub trait MyTraitSelection<K, V>: Insert<K, V> + Len {}

impl<K, V, C: Insert<K, V> + Len> MyTraitSelection<K, V> for C {}

fn main() {
  const N: usize = 10_000_000;
  let data = (0..N).map(|n| (n, n)).collect::<Vec<_>>();
  let collections: &mut [(&str, Box<dyn MyTraitSelection<_, _>>)] = &mut [
    ("Array", box [0_usize; N] as _),
    ("Vec", box vec![] as _),
    ("VecDeque", box VecDeque::new() as _),
    ("BTreeMap", box BTreeMap::new() as _),
    ("HashMap", box HashMap::new() as _),
  ];
  collections.iter_mut().for_each(|(name, c)| {
    let t = Instant::now();
    data.iter().for_each(|(k, v)| c.insert(*k, *v));
    println!(
      "{:<10}: inserted in {:<15}, len: {}",
      name,
      format!("{:#?}", t.elapsed()),
      c.len()
    );
  });
}
