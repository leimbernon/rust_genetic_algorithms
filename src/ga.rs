use std::{sync::{mpsc::{sync_channel}, Mutex, Arc}, thread, collections::HashMap};

use crate::{population::{Population}, traits::{GenotypeT, GeneT}, operations::{selection, crossover, mutation, survivor}, configuration::{ProblemSolving, LimitConfiguration}};
use crate::configuration::GaConfiguration;

/**
 * Function to run the genetic algorithms cycle
 */
pub fn run<T,U>(mut population: Population<T,U>, configuration: GaConfiguration)->Population<T,U>
where 
T:GeneT + Send + Sync, 
U:GenotypeT<T> + Send + Sync + 'static + Clone
{
    //Best individual within the generations and population returned
    let initial_population_size = population.size();
    let mut age = 0;

    //Calculation of the fitness and the best individual
    let mut best_individual = population_fitness_calculation_multithread(&mut population.individuals, configuration);

    //We start the cycles
    for i in 0..configuration.limit_configuration.max_generations {

        println!("Generation number: {}", i+1);
        age += 1;

        //1- Parent selection for reproduction
        let mut parents = selection::factory(configuration.selection, &population.individuals, configuration.selection_configuration, configuration.number_of_threads.unwrap_or(1));

        //2- Getting the offspring from the multithreading function
        let mut offspring = parent_crossover_multithread(&mut parents, &population.individuals, &configuration, age);

        //3- Sets the best individual
        for child in &offspring{
            best_individual = get_best_individual(&best_individual, &child, configuration.limit_configuration.problem_solving);
        }

        //4- Insert the children in the population
        population.individuals.append(&mut offspring);

        //5- Survivor selection
        survivor::factory(configuration.survivor, &mut population.individuals, initial_population_size, configuration.limit_configuration);

        //6- Identifies if the limit has been reached or not
        if limit_reached(configuration.limit_configuration, &population.individuals){
            break;
        }
    }

    return Population::new(vec![best_individual]);
}

/**
 * Function to determine which of the individuals is the best individual and return the best of them
 */
fn get_best_individual<T,U>(individual_1: &U, individual_2: &U, problem_solving: ProblemSolving) -> U
where
T:GeneT, 
U:GenotypeT<T>
{

    let mut best_individual = U::new();

    if problem_solving == ProblemSolving::Maximization {

        //We check if the fitness is the best and store it if it's the case
        if individual_1.get_fitness() >= individual_2.get_fitness(){
            *best_individual.get_dna_mut() = individual_1.get_dna().clone();
            *best_individual.get_fitness_mut() = individual_1.get_fitness().clone();
        }else{
            *best_individual.get_dna_mut() = individual_2.get_dna().clone();
            *best_individual.get_fitness_mut() = individual_2.get_fitness().clone();
        }

    } else {

        //We check if the fitness is the best and store it if it's the case
        if individual_1.get_fitness() >= individual_2.get_fitness(){
            *best_individual.get_dna_mut() = individual_2.get_dna().clone();
            *best_individual.get_fitness_mut() = individual_2.get_fitness().clone();
        }else{
            *best_individual.get_dna_mut() = individual_1.get_dna().clone();
            *best_individual.get_fitness_mut() = individual_1.get_fitness().clone();
        }

    }

    best_individual
}

/**
 * Function to indentify if the limit has been reached or not in the current generation
 */
fn limit_reached<T,U>(limit: LimitConfiguration, individuals: &Vec<U>)->bool
where 
T:GeneT, 
U:GenotypeT<T>
{

    let mut result = false;

    if limit.problem_solving == ProblemSolving::Minimization{
        //If the problem solving is minimization, fitness must be 0
        for genotype in individuals {
            if genotype.get_fitness() == &0.0 {
                result = true;
                break;
            }
        }
    }else if limit.problem_solving == ProblemSolving::FixedFitness{
        //If the problem solving is a fixed fitness
        for genotype in individuals {
            if genotype.get_fitness() == &limit.fitness_target.unwrap() {
                result = true;
                break;
            }
        }
    }

    return result;
}

/**
 * Sets the population fitness, age and the best individual
 */
