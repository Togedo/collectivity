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

fn insert<'a>(
  mut data: &mut impl Insert<usize, Cow<'a, str>>,
  pos: usize,
  val: Cow<'a, str>
) {
  data.insert(pos, val);
}

let mut v = vec![];
let s = "abc".into();
insert(&mut v, 0, s);
```

## Len

The `Len` trait provides the `len` method, which returns the number of entries stored within a collection.

### Examples
```rust
use collectivity::Len;

fn len(
  data: impl Len
) -> usize {
  data.len()
}
```

License: MIT
