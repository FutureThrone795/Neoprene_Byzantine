use num_bigint::BigUint;

use crate::byzantine::{ByzNode, TransitiveConsts};
use crate::rational::Rational;
use crate::rational_range::{RationalRange, RationalRangeDescriptor, get_descriptor};
use crate::byznode_sorted_vec::{ByzNodeCoefficientAddVec, ByzNodePowerMulVec, ByzNodeVec};

fn neoprene_transitive(transitive_const: TransitiveConsts, newton_iterations: &BigUint) -> RationalRange {
    0
}

fn neoprene_add(addends: ByzNodeCoefficientAddVec, newton_iterations: &BigUint) -> RationalRange {
    let rat = addends.get_rational_part();
    let vec = addends.get_vec();

    let mut range = RationalRange {
        min: rat.clone(),
        max: rat.clone()
    };

    for i in vec {
        let mut i_range = neoprene_byznode(i.1.as_ref(), min_deviance_goal);
        i_range.min *= &i.0;
        i_range.max *= &i.0;

        range += &i_range;
    }

    return range;
}

fn neoprene_mul(addends: ByzNodePowerMulVec, newton_iterations: &BigUint) -> RationalRange {
    let rat = addends.get_rational_part();
    let vec = addends.get_vec();

    let mut range = RationalRange {
        min: rat.clone(),
        max: rat.clone()
    };

    for i in vec {
        let mut i_range = neoprene_byznode(i.1.as_ref(), min_deviance_goal);
        

    }

    return range;
}

fn neoprene_pow(byznode: &ByzNode, exp: &Rational, newton_iterations: &BigUint) -> RationalRange {
    
}

fn neoprene_range_pow_raw(range: &mut RationalRange, exp: &Rational, newton_iterations: &BigUint) {
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

fn neoprene_rational_pow_raw(rational: &mut Rational, exp: &Rational, newton_iterations: &BigUint) {
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
}

fn neoprene_byznode(byznode: &ByzNode, newton_iterations: &BigUint) -> RationalRange {
    match byznode {
        ByzNode::TransitiveConst {transitive_const} => {
            return neoprene_transitive(*transitive_const, &min_deviance_goal);
        }
    }

    0
}