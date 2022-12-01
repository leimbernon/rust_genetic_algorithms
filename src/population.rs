pub struct Population<G>
where
    G: GenotypeT,
{
    // The individuals or members of the population.
    individuals: Vec<G>,
}

impl<G> Population<G>
where
    G: Genotype,
{
    // Creates a new `Population` with the given individuals as members.
    pub fn new(individuals: Vec<G>) -> Population<G> {
        Population { individuals }
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