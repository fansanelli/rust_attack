use num_bigint::{BigInt};
use num_traits::{Zero, One};
use std::ops::Add;

#[inline]
pub fn gcd(x: &BigInt, y: &BigInt) -> BigInt {
    // Use Euclid's algorithm
    let mut m = x.clone();
    let mut n = y.clone();
    while !m.is_zero() {
        let temp = m;
        m = n % &temp;
        n = temp;
    }
    n
}

#[inline]
pub fn is_congruent(a: &BigInt, b: &BigInt, m: &BigInt) -> bool {
    ((a.clone() - b.clone()) % m.clone()).is_zero()
}

#[inline]
pub fn is_square(n: &BigInt) -> bool {
    let root = isqrt(&n);
    *n == root.clone() * root
}

#[test]
fn is_square_works() {
  assert_eq!(is_square(&BigInt::from(9)), true);
  assert_eq!(is_square(&BigInt::from(10)), false);
}

#[inline]
pub fn isqrt_rem(n: &BigInt) -> (BigInt, BigInt) {
    let i2 = isqrt(&n);
    (i2.clone(), n - (i2.clone() * i2))
}

#[test]
fn isqrt_rem_works() {
  let res = isqrt_rem(&BigInt::from(10));
  assert_eq!(res.0, BigInt::from(3));
  assert_eq!(res.1, BigInt::one());
}

#[inline]
pub fn isqrt(n: &BigInt) -> BigInt {
    if n.is_zero() {
        return BigInt::zero();
    }
    let mut x: BigInt = n.clone();
    let mut y: BigInt = (n.clone() + BigInt::one()) >> 1;
    while y < x {
    	x = y.clone();
    	y = (y.clone() + n.clone() / y) >> 1;
    }
    x
}

#[test]
fn isqrt_works() {
  assert_eq!(isqrt(&BigInt::from(9)), BigInt::from(3));
  assert_eq!(isqrt(&BigInt::from(10)), BigInt::from(3));
}

