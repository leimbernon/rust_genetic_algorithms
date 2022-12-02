use std::collections::HashMap;

use crate::traits::GeneT;
use crate::traits::GenotypeT;
use crate::operations::selection::random::random;
use super::Selection;

pub mod random;


pub fn factory<T: GeneT, U: GenotypeT<T>>(selection: Selection, individuals: &Vec<U>) -> HashMap<usize, usize>{
    match selection {
        Selection::Random => {random(individuals)}
    }
}