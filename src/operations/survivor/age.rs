pub(crate) use crate::traits::{GeneT, GenotypeT};

pub fn age_based<T:GeneT, U:GenotypeT<T>>(individuals: &mut Vec<U>, population_size: usize)
{

    //We first sort the individuals by their fitness
    individuals.sort_by(|a, b| b.get_age().cmp(a.get_age()));

    //If there is more individuals than the defined population number
    if individuals.len() > population_size {
        let individuals_to_remove = individuals.len() - population_size;
        for _i in 0..individuals_to_remove{
            individuals.remove(individuals.len() - 1);
        }
    }
}