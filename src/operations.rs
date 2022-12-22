pub mod selection;
pub mod crossover;
pub mod mutation;
pub mod survivor;

#[derive(Copy, Clone)]
pub enum Selection {
    Random,
    RouletteWheel,
    StochasticUniversalSampling
}
#[derive(Copy, Clone)]
pub enum Crossover {
    Cycle,
    MultiPoint,
}
#[derive(Copy, Clone)]
pub enum Mutation {
    Swap,
    Inversion,
}
#[derive(Copy, Clone)]
pub enum Survivor {
    Fitness,
    Age,
}