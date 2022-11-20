pub trait GeneT {
    fn get_id(&self) -> &i64;
}

pub trait GenotypeT<T: GeneT>{
    fn get_dna(&self) -> &Vec<T>;
    fn get_phenotype(&self) -> &f64;
}