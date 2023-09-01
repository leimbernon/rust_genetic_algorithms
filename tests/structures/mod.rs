use genetic_algorithms::traits::{GeneT, GenotypeT};

//Structures definition
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Gene{
    pub id: i32,
}
impl GeneT for Gene{
    fn new()->Gene{
        Gene{id: -1}
    }
    fn get_id(&self) -> i32{
        self.id
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Genotype{
    pub dna: Vec<Gene>,
    pub fitness: f64,
    pub age: i32,
}
impl GenotypeT for Genotype{
    type Gene = Gene;
    fn get_dna(&self) -> &[Self::Gene] {
        &self.dna
    }
    fn get_fitness(&self) -> f64 {
        self.fitness
    }
    fn set_fitness(&mut self, fitness: f64) {
        self.fitness = fitness;
    }
    fn set_age(&mut self, age:i32){
        self.age = age;
    }
    fn get_age(&self) -> i32 {
        self.age
    }
    fn calculate_fitness(&mut self) {
        
        self.fitness = 0.0;

        for (i, gene) in self.dna.iter().enumerate() {
            let fitness = f64::from(gene.get_id()*i as i32);
            self.fitness += fitness;
        }
    }
    fn new() -> Self {
        Genotype{
            dna: Vec::new(),
            fitness: 0.0,
            age: 0,
        }
    }
    fn set_dna(&mut self, dna: &[Self::Gene]){
        self.dna = dna.to_vec();
    }
}