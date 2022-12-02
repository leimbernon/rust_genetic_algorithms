use crate::ga::ProblemSolving;
use crate::traits::GeneT;
use crate::traits::GenotypeT;
use crate::operations::survivor::fitness::fitness_based;
use super::Survivor;
pub mod fitness;

pub fn factory<T: GeneT, U: GenotypeT<T>>(survivor: Survivor, mut individuals: Vec<U>, population_size: usize, problem_solving: ProblemSolving) -> Vec<U>{
    match survivor {
        Survivor::Fitness => {fitness_based(individuals, population_size, problem_solving)}
    }
}