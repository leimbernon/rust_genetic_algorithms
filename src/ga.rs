use std::{sync::{mpsc::sync_channel, Mutex, Arc}, thread, collections::HashMap};
use rand::Rng;
use log::{trace, debug, info};
use std::env;

use crate::{population::Population, traits::GenotypeT, operations::{selection, crossover, mutation, survivor}, configuration::{ProblemSolving, LimitConfiguration, LogLevel}};
use crate::configuration::GaConfiguration;

/**
 * Function to run the genetic algorithms cycle
 */
pub fn run<U>(mut population: Population<U>, configuration: GaConfiguration)->Population<U>
where 
U:GenotypeT + Send + Sync + 'static + Clone
{

    //We set the environment variable from the configuration value
    let key = "RUST_LOG";
    let log_level = if configuration.log_level.is_none(){log::LevelFilter::Off}else{match configuration.log_level.unwrap(){
        LogLevel::Off => log::LevelFilter::Off,
        LogLevel::Error => log::LevelFilter::Error,
        LogLevel::Warn => log::LevelFilter::Warn,
        LogLevel::Info => log::LevelFilter::Info,
        LogLevel::Debug => log::LevelFilter::Debug,
        LogLevel::Trace => log::LevelFilter::Trace,
    }};
    env::set_var(key, log_level.as_str());
    let _ = env_logger::try_init();

    //Best individual within the generations and population returned
    let initial_population_size = population.size();
    let mut age = 0;

    //Calculation of the fitness and the best individual
    let mut best_individual = population_fitness_calculation_multithread(&mut population.individuals, configuration);
    let mut best_population: Population<U> = Population::new_empty();

    //We start the cycles
    for i in 0..configuration.limit_configuration.max_generations {

        info!(target="ga_events", method="run"; "Generation number: {}", i+1);
        age += 1;

        //1- Parent selection for reproduction
        let mut parents = selection::factory(&population.individuals, configuration.selection_configuration, configuration.number_of_threads.unwrap_or(1));
        debug!(target="ga_events", method="run"; "Parents selected for reproduction");

        //2- Getting the offspring from the multithreading function
        let mut offspring = parent_crossover_multithread(&mut parents, &population.individuals, &configuration, age);
        debug!(target="ga_events", method="run"; "Offspring created");

        //3- Sets the best individual
        for child in &offspring{
            best_individual = get_best_individual(&best_individual, child, configuration.limit_configuration.problem_solving);
        }
        debug!(target="ga_events", method="run"; "Best individual calculated - generation {}", i+1);

        //3.1- If we want to return the best individual by generation
        if configuration.limit_configuration.get_best_individual_by_generation.is_some() {
            best_population.add_individual_gn(best_individual.clone(), i);
        }

        //4- Insert the children in the population
        population.individuals.append(&mut offspring);

        //5- Survivor selection
        survivor::factory(configuration.survivor, &mut population.individuals, initial_population_size, configuration.limit_configuration);
        debug!(target="ga_events", method="run"; "Survivors selected");

        //6- Identifies if the limit has been reached or not
        if limit_reached(configuration.limit_configuration, &population.individuals){
            break;
        }
    }

    //If it's not required to return the best individuals by generation
    if (configuration.limit_configuration.get_best_individual_by_generation.is_some() && !configuration.limit_configuration.get_best_individual_by_generation.unwrap()) || configuration.limit_configuration.get_best_individual_by_generation.is_none() {
        best_population.add_individual_gn(best_individual, -1);
    }
    best_population
}

/**
 * Function to determine which of the individuals is the best individual and return the best of them
 */
fn get_best_individual<U>(individual_1: &U, individual_2: &U, problem_solving: ProblemSolving) -> U
where
U:GenotypeT
{

    debug!(target="ga_events", method="get_best_individual"; "Started the best individual method");
    let mut best_individual = U::new();
    trace!(target="ga_events", method="get_best_individual"; "Individual 1 fitness: {} - Individual 2 fitness: {}", individual_1.get_fitness(), individual_2.get_fitness());

    if problem_solving == ProblemSolving::Maximization {

        //We check if the fitness is the best and store it if it's the case
        if individual_1.get_fitness() >= individual_2.get_fitness(){
            best_individual.set_dna(individual_1.get_dna());
            best_individual.set_fitness(individual_1.get_fitness());
        }else{
            best_individual.set_dna(individual_2.get_dna());
            best_individual.set_fitness(individual_2.get_fitness());
        }

    } else {

        //We check if the fitness is the best and store it if it's the case
        if individual_1.get_fitness() >= individual_2.get_fitness(){
            best_individual.set_dna(individual_2.get_dna());
            best_individual.set_fitness(individual_2.get_fitness());
        }else{
            best_individual.set_dna(individual_1.get_dna());
            best_individual.set_fitness(individual_1.get_fitness());
        }

    }

    debug!(target="ga_events", method="get_best_individual"; "Best individual method finished");
    best_individual
}

