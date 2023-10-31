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

pub trait Configuration{
    fn new() -> Self;
    fn with_adaptive_ga(self, adaptive_ga: bool) -> Self;
    fn with_threads(self, number_of_threads: i32)-> Self;
    fn with_logs(self, log_level: LogLevel) -> Self;
    fn with_survivor_method(self, method: Survivor) -> Self;

    //Limit configuration
    fn with_problem_solving(self, problem_solving: ProblemSolving)-> Self;
    fn with_max_generations(self, max_generations: i32)-> Self;
    fn with_fitness_target(self, fitness_target: f64)-> Self;
    fn with_best_individual_by_generation(self, best_individual_by_generation: bool) -> Self;

    //Selection configuration
    fn with_number_of_couples(self, number_of_couples: i32)->Self;
    fn with_selection_method(self, selection_method: Selection)->Self;

    //Crossover configuration
    fn with_crossover_number_of_points(self, number_of_points: i32)->Self;
    fn with_crossover_probability_max(self, probability_max: f64)->Self;
    fn with_crossover_probability_min(self, probability_min: f64) -> Self;
    fn with_crossover_method(self, method: Crossover) -> Self;

    //Mutation configuration
    fn with_mutation_probability_max(self, probability_max: f64)->Self;
    fn with_mutation_probability_min(self, probability_min: f64) -> Self;
    fn with_mutation_method(self, method: Mutation) -> Self;

}