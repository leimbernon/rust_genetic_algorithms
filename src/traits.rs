pub trait GeneT: Default + Clone + Copy {
    fn new() -> Self;
    fn get_id(&self) -> i32{0}
}

pub trait GenotypeT: Clone{

    type Gene: GeneT;
    
    fn new() -> Self;
    fn new_gene() -> Self::Gene;
    fn get_dna(&self) -> &[Self::Gene];
    fn get_dna_mut(&mut self) -> &mut Vec<Self::Gene>;
    fn set_dna(&mut self, dna: &[Self::Gene]);
    fn set_gene(&mut self, gene_index: usize, gene: Self::Gene){
        let mut dna_temp = self.get_dna().to_vec();
        dna_temp[gene_index] = gene;
        self.set_dna(dna_temp.as_slice());
    }
    fn calculate_fitness(&mut self);
    fn get_fitness(&self) -> f64;
    fn set_fitness(&mut self, fitness: f64);
    fn set_age(&mut self, age: i32);
    fn get_age(&self) -> i32;

    fn get_fitness_distance(&self, fitness_target: &f64) -> f64 {
        (fitness_target - self.get_fitness()).abs()
    }
}