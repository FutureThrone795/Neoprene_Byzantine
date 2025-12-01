use std::cmp::Ordering;
use std::rc::Rc;
use std::fmt::{Debug, Formatter};

use crate::byzantine::ByzNode;
use crate::rational::Rational;

/// Know which type best fits your situation for storing an arbitrary equation. Store one of these instead of a ByzNode::Add or a ByzNode::Mul
pub trait ByzNodeVec {
    fn insert_single(&mut self, item: ByzNode) {
        self.insert((Rational::from(1), item));
    }

    fn insert_rational(&mut self, rational: Rational);

    fn insert(&mut self, item: (Rational, ByzNode)) {
        let vec = self.get_vec_mut();
        
        
        match vec.binary_search_by(|x| x.1.as_ref().cmp(&item.1)) {
            Ok(index) => {
                // Item was found, incrementing stored rational by supplied rational... (coefficient or power works for this)
                vec[index].0 += &item.0;

                if vec[index].0 == Rational::from(0) {
                    // If coefficient or power is 0, the item should be removed from the vec
                    vec.remove(index);
                }
            },
            Err(index) => {
                // Item was not found, adding to vec...
                vec.insert(index, (item.0, Rc::new(item.1)));
            },
        }
    }

    fn get_vec(&self) -> &Vec<(Rational, Rc<ByzNode>)>;
    fn get_vec_mut(&mut self) -> &mut Vec<(Rational, Rc<ByzNode>)>;

    fn get_rational_part(&self) -> &Rational;
    fn get_rational_part_mut(&mut self) -> &mut Rational;
}



////////////////////////////////////////////////////////////////////////////////
// Shared util functions 
////////////////////////////////////////////////////////////////////////////////



/// Note that this doesn't really do much more than compare the two equations as notation
/// Something like sqrt(2)/2 will not be shown to be equal to 1/sqrt(2)
/// It's quite difficult (possibly provably impossible) to figure out if two equations are equal without setting some kind of "close enough" precision
#[inline]
fn util_eq<T>(a: &T, b: &T) -> bool where T: ByzNodeVec {
    let rat = a.get_rational_part();
    let other_rat = b.get_rational_part();

    if rat != other_rat {
        return false;
    }

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

    let vec_cmp = vec.cmp(other_vec);

    match vec_cmp {
        Ordering::Equal => {
            let rat = a.get_rational_part();
            let other_rat = b.get_rational_part();

            return rat.cmp(other_rat);
        },
        _ => {
            return vec_cmp;
        }
    }
}



////////////////////////////////////////////////////////////////////////////////
// Individual definitions
////////////////////////////////////////////////////////////////////////////////



/// rational_summand + a*f_a() + b*f_b() + c*f_c() + ...
pub struct ByzNodeCoefficientAddVec {
    rational_part: Rational,
    vec: Vec<(Rational, Rc<ByzNode>)>
}

impl ByzNodeCoefficientAddVec {
    pub fn new() -> ByzNodeCoefficientAddVec {
        return ByzNodeCoefficientAddVec { rational_part: Rational::from(0), vec: Vec::new() }
    }
}

/// rational_factor * f_a()^a * f_b()^b * f_c()^c + ...
/// (Here"^ is used for exponent)
pub struct ByzNodePowerMulVec {
    rational_part: Rational,
    vec: Vec<(Rational, Rc<ByzNode>)>
}

impl ByzNodePowerMulVec {
    pub fn new() -> ByzNodePowerMulVec {
        return ByzNodePowerMulVec { rational_part: Rational::from(1), vec: Vec::new() }
    }
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

    fn get_rational_part(&self) -> &Rational {
        return &self.rational_part;
    }
    fn get_rational_part_mut(&mut self) -> &mut Rational {
        return &mut self.rational_part;
    }

    fn insert_rational(&mut self, rational: Rational) {
        self.rational_part += &rational;
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

impl Debug for ByzNodeCoefficientAddVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut a = String::new();
        let rat = self.get_rational_part();
        let vec = self.get_vec();

        if !rat.is_zero() {
            a.push_str(format!("{:?} + ", {rat}).as_str());
        }

        for i in 0..vec.len() {
            if i != 0 {
                a.push_str(" + ");
            }

            let item = &vec[i];
            if item.0 == Rational::from(1) {
                a.push_str(format!("{:?}", vec[i].1).as_str())
            } else {
                a.push_str(format!("{:?}*{:?}", vec[i].0, vec[i].1).as_str());
            }
        }

        return write!(f, "{}", a);
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

    fn get_rational_part(&self) -> &Rational {
        return &self.rational_part;
    }
    fn get_rational_part_mut(&mut self) -> &mut Rational {
        return &mut self.rational_part;
    }

    fn insert_rational(&mut self, rational: Rational) {
        self.rational_part *= &rational;
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

impl Debug for ByzNodePowerMulVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut a = String::new();
        let rat = self.get_rational_part();
        let vec = self.get_vec();

        if !rat.is_one() {
            a.push_str(format!("{:?} + ", {rat}).as_str());
        }
        
        for i in 0..vec.len() {
            if i != 0 {
                a.push_str(" * ");
            }

            let item = &vec[i];
            if item.0 == Rational::from(1) {
                a.push_str(format!("{:?}", vec[i].1).as_str())
            } else {
                a.push_str(format!("{:?}^{:?}", vec[i].1, vec[i].0).as_str());
            }
        }

        return write!(f, "{}", a);
    }
}