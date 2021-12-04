# collectivity

Generic collection traits. The crate contains definitions of various traits related to data collections, as well as their implementations for arrays, slices, and collection types from both the standard library and some popular community crates.

The goal of this project is to provide useful abstractions for working with collections that allow for decoupling their implementation details from application logic. This can make data structures interchangeable, making it easier to fine-tune performance characteristics of a program.

## Get

The `Get` trait provides safe access to a value at a specified position. The trait is generic over both keys and values of the collection, as well as their lifetimes. Due to the use of generic associated types, both the received index and the return value of the `get` method can be either borrowed or owned.

### Examples
```rust
use collectivity::Get;

fn get(
  data: impl for<'a> Get<Key<'a> = usize, Value<'a> = &'a usize>,
  pos: usize,
) -> Option<usize> {
  data.get(pos).map(|v| *v)
}
```

## Insert

The `Insert` trait provides the `insert` method, which insert a provided value at a provided index. The operation may panic, e.g. when the index is out of bounds.

### Examples
```rust
use std::borrow::Cow;
use collectivity::Insert;

fn insert<'a, V>(
  col: &mut impl Insert<usize, V>,
  pos: usize,
  val: V
) {
  col.insert(pos, val);
}

let mut v = vec![];
let s = Cow::Borrowed("abc");
insert(&mut v, 0, s);
assert_eq!(v[0], Cow::Borrowed("abc"));

let mut v = [0, 1];
insert(&mut v, 0, 1);
assert_eq!(v[0], 1);
```

## Len

The `Len` trait provides the `len` method, which returns the number of entries stored within a collection.

### Examples
```rust
use collectivity::Len;

fn len(
  col: &impl Len
) -> usize {
  col.len()
}

assert_eq!(len(&std::collections::HashSet::<()>::new()), 0);
assert_eq!(len(&vec![1, 2, 3]), 3);
```

## Push

The `Push` trait provides the `push` method, which adds the provided value to a collection.

### Examples
```rust
use std::collections::LinkedList;
use collectivity::{Get, Push};

fn push(
  col: &mut impl Push<i32>,
  v: i32
) {
  col.push(v);
}

let mut v = vec![];
push(&mut v, 0);
assert_eq!(v[0], 0);

let mut l = LinkedList::new();
push(&mut l, 0);
assert_eq!(l.get(0), Some(&0));
```

License: MIT
