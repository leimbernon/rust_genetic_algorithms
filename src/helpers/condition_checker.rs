use core::panic;

use crate::{population::Population, traits::{GenotypeT, GeneT}, configuration::GaConfiguration};

/**
 * Function to check that every individual has unique id's within their dna
 */
pub fn unique_gene_ids<U>(population: &Population<U>)
where 
U:GenotypeT + Send + Sync + 'static + Clone{

    //We analyze individual by individual
    for (individual_number, individual) in population.individuals.iter().enumerate(){
        //We check if the gene id is none or if it already exists in the dna
        for(gene_number, gene) in individual.get_dna().iter().enumerate(){
            for i in gene_number+1..individual.get_dna().len(){
                //If the gene id is equal to any other, we stop the run
                if gene.get_id().eq(&individual.get_dna().get(i).unwrap().get_id()){
                    panic!("Gene id must be unique within the DNA. The individual #{}, has same gene id at gene #{} and gene #{}", 
                            individual_number, gene_number, i);
                }
            }
        }
    }
}

/**
 * This function checks that fitness target is not none
 */
pub fn fitness_target_is_some(configuration: GaConfiguration, problem_type: String){

    //Checks that the fitness target is some
    if configuration.limit_configuration.fitness_target.is_none(){
        panic!("For {} problems, fitness_target must be set.", problem_type);
    }
}

/**
 * Checks that all the individual have the same dna length
 */
pub fn same_dna_length<U>(population: &Population<U>)
where 
U:GenotypeT + Send + Sync + 'static + Clone{
    //We analyze individual by individual
    for (individual_number, individual) in population.individuals.iter().enumerate(){
        for i in individual_number+1..population.individuals.len(){
            if individual.get_dna().len() != population.individuals.get(i).unwrap().get_dna().len(){
                panic!("All the individuals must have the same dna length. Individual #{} has a dna with length {} and individual #{} has a dna with length {}.",
                        individual_number, individual.get_dna().len(), i, population.individuals.get(i).unwrap().get_dna().len());
            }
        }
    }
}