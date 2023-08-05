use num_bigint::BigInt;
use num_traits::Zero;

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

