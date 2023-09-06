pub(crate) use crate::{traits::GenotypeT, configuration::{ProblemSolving, LimitConfiguration}};
use log::{trace, debug};

pub fn fitness_based<U:GenotypeT>(individuals: &mut Vec<U>, population_size: usize, limit_configuration: LimitConfiguration)
{
    debug!(target="survivor_events", method="fitness_based"; "Starting fitness based survivor method");
    if limit_configuration.problem_solving != ProblemSolving::FixedFitness {
        //We sort the individuals by their fitness if there is not a fixed fitness problem
        individuals.sort_by(|a, b| b.get_fitness().partial_cmp(&a.get_fitness()).unwrap());
    }else if limit_configuration.fitness_target.is_some(){
        //We sort the individuals by their distance with the fitness target in a fixed fitness problem
        individuals.sort_by(|a, b| b.get_fitness_distance(&limit_configuration.fitness_target.unwrap()).partial_cmp(&a.get_fitness_distance(&limit_configuration.fitness_target.unwrap())).unwrap());
    }else{
        //If we are solving a fixed fitness problem, without a fitness target, we should stop the run.
        panic!("For FixedFitness problems, fitness_target must be set.");
    }

    //If there is more individuals than the defined population number
    trace!(target="survivor_events", method="fitness_based"; "Individuals length {} - population size {}", individuals.len(), population_size);
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

    debug!(target="survivor_events", method="fitness_based"; "Fitness based survivor method finished");
}