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
    fn get_id(&self) -> &i32{
        return &self.id;
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Genotype<T: GeneT>{
    pub dna: Vec<T>,
    pub phenotype: f64,
    pub age: i32,
}
impl <T: GeneT> GenotypeT<T> for Genotype<T>{
    fn get_dna(&self) -> &Vec<T> {
        &self.dna
    }
    fn get_dna_mut(&mut self) -> &mut Vec<T> {
        &mut self.dna
    }
    fn get_phenotype(&self) -> &f64 {
        return &self.phenotype;
    }
    fn get_phenotype_mut(&mut self) -> &mut f64 {
        &mut self.phenotype
    }
    fn get_age_mut(&mut self) -> &mut i32 {
        &mut self.age
    }
    fn get_age(&self) -> &i32 {
        &self.age
    }
    fn calculate_phenotype(&mut self) {
        
        self.phenotype = 0.0;
        let mut position = 0;

        for i in &self.dna{
            let phenotype = f64::from(i.get_id()*position);
            self.phenotype += phenotype;
            position += 1;
        }
    }
    fn new() -> Self {
        return Genotype{
            dna: Vec::new(),
            phenotype: 0.0,
            age: 0,
        }
    }
}