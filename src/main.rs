mod rational;
mod rational_range;
mod byzantine;
mod byznode_sorted_vec;
mod generate_byznode_utils;
mod neoprene;
mod neoprene_taylor;
mod neoprene_comp;

use num_bigint::BigUint;

use crate::neoprene_comp::neoprene_comp;
use crate::rational::{Rational, Sign};
use crate::byznode_sorted_vec::{ByzNodeCoefficientAddVec, ByzNodePowerMulVec, ByzNodeVec};
use crate::rational_range::RationalRange;
use crate::generate_byznode_utils::{self as GBU, pow};
use crate::byzantine::TransitiveConsts;

fn main() {
    let a = GBU::transitive(TransitiveConsts::Pi);
    
    // Approximation of pi given by 9801/(2206*sqrt(2)) = (9801/2206) * (2)^(-1/2)
    let b = GBU::mul(
        Some(Rational::from((9801, 2206))), 
        vec![
            (None, GBU::pow(GBU::rational(Rational::from(2)), Rational::from((-1, 2))))
        ]
    );

    println!("a = {:?}", a);
    println!("b = {:?}", b);

    let cmp = neoprene_comp(&a, &b, &(12 as u8).into());

    match cmp {
        Ok(ord) => {
            println!("a {:?} b", ord);
        },
        Err(_err) => {
            println!("Failed to converge in determining if a > b");
        }
    }
}
