use std::fmt;

use crate::operations::{Crossover, Selection, Mutation, Survivor};


#[derive(Copy, Clone, PartialEq)]
pub enum ProblemSolving {
    Minimization,
    Maximization,
    FixedFitness,
}
impl fmt::Display for ProblemSolving {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProblemSolving::Minimization => write!(f, "Minimization"),
            ProblemSolving::Maximization => write!(f, "Maximization"),
            ProblemSolving::FixedFitness => write!(f, "FixedFitness"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Copy, Clone)]
pub struct SelectionConfiguration{
    pub number_of_couples: Option<i32>,
    pub method: Selection,
}
impl Default for SelectionConfiguration{
    fn default() -> Self {
        SelectionConfiguration { 
            number_of_couples: None, 
            method: Selection::Tournament 
        }
    }
}

#[derive(Copy, Clone)]
pub struct CrossoverConfiguration{
    pub number_of_points: Option<i32>,
    pub probability_max: Option<f64>,
    pub probability_min: Option<f64>,
    pub method: Crossover,
}
impl Default for CrossoverConfiguration{
    fn default() -> Self {
        CrossoverConfiguration { 
            number_of_points: None, 
            probability_max: None, 
            probability_min: None,
            method: Crossover::Uniform
        }
    }
}

#[derive(Copy, Clone)]
pub struct MutationConfiguration{
    pub probability: Option<f64>,
    pub method: Mutation,
}
impl Default for MutationConfiguration {
    fn default() -> Self {
        MutationConfiguration { 
            probability: None, 
            method: Mutation::Swap, 
        }
    }
}


#[derive(Copy, Clone)]
pub struct LimitConfiguration{
    pub problem_solving: ProblemSolving,
    pub max_generations: i32,
    pub fitness_target: Option<f64>, 
    pub get_best_individual_by_generation: Option<bool>,
}
impl Default for LimitConfiguration {
    fn default() -> Self {
        LimitConfiguration { 
            problem_solving: ProblemSolving::Minimization, 
            max_generations: 100, 
            fitness_target: None, 
            get_best_individual_by_generation: None
        }
    }
}

#[derive(Copy, Clone)]
pub struct GaConfiguration {
    pub adaptive_ga: bool,
    pub number_of_threads: Option<i32>,
    pub limit_configuration: LimitConfiguration,
    pub selection_configuration: SelectionConfiguration,
    pub crossover_configuration: CrossoverConfiguration,
    pub mutation_configuration: MutationConfiguration,
    pub survivor: Survivor,
    pub log_level: Option<LogLevel>,
}
impl Default for GaConfiguration{
    fn default() -> Self {
        GaConfiguration { 
            adaptive_ga: false, 
            number_of_threads: None, 
            limit_configuration: LimitConfiguration { ..Default::default() }, 
            selection_configuration: SelectionConfiguration { ..Default::default() }, 
            crossover_configuration: CrossoverConfiguration { ..Default::default() }, 
            mutation_configuration: MutationConfiguration { ..Default::default() }, 
            survivor: Survivor::Fitness, 
            log_level: None 
        }
    }
}