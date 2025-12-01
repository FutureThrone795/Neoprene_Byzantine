use num_bigint::BigUint;

use crate::byzantine::{ByzNode, TransitiveConsts};
use crate::rational::{Rational, Sign};
use crate::rational_range::{RationalRange, RationalRangeDescriptor, get_descriptor};
use crate::byznode_sorted_vec::{ByzNodeCoefficientAddVec, ByzNodePowerMulVec, ByzNodeVec};

pub fn neoprene_transitive(transitive_const: TransitiveConsts, newton_iterations: &BigUint) -> RationalRange {
    // These are perfectly accurate and do not need to be fixed later
    match transitive_const {
        TransitiveConsts::Pi => {
            return RationalRange { 
                min: Rational::new(Sign::Pos, BigUint::from(31 as u8), BigUint::from(10 as u8)), 
                max: Rational::new(Sign::Pos, BigUint::from(16 as u8), BigUint::from(5 as u8))
            }
        },
        TransitiveConsts::Euler => {
            return RationalRange::from((2, 3));
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