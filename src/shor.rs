use crate::rust_attack_utils::gcd;
use num_bigint::BigInt;
use num_traits::{One, Zero};

pub fn shor(n: BigInt) -> BigInt {
    /*
    Shor's algorithm: only the classical part of it, implemented in a very naive and linear way.
    Use the quantum period finding function: f(x) = a^x % N to find r, then a^r == 1 (mod N) and that is what the quantum computer
    gives advantage over classical algorithms.
    Here in this code we use a linear search of r of even numbers.
    Equivalent to solving DLP with bruteforce.
    https://en.wikipedia.org/wiki/Shor%27s_algorithm
    */
    for a in num_iter::range_inclusive(BigInt::from(2), n.clone()) {
        let g = gcd(&n, &a);
        if g.is_one() {
            for r in num_iter::range_step(BigInt::from(2), n.clone(), BigInt::from(2)) {
                let ar = a.modpow(&r, &n);
                if ar.is_one() {
                    let ar2 = a.modpow(&(r >> 1), &n);
                    if ar2 != -BigInt::one() {
                        let g1 = gcd(&(ar2.clone() - 1), &n);
                        let g2 = gcd(&(ar2 + 1), &n);
                        if (n > g1 && g1 > BigInt::one()) || (n > g2 && g2 > BigInt::one()) {
                            return n.clone().min(g1).max(BigInt::one()).max(n.min(g2).max(BigInt::one()));
                        }
                    }
                }
            }
        } else {
            return g;
        }
    }
    return BigInt::zero();
}

#[test]
fn shor_works() {
    assert_eq!(shor(BigInt::parse_bytes(b"34506963109", 10).unwrap()), BigInt::parse_bytes(b"263209", 10).unwrap());
}

