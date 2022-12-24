use crate::traits::GeneT;
use crate::traits::GenotypeT;
use std::collections::HashMap;
use rand::Rng;

pub fn tournament<T:GeneT, U:GenotypeT<T>>(individuals: &Vec<U>) -> HashMap<usize, usize>{
    let mut mating = HashMap::new();


    return mating;
}