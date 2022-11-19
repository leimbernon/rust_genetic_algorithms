use crate::structures::gene::GeneT;

pub struct Genotype<T: GeneT>{
    pub dna: Vec<T>,
    pub phenotype: f64,
}
impl<T: GeneT> Genotype<T>{
    
    pub fn new() -> Genotype<T>{
        Genotype{
            dna: Vec::new(),
            phenotype: 0.0,
        }
    }

    /**
     * Function to add the DNA inside the genotype and calculate the phenotype
     */
    pub fn add_dna(&mut self, dna: Vec<T>) {
        self.dna = dna;
        self.phenotype = self.calculate_phenotype();
    }

    /**
     * Function to add a gene inside a genotype
     */
    pub fn add_gene(&mut self, gene: T){
        self.dna.push(gene);
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
impl<T: GeneT> PartialEq for Genotype<T> {
    fn eq(&self, other: &Self) -> bool {

        //Checks the phenotype first
        let mut result = self.phenotype == other.phenotype;

        //Checks the dna
        result = result && (&self.dna.len() == &other.dna.len());

        //Walk through the DNA
        let mut index = 0;
        for gene in &self.dna{
            if !result {
                break;
            }
            result = result && gene.value() == other.dna[index].value();
            index += 1;
        }

        //Returns the result
        result
    }
}