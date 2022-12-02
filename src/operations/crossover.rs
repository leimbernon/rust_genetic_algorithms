use super::Crossover;
use crate::traits::{GenotypeT, GeneT};
use crate::operations::crossover::cycle::cycle;

pub mod cycle;

pub fn factory<T: GeneT, U: GenotypeT<T>>(crossover: Crossover, parent_1: &mut U, parent_2: &mut U) -> Option<Vec<U>>{
    match crossover {
        Crossover::Cycle => {cycle(parent_1, parent_2)}
    }
}