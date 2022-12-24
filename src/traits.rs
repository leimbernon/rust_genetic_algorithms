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
    fn get_phenotype_mut(&mut self) -> &mut f64;
    fn get_age_mut(&mut self) -> &mut i32;
    fn get_age(&self) -> &i32;
}