/**
 * Function to indentify if the limit has been reached or not in the current generation
 */
fn limit_reached<U>(limit: LimitConfiguration, individuals: &Vec<U>)->bool
where
U:GenotypeT
{

    debug!(target="ga_events", method="limit_reached"; "Started limit reached method");
    let mut result = false;

    if limit.problem_solving == ProblemSolving::Minimization{
        //If the problem solving is minimization, fitness must be 0
        for genotype in individuals {
            if genotype.get_fitness() == 0.0 {
                trace!(target="ga_events", method="limit_reached"; "limit reached for minimization");
                result = true;
                break;
            }
        }
    }else if limit.problem_solving == ProblemSolving::FixedFitness{

        //We check that fitness target is not none
        if limit.fitness_target.is_none(){
            panic!("For FixedFitness problems, fitness_target must be set.");
        }

        //If the problem solving is a fixed fitness
        for genotype in individuals {
            if genotype.get_fitness() == limit.fitness_target.unwrap() {
                trace!(target="ga_events", method="limit_reached"; "limit reached for fixed fitness");
                result = true;
                break;
            }
        }
    }

    debug!(target="ga_events", method="limit_reached"; "Limit reached method finished");
    result
}

/**
 * Sets the population fitness, age and the best individual
 */
fn population_fitness_calculation_multithread<U>(individuals: &mut Vec<U>, configuration: GaConfiguration) -> U
where
U:GenotypeT + Send + Sync + 'static + Clone
{

    debug!(target="ga_events", method="population_fitness_calculation_multithread"; "Started the population fitness calculation");
    let mut number_of_threads = configuration.number_of_threads.unwrap_or(1);
    let (tx, rx) = sync_channel(number_of_threads as usize);

    //Division of the individuals in different threads
    number_of_threads = if number_of_threads > individuals.len() as i32 {individuals.len() as i32} else {number_of_threads};

    //Setting the starting point and the jump
    let mut start_index = 0;
    let mut jump = individuals.len() as i32 / number_of_threads;

    //Cloning the individuals for multithreading
    let individuals_t = Vec::from_iter(individuals[..].iter().cloned());
    let individuals_t = Arc::new(Mutex::new(individuals_t));

    let best_individual_t = Arc::new(Mutex::new(U::new()));

    //Walking through the threads
    for _ in 0..number_of_threads {

        //We calculate the next jump
        if jump > individuals.len() as i32 - (start_index + jump) {
            jump += individuals.len() as i32 - (start_index + jump);
        }

        //Cloning the information from the main thread
        let (start_index_t, tx, jump_t, individuals_t, best_individual_t) = (start_index, tx.clone(),  jump, Arc::clone(&individuals_t), Arc::clone(&best_individual_t));

        //Starting the thread management
        thread::spawn(move || {

            let mut fitness_map = HashMap::new();
            let mut best_individual = U::new();

            //Calculates the fitness from the corresponding population
            for i in start_index_t..(start_index_t + jump_t){
                individuals_t.lock().unwrap()[i as usize].calculate_fitness();
                fitness_map.insert(i as usize, individuals_t.lock().unwrap()[i as usize].get_fitness());

                if !best_individual.get_dna().is_empty() {
                    best_individual = get_best_individual(&best_individual, &individuals_t.lock().unwrap()[i as usize], configuration.limit_configuration.problem_solving);
                } else{
                    best_individual.set_dna(individuals_t.lock().unwrap()[i as usize].get_dna());
                    best_individual.set_fitness(individuals_t.lock().unwrap()[i as usize].get_fitness());
                }
            }

            //Setting the best global individual
            if !best_individual_t.lock().unwrap().get_dna().is_empty() {
                let global_best_individual = get_best_individual(&best_individual_t.lock().unwrap().clone(), &best_individual, configuration.limit_configuration.problem_solving);
                best_individual_t.lock().unwrap().set_dna(global_best_individual.get_dna());
                best_individual_t.lock().unwrap().set_fitness(global_best_individual.get_fitness());
            }else{
                best_individual_t.lock().unwrap().set_dna(best_individual.get_dna());
                best_individual_t.lock().unwrap().set_fitness(best_individual.get_fitness());
            }

            //Sending the result
            tx.send(fitness_map).unwrap();
        });

        start_index += jump;
    }

    drop(tx);

    //We receive from the threads and set the fitness in individuals
    for received in rx {
        for element in received{
            individuals[element.0].set_fitness(element.1);
        }
    }

    let mut best_individual = U::new();
    best_individual.set_dna(best_individual_t.lock().unwrap().get_dna());
    best_individual.set_fitness(best_individual_t.lock().unwrap().get_fitness());

    debug!(target="ga_events", method="population_fitness_calculation_multithread"; "Population fitness calculation finished");

    best_individual
}

