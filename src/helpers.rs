use rand::Rng;

use crate::{configuration::GaConfiguration, population::Population, traits::{GenotypeT, GeneT}, operations::{self, survivor::fitness::ProblemSolving}};

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
        let mut gene = tmp_alleles.get(index).copied().unwrap();

        //If we need unique ids
        if needs_unique_ids {
            gene.set_id(j);
        }

        tmp_alleles.remove(index);

        dna.push(gene);
    }

    return dna;
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
        let mut gene = alleles.get(index).copied().unwrap();

        //If we need unique ids
        if needs_unique_ids {
            gene.set_id(j);
        }
        
        dna.push(gene);
    }

    return dna;
}