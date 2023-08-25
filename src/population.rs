use crate::traits::GenotypeT;

pub struct Population<U>
where
    U: GenotypeT,
{
    // The individuals or members of the population.
    pub individuals: Vec<U>,

    //The numbers of the generation of this population
    pub generation_numbers: Vec<i32>,
}

impl<U> Population<U>
where
    U: GenotypeT,
{
    // Creates a new empty `Population`
    pub fn new_empty() -> Population<U> {
        Population { individuals: vec![], generation_numbers: vec![] }
    }

    // Creates a new `Population` with the given individuals as members.
    pub fn new(individuals: Vec<U>) -> Population<U> {
        Population { individuals, generation_numbers: vec![]}
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