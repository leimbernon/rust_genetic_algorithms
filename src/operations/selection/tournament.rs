use crate::traits::GeneT;
use crate::traits::GenotypeT;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use rand::Rng;

/**
 * Main function for tournament selection
 */
pub fn tournament<T,U>(individuals: &Vec<U>, couples: i32, number_of_threads: i32) -> HashMap<usize, usize>
where
T:GeneT + Send + Sync, 
U:GenotypeT<T> + Send + Sync + 'static + Clone
{
    
    if number_of_threads == 1{
        return tournament_single_thread(individuals, couples);
    }else{
        let number_of_threads_t = if number_of_threads > couples {couples}else{number_of_threads};
        let number_of_threads_t = if number_of_threads_t & 1 == 1 {number_of_threads_t-1}else{number_of_threads_t};

        return tournament_multithread(individuals, couples, number_of_threads_t);
    }
}


/**
 * Function for tournament selection in a single thread 
 */
fn tournament_single_thread<T,U>(individuals: &Vec<U>, couples: i32) -> HashMap<usize, usize>
where
T:GeneT, 
U:GenotypeT<T>
{

    let mut rng = rand::thread_rng();
    let mut mating = HashMap::new();
    let individual_couples = couples*2;

    let mut indexes = Vec::new();

    //Getting the list of indexes
    for i in 0..individual_couples{
        indexes.push(i);
    }

    //Getting the parents from the population
    let mut parent_1 = None;

    for _i in 0..indexes.len(){
        let index_1 = rng.gen_range(0..indexes.len());
        let index_2 = rng.gen_range(0..indexes.len());
        let final_index;
        let index_to_delete;

        //Fights between both parents
        if individuals[index_1].get_fitness() >= individuals[index_2].get_fitness() {
            final_index = indexes[index_1];
            index_to_delete = index_1;
        }else{
            final_index = indexes[index_2];
            index_to_delete = index_2;
        }

        //Sets the mating
        if parent_1 == None {
            parent_1 = Some(final_index);
        }else{
            mating.insert(parent_1.unwrap() as usize, final_index as usize);
            parent_1 = None;
        }

        indexes.remove(index_to_delete);
    }

    return mating;
}

/**
 * Function for tournament selection in multithread 
 */
fn tournament_multithread<T,U>(individuals: &Vec<U>, couples: i32, number_of_threads: i32) -> HashMap<usize, usize>
where
T:GeneT + Send + Sync,
U:GenotypeT<T>+ Send + Sync + 'static + Clone
{

    let mating = HashMap::new();

    //Thread control
    let mut handles = vec![];

    //Vectors
    let left = Arc::new(Mutex::new(Vec::new()));
    let right = Arc::new(Mutex::new(Vec::new()));

    //Running the different threads
    for thread in 0..number_of_threads{

        //Copies
        let winners = if thread & 1 == 1 {Arc::clone(&left)}else{Arc::clone(&right)};

        //Run the thread
        let handle = thread::spawn(move || {
            winners.lock().unwrap().push(0);
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    //Gets the final vectors
    let left = left.lock().unwrap();
    let right = right.lock().unwrap();

    for item in 0..left.len() {
        println!("left: {}", left[item]);
        println!("right: {}", right[item]);
    }

    return mating;
}