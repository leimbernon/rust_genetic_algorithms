# RUST genetic algorithms library
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/a934b8668dbf4412b3c63a7b275ad949)](https://app.codacy.com/gh/leimbernon/rust_genetic_algorithms?utm_source=github.com&utm_medium=referral&utm_content=leimbernon/rust_genetic_algorithms&utm_campaign=Badge_Grade_Settings) [![CircleCI](https://dl.circleci.com/status-badge/img/gh/leimbernon/rust_genetic_algorithms/tree/develop.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/leimbernon/rust_genetic_algorithms/tree/develop)

## Description
This library provides a simple framework to implement [genetic algorithms (GA)](https://en.wikipedia.org/wiki/Genetic_algorithm) with Rust.

## Table of content
-   [RUST genetic algorithms library](#rust-genetic-algorithms-library)
  -   [Description](#description)
  -   [Features](#features)
    -   [Traits](#traits)
    -   [Operators](#operators)
    -   [Population](#population)
    -   [Runner](#runner)
  -   [Example](#example)
  -   [Usage](#usage)

## Features

### Traits

This version uses traits for generic implementations.

These traits are within the `traits` module:

-   `GeneT`: This trait must be implemented on your own gene representation.
  -   `new()`: This is the constructor function.
  -   `get_id()`: This function must return the id of the gene.
-   `GenotypeT`: This trait must be implemented on your own genotype representation.
  -   `new()`: This is the constructor function.
  -   `get_dna()`: Must return the vector of genes (`GeneT`).
  -   `get_dna_mut()`: Must return the mutable vector of genes (`GeneT`), manily for the mutation operator.
  -   `calculate_phenotype()`: This function must calculate the fitness of the indivudual (or the genotype) in f64.
  -   `get_phenotype()`: Returns the fitness previously calculated by `calculate_phenotype()`.

### Operators

Within the module `operations` we have the following operators:

-   Crossover
  -   Cycle
-   Mutation
  -   Swap
-   Selection
  -   Random
-   Survivor
  -   Fitness Based

### Population

In genetic algorithms, operators are applied over a population of individuals, and over a set of rules (not yet implemented).
Within the `population` module, `Population` structure will define the population.

### Runner

Because genetic algorithms run over different generations, in this library there is a `start` function within module `ga` that facilitates the process.
This function will need the `GaConfiguration` structure which contains the operators to use, the maximum number of generations, and the problem solver (Maximization or Minimization), and the `Population` structure, which is in the `population` module.

## Example

A simple example of use could be the minimization of a genotype whose gene has only an id.

### Creation of the gene and genotype structure

Use the traits.
`use use genetic_algorithms::{ga::{GaConfiguration, ProblemSolving, run}, operations::{Selection, Crossover, Mutation, Survivor}, population::Population, traits::GenotypeT};`

Define the gene structure.

```
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Gene{
    pub id: i32,
}
impl GeneT for Gene{
    fn new()->Gene{
        return Gene{id: -1};
    }
    fn get_id(&self) -> &i32{
        return &self.id;
    }
}
```

Define the genotype structure, and the phenotype calculation.

```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Genotype<T: GeneT>{
    pub dna: Vec<T>,
    pub phenotype: f64,
}
impl <T: GeneT> GenotypeT<T> for Genotype<T>{
    fn get_dna(&self) -> &Vec<T> {
        &self.dna
    }
    fn get_dna_mut(&mut self) -> &mut Vec<T> {
        &mut self.dna
    }
    fn get_phenotype(&self) -> &f64 {
        return &self.phenotype;
    }
    fn calculate_phenotype(&mut self) {
        
        self.phenotype = 0.0;
        let mut position = 0;

        for i in &self.dna{
            let phenotype = f64::from(i.get_id()*position);
            self.phenotype += phenotype;
            position += 1;
        }
    }
    fn new() -> Self {
       return Genotype{
        dna: Vec::new(),
        phenotype: 0.0,
       }
    }
}
```

Define the configuration of the GA.

```
let configuration = GaConfiguration{
        problem_solving: ProblemSolving::Maximization,
        max_generations: 100,
        selection: Selection::Random,
        crossover: Crossover::Cycle,
        mutation: Mutation::Swap,
        survivor: Survivor::Fitness,
    };
```

Define the DNA, the individuals and the population.

```
let dna_1 = vec![Gene{id:1}, Gene{id:2}, Gene{id:3}, Gene{id:4}];
    let dna_2 = vec![Gene{id:2}, Gene{id:3}, Gene{id:4}, Gene{id:1}];
    let dna_3 = vec![Gene{id:3}, Gene{id:4}, Gene{id:1}, Gene{id:2}];
    let dna_4 = vec![Gene{id:4}, Gene{id:1}, Gene{id:2}, Gene{id:3}];
    let dna_5 = vec![Gene{id:2}, Gene{id:1}, Gene{id:3}, Gene{id:4}];
    let dna_6 = vec![Gene{id:1}, Gene{id:3}, Gene{id:4}, Gene{id:2}];
    let dna_7 = vec![Gene{id:3}, Gene{id:4}, Gene{id:2}, Gene{id:1}];
    let dna_8 = vec![Gene{id:4}, Gene{id:2}, Gene{id:1}, Gene{id:3}];
    let dna_9 = vec![Gene{id:2}, Gene{id:1}, Gene{id:4}, Gene{id:3}];
    let dna_10 = vec![Gene{id:1}, Gene{id:4}, Gene{id:3}, Gene{id:2}];

let individuals = vec![Genotype{dna: dna_1, phenotype: 1.0}, Genotype{dna: dna_2, phenotype: 2.0},
    Genotype{dna: dna_3, phenotype: 3.0}, Genotype{dna: dna_4, phenotype: 4.0}, Genotype{dna: dna_5, phenotype: 5.0}, 
    Genotype{dna: dna_6, phenotype: 6.0}, Genotype{dna: dna_7, phenotype: 7.0}, Genotype{dna: dna_8, phenotype: 8.0},
    Genotype{dna: dna_9, phenotype: 9.0}, Genotype{dna: dna_10, phenotype: 10.0}];

    let mut population = Population::new(individuals);
```

Finally, run the GA.

```
population = run(population, configuration);
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
genetic_algorithms = "0.1.1"
```