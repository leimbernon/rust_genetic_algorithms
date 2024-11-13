use crate::traits::ChromosomeT;
use crate::genotypes::Binary as BinaryGenotype;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Binary {
    dna: Vec<BinaryGenotype>,
    pub fitness: f64,
    pub age: i32,
}
impl ChromosomeT for Binary {
    type Gene = BinaryGenotype;

    fn get_dna(&self) -> &[Self::Gene] {
        &self.dna
    }
    fn get_fitness(&self) -> f64 {
        self.fitness
    }
    fn set_fitness(&mut self, fitness: f64) -> &mut Self {
        self.fitness = fitness;
        self
    }
    fn set_age(&mut self, age: i32) -> &mut Self {
        self.age = age;
        self
    }
    fn get_age(&self) -> i32 {
        self.age
    }
    fn set_dna(&mut self, dna: &[Self::Gene]) -> &mut Self {
        self.dna = dna.to_vec();
        self
    }
    fn calculate_fitness(&mut self) {
        self.fitness = 0.0;
    }
}