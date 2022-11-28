use crate::structures::{Gene, Genotype};
use genetic_algorithms::operations::{crossover::cycle};


#[test]
fn test_cycle_crossover(){

    //we create 2 dnas of 4 genes for 2 individuals
    let dna_1 = vec![Gene{id:1}, Gene{id:2}, Gene{id:3}, Gene{id:4}];
    let dna_2 = vec![Gene{id:4}, Gene{id:3}, Gene{id:2}, Gene{id:1}];

    let mut parent_1 = Genotype{dna: dna_1, phenotype: 0.0};
    let mut parent_2 = Genotype{dna: dna_2, phenotype: 0.0};

    //Getting the offspring
    let mut offspring = cycle::cycle(&mut parent_1, &mut parent_2).unwrap();

    //Setting the child
    let child_1 = offspring.pop().unwrap();
    let child_2 = offspring.pop().unwrap();

    //Checking that children have the same number of genes
    assert_eq!(child_1.dna.len(), parent_1.dna.len());
    assert_eq!(child_2.dna.len(), child_2.dna.len());
    assert_eq!(parent_1.dna.len(), parent_2.dna.len());

    //Checking that the crossover has been well executed for the child 1
    assert_eq!(child_1.dna.get(0).unwrap().id, 4);
    assert_eq!(child_1.dna.get(1).unwrap().id, 2);
    assert_eq!(child_1.dna.get(2).unwrap().id, 3);
    assert_eq!(child_1.dna.get(3).unwrap().id, 1);

    //Checking that the crossover has been well executed for the child 2
    assert_eq!(child_2.dna.get(0).unwrap().id, 1);
    assert_eq!(child_2.dna.get(1).unwrap().id, 3);
    assert_eq!(child_2.dna.get(2).unwrap().id, 2);
    assert_eq!(child_2.dna.get(3).unwrap().id, 4);
}