use std::cmp::Ordering;
use std::rc::Rc;

use crate::byzantine::ByzNode;
use crate::rational::Rational;

/// Know which type best fits your situation for storing an arbitrary equation. Storing a single ByzNode is strongly not recommended!
pub trait ByzNodeVec {
    fn insert(&mut self, item: ByzNode) {
        let vec = self.get_vec_mut();
        
        match vec.binary_search_by(|x| x.1.as_ref().cmp(&item)) {
            Ok(index) => {
                // Item was found, incrementing stored rational by one... (coefficient or power applies)
                vec[index].0 += &Rational::from(1);
            },
            Err(index) => {
                // Item was not found, adding to vec...
                // Starts with either a coefficient of 1 or an exponent of 1, both of which are identity functions when applied
                vec.insert(index, (Rational::from(1), Rc::new(item)));
            },
        }
    }

    fn get_vec(&self) -> &Vec<(Rational, Rc<ByzNode>)>;
    fn get_vec_mut(&mut self) -> &mut Vec<(Rational, Rc<ByzNode>)>;
}



////////////////////////////////////////////////////////////////////////////////
// Shared util functions 
////////////////////////////////////////////////////////////////////////////////



/// Note that this doesn't really do much more than compare the two equations as notation
/// Something like sqrt(2)/2 will not be shown to be equal to 1/sqrt(2)
/// It's quite difficult (possibly provably impossible) to figure out if two equations are equal without setting some kind of "close enough" precision
#[inline]
fn util_eq<T>(a: &T, b: &T) -> bool where T: ByzNodeVec {
    let vec = a.get_vec();
    let other_vec = b.get_vec();

    if vec.len() != other_vec.len() {
        return false;
    }

    // Because both lists are sorted, this is sufficient and necessary to determine if the two have the exact same elements 
    for i in 0..vec.len() {
        // Comparing the two tuples will ensure both values match. Comparing Rationals is already well-defined
        if vec[i] != other_vec[i] {
            return false;
        }
    }

    return true;
}

/// Note that this is comparing notation, not any kind of numeric value
#[inline]
fn util_cmp<T>(a: &T, b: &T) -> Ordering where T: ByzNodeVec {
    let vec = a.get_vec();
    let other_vec = b.get_vec();

    return vec.cmp(other_vec);
}



////////////////////////////////////////////////////////////////////////////////
// Individual definitions
////////////////////////////////////////////////////////////////////////////////



/// rational_summand + a*f_a() + b*f_b() + c*f_c() + ...
pub struct ByzNodeCoefficientAddVec {
    vec: Vec<(Rational, Rc<ByzNode>)>
}

/// rational_factor * f_a()^a * f_b()^b * f_c()^c + ...
/// (Here"^ is used for exponent)
pub struct ByzNodePowerMulVec {
    vec: Vec<(Rational, Rc<ByzNode>)>
}



////////////////////////////////////////////////////////////////////////////////
// Implementation for ByzNodeCoefficientAddVec
////////////////////////////////////////////////////////////////////////////////



impl ByzNodeVec for ByzNodeCoefficientAddVec {
    fn get_vec(&self) -> &Vec<(Rational, Rc<ByzNode>)> {
        return &self.vec;
    }
    fn get_vec_mut(&mut self) -> &mut Vec<(Rational, Rc<ByzNode>)> {
        return &mut self.vec;
    }    
}

impl PartialEq for ByzNodeCoefficientAddVec {
    fn eq(&self, other: &ByzNodeCoefficientAddVec) -> bool { 
        return util_eq(self, other);
    }
}

impl Eq for ByzNodeCoefficientAddVec {}

impl PartialOrd for ByzNodeCoefficientAddVec {
    fn partial_cmp(&self, other: &ByzNodeCoefficientAddVec) -> Option<Ordering> {
        return Some(util_cmp(self, other));
    }
}

impl Ord for ByzNodeCoefficientAddVec {
    fn cmp(&self, other: &ByzNodeCoefficientAddVec) -> Ordering {
        return util_cmp(self, other);
    }
}



////////////////////////////////////////////////////////////////////////////////
// Implementation for ByzNodePowerMulVec
////////////////////////////////////////////////////////////////////////////////



impl ByzNodeVec for ByzNodePowerMulVec {
    fn get_vec(&self) -> &Vec<(Rational, Rc<ByzNode>)> {
        return &self.vec;
    }
    fn get_vec_mut(&mut self) -> &mut Vec<(Rational, Rc<ByzNode>)> {
        return &mut self.vec;
    }    
}

impl PartialEq for ByzNodePowerMulVec {
    fn eq(&self, other: &ByzNodePowerMulVec) -> bool { 
        return util_eq(self, other);
    }
}

impl Eq for ByzNodePowerMulVec {}

impl PartialOrd for ByzNodePowerMulVec {
    fn partial_cmp(&self, other: &ByzNodePowerMulVec) -> Option<Ordering> {
        return Some(util_cmp(self, other));
    }
}

impl Ord for ByzNodePowerMulVec {
    fn cmp(&self, other: &ByzNodePowerMulVec) -> Ordering {
        return util_cmp(self, other);
    }
}