#[cfg(test)]
mod structures;
use crate::structures::{Genotype, Gene};
use genetic_algorithms::population::Population;

#[test]
fn test_add_individual_gn_aga(){

    //Setup of the project
    let individual_1 = Genotype{dna: Vec::<Gene>::new(), fitness: 20.0, age: 0};
    let individual_2 = Genotype{dna: Vec::<Gene>::new(), fitness: 40.0, age: 0};
    let individual_3 = Genotype{dna: Vec::<Gene>::new(), fitness: 120.0, age: 0};
    let mut population  = Population::new_empty();

    //We add the individuals in the population 1 by 1 
    population.add_individual_gn(individual_1, 0, true);
    population.add_individual_gn(individual_2, 0, true);
    population.add_individual_gn(individual_3, 0, true);

    //We check the computations
    assert_eq!(population.f_max, 120.0);
    assert_eq!(population.f_avg, 60.0);
    assert_eq!(population.size(), 3);
}

#[test]
fn test_add_individual_gn(){

    //Setup of the project
    let individual_1 = Genotype{dna: Vec::<Gene>::new(), fitness: 20.0, age: 0};
    let individual_2 = Genotype{dna: Vec::<Gene>::new(), fitness: 40.0, age: 0};
    let individual_3 = Genotype{dna: Vec::<Gene>::new(), fitness: 120.0, age: 0};
    let mut population  = Population::new_empty();

    //We add the individuals in the population 1 by 1 
    population.add_individual_gn(individual_1, 0, false);
    population.add_individual_gn(individual_2, 0, false);
    population.add_individual_gn(individual_3, 0, false);

    //We check the computations
    assert_eq!(population.size(), 3);
}


#[test]
fn test_add_individuals_aga(){

    //Setup of the project
    let individual_1 = Genotype{dna: Vec::<Gene>::new(), fitness: 20.0, age: 0};
    let individual_2 = Genotype{dna: Vec::<Gene>::new(), fitness: 40.0, age: 0};
    let individual_3 = Genotype{dna: Vec::<Gene>::new(), fitness: 120.0, age: 0};
    let mut individuals = vec![individual_1, individual_2, individual_3];
    let mut population  = Population::new_empty();

    //We add the individuals in the population 1 by 1 
    population.add_individuals(&mut individuals, true);

    //We check the computations
    assert_eq!(population.f_max, 120.0);
    assert_eq!(population.f_avg, 60.0);
    assert_eq!(population.size(), 3);
}

#[test]
fn test_add_individuals(){

    //Setup of the project
    let individual_1 = Genotype{dna: Vec::<Gene>::new(), fitness: 20.0, age: 0};
    let individual_2 = Genotype{dna: Vec::<Gene>::new(), fitness: 40.0, age: 0};
    let individual_3 = Genotype{dna: Vec::<Gene>::new(), fitness: 120.0, age: 0};
    let mut individuals = vec![individual_1, individual_2, individual_3];
    let mut population  = Population::new_empty();

    //We add the individuals in the population 1 by 1 
    population.add_individuals(&mut individuals, false);

    //We check the computations
    assert_eq!(population.size(), 3);
}