fn population_fitness_calculation_multithread<T, U>(individuals: &mut Vec<U>, configuration: GaConfiguration) -> U
where
T:GeneT, 
U:GenotypeT<T> + Send + Sync + 'static + Clone
{

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
        let (start_index_t, tx, jump_t, individuals_t, best_individual_t) = (start_index.clone(), tx.clone(),  jump.clone(), Arc::clone(&individuals_t), Arc::clone(&best_individual_t));

        //Starting the thread management
        thread::spawn(move || {

            let mut fitness_map = HashMap::new();
            let mut best_individual = U::new();

            //Calculates the fitness from the corresponding population
            for i in start_index_t..(start_index_t + jump_t){
                individuals_t.lock().unwrap()[i as usize].calculate_fitness();
                fitness_map.insert(i as usize, *individuals_t.lock().unwrap()[i as usize].get_fitness());

                if best_individual.get_dna().len() > 0 {
                    best_individual = get_best_individual(&best_individual, &individuals_t.lock().unwrap()[i as usize], configuration.limit_configuration.problem_solving);
                } else{
                    *best_individual.get_dna_mut() = individuals_t.lock().unwrap()[i as usize].get_dna().clone();
                    *best_individual.get_fitness_mut() = individuals_t.lock().unwrap()[i as usize].get_fitness().clone();
                }
            }

            //Setting the best global individual
            if best_individual_t.lock().unwrap().get_dna().len() > 0 {
                let global_best_individual = get_best_individual(&best_individual_t.lock().unwrap().clone(), &best_individual, configuration.limit_configuration.problem_solving);
                *best_individual_t.lock().unwrap().get_dna_mut() = global_best_individual.get_dna().clone();
                *best_individual_t.lock().unwrap().get_fitness_mut() = global_best_individual.get_fitness().clone();
            }else{
                *best_individual_t.lock().unwrap().get_dna_mut() = best_individual.get_dna().clone();
                *best_individual_t.lock().unwrap().get_fitness_mut() = best_individual.get_fitness().clone();
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
            *individuals[element.0].get_fitness_mut() = element.1;
        }
    }

    let mut best_individual = U::new();
    *best_individual.get_dna_mut() = best_individual_t.lock().unwrap().get_dna().clone();
    *best_individual.get_fitness_mut() = best_individual_t.lock().unwrap().get_fitness_mut().clone();

    return best_individual;
}

/**
 * Function for parent crossover in multithreading
 */
fn parent_crossover_multithread<T,U>(parents: &mut HashMap<usize, usize>, individuals: &Vec<U>, configuration: &GaConfiguration, age: i32) -> Vec<U>
where 
T:GeneT, 
U:GenotypeT<T> + Send + Sync + 'static + Clone
{
    //Setting the control variables
    let mut number_of_threads = if configuration.number_of_threads == None {1} else {configuration.number_of_threads.unwrap()}; 
    number_of_threads = if number_of_threads > parents.len() as i32 {parents.len() as i32}else{number_of_threads};
    let jump = parents.len() as i32 / number_of_threads;

    let mut handles = Vec::new();
    let offspring = Arc::new(Mutex::new(Vec::new()));

    //Run all the threads
    for t in 0..number_of_threads{

        //We copy the parents that we want to crossover inside the thread
        let (individuals, configuration, offspring) = (individuals.clone(), configuration.clone(), Arc::clone(&offspring));
        let mut parents_t = HashMap::new();
        let parents_c = parents.clone();
        let mut index = 0;

        for i in parents_c.keys(){

            //If we reach the number of crossovers / thread
            if t < number_of_threads - 1 && index >= jump {
                break;
            }

            let key = *parents.get_key_value(i).unwrap().0;
            parents_t.insert(key, *parents.get_key_value(i).unwrap().1);
            parents.remove(&key);

            index +=1;
        }

        //Starts the thread
        let handle = thread::spawn(move || {

            for(key, value) in parents_t.iter(){
                //Getting the parent 1 and 2 for crossover                
                let parent_1 = individuals.get(*key).unwrap().clone();
                let parent_2 = individuals.get(*value).unwrap().clone();

                let mut offspring_t = crossover::factory(configuration.crossover, &parent_1, &parent_2, configuration.crossover_configuration).unwrap();
                let mut child_1 = offspring_t.pop().unwrap();
                let mut child_2 = offspring_t.pop().unwrap();

                //Making the mutation of each child
                mutation::factory(configuration.mutation, &mut child_1);
                mutation::factory(configuration.mutation, &mut child_2);

                //Calculate the fitness of both children and set their age
                child_1.calculate_fitness();
                child_2.calculate_fitness();

                *child_1.get_age_mut() = age;
                *child_2.get_age_mut() = age;

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

    return offspring.lock().unwrap().to_vec();

}