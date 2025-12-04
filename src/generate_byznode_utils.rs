/// This file should be imported as GBU; i.e. ```use generate_byznode_utils as GBU;```

use std::rc::Rc;

use crate::byzantine::ByzNode;
use crate::byznode_sorted_vec::{ByzNodeCoefficientAddVec, ByzNodePowerMulVec, ByzNodeVec};
use crate::rational::Rational;
use crate::byzantine::TransitiveConsts;

pub fn rational(rat: Rational) -> ByzNode {
    return ByzNode::Rational { rational: rat };
}

pub fn pow(a: ByzNode, b: Rational) -> ByzNode {
    return ByzNode::Pow { base: Rc::new(a), exp: b };
}

pub fn add(rat: Rational, vec: Vec<(Option<Rational>, ByzNode)>) -> ByzNode {
    let mut c = ByzNodeCoefficientAddVec::new();

    c.insert_rational(rat);

    for i in vec {
        match i.0 {
            Some(rational) => {
                c.insert((rational, i.1));
            },
            None => {
                c.insert((Rational::one(), i.1));
            }
        }
    }

    return ByzNode::Add { addends: c };
}

pub fn mul(rat: Option<Rational>, vec: Vec<(Option<Rational>, ByzNode)>) -> ByzNode {
    let mut c = ByzNodePowerMulVec::new();

    match rat {
        Some(rational) => {
            c.insert_rational(rational);
        },
        _ => ()
    }

    for i in vec {
        match i.0 {
            Some(rational) => {
                c.insert((rational, i.1));
            },
            None => {
                c.insert((Rational::one(), i.1));
            }
        }
    }

    return ByzNode::Mul { products: c };
}

pub fn transitive(transitive_const: TransitiveConsts) -> ByzNode {
    return ByzNode::TransitiveConst { transitive_const }
}