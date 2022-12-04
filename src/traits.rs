pub trait GeneT: Default + Clone + Copy {
    fn new() -> Self;
    fn get_id(&self) -> &i32;
}

pub trait GenotypeT<T: GeneT + Default>{
    fn new() -> Self;
    fn get_dna(&self) -> &Vec<T>;
    fn get_dna_mut(&mut self) -> &mut Vec<T>;
    fn calculate_phenotype(&mut self);
    fn get_phenotype(&self) -> &f64;
}