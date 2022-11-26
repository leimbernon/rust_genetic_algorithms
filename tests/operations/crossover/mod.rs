use genetic_algorithms::traits::{GeneT, GenotypeT};
use genetic_algorithms::operations::{crossover::cycle};

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
    fn new() -> Genotype<T> {
        Genotype { dna: Vec::new(), phenotype: 0.0 }
    }
}


#[test]
fn test_cycle_crossover(){

    //we create 2 dnas of 4 genes for 2 individuals
    let dna_1 = vec![Gene{id:1}, Gene{id:2}, Gene{id:3}, Gene{id:4}];
    let dna_2 = vec![Gene{id:4}, Gene{id:3}, Gene{id:2}, Gene{id:1}];

    let parent_1 = Genotype{dna: dna_1, phenotype: 0.0};
    let parent_2 = Genotype{dna: dna_2, phenotype: 0.0};

    //Getting the offspring
    let mut offspring = cycle::cycle(&parent_1, &parent_2).unwrap();

    //Setting the child
    let dna_child_1 = offspring.pop().unwrap();
    let dna_child_2 = offspring.pop().unwrap();

    //Checking that children have the same number of genes
    assert_eq!(dna_child_1.len(), parent_1.dna.len());
    assert_eq!(dna_child_2.len(), dna_child_2.len());
    assert_eq!(parent_1.dna.len(), parent_2.dna.len());
}