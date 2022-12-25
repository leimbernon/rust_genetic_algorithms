use std::collections::HashMap;

use crate::configuration::SelectionConfiguration;
use crate::traits::GeneT;
use crate::traits::GenotypeT;

use self::random::random;
use self::fitness_proportionate::roulette_wheel_selection;
use self::fitness_proportionate::stochastic_universal_sampling;
use self::tournament::tournament;

use super::Selection;

pub mod random;
pub mod fitness_proportionate;
pub mod tournament;

pub fn factory<T: GeneT, U: GenotypeT<T>>(selection: Selection, individuals: &Vec<U>, configuration: Option<SelectionConfiguration>) -> HashMap<usize, usize>{
    match selection {
        Selection::Random => {random(individuals)},
        Selection::RouletteWheel => {roulette_wheel_selection(individuals)},
        Selection::StochasticUniversalSampling => {stochastic_universal_sampling(individuals, configuration.unwrap().number_of_couples)},
        Selection::Tournament => {tournament(individuals, configuration.unwrap().number_of_couples)},
    }
}