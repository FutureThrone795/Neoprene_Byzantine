use num_bigint::BigUint;

use crate::byzantine::{ByzNode, TransitiveConsts};
use crate::rational::{Rational, Sign};
use crate::rational_range::{RationalRange, RationalRangeDescriptor};
use crate::byznode_sorted_vec::{ByzNodeCoefficientAddVec, ByzNodePowerMulVec, ByzNodeVec};
use crate::neoprene_taylor;

pub fn neoprene_transitive(transitive_const: TransitiveConsts, newton_iterations: &BigUint, limit_denom: &BigUint) -> RationalRange {
    // These are perfectly accurate and do not need to be fixed later
    match transitive_const {
        TransitiveConsts::Pi => {
            return neoprene_taylor::compute_pi(newton_iterations, limit_denom);
        },
        TransitiveConsts::Euler => {
            return neoprene_taylor::compute_euler(newton_iterations, limit_denom);
        }
    }
}

pub fn neoprene_add(addends: &ByzNodeCoefficientAddVec, newton_iterations: &BigUint, limit_denom: &BigUint) -> RationalRange {
    let rat = addends.get_rational_part();
    let vec = addends.get_vec();

    let mut range = RationalRange::from((rat.clone(), rat.clone()));

    for i in vec {
        let mut i_range = neoprene_byznode(i.1.as_ref(), newton_iterations, limit_denom);
        i_range.min *= &i.0;
        i_range.max *= &i.0;

        range += &i_range;
    }

    return range;
}

pub fn neoprene_mul(products: &ByzNodePowerMulVec, newton_iterations: &BigUint, limit_denom: &BigUint) -> RationalRange {
    let rat = products.get_rational_part();
    let vec = products.get_vec();

    let mut range = RationalRange::from((rat.clone(), rat.clone()));

    for i in vec {
        let mut i_range = neoprene_byznode(i.1.as_ref(), newton_iterations, limit_denom);
        
        i_range = neoprene_taylor::rational_range_pow(&mut i_range, &i.0, newton_iterations, limit_denom);

        range *= &i_range;
    }

    return range;
}

pub fn neoprene_pow(byznode: &ByzNode, exp: &Rational, newton_iterations: &BigUint, limit_denom: &BigUint) -> RationalRange {
    let mut range = neoprene_byznode(byznode, newton_iterations, limit_denom);
    range = neoprene_taylor::rational_range_pow(&mut range, exp, newton_iterations, limit_denom);
    return range;
}

pub fn neoprene_byznode(byznode: &ByzNode, newton_iterations: &BigUint, limit_denom: &BigUint) -> RationalRange {
    match byznode {
        ByzNode::Rational { rational } => {
            return RationalRange::from((rational.clone(), rational.clone()));
        }
        ByzNode::TransitiveConst {transitive_const} => {
            return neoprene_transitive(*transitive_const, &newton_iterations, limit_denom);
        },
        ByzNode::Add { addends } => {
            return neoprene_add(addends, newton_iterations, limit_denom);
        },
        ByzNode::Mul { products } => {
            return neoprene_mul(products, newton_iterations, limit_denom);
        },
        ByzNode::Pow { base, exp } => {
            return neoprene_pow(base, exp, newton_iterations, limit_denom);
        }
    }
}