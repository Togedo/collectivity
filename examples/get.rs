use collectivity::Get;

fn multiply_at(
  data: &impl for<'b> Get<usize, Value<'b> = &'b usize>,
  pos: usize,
  multiplier: usize,
) -> Option<usize> {
  data.get(pos).map(|v| v * multiplier)
}

fn main() {
  assert_eq!(multiply_at(&vec![0, 1, 2, 3], 0, 5), Some(0));
  assert_eq!(multiply_at(&[0, 1, 2, 3], 1, 5), Some(5));
  assert_eq!(multiply_at(&&[0, 1, 2, 3][..], 3, 5), Some(15));
}
