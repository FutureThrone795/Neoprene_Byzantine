mod rational;
mod rational_range;
mod byzantine;
mod byznode_sorted_vec;
mod generate_byznode_utils;
mod neoprene;
mod neoprene_taylor;

use num_bigint::BigUint;

use crate::rational::Rational;
use crate::byznode_sorted_vec::{ByzNodeCoefficientAddVec, ByzNodeVec};
use crate::rational_range::RationalRange;
use crate::generate_byznode_utils as GBU;
use crate::byzantine::TransitiveConsts;

fn main() {
    let mut a = ByzNodeCoefficientAddVec::new();

    let b = GBU::add(Rational::one(), vec![(Rational::one(), GBU::pow(GBU::transitive(TransitiveConsts::Euler), Rational::from(2)))]);

    a.insert_single(GBU::pow(b, Rational::from(2)));

    a.insert((Rational::from(2), GBU::transitive(TransitiveConsts::Euler)));

    println!("{:?}", a);
    println!("...which is within the range {:?}", neoprene::neoprene_add(&a, &BigUint::from(10 as u8)));
    println!();

    println!("Range multiplication table");
    let range_list = [RationalRange::from((2, 3)), RationalRange::from((-3, 3)), RationalRange::from((-3, -2))];

    for x in &range_list {
        for y in &range_list {
            let mut xy = x.clone();
            xy *= y;
            println!("{:?}\t * {:?}\t = {:?}", x, y, xy);
        }
    }
}
