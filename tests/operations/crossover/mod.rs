use crate::structures::{Gene, Genotype};
use genetic_algorithms::operations::crossover::{cycle, multipoint::multipoint_crossover, uniform_crossover};


#[test]
fn test_cycle_crossover(){

    //we create 2 dnas of 4 genes for 2 individuals
    let dna_1 = vec![Gene{id:1}, Gene{id:2}, Gene{id:3}, Gene{id:4}];
    let dna_2 = vec![Gene{id:4}, Gene{id:3}, Gene{id:2}, Gene{id:1}];

    let parent_1 = Genotype{dna: dna_1, fitness: 0.0, age: 0};
    let parent_2 = Genotype{dna: dna_2, fitness: 0.0, age: 0};

    //Getting the offspring
    let mut offspring = cycle::cycle(&parent_1, &parent_2).unwrap();

    //Setting the child
    let child_2 = offspring.pop().unwrap();
    let child_1 = offspring.pop().unwrap();

    //Checking that children have the same number of genes
    assert_eq!(child_1.dna.len(), parent_1.dna.len());
    assert_eq!(child_2.dna.len(), child_2.dna.len());
    assert_eq!(parent_1.dna.len(), parent_2.dna.len());

    //Checking that the crossover has been well executed for the child 1
    assert_eq!(child_2.dna.get(0).unwrap().id, 4);
    assert_eq!(child_2.dna.get(1).unwrap().id, 2);
    assert_eq!(child_2.dna.get(2).unwrap().id, 3);
    assert_eq!(child_2.dna.get(3).unwrap().id, 1);

    //Checking that the crossover has been well executed for the child 2
    assert_eq!(child_1.dna.get(0).unwrap().id, 1);
    assert_eq!(child_1.dna.get(1).unwrap().id, 3);
    assert_eq!(child_1.dna.get(2).unwrap().id, 2);
    assert_eq!(child_1.dna.get(3).unwrap().id, 4);
}



#[test]
fn test_multipoint_crossover_2_points(){

    //we create 2 dnas of 4 genes for 2 individuals
    let dna_1 = vec![Gene{id:1}, Gene{id:2}, Gene{id:3}, Gene{id:4}, Gene{id:5}, Gene{id:6}];
    let dna_2 = vec![Gene{id:6}, Gene{id:5}, Gene{id:4}, Gene{id:3}, Gene{id:2}, Gene{id:1}];

    let parent_1 = Genotype{dna: dna_1, fitness: 0.0, age: 0};
    let parent_2 = Genotype{dna: dna_2, fitness: 0.0, age: 0};

    //Getting the offspring
    let mut offspring = multipoint_crossover(&parent_1, &parent_2, &2).unwrap();

    //Setting the child
    let child_2 = offspring.pop().unwrap();
    let child_1 = offspring.pop().unwrap();

    //Checking that children have the same number of genes
    assert_eq!(child_1.dna.len(), parent_1.dna.len());
    assert_eq!(child_2.dna.len(), child_2.dna.len());
    assert_eq!(parent_1.dna.len(), parent_2.dna.len());

    //Checking that the crossover has been well executed for the child 1
    assert_eq!(child_1.dna.get(0).unwrap().id, 1);
    assert_eq!(child_1.dna.get(1).unwrap().id, 2);
    assert_eq!(child_1.dna.get(2).unwrap().id, 4);
    assert_eq!(child_1.dna.get(3).unwrap().id, 3);
    assert_eq!(child_1.dna.get(4).unwrap().id, 5);
    assert_eq!(child_1.dna.get(5).unwrap().id, 6);

    //Checking that the crossover has been well executed for the child 2
    assert_eq!(child_2.dna.get(0).unwrap().id, 6);
    assert_eq!(child_2.dna.get(1).unwrap().id, 5);
    assert_eq!(child_2.dna.get(2).unwrap().id, 3);
    assert_eq!(child_2.dna.get(3).unwrap().id, 4);
    assert_eq!(child_2.dna.get(4).unwrap().id, 2);
    assert_eq!(child_2.dna.get(5).unwrap().id, 1);
}

#[test]
fn test_multipoint_crossover_4_points(){

    //we create 2 dnas of 4 genes for 2 individuals
    let dna_1 = vec![Gene{id:1}, Gene{id:2}, Gene{id:3}, Gene{id:4}, Gene{id:5}, Gene{id:6}];
    let dna_2 = vec![Gene{id:6}, Gene{id:5}, Gene{id:4}, Gene{id:3}, Gene{id:2}, Gene{id:1}];

    let parent_1 = Genotype{dna: dna_1, fitness: 0.0, age: 0};
    let parent_2 = Genotype{dna: dna_2, fitness: 0.0, age: 0};

    //Getting the offspring
    let mut offspring = multipoint_crossover(&parent_1, &parent_2, &4).unwrap();

    //Setting the child
    let child_2 = offspring.pop().unwrap();
    let child_1 = offspring.pop().unwrap();

    //Checking that children have the same number of genes
    assert_eq!(child_1.dna.len(), parent_1.dna.len());
    assert_eq!(child_2.dna.len(), child_2.dna.len());
    assert_eq!(parent_1.dna.len(), parent_2.dna.len());

    //Checking that the crossover has been well executed for the child 1
    assert_eq!(child_1.dna.get(0).unwrap().id, 1);
    assert_eq!(child_1.dna.get(1).unwrap().id, 5);
    assert_eq!(child_1.dna.get(2).unwrap().id, 3);
    assert_eq!(child_1.dna.get(3).unwrap().id, 3);
    assert_eq!(child_1.dna.get(4).unwrap().id, 5);
    assert_eq!(child_1.dna.get(5).unwrap().id, 1);

    //Checking that the crossover has been well executed for the child 2
    assert_eq!(child_2.dna.get(0).unwrap().id, 6);
    assert_eq!(child_2.dna.get(1).unwrap().id, 2);
    assert_eq!(child_2.dna.get(2).unwrap().id, 4);
    assert_eq!(child_2.dna.get(3).unwrap().id, 4);
    assert_eq!(child_2.dna.get(4).unwrap().id, 2);
    assert_eq!(child_2.dna.get(5).unwrap().id, 6);
}

#[test]
fn test_uniform_crossover(){

    //we create 2 dnas of 4 genes for 2 individuals
    let dna_1 = vec![Gene{id:1}, Gene{id:2}, Gene{id:3}, Gene{id:4}];
    let dna_2 = vec![Gene{id:4}, Gene{id:3}, Gene{id:2}, Gene{id:1}];

    let parent_1 = Genotype{dna: dna_1, fitness: 0.0, age: 0};
    let parent_2 = Genotype{dna: dna_2, fitness: 0.0, age: 0};

    //Getting the offspring
    let mut offspring = uniform_crossover::uniform(&parent_1, &parent_2).unwrap();

    //Setting the child
    let child_2 = offspring.pop().unwrap();
    let child_1 = offspring.pop().unwrap();

    //Checking that children have the same number of genes
    assert_eq!(child_1.dna.len(), parent_1.dna.len());
    assert_eq!(child_2.dna.len(), child_2.dna.len());
    assert_eq!(parent_1.dna.len(), parent_2.dna.len());
}