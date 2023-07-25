pub(crate) use crate::traits::GeneT;
use crate::traits::GenotypeT;
use self::swap::swap;
use self::inversion::inversion;
use self::scramble::scramble;
use super::Mutation;

pub mod swap;
pub mod inversion;
pub mod scramble;

pub fn factory<T,U>(mutation: Mutation ,individual: &mut U)
where
T: GeneT, 
U: GenotypeT<T> + 'static
{
    match mutation {
        Mutation::Swap => {swap(individual)},
        Mutation::Inversion => {inversion(individual)},
        Mutation::Scramble => {scramble(individual)},
    }
}