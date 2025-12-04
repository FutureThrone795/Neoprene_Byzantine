use std::ops::{AddAssign, MulAssign};
use std::fmt::{Debug, Formatter};

use num_bigint::BigUint;

use crate::rational::Rational;

#[derive(Clone)]
pub struct RationalRange {
    pub min: Rational,
    pub max: Rational
}

pub enum RationalRangeDescriptor {
    BothPos,
    BothNeg,
    OverlapZero
}
use RationalRangeDescriptor::{BothPos, BothNeg, OverlapZero};

impl RationalRange {
    pub fn to_with_denominator(&mut self, new_denom: &BigUint) {
        self.min.to_with_denominator(new_denom, false);
        self.max.to_with_denominator(new_denom, false);
    }

    pub fn descriptor(&self) -> RationalRangeDescriptor {
        match (self.min.is_negative(), self.max.is_negative()) {
            (false, false) => {
                return BothPos;
            },
            (true, true) => {
                return BothNeg;
            }
            (true, false) => {
                return OverlapZero;
            }
            (false, true) => {
                panic!("While getting the descriptor of a RationalRange, the min was larger than the maximum");
            }
        }
    }

    pub fn reciprocate(&mut self) {
        match self.descriptor() {
            OverlapZero => {
                panic!("Tried to get the reciprocal of a rational range that overlaps zero");
            },
            _ => {
                self.min.invert();
                self.max.invert();
                // a > b implies (1/a) < (1/b)
                std::mem::swap(&mut self.min, &mut self.max);
            }
        }
    }
}

impl Debug for RationalRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        //return write!(f, "[{:?} -> {:?}]", self.min, self.max); 
        return write!(f, "[{:?} -> {:?}] f64 [{} -> {}]", self.min, self.max, self.min.to_float(), self.max.to_float()); 
    }
}

impl From<(isize, isize)> for RationalRange {
    fn from(i: (isize, isize)) -> RationalRange { 
        if i.0 > i.1 {
            panic!("Attempted to create RationalRange with min > max");
        }
        
        return RationalRange { 
            min: Rational::from(i.0), 
            max: Rational::from(i.1)
        } 
    }
}

impl From<(Rational, Rational)> for RationalRange {
    fn from(i: (Rational, Rational)) -> RationalRange { 
        if i.0 > i.1 {
            println!("{:?} {:?}", i.0, i.1);
            panic!("Attempted to create RationalRange with min > max");
        }
        
        return RationalRange { 
            min: i.0, 
            max: i.1
        } 
    }
}

impl AddAssign<&RationalRange> for RationalRange {
    fn add_assign(&mut self, rhs: &RationalRange) { 
        self.min += &rhs.min;
        self.max += &rhs.max;
    }
}

impl MulAssign<&RationalRange> for RationalRange {
    fn mul_assign(&mut self, rhs: &RationalRange) {
        match (self.descriptor(), rhs.descriptor()) {
            (BothPos, BothPos) => {
                self.min *= &rhs.min;
                self.max *= &rhs.max;
            },
            (BothNeg, BothNeg) => {
                self.min *= &rhs.min;
                self.max *= &rhs.max;
                std::mem::swap(&mut self.min, &mut self.max);
            },

            // Begin more complicated match cases

            (BothPos, BothNeg) => {
                std::mem::swap(&mut self.min, &mut self.max);
                self.min *= &rhs.min;
                self.max *= &rhs.max;
            },
            (BothPos, OverlapZero) => {
                self.min = self.max.clone();
                self.min *= &rhs.min;
                self.max *= &rhs.max;
            }, 
            
            (BothNeg, BothPos) => {
                self.min *= &rhs.max;
                self.max *= &rhs.min;
            },
            (BothNeg, OverlapZero) => {
                self.max = self.min.clone();
                self.min *= &rhs.max;
                self.max *= &rhs.min;
            },

            (OverlapZero, BothPos) => {
                self.min *= &rhs.max;
                self.max *= &rhs.max;
            },
            (OverlapZero, BothNeg) => {
                std::mem::swap(&mut self.min, &mut self.max);
                self.min *= &rhs.min;
                self.max *= &rhs.min;
            },
            (OverlapZero, OverlapZero) => {
                let mut min_branch = self.max.clone();
                min_branch *= &rhs.min;
                let mut max_branch = self.min.clone();
                max_branch *= &rhs.min;

                self.min *= &rhs.max;
                if self.min < min_branch {
                    self.min = min_branch;
                }

                self.max *= &rhs.max;
                if self.max < max_branch {
                    self.max = max_branch;
                }
            }
        }
    }
}