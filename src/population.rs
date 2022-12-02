use std::marker::PhantomData;
use crate::traits::{GenotypeT, GeneT};

pub struct Population<T, G>
where
    T: GeneT,
    G: GenotypeT<T>,
{
    //To avoid having problems with type T, we create the phantom data
    _gene_type: PhantomData<T>,

    // The individuals or members of the population.
    individuals: Vec<G>,
}

impl<T,G> Population<T,G>
where
    T: GeneT,
    G: GenotypeT<T>,
{
    // Creates a new `Population` with the given individuals as members.
    pub fn new(individuals: Vec<G>) -> Population<T,G> {
        Population { individuals, _gene_type: PhantomData}
    }

    // Returns all individuals
    pub fn individuals(&self) -> &Vec<G> {
        &self.individuals
    }

    // Returns the number of individuals in the population.
    pub fn size(&self) -> usize {
        self.individuals.len()
    }
}