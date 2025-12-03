use num_bigint::BigUint;

use crate::byzantine::{ByzNode, TransitiveConsts};
use crate::rational::{Rational, Sign};
use crate::rational_range::{RationalRange, RationalRangeDescriptor, get_descriptor};
use crate::byznode_sorted_vec::{ByzNodeCoefficientAddVec, ByzNodePowerMulVec, ByzNodeVec};
use crate::neoprene_taylor;

/// Panics when given a value that cannot fit in a u32
fn biguint_to_u32(x: &BigUint) -> u32 {
    let a = x.to_u32_digits();

    if a.len() != 1 {
        panic!("Attempted to run biguint_to_u32(..) on a BigUint that cannot be expressed as a u32");
    }

    return a[0];
}

pub fn neoprene_transitive(transitive_const: TransitiveConsts, newton_iterations: &BigUint) -> RationalRange {
    // These are perfectly accurate and do not need to be fixed later
    match transitive_const {
        TransitiveConsts::Pi => {
            // Using the Gregory-Leibniz series

            let k = biguint_to_u32(newton_iterations);

            let mut a = Rational::from(3);
            let b: Rational;

            for n in 1..(k+1) {
                let mut c = BigUint::from(2*n);
                c *= 2*n+1;
                c *= 2*n+2;
                let d = Rational {
                    sign: if n%2 == 1 { Sign::Pos } else  { Sign::Neg },
                    numer: BigUint::from(4 as u8),
                    denom: c
                };

                a += &d;
            }

            // Gregory-Leibniz alternates between an underapproximation and an overapproximation,
            // So I need to use both the 2nd-to-last and the last value
            b = a.clone();

            let mut c = BigUint::from(2*k+2);
            c *= 2*k+3;
            c *= 2*k+4;
            let d = Rational {
                sign: if k%2 == 0 { Sign::Pos } else  { Sign::Neg },
                numer: BigUint::from(4 as u8),
                denom: c
            };
            a += &d;

            // Gregory-Leibniz alternates between an underapproximation and an overapproximation,
            // So which approximation is min or max depends on if k is odd or even
            let min: Rational;
            let max: Rational;
            if k%2 == 1 {
                min = a;
                max = b;
            } else {
                min = b;
                max = a;
            }

            return RationalRange { min, max };
        },
        TransitiveConsts::Euler => {
            // Using the taylor expansion of e^x evaluated at x=1, meaning this is just the sum of the inverses of the factorials up to k

            let k = biguint_to_u32(newton_iterations);

            let mut min = Rational::from(2);

            // Note that addition of rationals is quite expensive, and there is definitely a way I can speed this up with the knowledge of how subsequent terms will add
            for n in 2..(k+3) {
                let mut a = neoprene_taylor::factorial(n);
                a.invert();
                min += &a; 
            }

            let mut max = min.clone();
            // Error term is given by e/(k+1)!
            // I use the current approximation of e as the numerator, which seems to work just fine even if it results in an underestimate on the error term
            let mut error = min.clone();
            error.denom *= neoprene_taylor::factorial_biguint(k+3);
            error.simplify();
            max += &error;

            return RationalRange { min, max }
        }
    }
}

pub fn neoprene_add(addends: &ByzNodeCoefficientAddVec, newton_iterations: &BigUint) -> RationalRange {
    let rat = addends.get_rational_part();
    let vec = addends.get_vec();

    let mut range = RationalRange {
        min: rat.clone(),
        max: rat.clone()
    };

    for i in vec {
        let mut i_range = neoprene_byznode(i.1.as_ref(), newton_iterations);
        i_range.min *= &i.0;
        i_range.max *= &i.0;

        range += &i_range;
    }

    return range;
}

pub fn neoprene_mul(products: &ByzNodePowerMulVec, newton_iterations: &BigUint) -> RationalRange {
    let rat = products.get_rational_part();
    let vec = products.get_vec();

    let mut range = RationalRange {
        min: rat.clone(),
        max: rat.clone()
    };

    for i in vec {
        let mut i_range = neoprene_byznode(i.1.as_ref(), newton_iterations);
        
        neoprene_range_pow_raw(&mut i_range, &i.0, newton_iterations);

        range *= &i_range;
    }

    return range;
}

pub fn neoprene_pow(byznode: &ByzNode, exp: &Rational, newton_iterations: &BigUint) -> RationalRange {
    let mut range = neoprene_byznode(byznode, newton_iterations);
    neoprene_range_pow_raw(&mut range, exp, newton_iterations);
    return range;
}

pub fn neoprene_range_pow_raw(range: &mut RationalRange, exp: &Rational, newton_iterations: &BigUint) {
    match get_descriptor(range) {
        RationalRangeDescriptor::BothPos => {
            neoprene_rational_pow_raw(&mut range.min, exp, newton_iterations);
            neoprene_rational_pow_raw(&mut range.max, exp, newton_iterations);
        },
        RationalRangeDescriptor::BothNeg => {
            neoprene_rational_pow_raw(&mut range.min, exp, newton_iterations);
            neoprene_rational_pow_raw(&mut range.max, exp, newton_iterations);
            if range.min > range.max {
                std::mem::swap(&mut range.min, &mut range.max);
            }
        },
        RationalRangeDescriptor::OverlapZero => {
            neoprene_rational_pow_raw(&mut range.min, exp, newton_iterations);
            neoprene_rational_pow_raw(&mut range.max, exp, newton_iterations);
            if range.min > range.max {
                std::mem::swap(&mut range.min, &mut range.max);
            }
            if !range.min.is_negative() && !range.max.is_negative() {
                // Consider (-1..2)^2, which can range between (0..4)
                range.min = Rational::zero();
            }
        }
    }
}

pub fn neoprene_rational_pow_raw(rational: &mut Rational, exp: &Rational, newton_iterations: &BigUint) {
    // This function will assume all the rationals are simplified...
    if !exp.is_simplified() {
        panic!("Attempted to compute neoprene_rational_pow_raw(..) with an unsimplified exp");
    }
    if exp.is_zero() {
        panic!("Attempted to compute neoprene_rational_pow_raw(..) with an exp of 0");
    }
    if exp.is_one() {
        panic!("Attempted to compute neoprene_rational_pow_raw(..) with an exp of 1");
    }
    if exp > &Rational::from(8) {
        panic!("Attempted to compute neoprene_rational_pow_raw(..) with an exp larger than 8")
    }

    if rational.is_negative() && exp.is_denom_odd() {
        // If base is negative, exp.denom must be odd to have a real root
        panic!("Attempted to compute exact_rational_pow(..) in such a way that a complex number would be produced");
    }

    let a: f64 = 0.5;
    let b: f64 = 1.5;
    let c: f64 = a.powf(b);

    if exp.is_int() {
        rational.powi(&exp.numer);
        if exp.is_negative() {
            rational.invert();
        }
        return;
    }

    // There are more exact answers after this point (consider (9/4)^(1/2)), but I'm too lazy to implement all of them
    // Begin approximation zone
    todo!();
}

pub fn neoprene_byznode(byznode: &ByzNode, newton_iterations: &BigUint) -> RationalRange {
    match byznode {
        ByzNode::TransitiveConst {transitive_const} => {
            return neoprene_transitive(*transitive_const, &newton_iterations);
        },
        ByzNode::Add { addends } => {
            return neoprene_add(addends, newton_iterations);
        },
        ByzNode::Mul { products } => {
            return neoprene_mul(products, newton_iterations);
        },
        ByzNode::Pow { base, exp } => {
            return neoprene_pow(base, exp, newton_iterations);
        }
    }
}