/**
 * Function for parent crossover in multithreading
 */
fn parent_crossover_multithread<U>(parents: &mut HashMap<usize, usize>, individuals: &Vec<U>, configuration: &GaConfiguration, age: i32) -> Vec<U>
where 
U:GenotypeT + Send + Sync + 'static + Clone
{
    //Setting the control variables
    debug!(target="ga_events", method="parent_crossover_multithread"; "Started the parent crossover");
    let mut number_of_threads = if configuration.number_of_threads.is_none() {1} else {configuration.number_of_threads.unwrap()}; 
    number_of_threads = if number_of_threads > parents.len() as i32 {parents.len() as i32}else{number_of_threads};
    let jump = parents.len() / number_of_threads as usize;

    let mut handles = Vec::new();
    let offspring = Arc::new(Mutex::new(Vec::new()));

    //Run all the threads
    for t in 0..number_of_threads{

        //We copy the parents that we want to crossover inside the thread
        let (individuals, configuration, offspring) = (individuals.clone(), *configuration, Arc::clone(&offspring));
        let mut parents_t = HashMap::new();
        let parents_c = parents.clone();

        for (index, i) in parents_c.keys().enumerate(){

            //If we reach the number of crossovers / thread
            if t < number_of_threads - 1 && index >= jump {
                break;
            }

            let key = *parents.get_key_value(i).unwrap().0;
            parents_t.insert(key, *parents.get_key_value(i).unwrap().1);
            parents.remove(&key);
        }

        //Starts the thread
        let handle = thread::spawn(move || {

            //Getting random numbers in this thread
            let mut rng = rand::thread_rng();

            for(key, value) in parents_t.iter(){
                //Getting the parent 1 and 2 for crossover                
                let parent_1 = individuals.get(*key).unwrap().clone();
                let parent_2 = individuals.get(*value).unwrap().clone();

                //Making the crossover of the parents when the random number is below or equal to the given probability
                let crossover_probability = rng.gen_range(0.0..1.0);
                let crossover_probability_config = if configuration.crossover_configuration.probability.is_none(){1.0}else{configuration.crossover_configuration.probability.unwrap()};
                
                debug!(target="ga_events", method="parent_crossover_multithread"; "Started the parent crossover");

                let mut child_1: U;
                let mut child_2: U;
                let mut offspring_t: Vec<U> = vec![];

                if crossover_probability <= crossover_probability_config {
                    offspring_t = crossover::factory(&parent_1, &parent_2, configuration.crossover_configuration).unwrap();
                    child_1 = offspring_t.pop().unwrap();
                    child_2 = offspring_t.pop().unwrap();
                }else{
                    child_1 = parent_1;
                    child_2 = parent_2;
                }

                //Making the mutation of each child when the random number is below or equal the given probability
                let mut mutation_probability = rng.gen_range(0.0..1.0);
                let mutation_probability_config = if configuration.mutation_configuration.probability.is_none(){1.0}else{configuration.mutation_configuration.probability.unwrap()};
                debug!(target="ga_events", method="parent_crossover_multithread"; "mutation_probability_config {} - mutation probability {}", mutation_probability_config, mutation_probability);

                if mutation_probability < mutation_probability_config {
                    mutation::factory(configuration.mutation_configuration.method, &mut child_1);
                }

                mutation_probability = rng.gen_range(0.0..1.0);
                if mutation_probability <= mutation_probability_config {
                    mutation::factory(configuration.mutation_configuration.method, &mut child_2);
                }

                //Calculate the fitness of both children and set their age
                child_1.calculate_fitness();
                child_2.calculate_fitness();

                child_1.set_age(age);
                child_2.set_age(age);

                //Adds the children in the offspring
                offspring_t.push(child_1);
                offspring_t.push(child_2);
                
                //Then sets the offspring in the result vector
                offspring.lock().unwrap().append(&mut offspring_t);
            }
            
        });
        handles.push(handle);
    }

    //Joining all the threads
    for handle in handles{
        handle.join().unwrap();
    }

    debug!(target="ga_events", method="parent_crossover_multithread"; "Parent crossover finished");
    return offspring.lock().unwrap().to_vec();
}