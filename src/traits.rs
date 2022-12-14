pub trait GeneT: Default + Clone + Copy {
    fn new() -> Self;
    fn get_id(&self) -> &i32;
}

pub trait GenotypeT<T: GeneT + Default>{
    fn new() -> Self;
    fn get_dna(&self) -> &Vec<T>;
    fn get_dna_mut(&mut self) -> &mut Vec<T>;
    fn calculate_fitness(&mut self);
    fn get_fitness(&self) -> &f64;
    fn get_fitness_mut(&mut self) -> &mut f64;
    fn get_age_mut(&mut self) -> &mut i32;
    fn get_age(&self) -> &i32;

    fn get_fitness_distance(&self, fitness_target: &f64) -> f64 {
        (fitness_target - self.get_fitness()).abs()
    }
}