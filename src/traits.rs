pub trait GeneT: Default + Clone {
    fn get_id(&self) -> &i64;
}

pub trait GenotypeT<T: GeneT>{
    fn create() -> Self;
    fn get_dna(&self) -> Vec<T>;
    fn get_phenotype(&self) -> &f64;
}