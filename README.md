# collectivity

Generic collection traits.

## Get

The `Get` trait provides safe access to a value at a specified position. The trait is generic over both keys and values of the collection, as well as their lifetimes. Due to the use of generic associated types, both the received index and the return value of the `get` method can be either borrowed or owned.

### Examples
```rust
use collectivity::Get;

fn multiply_at(
  data: impl for<'b> Get<Key<'b> = usize, Value<'b> = &'b usize>,
  pos: usize,
  multiplier: usize,
) -> Option<usize> {
  data.get(pos).map(|v| v * multiplier)
}
```

## Len

The `Len` trait provides the `len` method, which returns the number of entries within a collection.

### Examples
```rust
use collectivity::Len;

fn check_len(
  data: impl Len
) -> usize {
  data.len()
}
```

License: MIT
