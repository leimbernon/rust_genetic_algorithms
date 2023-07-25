use std::marker::PhantomData;
use crate::traits::{GenotypeT, GeneT};

pub struct Population<T, U>
where
    T: GeneT,
    U: GenotypeT<T>,
{
    //To avoid having problems with type T, we create the phantom data
    _gene_type: PhantomData<T>,

    // The individuals or members of the population.
    pub individuals: Vec<U>,

    //The numbers of the generation of this population
    pub generation_numbers: Vec<i32>,
}

impl<T,U> Population<T,U>
where
    T: GeneT,
    U: GenotypeT<T>,
{
    // Creates a new empty `Population`
    pub fn new_empty() -> Population<T,U> {
        Population { _gene_type: PhantomData, individuals: vec![], generation_numbers: vec![] }
    }

    // Creates a new `Population` with the given individuals as members.
    pub fn new(individuals: Vec<U>) -> Population<T,U> {
        Population { individuals, _gene_type: PhantomData, generation_numbers: vec![]}
    }

    // Adds an individual with a generation number.
    pub fn add_individual_gn(&mut self, individual: U, generation_number: i32){
        self.individuals.push(individual);
        self.generation_numbers.push(generation_number);
    }

    // Returns the number of individuals in the population.
    pub fn size(&self) -> usize {
        self.individuals.len()
    }
}