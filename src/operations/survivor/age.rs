pub(crate) use crate::traits::GenotypeT;

pub fn age_based<U:GenotypeT>(individuals: &mut Vec<U>, population_size: usize)
{

    //We first sort the individuals by their fitness
    individuals.sort_by_key(|a| std::cmp::Reverse(a.get_age()));

    //If there is more individuals than the defined population number
    if individuals.len() > population_size {
        let individuals_to_remove = individuals.len() - population_size;
        for _i in 0..individuals_to_remove{
            individuals.remove(individuals.len() - 1);
        }
    }
}