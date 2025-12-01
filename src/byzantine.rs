use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use crate::rational::Rational;
use crate::byznode_sorted_vec::ByzNodeCoefficientAddVec;
use crate::byznode_sorted_vec::ByzNodePowerMulVec;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum TransitiveConsts {
    Pi,
    Euler
}

pub enum ByzNode {
    TransitiveConst{
        transitive_const: TransitiveConsts
    },
    Add{
        addends: ByzNodeCoefficientAddVec
    },
    Mul{
        products: ByzNodePowerMulVec
    },
    Pow{
        base: Rc<ByzNode>, 
        exp: Rational
    }
}

impl PartialEq for ByzNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ByzNode::TransitiveConst { transitive_const }, ByzNode::TransitiveConst { transitive_const: transitive_conts_other }) => {
                return *transitive_const == *transitive_conts_other;
            },
            (ByzNode::Add { addends }, ByzNode::Add { addends: addends_other }) => {
                return *addends == *addends_other;
            },
            (ByzNode::Mul { products }, ByzNode::Mul { products: products_other }) => {
                return *products == *products_other;
            },
            (ByzNode::Pow { base, exp }, ByzNode::Pow { base: base_other, exp: exp_other }) => {
                return *exp == *exp_other && *base.as_ref() == *base_other.as_ref();
            }
            _ => {
                return false;
            }
        }
    }
}

impl Eq for ByzNode {}

impl PartialOrd for ByzNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for ByzNode {
    fn cmp(&self, other: &ByzNode) -> Ordering {
        match (self, other) {
            (ByzNode::TransitiveConst { transitive_const }, ByzNode::TransitiveConst { transitive_const: transitive_const_other }) => {
                return transitive_const.cmp(transitive_const_other);
            },
            (ByzNode::Add { addends }, ByzNode::Add { addends: addends_other }) => {
                return addends.cmp(addends_other);
            },
            (ByzNode::Mul { products }, ByzNode::Mul { products: products_other }) => {
                return products.cmp(products_other);
            },
            (ByzNode::Pow { base, exp }, ByzNode::Pow { base: base_other, exp: exp_other }) => {
                let base_cmp = base.cmp(base_other);

                match base_cmp {
                    Ordering::Equal => {
                        return exp.cmp(exp_other);
                    }
                    _ => {
                        return base_cmp;
                    }
                }
            },
            _ => {
                // The types differ, use identifying type id
                return self.to_identifying_type_int().cmp(&other.to_identifying_type_int());
            }
        }
    }
}

impl Debug for ByzNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        match self {
            ByzNode::TransitiveConst { transitive_const } => {
                return write!(f, "{:?}", transitive_const);
            },
            ByzNode::Add { addends } => {
                return write!(f, "({:?})", addends);
            },
            ByzNode::Mul { products } => {
                return write!(f, "({:?})", products);
            },
            ByzNode::Pow { base, exp } => {
                return write!(f, "{:?}^{:?}", base, exp);
            }
        }
    }
}

impl ByzNode {
    pub fn to_identifying_type_int(&self) -> usize {
        match self {
            ByzNode::TransitiveConst { .. } => {
                return 0;
            }
            ByzNode::Add { .. } => {
                return 1;
            }
            ByzNode::Mul { .. } => {
                return 2;
            }
            ByzNode::Pow { .. } => {
                return 3;
            }
        }
    }

    pub fn basic_type_eq(&self, other: &ByzNode) -> bool {
        return self.to_identifying_type_int() == other.to_identifying_type_int();
    }
}