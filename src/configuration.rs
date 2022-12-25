use crate::{operations::{Crossover, Selection, Mutation, Survivor}};


#[derive(Copy, Clone, PartialEq)]
pub enum ProblemSolving {
    Minimization,
    Maximization,
}

#[derive(Copy, Clone)]
pub struct SelectionConfiguration{
    pub number_of_couples: i32,
}

#[derive(Copy, Clone)]
pub struct CrossoverConfiguration{
    pub number_of_points: i32,
}

#[derive(Copy, Clone)]
pub struct LimitConfiguration{
    pub max_generations: i32,
    pub fitness_target: Option<f64>, 
}

#[derive(Copy, Clone)]
pub struct GaConfiguration {
    pub problem_solving: ProblemSolving,
    pub limit_configuration: LimitConfiguration,
    pub selection_configuration: Option<SelectionConfiguration>,
    pub crossover_configuration: Option<CrossoverConfiguration>,
    pub selection: Selection,
    pub crossover: Crossover,
    pub mutation: Mutation,
    pub survivor: Survivor,
}