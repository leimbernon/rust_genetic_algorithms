use super::Crossover;
use crate::ga::GaConfiguration;
use crate::traits::{GenotypeT, GeneT};
use self::cycle::cycle;
use self::multipoint::multipoint_crossover;
use self::uniform_crossover::uniform;

pub mod cycle;
pub mod multipoint;
pub mod uniform_crossover;

pub fn factory<T: GeneT, U: GenotypeT<T>>(crossover: Crossover, parent_1: &U, parent_2: &U, configuration: &GaConfiguration) -> Option<Vec<U>>{
    match crossover {
        Crossover::Cycle => {cycle(parent_1, parent_2)},
        Crossover::MultiPoint => {multipoint_crossover(parent_1, parent_2, &configuration.crossover_number_of_points)},
        Crossover::Uniform => {uniform(parent_1, parent_2)},
    }
}