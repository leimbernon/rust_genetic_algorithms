# RUST genetic algorithms library
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/a934b8668dbf4412b3c63a7b275ad949)](https://app.codacy.com/gh/leimbernon/rust_genetic_algorithms?utm_source=github.com&utm_medium=referral&utm_content=leimbernon/rust_genetic_algorithms&utm_campaign=Badge_Grade_Settings)
[![Rust Unit Tests](https://github.com/leimbernon/rust_genetic_algorithms/actions/workflows/rust-unit-tests.yml/badge.svg)](https://github.com/leimbernon/rust_genetic_algorithms/actions/workflows/rust-unit-tests.yml)

## Description
This library provides a simple framework for implementing [genetic algorithms (GA)](https://en.wikipedia.org/wiki/Genetic_algorithm) with Rust.

This library also supports multithreading and adaptive genetic algorithms.

## Table of content
- [RUST genetic algorithms library](#rust-genetic-algorithms-library)
  - [Description](#description)
  - [Table of content](#table-of-content)
  - [Documentation](#documentation)
  - [Features](#features)
    - [Traits](#traits)
    - [Operators](#operators)
    - [Population](#population)
    - [Runner](#runner)
    - [Initialization](#initialization)
    - [GA Configuration](#ga-configuration)
  - [Example](#example)
    - [Creation of the gene and genotype structure](#creation-of-the-gene-and-genotype-structure)
    - [Other examples](#other-examples)
  - [Usage](#usage)

## Documentation

See [docs.rs](https://docs.rs/genetic_algorithms/latest/genetic_algorithms)

## Features

### Traits

This release uses traits for generic implementations.

These traits are inside the `traits` module:

- `GeneT`: This trait must be implemented on your own gene representation.
  - `new()`: This is the constructor function.
  - `get_id()`: Optional. This function must return the id of the gene.
- `GenotypeT`: This trait must be implemented on your own genotype representation.
  - `Gene`: This is the `GeneT` associated type.
  - `new()`: This is the constructor function.
  - `new_gene()`: Optional. Must return `Self::Gene`.
  - `get_dna()`: Must return the array of genes (`GeneT`).
  - `set_dna(dna: &[Self::Gene])`: Must set the array of genes (`GeneT`).
  - `set_gene(gene_index: usize, gene: Self::Gene)`: Optional. This method replaces a gene at the specified gene_index position.
  - `calculate_fitness()`: Optional. This function must calculate the fitness of the individual (or the genotype) in f64.
  - `get_fitness()`: Returns the fitness previously calculated by `calculate_fitness()`.
  - `set_fitness(fitness: f64)`: Sets the fitness value.
  - `get_age()`: Returns the age of the genotype.
  - `set_age(age: i32)`: Sets the age of the genotype.

### Operators

Within the `operations` module we have the following operators:

- Crossover
  - Cycle
  - Multipoint
  - Uniform
- Mutation
  - Swap
  - Inversion
  - Scramble
- Selection
  - Random
  - Roulette Wheel
  - Stochastic Universal Sampling
  - Tournament
- Survivor
  - Fitness based
  - Age based

### Population

In genetic algorithms, operators are applied over a population of individuals, and over a set of rules (not yet implemented).
Within the `population` module, the `Population` structure will define the population.

### Runner

Since genetic algorithms run over several generations, there is a `start` function in this library within the `ga` module that facilitates the process.
This function needs the `GaConfiguration` structure, which contains the operators to be used, the maximum number of generations, the problem solver (Maximization or Minimization), etc., and the `Population` structure, which is in the `population` module.

### Initialization

If it is desired to perform the initialization of the population randomly through this library, it can be done through the function `random_initialization`. This function requires the following parameters:
- `alleles`: This is an array containing the elements that can be combined in the population. They have to comply with the `GenotypeT` trait.
- `population_size`: Indicates the size of the population we want to have.
- `genes_per_individual`: Sets the number of genes that each individual in the population has.
- `needs_unique_ids`: This variable indicates whether each gene of each individual has to contain a unique ID. This is useful mainly for the `cycle` crossover method.
- `alleles_can_be_repeated`: Within an individual, the same allele can be repeated several times. This variable indicates whether this behavior is desired, or whether it is preferred that the alleles be unique within each individual.

### GA Configuration

Within this library, you can configure the way genetic algorithms are executed by using the configuration structure `GaConfiguration`.
This structure has the following attributes:
- `adaptive_ga`: Specifies if the Genetic Algorithms are adaptive or not.
- `number_of_threads`: Optional. Indicates how many threads will be executed simultaneously.
- `limit_configuration`: It configures the limits of the Genetic Algorithms with the `LimitConfiguration` structure.
- `selection_configuration`: It configures the selection method with the `SelectionConfiguration` structure.
- `crossover_configuration`: It configures the crossover method with the `CrossoverConfiguration` structure.
- `mutation_configuration`: It configures the mutation method with the `MutationConfiguration` structure.
- `survivor`: Specifies which survivor operator to use.
- `log_level`: Optional. It configures the maximum log level we want to have. If this value is none, logs will be disabled.

`SelectionConfiguration`:
- `number_of_couples`: Optional. This attribute applies only to stochastic universal sampling. It specifies the number of pairs to select from the population.
- `method`: Specifies which selection operator to use.

`CrossoverConfiguration`:
- `number_of_points`: Optional. This attribute is only valid for crossover multipoint and indicates how many points are made within the DNA during crossover operations.
- `probability_max`: Optional. Specifies the maximum probability that two parents are crossed. This number must be between 0.0 and 1.0, both inclusive. In case of adaptive genetic algorithms, this parameter is mandatory and must be greater than `probability_min`.
- `probability_min`: Optional. Specifies the minimum probability that two parents are crossed. This number must be between 0.0 and 1.0, both inclusive. In case of adaptive genetic algorithms, this parameter is mandatory and must be lower than `probability_max`.
- `method`: Specifies which crossover operator to use.

`MutationConfiguration`:
- `probability_max`: Optional. Specifies the maximum probability that a genotype is mutated. This number must be between 0.0 and 1.0, both inclusive. In case of adaptive genetic algorithms, this parameter is mandatory and must be greater than `probability_min`.
- `probability_min`: Optional. Specifies the minimum probability that a genotype is mutated. This number must be between 0.0 and 1.0, both inclusive. In case of adaptive genetic algorithms, this parameter is mandatory and must be lower than `probability_max`.

- `method`: Specifies which mutation operator to use.

`LimitConfiguration`:
- `problem_solving`: You can choose between a minimization problem and a maximization problem.
- `max_generations`: If the result is not optimal, this attribute indicates the maximum number of generations to run before stopping.
- `fitness_target`: Optional. The fitness of the best individual.
- `get_best_individual_by_generation`: Optional. Tells the runner to return the best individual by generation.

## Example

A simple example of use could be minimizing a genotype whose gene has only one id.
### Creation of the gene and genotype structure

Use the traits.
`use genetic_algorithms::{operations::{Selection, Crossover, Mutation, Survivor}, population::Population, traits::{GenotypeT, ConfigurationT}, configuration::ProblemSolving, ga};`

Define the gene structure.

```rust
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
    fn set_id(&mut self, id: i32) {
        self.id = id;
    }
}
```

Define the genotype structure, and the fitness calculation.

```rust
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Genotype{
    pub dna: Vec<Gene>,
    pub fitness: f64,
    pub age: i32,
}
impl GenotypeT for Genotype{
    type Gene = Gene;
    fn get_dna(&self) -> &[Self::Gene] {
        &self.dna
    }
    fn get_fitness(&self) -> f64 {
        return self.fitness;
    }
    fn set_fitness(&mut self, fitness: f64) {
        self.fitness = fitness;
    }
    fn set_age(&mut self, age:i32){
        self.age = age;
    }
    fn get_age(&self) -> i32 {
        self.age
    }
    fn calculate_fitness(&mut self) {
        
        self.fitness = 0.0;
        let mut position = 0;

        for i in &self.dna{
            let fitness = f64::from(i.get_id()*position);
            self.fitness += fitness;
            position += 1;
        }
    }
    fn new() -> Self {
        return Genotype{
            dna: Vec::new(),
            fitness: 0.0,
            age: 0,
        }
    }
    fn set_dna(&mut self, dna: &[Self::Gene]){
        self.dna = dna.to_vec();
    }
}
```

Define the Alleles, and initialize the population.

```rust
  let binding =  vec![Gene{id:1}, Gene{id:2}, Gene{id:3}, Gene{id:4},
                                   Gene{id:5}, Gene{id:6}, Gene{id:7}, Gene{id:8}];
  let alleles = binding.as_slice();
  static GENES_PER_INDIVIDUAL: i32 = 6;
  static POPULATION_SIZE: i32 = 100;
  static NEEDS_UNIQUE_IDS: bool = false;
  static ALLELES_CAN_BE_REPEATED: bool = false;
  static NUMBER_OF_THREADS: i32 = 8;

  let mut population = ga::random_initialization::<Genotype>(alleles, POPULATION_SIZE, GENES_PER_INDIVIDUAL, NEEDS_UNIQUE_IDS, ALLELES_CAN_BE_REPEATED, NUMBER_OF_THREADS);

```

Finally, configure and run the GA.

```rust
population = ga::Ga::new()
                .with_problem_solving(ProblemSolving::Maximization)
                .with_selection_method(Selection::Random)
                .with_crossover_method(Crossover::Cycle)
                .with_mutation_method(Mutation::Swap)
                .with_survivor_method(Survivor::Fitness)
                .run(population);
```

### Other examples
- Travelling salesman problem: [https://en.wikipedia.org/wiki/Travelling_salesman_problem](https://en.wikipedia.org/wiki/Travelling_salesman_problem)
  - See [https://github.com/leimbernon/traveller_problem](https://github.com/leimbernon/traveller_problem)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
genetic_algorithms = "1.2.0"
```