use crate::rust_attack_utils::{isqrt_rem, is_congruent, is_square, isqrt};
use num_bigint::BigInt;
use num_traits::{One, Zero};

pub fn fermat(n: BigInt) -> BigInt {
    if is_congruent(&n, &BigInt::from(2), &BigInt::from(4)) {
        return BigInt::zero();
    }
    let a_rem = isqrt_rem(&n);
    let mut b2 = -a_rem.1;
    let c0 = (a_rem.0 << 1u32) + BigInt::one();
    let mut c = c0;
    while !is_square(&b2) {
        b2 += c.clone();
        c += BigInt::from(2);
    }
    let a = (c - BigInt::one()) >> 1u32;
    let b = isqrt(&b2);
    return a - b;
}

#[test]
fn fermat_works() {
    assert_eq!(
        fermat(BigInt::parse_bytes(b"94738740796943840961823530695778701408987757287583492665919730017973847138345511139064596113422435977583856843887008168202003855906708039013487349390571801141407245039011598810542232029634564848797998534872251549660454277336502838185642937637576121533945369150901808833844341421315429263207612372324026271327", 10).unwrap()),
        BigInt::parse_bytes(b"9733382803370256893136109840971590971460094779242334919432347801491641617443615856221168611138933576118196795282443503609663168324106758595642231987245583", 10).unwrap()
    );
}

