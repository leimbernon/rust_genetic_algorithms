pub(crate) use crate::{traits::{GeneT, GenotypeT}, configuration::{ProblemSolving, LimitConfiguration}};

pub fn fitness_based<T:GeneT, U:GenotypeT<T>>(individuals: &mut Vec<U>, population_size: usize, limit_configuration: LimitConfiguration)
{
    if limit_configuration.problem_solving != ProblemSolving::FixedFitness {
        //We sort the individuals by their fitness if there is not a fixed fitness problem
        individuals.sort_by(|a, b| b.get_fitness().partial_cmp(a.get_fitness()).unwrap());
    }else{
        //We sort the individuals by their distance with the fitness target in a fixed fitness problem
        individuals.sort_by(|a, b| b.get_fitness_distance(&limit_configuration.fitness_target.unwrap()).partial_cmp(&a.get_fitness_distance(&limit_configuration.fitness_target.unwrap())).unwrap());
    }

    //If there is more individuals than the defined population number
    if individuals.len() > population_size {
        let individuals_to_remove = individuals.len() - population_size;

        match limit_configuration.problem_solving {
            ProblemSolving::Maximization => {
                for _i in 0..individuals_to_remove{
                    individuals.remove(individuals.len() - 1);
                }
            },
            ProblemSolving::Minimization => {
                for _i in 0..individuals_to_remove{
                    individuals.remove(0);
                }
            },
            ProblemSolving::FixedFitness => {
                for _i in 0..individuals_to_remove{
                    individuals.remove(0);
                }
            },
        }
    }
}