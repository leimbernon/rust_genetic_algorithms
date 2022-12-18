use std::collections::HashMap;

use crate::traits::GeneT;
use crate::traits::GenotypeT;
use crate::operations::selection::random::random;
use self::fitness_proportionate::roulette_wheel_selection;

use super::Selection;

pub mod random;
pub mod fitness_proportionate;

pub fn factory<T: GeneT, U: GenotypeT<T>>(selection: Selection, individuals: &Vec<U>) -> HashMap<usize, usize>{
    match selection {
        Selection::Random => {random(individuals)},
        Selection::FitnessProportionate => {roulette_wheel_selection(individuals)}
    }
}