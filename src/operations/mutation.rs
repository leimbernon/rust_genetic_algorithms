use crate::traits::GeneT;
use crate::traits::GenotypeT;
use crate::operations::mutation::swap::swap;
use crate::operations::mutation::inversion::inversion;
use super::Mutation;
pub mod swap;
pub mod inversion;

pub fn factory<T: GeneT, U: GenotypeT<T>>(mutation: Mutation ,individual: &mut U){
    match mutation {
        Mutation::Swap => {swap(individual)},
        Mutation::Inversion => {inversion(individual)}
    }
}