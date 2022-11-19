use genetic_algorithms::structures::{gene::GeneT, genotype::Genotype};
use genetic_algorithms::operations::{mating::random};
use super::*;

#[derive(Debug)]
#[derive(Copy, Clone)]
struct Gene{
    pub id: i64,
    value: f64,
}
impl GeneT for Gene{
    fn value(&self) -> &f64{
        return &self.value;
    }
}
impl PartialEq for Gene {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value
    }
}

#[test]
fn test_random_even_mating(){

    //We create 6 dna's for 6 individuals
    let dna_1 = vec![Gene{id:1,value:10.1}, Gene{id:2,value:20.2}];
    let dna_2 = vec![Gene{id:3,value:30.3}, Gene{id:4,value:40.4}];
    let dna_3 = vec![Gene{id:5,value:50.5}, Gene{id:6,value:60.6}];
    let dna_4 = vec![Gene{id:7,value:70.7}, Gene{id:8,value:80.8}];
    let dna_5 = vec![Gene{id:9,value:90.9}, Gene{id:10,value:100.10}];
    let dna_6 = vec![Gene{id:11,value:110.11}, Gene{id:12,value:120.12}];

    //We create the individuals
    let individual_1 = Genotype{dna: dna_1, phenotype: 0.0};
    let individual_2 = Genotype{dna: dna_2, phenotype: 0.0};
    let individual_3 = Genotype{dna: dna_3, phenotype: 0.0};
    let individual_4 = Genotype{dna: dna_4, phenotype: 0.0};
    let individual_5 = Genotype{dna: dna_5, phenotype: 0.0};
    let individual_6 = Genotype{dna: dna_6, phenotype: 0.0};

    //We create the population and create the random mating
    let population = vec![individual_1, individual_2, individual_3, individual_4, individual_5, individual_6];
    let mating_population = random::random_mating(&population);
    assert_eq!(mating_population.len(), 3);

}

#[test]
fn test_random_odd_mating(){
    
    //We create 6 dna's for 6 individuals
    let dna_1 = vec![Gene{id:1,value:10.1}, Gene{id:2,value:20.2}];
    let dna_2 = vec![Gene{id:3,value:30.3}, Gene{id:4,value:40.4}];
    let dna_3 = vec![Gene{id:5,value:50.5}, Gene{id:6,value:60.6}];
    let dna_4 = vec![Gene{id:7,value:70.7}, Gene{id:8,value:80.8}];
    let dna_5 = vec![Gene{id:9,value:90.9}, Gene{id:10,value:100.10}];

    //We create the individuals
    let individual_1 = Genotype{dna: dna_1, phenotype: 0.0};
    let individual_2 = Genotype{dna: dna_2, phenotype: 0.0};
    let individual_3 = Genotype{dna: dna_3, phenotype: 0.0};
    let individual_4 = Genotype{dna: dna_4, phenotype: 0.0};
    let individual_5 = Genotype{dna: dna_5, phenotype: 0.0};

    //We create the population and create the random mating
    let population = vec![individual_1, individual_2, individual_3, individual_4, individual_5];
    let mating_population = random::random_mating(&population);
    assert_eq!(mating_population.len(), 2);
}