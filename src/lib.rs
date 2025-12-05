pub mod rational;
pub mod rational_range;
pub mod byzantine;
pub mod byznode_sorted_vec;
pub mod generate_byznode_utils;
pub mod neoprene;
pub mod neoprene_taylor;
pub mod neoprene_comp;

/*
pub use crate::rational::*;
pub use crate::rational_range::*;
pub use crate::byzantine::*;
pub use byznode_sorted_vec::*;
pub use generate_byznode_utils::*;
pub use neoprene::*;
pub use neoprene_taylor::*;
pub use neoprene_comp::*;
*/

/*
use crate::neoprene_comp::neoprene_comp;
use crate::rational::Rational;
use crate::generate_byznode_utils as GBU;
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
    println!();

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
*/