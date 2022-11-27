pub trait GeneT: Default + Clone {
    fn new() -> Self;
    fn get_id(&self) -> &i64;
}

pub trait GenotypeT<T: GeneT>{
    fn new() -> Self;
    fn get_dna(&mut self) -> &mut Vec<T>;
    fn get_phenotype(&self) -> &f64;
}