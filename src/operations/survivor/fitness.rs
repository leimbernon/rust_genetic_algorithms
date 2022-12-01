use crate::traits::{GeneT, GenotypeT};

pub enum ProblemSolving {
    Minimization,
    Maximization,
}


pub fn fitness_based<T:GeneT, U:GenotypeT<T>>(mut individuals: Vec<U>, population_size: usize, problem_solving: ProblemSolving) -> Vec<U>{

    //We first sort the individuals by their fitness
    individuals.sort_by(|a, b| b.get_phenotype().partial_cmp(&a.get_phenotype()).unwrap());

    //If there is more individuals than the defined population number
    if individuals.len() > population_size {
        let individuals_to_remove = individuals.len() - population_size;

        match problem_solving {
            ProblemSolving::Minimization => {
                for _i in 0..individuals_to_remove{
                    individuals.remove(individuals.len() - 1);
                }
            },
            ProblemSolving::Maximization => {
                for _i in 0..individuals_to_remove{
                    individuals.remove(0);
                }
            },
        }
    }

    return individuals;
}