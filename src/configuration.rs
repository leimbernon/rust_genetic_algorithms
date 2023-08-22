use crate::operations::{Crossover, Selection, Mutation, Survivor};


#[derive(Copy, Clone, PartialEq)]
pub enum ProblemSolving {
    Minimization,
    Maximization,
    FixedFitness,
}

#[derive(Copy, Clone)]
pub struct SelectionConfiguration{
    pub number_of_couples: i32,
}

#[derive(Copy, Clone)]
pub struct CrossoverConfiguration{
    pub number_of_points: Option<i32>,
    pub probability: Option<f64>,
    pub method: Crossover,
}
#[derive(Copy, Clone)]
pub struct MutationConfiguration{
    pub probability: Option<f64>,
    pub method: Mutation,
}


#[derive(Copy, Clone)]
pub struct LimitConfiguration{
    pub problem_solving: ProblemSolving,
    pub max_generations: i32,
    pub fitness_target: Option<f64>, 
    pub get_best_individual_by_generation: Option<bool>,
}

#[derive(Copy, Clone)]
pub struct GaConfiguration {
    pub number_of_threads: Option<i32>,
    pub limit_configuration: LimitConfiguration,
    pub selection_configuration: Option<SelectionConfiguration>,
    pub crossover_configuration: CrossoverConfiguration,
    pub mutation_configuration: MutationConfiguration,
    pub selection: Selection,
    pub survivor: Survivor,
}