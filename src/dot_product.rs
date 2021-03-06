// Implements http://rosettacode.org/wiki/Dot_product

use std::ops::{Add, Mul};
use std::num::Zero;

fn dotp<'a, T:Add<T, T> + Mul<T, T> + Zero + Copy>(this: &'a [T], other: &'a [T]) -> T {
  assert!(this.len() == other.len(), "The dimensions must be equal");

  let zero : T = Zero::zero();
  this.iter().zip(other.iter())
             .map(|(&a, &b)| a * b)
             .fold(zero, |sum, n| sum + n)
}

#[cfg(not(test))]
fn main() {
    let a = &[1.0, 3.0, -5.0];
    let b = &[4.0, -2.0, -1.0];
    println!("{}", dotp(a, b));
}

#[test]
fn test_dotp() {
  let result = dotp(&[1, 3, -5], &[4, -2, -1]);
  assert_eq!(result, 3);
}
