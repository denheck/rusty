fn main() {
  // "as" is used for type casting at compile time
  let x: f64 = 4.0;
  let y: uint = x as uint;
  assert!(y == 4u);
}
