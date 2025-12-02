use num_bigint::BigUint;
use crate::rational::{Rational, Sign};
use crate::rational_range::RationalRange;

pub fn factorial_biguint(x: u32) -> BigUint {
    let mut a = BigUint::from(1 as u8);

    if x < 2 {
        return a;
    }

    for i in 1..x {
        a *= i;
    }

    return a;
}

pub fn factorial(x: u32) -> Rational {
    return Rational { 
        sign: Sign::Pos, 
        numer: factorial_biguint(x), 
        denom:  BigUint::from(1 as u8)
    }
}

/*
pub fn neoprene_taylor_exp(x: &Rational, iterations: &BigUint) -> RationalRange {
    // This uses the continued fraction for e^(x/y), which I found on Wikipedia here: https://en.wikipedia.org/wiki/Continued_fraction#Examples


}

pub fn neoprene_taylor_ln() -> RationalRange {

}
*/