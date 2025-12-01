use std::fmt::{Debug, Formatter};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, Not, BitXor};
use std::cmp::Ordering;

use num_bigint::BigUint;

pub fn gcd(a_r: &BigUint,  b_r: &BigUint) -> BigUint {
    if *a_r==*b_r {
        return a_r.clone();
    }

    let mut a = a_r.clone();
    let mut b = b_r.clone();

    if b > a {
        std::mem::swap(&mut a, &mut b);
    }

    while b > BigUint::ZERO {
        let mut temp = a;
        temp %= &b;
        a = b;
        b = temp;
    }

    return a;
}

pub fn lcd(a_r: &BigUint, b_r: &BigUint) -> BigUint {
    if *a_r==*b_r {
        return a_r.clone();
    }

    let mut b = b_r.clone();
    b /= gcd(a_r, b_r);
    b *= a_r;

    return b;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Pos,
    Neg
}

impl BitXor for Sign {
    type Output = Sign;

    fn bitxor(self, rhs: Sign) -> Sign { 
        return match (self, rhs) {
            (Sign::Pos, Sign::Pos) => Sign::Pos,
            (Sign::Neg, Sign::Neg) => Sign::Pos,
            (_, _) => Sign::Neg
        };
    }
}

impl Not for Sign {
    type Output = Sign;

    fn not(self) -> Sign {
        return match self {
            Sign::Pos => Sign::Neg,
            Sign::Neg => Sign::Pos
        }
    }
}

impl Debug for Sign {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> 
    { 
        match self {
            Self::Pos => {
                return write!(f, "+");
            },
            Self::Neg => {
                return write!(f, "-");
            }
        }
    }
}

#[derive(Clone)]
pub struct Rational {
    pub sign: Sign,
    pub numer: BigUint,
    pub denom: BigUint
}

impl Rational {
    pub fn new(sign: Sign, numer: BigUint, denom: BigUint) -> Rational {
        let mut a = Rational { 
            sign, 
            numer, 
            denom
        };

        a.simplify();

        return a;
    }

    pub fn new_usize(sign: Sign, numer: usize, denom: usize) -> Rational {
        return Rational::new(sign, BigUint::from(numer), BigUint::from(denom));
    }

    pub fn simplify(&mut self) {
        let gcd = gcd(&self.numer, &self.denom);

        if gcd != BigUint::from(1 as u8) {
            self.numer /= &gcd;
            self.denom /= &gcd;
        }

        if self.denom == BigUint::ZERO {
            panic!("Rational with zero denominator detected in simplify function");
        }
    }

    pub fn is_zero(&self) -> bool {
        return self.numer == BigUint::ZERO;
    }
    pub fn zero() -> Rational {
        return Rational::from(0);
    }

    pub fn is_one(&self) -> bool {
        return self.numer != BigUint::ZERO && self.numer == self.denom;
    }
    pub fn one() -> Rational {
        return Rational::from(1);
    }


    pub fn negate(&mut self) {
        self.sign = !self.sign;
    }

    pub fn invert(&mut self) {
        if self.numer == BigUint::ZERO {
            panic!("Rational with zero numerator attempted to invert")
        }

        std::mem::swap(&mut self.numer, &mut self.denom);
    }

    pub fn powi(&mut self, exp: &BigUint) {
        let u32_digits = exp.to_u32_digits();

        if u32_digits.len() != 1 || *exp > BigUint::from(4 as u8) {
            panic!("Attempted to do powi with a pretty large exponent");
        }

        self.numer.pow(u32_digits[0]);
        self.denom.pow(u32_digits[0]);

        // No need to simplify, as if numer and denom are already simplified then their powers will also be
        // (Consider that their prime factorizations will still contain the same prime bases after the operation)
    }

    pub fn is_simplified(&self) -> bool {
        return gcd(&self.numer, &self.denom) == BigUint::from(1 as u8);
    }

    pub fn is_negative(&self) -> bool {
        return !self.is_zero() && matches!(self.sign, Sign::Neg);
    }

    pub fn is_int(&self) -> bool {
        if self.denom == BigUint::from(1 as u8) {
            return true;
        }

        return gcd(&self.numer, &self.denom) == self.denom;
    }

    pub fn is_denom_odd(&self) -> bool {
        return self.denom.bit(0);
    }

    pub fn is_int_assume_simplified(&self) -> bool {
        return self.denom == BigUint::from(1 as u8);
    }
}

impl Debug for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> 
    { 
        let a = match self.sign {
            Sign::Pos => {
                ""
            },
            Sign::Neg => {
                "-"
            }
        };

        if self.denom == BigUint::from(1 as u8) {
            return write!(f, "{}{}", a, self.numer);
        }
        return write!(f, "{}{}/{}", a, self.numer, self.denom);
    }
}

