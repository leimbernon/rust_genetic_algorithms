pub mod selection;
pub mod crossover;
pub mod mutation;
pub mod survivor;

#[derive(Copy, Clone)]
pub enum Selection {
    Random,
    FitnessProportionate,
}
#[derive(Copy, Clone)]
pub enum Crossover {
    Cycle,
}
#[derive(Copy, Clone)]
pub enum Mutation {
    Swap,
}
#[derive(Copy, Clone)]
pub enum Survivor {
    Fitness,
}