use crate::{configuration::{LogLevel, ProblemSolving}, operations::{Survivor, Selection, Crossover, Mutation}};

pub trait GeneT: Default + Clone + Copy + Sync + Send {
    fn new() -> Self;
    fn get_id(&self) -> i32{0}
    fn set_id(&mut self, id: i32);
}

pub trait GenotypeT: Clone{

    type Gene: GeneT;
    
    fn new() -> Self;
    fn new_gene() -> Self::Gene{
        Self::Gene::new()
    }
    fn get_dna(&self) -> &[Self::Gene];
    fn set_dna(&mut self, dna: &[Self::Gene]);
    fn set_gene(&mut self, gene_index: usize, gene: Self::Gene){
        let mut dna_temp = self.get_dna().to_vec();
        dna_temp[gene_index] = gene;
        self.set_dna(dna_temp.as_slice());
    }
    fn calculate_fitness(&mut self);
    fn get_fitness(&self) -> f64;
    fn set_fitness(&mut self, fitness: f64);
    fn set_age(&mut self, age: i32);
    fn get_age(&self) -> i32;

    fn get_fitness_distance(&self, fitness_target: &f64) -> f64 {
        (fitness_target - self.get_fitness()).abs()
    }
}

pub trait ConfigurationT{
    fn new() -> Self;
    fn with_adaptive_ga(&mut self, adaptive_ga: bool) -> &mut Self;
    fn with_threads(&mut self, number_of_threads: i32)-> &mut Self;
    fn with_logs(&mut self, log_level: LogLevel) -> &mut Self;
    fn with_survivor_method(&mut self, method: Survivor) -> &mut Self;

    //Limit configuration
    fn with_problem_solving(&mut self, problem_solving: ProblemSolving)->&mut Self;
    fn with_max_generations(&mut self, max_generations: i32)-> &mut Self;
    fn with_fitness_target(&mut self, fitness_target: f64)-> &mut Self;
    fn with_best_individual_by_generation(&mut self, best_individual_by_generation: bool) -> &mut Self;

    //Selection configuration
    fn with_number_of_couples(&mut self, number_of_couples: i32)->&mut Self;
    fn with_selection_method(&mut self, selection_method: Selection)->&mut Self;

    //Crossover configuration
    fn with_crossover_number_of_points(&mut self, number_of_points: i32)->&mut Self;
    fn with_crossover_probability_max(&mut self, probability_max: f64)->&mut Self;
    fn with_crossover_probability_min(&mut self, probability_min: f64) -> &mut Self;
    fn with_crossover_method(&mut self, method: Crossover) -> &mut Self;

    //Mutation configuration
    fn with_mutation_probability_max(&mut self, probability_max: f64)->&mut Self;
    fn with_mutation_probability_min(&mut self, probability_min: f64) -> &mut Self;
    fn with_mutation_method(&mut self, method: Mutation) -> &mut Self;

}