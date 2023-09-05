pub(crate) use crate::traits::GenotypeT;
use log::{trace, debug};

pub fn age_based<U:GenotypeT>(individuals: &mut Vec<U>, population_size: usize)
{

    //We first sort the individuals by their fitness
    debug!(target="survivor_events", method="age_based"; "Starting age based survivor method");
    individuals.sort_by_key(|a| std::cmp::Reverse(a.get_age()));

    //If there is more individuals than the defined population number
    trace!(target="survivor_events", method="age_based"; "Individuals length {} - population size {}", individuals.len(), population_size);
    if individuals.len() > population_size {
        let individuals_to_remove = individuals.len() - population_size;
        for _i in 0..individuals_to_remove{
            individuals.remove(individuals.len() - 1);
        }
    }
    debug!(target="survivor_events", method="age_based"; "Age based survivor method finished");
}