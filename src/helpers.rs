use rand::Rng;

use crate::{configuration::GaConfiguration, population::Population, traits::{GenotypeT, GeneT}, operations::{self, survivor::fitness::ProblemSolving}};

pub mod condition_checker;

/*
 * Function to call the different condition checkers 
 */
pub fn condition_checker_factory<U>(configuration: Option<&GaConfiguration>, population: Option<&Population<U>>, 
                                    alleles: Option<&[U::Gene]>, default_population: bool)
where
U: GenotypeT + Send + Sync + 'static + Clone
{
    //1- We call the condition for checking the length of every individual
    if let Some(population) = population{
        condition_checker::same_dna_length(population);
    }

    //2- Checks the configuration
    if let Some(configuration) = configuration {

        //2.1- We call the condition for fixed fitness
        if configuration.limit_configuration.problem_solving == ProblemSolving::FixedFitness{
            condition_checker::fitness_target_is_some(configuration, configuration.limit_configuration.problem_solving.to_string());
        }

        //2.2- Checks the population
        if let Some(population) = population {

            //2.2.1- Checks the conditions for cycle crossover operation
            if configuration.crossover_configuration.method == operations::Crossover::Cycle{
                condition_checker::unique_gene_ids(population);
            }
        }

        //2.3- Condition checkers for the adaptive genetic algorithms
        if configuration.adaptive_ga{
            //2.3.1- Checks for the crossover parameters
            condition_checker::aga_crossover_probabilities(configuration);
        } 

        //2.4- Condition checkers for the repetition of the alleles
        if configuration.limit_configuration.alleles_can_be_repeated{
            if let Some(alleles) = alleles {
                condition_checker::check_genotype_length_not_bigger_than_alleles::<U>(alleles, configuration.limit_configuration.genes_per_individual);
            }
        }

        //2.5- Condition checkers for the default population
        if default_population{
            condition_checker::check_genes_per_individual_is_set(configuration);
            condition_checker::check_population_size_is_set(configuration);
            condition_checker::check_alleles_are_set::<U>(alleles);
        } 

        //2.6- Condition checker for the couples
        condition_checker::check_number_of_couples_is_set(configuration);
    } 
}

/**
 * Function to initialize the dna of an individual without repeating an array of alleles
 */
pub fn initialize_dna_without_repeated_alleles<U>(alleles: &[U::Gene], genes_per_individual: i32, needs_unique_ids: bool)->Vec<U::Gene>
where
U: GenotypeT + Send + Sync + 'static + Clone{
    
    let mut rng = rand::thread_rng();
    let mut dna = Vec::new();

    let mut tmp_alleles = alleles.to_vec().clone();

    //Selects the genes randomly from the vector without repeating them
    for j in 0..genes_per_individual{
        let index = rng.gen_range(0..tmp_alleles.len());
        let mut gene = tmp_alleles.get(index).cloned().unwrap();

        //If we need unique ids
        if needs_unique_ids {
            gene.set_id(j);
        }

        tmp_alleles.remove(index);

        dna.push(gene);
    }

    dna
}

/**
 * Function to initialize the dna of an individual
 */
pub fn initialize_dna<U>(alleles: &[U::Gene], genes_per_individual: i32, needs_unique_ids: bool)->Vec<U::Gene>
where
U: GenotypeT + Send + Sync + 'static + Clone{
    
    let mut rng = rand::thread_rng();
    let mut dna = Vec::new();

    //Selects the genes randomly from the vector without repeating them
    for j in 0..genes_per_individual{
        let index = rng.gen_range(0..alleles.len());
        let mut gene = alleles.get(index).cloned().unwrap();

        //If we need unique ids
        if needs_unique_ids {
            gene.set_id(j);
        }
        
        dna.push(gene);
    }

    dna
}