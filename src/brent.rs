use crate::rust_attack_utils::gcd;
use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Signed, Zero};

fn brent(n: BigInt) -> BigInt {
    if n.bit(1u64) {
        return BigInt::from(2);
    }
    let mut rng = rand::thread_rng();
    let g: BigInt = n.clone();
    while g == n {
        let max_range = n.clone() - BigInt::one();
        let mut y: BigInt = rng.gen_bigint_range(&BigInt::one(), &max_range);
        let c: BigInt = rng.gen_bigint_range(&BigInt::one(), &max_range);
        let m: BigInt = rng.gen_bigint_range(&BigInt::one(), &max_range);

        let mut g: BigInt = BigInt::one();
        let mut r: BigInt = BigInt::one();
        let mut q: BigInt = BigInt::one();

        let mut x: BigInt = y.clone();
        let mut ys: BigInt = y.clone();
        while g.is_one() {
            x = y.clone();
            let mut i: BigInt = BigInt::zero();
            while i <= r {
                y = (y.modpow(&BigInt::from(2), &n) + c.clone()) % n.clone();
                i += BigInt::one();
            }
            let mut k: BigInt = BigInt::zero();
            while k < r && g.is_one() {
                ys = y.clone();
                i = BigInt::zero();
                while i <= m.clone().min(r.clone() - k.clone()) {
                    y = (y.modpow(&BigInt::from(2), &n) + c.clone()) % n.clone();
                    q = q * ((x.clone() - y.clone()).abs()) % n.clone();
                    i += BigInt::one();
                }
                g = gcd(&q, &n);
                k = k + m.clone();
                if n > g && g > BigInt::one() {
                    return g;
                }
            }
            r = r << 1;
        }
        if g == n {
            loop {
                ys = (ys.modpow(&BigInt::from(2), &n) + c.clone()) % n.clone();
                g = gcd(&(x.clone() - ys.clone()).abs(), &n);
                if n > g && g > BigInt::one() {
                    return g;
                }
            }
        }
    }
    return BigInt::zero();
}
