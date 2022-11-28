use genetic_algorithms::traits::{GeneT, GenotypeT};

//Structures definition
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Gene{
    pub id: i64,
}
impl GeneT for Gene{
    fn new()->Gene{
        return Gene{id: -1};
    }
    fn get_id(&self) -> &i64{
        return &self.id;
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Genotype<T: GeneT>{
    pub dna: Vec<T>,
    pub phenotype: f64,
}
impl <T: GeneT> GenotypeT<T> for Genotype<T>{
    fn get_dna(&mut self) -> &mut Vec<T> {
        &mut self.dna
    }
    fn get_phenotype(&self) -> &f64 {
        return &self.phenotype;
    }
    fn new() -> Self {
       return Genotype{
        dna: Vec::new(),
        phenotype: 0.0,
       }
    }
}