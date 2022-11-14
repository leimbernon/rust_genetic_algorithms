use crate::structures::gene::Gene;

pub struct Genotype<T: Gene>{
    pub dna: Vec<T>,
    pub phenotype: f64,
}

impl<T: Gene> Genotype<T>{
    
    pub fn new() -> Genotype<T>{
        Genotype{
            dna: Vec::new(),
            phenotype: 0.0,
        }
    }

    /**
     * Calculates the phenotype, adding the values of the dna
     */
    fn calculate_phenotype(&self) -> f64{
        let mut phenotype = 0.0;
        for gene in &self.dna {
            phenotype += gene.value();
        }
        phenotype
    }
}