pub(crate) use super::Crossover;
use crate::configuration::CrossoverConfiguration;
use crate::traits::GenotypeT;
use self::cycle::cycle;
use self::multipoint::multipoint_crossover;
use self::uniform_crossover::uniform;

pub mod cycle;
pub mod multipoint;
pub mod uniform_crossover;

pub fn factory<U: GenotypeT>(parent_1: &U, parent_2: &U, configuration: CrossoverConfiguration) -> Option<Vec<U>>{
    match configuration.method {
        Crossover::Cycle => {cycle(parent_1, parent_2)},
        Crossover::MultiPoint => {multipoint_crossover(parent_1, parent_2, &configuration.number_of_points.unwrap())},
        Crossover::Uniform => {uniform(parent_1, parent_2)},
    }
}