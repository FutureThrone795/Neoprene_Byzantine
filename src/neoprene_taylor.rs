use num_bigint::BigUint;
use crate::rational::{Rational, Sign};
use crate::rational_range::{RationalRange, RationalRangeDescriptor};

/// Panics when given a value that cannot fit in a u32
fn biguint_to_u32(x: &BigUint) -> u32 {
    let a = x.to_u32_digits();

    if a.len() != 1 {
        panic!("Attempted to run biguint_to_u32(..) on a BigUint that cannot be expressed as a u32");
    }

    return a[0];
}

fn rational_range_midpoint(x: &RationalRange) -> Rational {
    let mut a = x.min.clone();
    a += &x.max;

    a.denom *= 2 as u8;
    a.simplify();

    return a;
}

/// Using the Gregory-Leibniz series
pub fn compute_pi(newton_iterations: &BigUint, limit_denom: &BigUint) -> RationalRange {
    let k = biguint_to_u32(newton_iterations) * 8;

    let mut a = Rational::from(3);
    let b: Rational;

    for n in 1..(k+1) {
        let mut c = BigUint::from(2*n);
        c *= 2*n+1;
        c *= 2*n+2;
        let d = Rational {
            sign: if n%2 == 1 { Sign::Pos } else  { Sign::Neg },
            numer: BigUint::from(4 as u8),
            denom: c
        };

        a += &d;
    }

    // Gregory-Leibniz alternates between an underapproximation and an overapproximation,
    // So I need to use both the 2nd-to-last and the last value
    b = a.clone();

    let mut c = BigUint::from(2*k+2);
    c *= 2*k+3;
    c *= 2*k+4;
    let d = Rational {
        sign: if k%2 == 0 { Sign::Pos } else  { Sign::Neg },
        numer: BigUint::from(4 as u8),
        denom: c
    };
    a += &d;

    // Gregory-Leibniz alternates between an underapproximation and an overapproximation,
    // So which approximation is min or max depends on if k is odd or even
    let min: Rational;
    let max: Rational;
    if k%2 == 1 {
        min = a;
        max = b;
    } else {
        min = b;
        max = a;
    }

    let mut range = RationalRange::from((min, max));
    range.to_with_denominator(limit_denom);
    return range;
}

/// Using the taylor expansion of e^x evaluated at x=1, meaning this is just the sum of the inverses of the factorials up to k
pub fn compute_euler(newton_iterations: &BigUint, limit_denom: &BigUint) -> RationalRange {
    let k = biguint_to_u32(newton_iterations);

    let mut min = Rational::from(2);

    // Note that addition of rationals is quite expensive, and there is definitely a way I can speed this up with the knowledge of how subsequent terms will add
    for n in 2..(k+3) {
        let mut a = factorial(n);
        a.invert();
        min += &a; 
    }

    let mut max = min.clone();
    // Error term is given by e/(k+1)!
    // I use the current approximation of e as the numerator, which seems to work just fine even if it results in an underestimate on the error term
    let mut error = min.clone();
    error.denom *= factorial_biguint(k+3);
    error.simplify();
    max += &error;

    let mut range = RationalRange::from((min, max));
    range.to_with_denominator(limit_denom);
    return range;
}

pub fn factorial_biguint(x: u32) -> BigUint {
    let mut a = BigUint::from(1 as u8);

    if x < 2 {
        return a;
    }

    for i in 2..(x+1) {
        a *= i;
    }

    return a;
}

pub fn factorial(x: u32) -> Rational {
    return Rational { 
        sign: Sign::Pos, 
        numer: factorial_biguint(x), 
        denom:  BigUint::from(1 as u8)
    };
}

/// Initial coarse bounds for nth root of rational by just applying nth roots to the numerator and denominator individually
fn initial_root_bounds(base: &Rational, root: &BigUint) -> RationalRange {
    let root = biguint_to_u32(root);

    let numer_root = base.numer.nth_root(root);
    let denom_root = base.denom.nth_root(root);

    let min_root = Rational {
        sign: Sign::Pos,
        numer: numer_root.clone(),
        denom: denom_root.clone() + 1 as u8
    };
    let max_root = Rational {
        sign: Sign::Pos,
        numer: numer_root + 1 as u8, 
        denom: denom_root
    };

    return RationalRange::from((min_root, max_root));
}

