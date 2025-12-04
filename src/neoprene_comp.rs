use std::cmp::Ordering;

use num_bigint::BigUint;

use crate::byzantine::ByzNode;
use crate::neoprene::neoprene_byznode;

pub enum NeopreneCompError {
    FailedToConverge
}

pub fn neoprene_comp(a: &ByzNode, b: &ByzNode, max_iterations: &BigUint) -> Result<Ordering, NeopreneCompError> {
    let mut current_iterations = BigUint::from(3 as u8);
    loop {
        let a_range = neoprene_byznode(&a, &current_iterations);
        let b_range = neoprene_byznode(&b, &current_iterations);

        if a_range.min > b_range.max {
            return Ok(Ordering::Greater);
        }
        if a_range.max > b_range.min {
            return Ok(Ordering::Less);
        }
        
        if &current_iterations > max_iterations {
            return Err(NeopreneCompError::FailedToConverge);
        }

        current_iterations += 1 as u8;
    }
}