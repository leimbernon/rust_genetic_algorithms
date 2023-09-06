use crate::{configuration::GaConfiguration, population::Population, traits::GenotypeT, operations::{self, survivor::fitness::ProblemSolving}};

pub mod condition_checker;

/*
 * Function to call the different condition checkers 
 */
pub fn condition_checker_factory<U>(configuration: GaConfiguration, population: &Population<U>)
where
U: GenotypeT + Send + Sync + 'static + Clone
{

    //1- We call the condition for checking the length of every individual
    condition_checker::same_dna_length(population);

    //2- We call the condition for fixed fitness
    if configuration.limit_configuration.problem_solving == ProblemSolving::FixedFitness{
        condition_checker::fitness_target_is_some(configuration, configuration.limit_configuration.problem_solving.to_string());
    }

    //3- We call the condition checkers for the cycle crossover operation
    if configuration.crossover_configuration.method == operations::Crossover::Cycle{
        //1.1- We check that genes ids are uniques for each individual
        condition_checker::unique_gene_ids(population);
    }
}

