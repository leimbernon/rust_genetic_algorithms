use genetic_algorithms::structures::{gene::GeneT, genotype::Genotype};
use super::*;

struct Gene{
    pub id: i64,
    value: f64,
}
impl GeneT for Gene{
    fn value(&self) -> &f64{
        return &self.value;
    }
}

fn random_mating(){

    //We create 6 dna's for 6 individuals
    let dna_1 = vec![Gene{id:1,value:10.1}, Gene{id:2,value:20.2}];
    let dna_3 = vec![Gene{id:3,value:30.3}, Gene{id:4,value:40.4}];
    let dna_4 = vec![Gene{id:5,value:50.5}, Gene{id:6,value:60.6}];
    let dna_5 = vec![Gene{id:5,value:70.7}, Gene{id:8,value:80.8}];
    let dna_5 = vec![Gene{id:6,value:90.9}, Gene{id:10,value:100.10}];
    let dna_6 = vec![Gene{id:11,value:110.11}, Gene{id:120,value:120.12}];

    let individual_1 = Genotype{dna: dna_1, phenotype: 0.0};

    assert_eq!(4, 4);
}