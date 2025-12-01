use std::ops::{AddAssign, MulAssign};
use std::fmt::{Debug, Formatter};

use crate::rational::Rational;

#[derive(Clone)]
pub struct RationalRange {
    pub min: Rational,
    pub max: Rational
}

impl Debug for RationalRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        return write!(f, "[{:?} -> {:?}]", self.min, self.max); 
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

pub enum RationalRangeDescriptor {
    BothPos,
    BothNeg,
    OverlapZero
}
use RationalRangeDescriptor::{BothPos, BothNeg, OverlapZero};

pub fn get_descriptor(range: &RationalRange) -> RationalRangeDescriptor {
    match (range.min.is_negative(), range.max.is_negative()) {
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

impl AddAssign<&RationalRange> for RationalRange {
    fn add_assign(&mut self, rhs: &RationalRange) { 
        self.min += &rhs.min;
        self.max += &rhs.max;
    }
}

impl MulAssign<&RationalRange> for RationalRange {
    fn mul_assign(&mut self, rhs: &RationalRange) {
        match (get_descriptor(self), get_descriptor(rhs)) {
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