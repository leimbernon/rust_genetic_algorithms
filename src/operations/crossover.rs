use super::Crossover;
use crate::traits::{GenotypeT, GeneT};
use crate::operations::crossover::cycle::cycle;

pub mod cycle;

pub fn factory<T: GeneT, U: GenotypeT<T>>(crossover: Crossover, parent_1: &U, parent_2: &U) -> Option<Vec<U>>{
    match crossover {
        Crossover::Cycle => {cycle(parent_1, parent_2)}
    }
}