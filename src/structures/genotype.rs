use crate::structures::gene::Gene;

pub struct Genotype<T: Gene>{
    pub dna: Vec<T>
}
impl<T: Gene> Genotype<T>{
    pub fn new() -> Genotype<T>{
        Genotype{
            dna: Vec::new(),
        }
    }
    pub fn phenotype(&self) -> f64{
        return 0.0
    }
}