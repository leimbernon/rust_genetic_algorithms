use genetic_algorithms::traits::{GeneT, ChromosomeT};

//Structures definition
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Gene{
    pub id: i32,
}
impl GeneT for Gene{
    fn get_id(&self) -> i32{
        self.id
    }
    fn set_id(&mut self, id: i32)->&mut Self {
        self.id = id;
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Chromosome{
    pub dna: Vec<Gene>,
    pub fitness: f64,
    pub age: i32,
}
impl ChromosomeT for Chromosome{
    type Gene = Gene;
    fn get_dna(&self) -> &[Self::Gene] {
        &self.dna
    }
    fn get_fitness(&self) -> f64 {
        self.fitness
    }
    fn set_fitness(&mut self, fitness: f64)->&mut Self {
        self.fitness = fitness;
        self
    }
    fn set_age(&mut self, age:i32)->&mut Self{
        self.age = age;
        self
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
    fn set_dna(&mut self, dna: &[Self::Gene])->&mut Self{
        self.dna = dna.to_vec();
        self
    }
}