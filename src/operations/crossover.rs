use super::Crossover;
use crate::ga::GaConfiguration;
use crate::traits::{GenotypeT, GeneT};
use crate::operations::crossover::cycle::cycle;
use crate::operations::crossover::multipoint::multipoint_crossover;

pub mod cycle;
pub mod multipoint;

pub fn factory<T: GeneT, U: GenotypeT<T>>(crossover: Crossover, parent_1: &U, parent_2: &U, configuration: &GaConfiguration) -> Option<Vec<U>>{
    match crossover {
        Crossover::Cycle => {cycle(parent_1, parent_2)},
        Crossover::MultiPoint => {multipoint_crossover(parent_1, parent_2, &configuration.crossover_number_of_points)},
    }
}