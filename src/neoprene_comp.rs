use std::cmp::Ordering;

use num_bigint::BigUint;

use crate::byzantine::ByzNode;
use crate::neoprene::neoprene_byznode;

pub enum NeopreneCompError {
    FailedToConverge
}

pub fn neoprene_comp(a: &ByzNode, b: &ByzNode, max_iterations: &BigUint) -> Result<Ordering, NeopreneCompError> {
    let mut current_iterations = BigUint::from(3 as u8);
    let mut current_limit_denom = BigUint::from(6091 as u32); // 795th prime :^)
    loop {
        let a_range = neoprene_byznode(&a, &current_iterations, &current_limit_denom);
        let b_range = neoprene_byznode(&b, &current_iterations, &current_limit_denom);

        println!("a_range = {:?}", a_range);
        println!("b_range = {:?}", b_range);

        if a_range.min > b_range.max {
            return Ok(Ordering::Greater);
        }
        if a_range.max < b_range.min {
            return Ok(Ordering::Less);
        }
        
        if &current_iterations > max_iterations {
            return Err(NeopreneCompError::FailedToConverge);
        }

        current_iterations += 1 as u8;
        current_limit_denom *= 3 as u8; // Might be better to find the next prime, but this is probably fine
    }
}