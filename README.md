# RUST genetic algorithms library
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/a934b8668dbf4412b3c63a7b275ad949)](https://app.codacy.com/gh/leimbernon/rust_genetic_algorithms?utm_source=github.com&utm_medium=referral&utm_content=leimbernon/rust_genetic_algorithms&utm_campaign=Badge_Grade_Settings) [![CircleCI](https://dl.circleci.com/status-badge/img/gh/leimbernon/rust_genetic_algorithms/tree/develop.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/leimbernon/rust_genetic_algorithms/tree/develop)

## Description
This library provides a simple framework to implement [genetic algorithms (GA)](https://en.wikipedia.org/wiki/Genetic_algorithm) with Rust.

This library also supports multithreading.

## Table of content
- [RUST genetic algorithms library](#rust-genetic-algorithms-library)
  - [Description](#description)
  - [Table of content](#table-of-content)
  - [Features](#features)
    - [Traits](#traits)
    - [Operators](#operators)
    - [Population](#population)
    - [Runner](#runner)
    - [GA Configuration](#ga-configuration)
  - [Example](#example)
    - [Creation of the gene and genotype structure](#creation-of-the-gene-and-genotype-structure)
  - [Usage](#usage)

## Features

### Traits

This version uses traits for generic implementations.

These traits are within the `traits` module:

- `GeneT`: This trait must be implemented on your own gene representation.
  - `new()`: This is the constructor function.
  - `get_id()`: This function must return the id of the gene.
- `GenotypeT`: This trait must be implemented on your own genotype representation.
  - `new()`: This is the constructor function.
  - `get_dna()`: Must return the vector of genes (`GeneT`).
  - `get_dna_mut()`: Must return the mutable vector of genes (`GeneT`), manily for the mutation operator.
  - `calculate_fitness()`: This function must calculate the fitness of the indivudual (or the genotype) in f64.
  - `get_fitness()`: Returns the fitness previously calculated by `calculate_fitness()`.
  - `get_age()`: Returns the age of the genotype.
  - `get_age_mut()`: Must return the mutable age of the genotype.

### Operators

Within the module `operations` we have the following operators:

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
Within the `population` module, `Population` structure will define the population.

### Runner

Because genetic algorithms run over different generations, in this library there is a `start` function within module `ga` that facilitates the process.
This function will need the `GaConfiguration` structure which contains the operators to use, the maximum number of generations, the problem solver (Maximization or Minimization), etc, and the `Population` structure, which is in the `population` module.

### GA Configuration

Within this library you can configure the way to run genetic algorithms through the configuration structure `GaConfiguration`.
This structure contains the following attributes:
- `number_of_threads`: Optional. It indicates how many threads will be executed at the same time.
- `limit_configuration`: It configures the limits of the Genetic Algorithms with the `LimitConfiguration` structure.
- `selection_configuration`: Optional. It configures the selection method with the `SelectionConfiguration` structure.
- `crossover_configuration`: Optional. It configures the crossover method with the `CrossoverConfiguration` structure.
- `selection`: Indicates what selection operator to use.
- `crossover`: Indicates what crossover operator to use.
- `mutation`: Indicates what mutation operator to use.
- `survivor`: Indicates what survivor operator to use.

`SelectionConfiguration`:
- `number_of_couples`: This attribute is only valid for stochastic universal sampling. It indicates the number of couples to select from the population.

`CrossoverConfiguration`:
- `number_of_points`: This attribute is only valid for crossover multipoint, and it indicates how many points will be made within the dna in crossover operations.

`LimitConfiguration`:
- `problem_solving`: You can select from a Minimization problem or a Maximization problem.
- `max_generations`: In case of not getting the optimal result, this attribute indicates the maximum number of generations to execute before stopping.
- `fitness_target`: Optional. The fitness of the best individual.

## Example

A simple example of use could be the minimization of a genotype whose gene has only an id.

### Creation of the gene and genotype structure

Use the traits.
`use genetic_algorithms::{ga::run, operations::{Selection, Crossover, Mutation, Survivor}, population::Population, traits::GenotypeT, configuration::{GaConfiguration, ProblemSolving, LimitConfiguration}};`

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

Define the genotype structure, and the fitness calculation.

```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Genotype<T: GeneT>{
    pub dna: Vec<T>,
    pub fitness: f64,
    pub age: i32,
}
impl <T: GeneT> GenotypeT<T> for Genotype<T>{
    fn get_dna(&self) -> &Vec<T> {
        &self.dna
    }
    fn get_dna_mut(&mut self) -> &mut Vec<T> {
        &mut self.dna
    }
    fn get_fitness(&self) -> &f64 {
        return &self.fitness;
    }
     fn get_age_mut(&mut self) -> &mut i32 {
        &mut self.age
    }
    fn get_age(&self) -> &i32 {
        &self.age
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
}
```

Define the configuration of the GA.

```
let configuration = GaConfiguration{
        number_of_threads: Some(2),
        limit_configuration: LimitConfiguration{max_generations: 100, fitness_target: None, problem_solving: ProblemSolving::Maximization},
        selection_configuration: Some(SelectionConfiguration{number_of_couples: 10}),
        crossover_configuration: None,
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

let individuals = vec![Genotype{dna: dna_1, fitness: 1.0, age: 0}, Genotype{dna: dna_2, fitness: 2.0, age: 0},
    Genotype{dna: dna_3, fitness: 3.0, age: 0}, Genotype{dna: dna_4, fitness: 4.0, age: 0}, Genotype{dna: dna_5, fitness: 5.0, age: 0}, Genotype{dna: dna_6, fitness: 6.0, age: 0}, Genotype{dna: dna_7, fitness: 7.0, age: 0}, Genotype{dna: dna_8, fitness: 8.0, age: 0}, Genotype{dna: dna_9, fitness: 9.0, age: 0}, Genotype{dna: dna_10, fitness: 10.0, age: 0}];

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
genetic_algorithms = "0.7.0"
```