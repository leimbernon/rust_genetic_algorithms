use genetic_algorithms::traits::{GeneT, GenotypeT};

//Structures definition
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Gene{
    pub id: i32,
}
impl GeneT for Gene{
    fn new()->Gene{
        return Gene{id: -1};
    }
    fn get_id(&self) -> i32{
        return self.id;
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Genotype<T: GeneT>{
    pub dna: Vec<T>,
    pub fitness: f64,
    pub age: i32,
}
impl <T: GeneT> GenotypeT<T> for Genotype<T>{
    fn get_dna(&self) -> &Vec<T> {
        &self.dna
    }
    fn get_dna_mut(&mut self) -> &mut Vec<T> {
        &mut self.dna
    }
    fn get_fitness(&self) -> &f64 {
        return &self.fitness;
    }
    fn set_fitness(&mut self, fitness: f64) {
        self.fitness = fitness;
    }
    fn set_age(&mut self, age:i32){
        self.age = age;
    }
    fn get_age(&self) -> &i32 {
        &self.age
    }
    fn calculate_fitness(&mut self) {
        
        self.fitness = 0.0;
        let mut position = 0;

        for i in &self.dna{
            let fitness = f64::from(i.get_id()*position);
            self.fitness += fitness;
            position += 1;
        }
    }
    fn new() -> Self {
        return Genotype{
            dna: Vec::new(),
            fitness: 0.0,
            age: 0,
        }
    }
}