/// Using Newton's method of computing principal roots
/// The function we're solving is 0 = (output)^(root) - base
fn nth_root(base: &Rational, root: &BigUint, newton_iterations: &BigUint, limit_denom: &BigUint) -> RationalRange {
    let mut current_guess = initial_root_bounds(base, root);

    let k = biguint_to_u32(newton_iterations);

    for _ in 0..k {
        let midpoint = rational_range_midpoint(&current_guess);

        let mut f_midpoint = midpoint.clone();
        f_midpoint.powi(root);
        f_midpoint -= base;
        
        let mut derivative_interval = current_guess;
        let q = root - 1 as u8;
        derivative_interval.min.powi(&q);
        derivative_interval.min.numer *= root;
        derivative_interval.min.simplify();
        derivative_interval.max.powi(&q);
        derivative_interval.max.numer *= root;
        derivative_interval.max.simplify();

        let f_midpoint_div_derivative_interval: RationalRange;
        let mut a = f_midpoint.clone();
        a /= &derivative_interval.min;
        let mut b = f_midpoint;
        b /= &derivative_interval.max;
        if a > b {
            f_midpoint_div_derivative_interval = RationalRange::from((b, a));
        } else {
            f_midpoint_div_derivative_interval = RationalRange::from((a, b));
        }

        let mut new_guess_min = midpoint.clone();
        new_guess_min -= &f_midpoint_div_derivative_interval.max;

        let mut new_guess_max = midpoint;
        new_guess_max -= &f_midpoint_div_derivative_interval.min;

        current_guess = RationalRange::from((new_guess_min, new_guess_max));

        current_guess.to_with_denominator(limit_denom);
    }
    
    return current_guess;
}

fn nth_root_range(base: &RationalRange, root: &BigUint, newton_iterations: &BigUint, limit_denom: &BigUint) -> RationalRange {
    let base_min_range = nth_root(&base.min, root, newton_iterations, limit_denom);
    let base_max_range = nth_root(&base.min, root, newton_iterations, limit_denom);

    if base_min_range.min > base_max_range.max {
        return RationalRange::from((base_min_range.max, base_max_range.min));
    } else {
        return RationalRange::from((base_min_range.min, base_max_range.max));
    }
}

pub fn rational_range_pow(base: &RationalRange, exp: &Rational, newton_iterations: &BigUint, limit_denom: &BigUint) -> RationalRange {
    if !exp.is_simplified() {
        panic!("Attempted to compute rational_range_pow(..) with an unsimplified exp");
    }
    if exp.is_zero() {
        panic!("Attempted to compute rational_range_pow(..) with an exp of 0");
    }
    if exp.is_one() {
        return base.clone();
    }
    if exp > &Rational::from(8) {
        panic!("Attempted to compute rational_range_pow(..) with an exp larger than 8")
    }

    if (base.min.is_negative() || base.max.is_negative()) && exp.is_denom_odd() {
        // If base is negative, exp.denom must be odd to have a real root
        panic!("Attempted to compute rational_range_pow(..) in such a way that a complex number would be produced");
    }

    let mut a = base.clone();
    a.min.powi(&exp.numer);
    a.max.powi(&exp.numer);

    let mut pow_range = nth_root_range(&a, &exp.denom, newton_iterations, limit_denom);
    match a.descriptor() {
        RationalRangeDescriptor::OverlapZero => {
            if !pow_range.min.is_negative() && !pow_range.max.is_negative() {
                // Consider (-1..2)^2, which can range between (0..4)
                pow_range.min = Rational::zero();
            }
        },
        _ => ()
    }

    if exp.is_negative() {
        pow_range.reciprocate();
    }

    return pow_range;
}