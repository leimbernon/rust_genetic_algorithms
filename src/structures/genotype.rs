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
     * Function to add the DNA inside the genotype and calculate the phenotype
     */
    pub fn add_dna(mut self, dna: Vec<T>) {
        self.dna = dna;
        self.phenotype = self.calculate_phenotype();
    }

    /**
     * Calculates the phenotype, adding the values of the dna
     */
    fn calculate_phenotype(&self) -> f64 {
        let mut phenotype = 0.0;
        for gene in &self.dna {
            phenotype += gene.value();
        }
       phenotype
    }
}