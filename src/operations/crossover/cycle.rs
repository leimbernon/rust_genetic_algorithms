use crate::traits::GenotypeT;
use crate::traits::GeneT;
use log::{trace, debug};


pub fn cycle<U: GenotypeT>(parent_1: &U, parent_2: &U) -> Option<Vec<U>> {
    // Check DNA length consistency between parents
    if parent_1.get_dna().len() != parent_2.get_dna().len() {
        eprintln!(
            "Parent 1 and Parent 2 must have the same DNA length. \
            Parent 1 has length {}, Parent 2 has length {}",
            parent_1.get_dna().len(),
            parent_2.get_dna().len()
        );
        return None;
    }

    // Initialize variables
    let mut cycle_number = 0;
    let mut indexes: Vec<usize> = Vec::new();
    let mut dna_child_1 = vec![U::Gene::new(); parent_1.get_dna().len()];
    let mut dna_child_2 = vec![U::Gene::new(); parent_2.get_dna().len()];
    debug!(target="crossover_events", method="cycle"; "Starting the crossover");

    let mut child_1 = U::new();
    let mut child_2 = U::new();

    // Perform cycles until all elements are processed
    while indexes.len() < parent_1.get_dna().len() {
        trace!(target="crossover_events", method="cycle"; "Getting cycle indexes, cycle number {}", cycle_number);
        let cycle_indexes = match local_cycle(&indexes, parent_1.get_dna(), parent_2.get_dna()) {
            Some(indexes) => indexes,
            None => return None, // Return None if a cycle cannot be completed
        };
        indexes.extend(cycle_indexes.iter().copied());
        trace!(target="crossover_events", method="cycle"; "Cycle indexes completed for cycle number {}", cycle_number);

        let is_odd_cycle = cycle_number % 2 == 1;

        // Populate child DNA based on cycle
        for &index in &cycle_indexes {
            if is_odd_cycle {
                dna_child_1[index] = parent_2.get_dna()[index].clone();
                dna_child_2[index] = parent_1.get_dna()[index].clone();
            } else {
                dna_child_1[index] = parent_1.get_dna()[index].clone();
                dna_child_2[index] = parent_2.get_dna()[index].clone();
            }
        }
        cycle_number += 1;
    }

    // Set DNA for children and finish crossover
    child_1.set_dna(&dna_child_1);
    child_2.set_dna(&dna_child_2);
    debug!(target="crossover_events", method="cycle"; "Crossover completed");

    Some(vec![child_1, child_2])
}

fn local_cycle<T: GeneT>(indexes: &[usize], dna_parent_1: &[T], dna_parent_2: &[T]) -> Option<Vec<usize>> {
    // Start index for the cycle that hasn't been used
    let start_index = (0..dna_parent_1.len()).find(|&i| !indexes.contains(&i))?;

    let starting_value = dna_parent_1[start_index].get_id();
    let mut cycle_indexes = vec![start_index];
    let mut current_index = start_index;

    loop {
        let current_value = dna_parent_2[current_index].get_id();
        if current_value == starting_value {
            break;
        }
        match dna_parent_1.iter().position(|g| g.get_id() == current_value) {
            Some(index) => {
                cycle_indexes.push(index);
                current_index = index;
            }
            None => {
                eprintln!(
                    "Error finding gene with ID {} of Parent 2 in Parent 1. \
                    Ensure both parents have identical sets of genes.",
                    current_value
                );
                return None; // Return None if cycle can't be completed
            }
        }
    }

    Some(cycle_indexes)
}