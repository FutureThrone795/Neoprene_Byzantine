mod rational;
mod byzantine;
mod byz_node_sorted_vec;
mod byz_node_mul_vec;
mod neoprene;

use num_bigint::{self, BigUint};

use crate::rational::{Rational, gcd};

fn main() {
    println!("{}", gcd(&BigUint::from(525 as u16), &BigUint::from(1325 as u16)));

    let a= Rational::new(rational::Sign::Pos, BigUint::from(5 as u8), BigUint::from(8 as u8));
    println!("a: {:?}", a);

    let b= Rational::new(rational::Sign::Neg, BigUint::from(7 as u8), BigUint::from(15 as u8));
    println!("b: {:?}", b);

    let mut c = a.clone();
    c -= &b;
    c *= &b;

    println!("c: {:?}", c);
}
