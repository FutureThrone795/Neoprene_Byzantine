use num_bigint::BigUint;

use crate::byzantine::{ByzNode, TransitiveConsts};
use crate::rational::{Rational, Sign};
use crate::rational_range::{RationalRange, RationalRangeDescriptor};
use crate::byznode_sorted_vec::{ByzNodeCoefficientAddVec, ByzNodePowerMulVec, ByzNodeVec};
use crate::neoprene_taylor;

pub fn neoprene_transitive(transitive_const: TransitiveConsts, approximation_iterations: &BigUint, limit_denom: &BigUint) -> RationalRange {
    // These are perfectly accurate and do not need to be fixed later
    match transitive_const {
        TransitiveConsts::Pi => {
            return neoprene_taylor::compute_pi(approximation_iterations, limit_denom);
        },
        TransitiveConsts::Euler => {
            return neoprene_taylor::compute_euler(approximation_iterations, limit_denom);
        }
    }
}

pub fn neoprene_add(addends: &ByzNodeCoefficientAddVec, approximation_iterations: &BigUint, limit_denom: &BigUint) -> RationalRange {
    let rat = addends.get_rational_part();
    let vec = addends.get_vec();

    let mut range = RationalRange::from((rat.clone(), rat.clone()));

    for i in vec {
        let mut i_range = neoprene_byznode(i.1.as_ref(), approximation_iterations, limit_denom);
        i_range.min *= &i.0;
        i_range.max *= &i.0;

        range += &i_range;
    }

    return range;
}

pub fn neoprene_mul(products: &ByzNodePowerMulVec, approximation_iterations: &BigUint, limit_denom: &BigUint) -> RationalRange {
    let rat = products.get_rational_part();
    let vec = products.get_vec();

    let mut range = RationalRange::from((rat.clone(), rat.clone()));

    for i in vec {
        let mut i_range = neoprene_byznode(i.1.as_ref(), approximation_iterations, limit_denom);
        
        i_range = neoprene_taylor::rational_range_pow(&mut i_range, &i.0, approximation_iterations, limit_denom);

        range *= &i_range;
    }

    return range;
}

pub fn neoprene_pow(byznode: &ByzNode, exp: &Rational, approximation_iterations: &BigUint, limit_denom: &BigUint) -> RationalRange {
    let mut range = neoprene_byznode(byznode, approximation_iterations, limit_denom);
    range = neoprene_taylor::rational_range_pow(&mut range, exp, approximation_iterations, limit_denom);
    return range;
}

pub fn neoprene_byznode(byznode: &ByzNode, approximation_iterations: &BigUint, limit_denom: &BigUint) -> RationalRange {
    match byznode {
        ByzNode::Rational { rational } => {
            return RationalRange::from((rational.clone(), rational.clone()));
        }
        ByzNode::TransitiveConst {transitive_const} => {
            return neoprene_transitive(*transitive_const, &approximation_iterations, limit_denom);
        },
        ByzNode::Add { addends } => {
            return neoprene_add(addends, approximation_iterations, limit_denom);
        },
        ByzNode::Mul { products } => {
            return neoprene_mul(products, approximation_iterations, limit_denom);
        },
        ByzNode::Pow { base, exp } => {
            return neoprene_pow(base, exp, approximation_iterations, limit_denom);
        }
    }
}