impl AddAssign<&Rational> for Rational {
    fn add_assign(&mut self, rhs: &Rational) { 
        let denom_lcd = lcd(&self.denom, &rhs.denom);

        let self_factor = denom_lcd.clone() / &self.denom;
        let rhs_factor = denom_lcd / &rhs.denom;

        self.numer *= &self_factor;
        self.denom *= &self_factor;

        let rhs_normalized_numer = rhs.numer.clone() * rhs_factor;

        match (self.sign, rhs.sign, self.numer > rhs_normalized_numer) {
            (Sign::Pos, Sign::Pos, _) => {
                self.numer += rhs_normalized_numer;
            },
            (Sign::Neg, Sign::Neg, _) => {
                self.numer += rhs_normalized_numer;
            }
            (Sign::Pos, Sign::Neg, true) => {
                self.numer -= rhs_normalized_numer;
            },
            (Sign::Pos, Sign::Neg, false) => {
                self.numer = rhs_normalized_numer - &self.numer;
                self.sign = Sign::Neg;
            },
            (Sign::Neg, Sign::Pos, true) => {
                self.numer += rhs_normalized_numer;
            },
            (Sign::Neg, Sign::Pos, false) => {
                self.numer = rhs_normalized_numer - &self.numer;
                self.sign = Sign::Pos;
            }
        }
        // Phew!

        self.simplify();
    }
}

impl SubAssign<&Rational> for Rational {
    fn sub_assign(&mut self, rhs: &Rational) { 
        let denom_lcd = lcd(&self.denom, &rhs.denom);

        let self_factor = denom_lcd.clone() / &self.denom;
        let rhs_factor = denom_lcd / &rhs.denom;

        self.numer *= &self_factor;
        self.denom *= &self_factor;

        let rhs_normalized_numer = rhs.numer.clone() * rhs_factor;

        match (self.sign, !rhs.sign, self.numer > rhs_normalized_numer) {
            (Sign::Pos, Sign::Pos, _) => {
                self.numer += rhs_normalized_numer;
            },
            (Sign::Neg, Sign::Neg, _) => {
                self.numer += rhs_normalized_numer;
            }
            (Sign::Pos, Sign::Neg, true) => {
                self.numer -= rhs_normalized_numer;
            },
            (Sign::Pos, Sign::Neg, false) => {
                self.numer = rhs_normalized_numer - &self.numer;
                self.sign = Sign::Neg;
            },
            (Sign::Neg, Sign::Pos, true) => {
                self.numer += rhs_normalized_numer;
            },
            (Sign::Neg, Sign::Pos, false) => {
                self.numer = rhs_normalized_numer - &self.numer;
                self.sign = Sign::Pos;
            }
        }
        // Phetwo!

        self.simplify();
    }
}

impl MulAssign<&Rational> for Rational {
    fn mul_assign(&mut self, rhs: &Rational) {
        self.sign = self.sign ^ rhs.sign;
        self.numer *= &rhs.numer;
        self.denom *= &rhs.denom;
    
        self.simplify();
    }
}

impl DivAssign<&Rational> for Rational {
    fn div_assign(&mut self, rhs: &Rational) {
        self.sign = self.sign ^ rhs.sign;
        self.numer *= &rhs.denom;
        self.denom *= &rhs.numer;

        self.simplify();
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        if self.numer == BigUint::ZERO && other.denom == BigUint::ZERO {
            // Both are zero
            return true;
        }
        if self.sign != other.sign {
            // Signs differ
            return false;
        }
        if self.numer == other.numer && self.denom == other.denom {
            // Trivially equal
            return true;
        }

        let denom_lcd = gcd(&self.denom, &other.denom);

        let mut a = denom_lcd.clone() / &self.denom;
        a *= &self.numer;
        let mut b = denom_lcd / &other.denom;
        b *= &other.numer;

        return a == b;
    }
    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

impl Eq for Rational {}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.numer == BigUint::ZERO && other.denom == BigUint::ZERO {
            // Both are zero
            return Ordering::Equal;
        }
        match (self.sign, other.sign) {
            (Sign::Pos, Sign::Neg) => {
                return Ordering::Greater;
            }
            (Sign::Neg, Sign::Pos) => {
                return Ordering::Less;
            }
            _ => {}
        }
        if self.numer == other.numer && self.denom == other.denom {
            // Trivially equal
            return Ordering::Equal;
        }

        let denom_lcd = gcd(&self.denom, &other.denom);

        let mut a = denom_lcd.clone() / &self.denom;
        a *= &self.numer;
        let mut b = denom_lcd / &other.denom;
        b *= &other.numer;

        if a == b {
            return Ordering::Equal;
        }

        // Note that at this point we know self.sign == other.sign
        return match (a > b, self.sign) {
            (true, Sign::Pos) => Ordering::Greater,
            (true, Sign::Neg) => Ordering::Less,
            (false, Sign::Pos) => Ordering::Less,
            (false, Sign::Neg) => Ordering::Greater
        }
    }
}

impl From<isize> for Rational {
    fn from(i: isize) -> Rational { 
        return Rational { 
            sign: if i.is_positive() { Sign::Pos } else { Sign::Neg }, 
            numer: BigUint::from(i.unsigned_abs()), 
            denom: BigUint::from(1 as u8) 
        } 
    }
}