use genetic_algorithms::traits::{GeneT, GenotypeT};
use genetic_algorithms::operations::{mating::random};

//Structures definition
#[derive(Debug, Copy, Clone, Default)]
struct Gene{
    pub id: i64,
}
impl GeneT for Gene{
    fn new()->Gene{
        return Gene{id: -1};
    }
    fn get_id(&self) -> &i64{
        return &self.id;
    }
}

#[derive(Debug)]
pub struct Genotype<T: GeneT>{
    pub dna: Vec<T>,
    pub phenotype: f64,
}
impl <T: GeneT> GenotypeT<T> for Genotype<T>{
    fn get_dna(&self) -> Vec<T> {
        self.dna.to_vec()
    }
    fn get_phenotype(&self) -> &f64 {
        return &self.phenotype;
    }
    fn new() -> Self {
       return Genotype{
        dna: Vec::new(),
        phenotype: 0.0,
       }
    }
}

#[test]
fn test_random_even_mating(){

    //We create 6 dna's for 6 individuals
    let dna_1 = vec![Gene{id:1}, Gene{id:2}];
    let dna_2 = vec![Gene{id:3}, Gene{id:4}];
    let dna_3 = vec![Gene{id:5}, Gene{id:6}];
    let dna_4 = vec![Gene{id:7}, Gene{id:8}];
    let dna_5 = vec![Gene{id:9}, Gene{id:10}];
    let dna_6 = vec![Gene{id:11}, Gene{id:12}];

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
    let dna_1 = vec![Gene{id:1}, Gene{id:2}];
    let dna_2 = vec![Gene{id:3}, Gene{id:4}];
    let dna_3 = vec![Gene{id:5}, Gene{id:6}];
    let dna_4 = vec![Gene{id:7}, Gene{id:8}];
    let dna_5 = vec![Gene{id:9}, Gene{id:10}];

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