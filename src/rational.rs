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

pub fn lcm(a_r: &BigUint, b_r: &BigUint) -> BigUint {
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
        if *exp == BigUint::ZERO {
            *self = Rational::one();
            return;
        }

        if *exp == BigUint::from(1 as u8) {
            return;
        }

        let u32_digits = exp.to_u32_digits();

        if u32_digits.len() != 1 || *exp > BigUint::from(12 as u8) {
            panic!("Attempted to do powi with a pretty large exponent");
        }

        if exp % BigUint::from(2 as u8) == BigUint::ZERO {
            self.sign = Sign::Pos;
        }

        self.numer = self.numer.pow(u32_digits[0]);
        self.denom = self.denom.pow(u32_digits[0]);

        // No need to simplify, as if numer and denom are already simplified then their powers will also be
        // (Consider that their prime factorizations will still contain the same prime bases after the operation)
    }

    /// Forces the denominator into a set value and modifies the numerator to have the closest value to the initial value
    /// When round_up is false, it will round down
    pub fn to_with_denominator(&mut self, new_denom: &BigUint, round_up: bool) {
        let mut new_mod_old = new_denom.clone();
        new_mod_old %= &self.denom;
        let mut new_div_old = new_denom.clone();
        new_div_old /= &self.denom;

        // At this point I start playing with my values so the names don't really match
        new_div_old *= &self.numer;
        new_mod_old *= &self.numer;
        new_mod_old /= &self.denom;

        self.numer = new_div_old;
        self.numer += &new_mod_old;

        // This could account for when the fraction is already an exact representation of the previous value but I think detecting that
        // Would be worse for performance than just adding one whenever
        if round_up {
            self.numer += 1 as u8;
        }

        self.denom = new_denom.clone();
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

    /// This function is highly lossy and should absolutely never be used for any reason in Neoprene or Byzantine code
    /// (I only use it to easily see a numerical value for a rational with large numbers)
    pub fn to_float(&self) -> f64 {
        let mut a = self.numer.clone();
        a *= 2_u32.pow(20);
        a /= &self.denom;

        let mut b = 0.0_f64;
        let a = a.to_u64_digits();
        for (i, a) in a.iter().enumerate() {
            b += (*a as f64) * (2_u32.pow(i as u32) as f64);
        }

        return b / (2_u32.pow(20) as f64);
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
        return write!(f, "({}{}/{})", a, self.numer, self.denom);
    }
}

impl AddAssign<&Rational> for Rational {
    fn add_assign(&mut self, rhs: &Rational) { 
        let denom_lcm = lcm(&self.denom, &rhs.denom);

        let self_factor = denom_lcm.clone() / &self.denom;
        let rhs_factor = denom_lcm / &rhs.denom;

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
        let denom_lcm = lcm(&self.denom, &rhs.denom);

        let self_factor = denom_lcm.clone() / &self.denom;
        let rhs_factor = denom_lcm / &rhs.denom;

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

        let denom_lcm = lcm(&self.denom, &other.denom);

        let mut a = denom_lcm.clone() / &self.denom;
        a *= &self.numer;
        let mut b = denom_lcm / &other.denom;
        b *= &other.numer;

        return a == b;
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
        if self.numer == BigUint::ZERO && other.numer == BigUint::ZERO {
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

        let denom_lcm = lcm(&self.denom, &other.denom);

        let mut a = denom_lcm.clone() / &self.denom;
        a *= &self.numer;
        let mut b = denom_lcm / &other.denom;
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

impl From<(isize, isize)> for Rational {
    fn from(i: (isize, isize)) -> Rational { 
        return Rational { 
            sign: if i.0.is_positive() ^ i.1.is_positive() { Sign::Neg } else { Sign::Pos }, 
            numer: BigUint::from(i.0.unsigned_abs()), 
            denom: BigUint::from(i.1.unsigned_abs()) 
        } 
    }
}