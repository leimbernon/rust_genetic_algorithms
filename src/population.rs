use crate::traits::GenotypeT;

pub struct Population<U>
where
    U: GenotypeT,
{
    // The individuals or members of the population.
    pub individuals: Vec<U>,

    //The numbers of the generation of this population
    pub generation_numbers: Vec<i32>,

    //Average fitness of the population
    pub f_avg: f64,

    //Population largest fitness value
    pub f_max: f64,
}

impl<U> Population<U>
where
    U: GenotypeT,
{
    // Creates a new empty `Population`
    pub fn new_empty() -> Population<U> {
        Population { individuals: vec![], generation_numbers: vec![], f_avg: 0.0, f_max: 0.0 }
    }

    // Creates a new `Population` with the given individuals as members.
    pub fn new(individuals: Vec<U>) -> Population<U> {
        Population { individuals, generation_numbers: vec![], f_avg: 0.0, f_max: 0.0}
    }

    // Function to calculate f_avg and f_max of the first population
    pub fn aga_init(& mut self){
        self.f_max = 0.0;
        self.f_avg = 0.0;
        for individual in self.individuals.as_slice(){
            self.f_max = if individual.get_fitness() > self.f_max {individual.get_fitness()} else{self.f_max};
            self.f_avg += individual.get_fitness();
        }
        self.f_avg /= self.individuals.len() as f64;
    }

    // Adds an individual with a generation number.
    pub fn add_individual_gn(&mut self, individual: U, generation_number: i32, aga: bool){
        
        if aga {
            //We calculate the f_max
            self.f_max = if individual.get_fitness() > self.f_max {individual.get_fitness()} else{self.f_max};

            //We recalculate the average fitness and add the individual
            self.f_avg *= self.individuals.len() as f64;
            self.f_avg += individual.get_fitness();
        }

        self.individuals.push(individual);
        self.generation_numbers.push(generation_number);

        if aga {
            self.f_avg /= self.individuals.len() as f64;
        }

    }

    //Function to add individuals in the list and recalculate the fitness without going through the entire population
    pub fn add_individuals(&mut self, individuals: &mut Vec<U>, aga: bool){

        if aga {
            self.f_avg *= self.individuals.len() as f64;
        }
        let individuals_c = individuals.clone();
        self.individuals.append(individuals);
        if aga {
            for individual in individuals_c{
                //We calculate the f_max and add the values to the f_avg
                self.f_max = if individual.get_fitness() > self.f_max {individual.get_fitness()} else{self.f_max};
                self.f_avg += individual.get_fitness();
            }
            self.f_avg /= self.individuals.len() as f64;
        }
    }

    // Returns the number of individuals in the population.
    pub fn size(&self) -> usize {
        self.individuals.len()
    }
}