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
    individuals: Vec<U>,
}

impl<T,U> Population<T,U>
where
    T: GeneT,
    U: GenotypeT<T>,
{
    // Creates a new `Population` with the given individuals as members.
    pub fn new(individuals: Vec<U>) -> Population<T,U> {
        Population { individuals, _gene_type: PhantomData}
    }

    // Returns all individuals
    pub fn individuals(&self) -> &Vec<U> {
        &self.individuals
    }

    // Returns the number of individuals in the population.
    pub fn size(&self) -> usize {
        self.individuals.len()
    }
}