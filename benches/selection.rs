use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use pprof::criterion::{Output, PProfProfiler};

use genetic_algorithms::operations::selection::random::random;
use genetic_algorithms::traits::{GeneT, GenotypeT};

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Gene{
    pub id: i32,
}
impl GeneT for Gene{
    fn get_id(&self) -> i32{
        self.id
    }
    fn set_id(&mut self, id: i32)->&mut Self {
        self.id = id;
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
struct SimpleGenotype {
    dna: Vec<Gene>,
    pub fitness: f64,
    pub age: i32,
}
impl GenotypeT for SimpleGenotype {
    type Gene = Gene;

    fn get_dna(&self) -> &[Self::Gene] {
        &self.dna
    }
    fn get_fitness(&self) -> f64 {
        self.fitness
    }
    fn set_fitness(&mut self, fitness: f64)->&mut Self {
        self.fitness = fitness;
        self
    }
    fn set_age(&mut self, age:i32)->&mut Self{
        self.age = age;
        self
    }
    fn get_age(&self) -> i32 {
        self.age
    }
    fn set_dna(&mut self, dna: &[Self::Gene])->&mut Self{
        self.dna = dna.to_vec();
        self
    }
    fn calculate_fitness(&mut self) {
        self.fitness = 0.0;
    }
}

fn benchmark_random_selection(c: &mut Criterion) {

    let population_size = 1000;
    let individuals: Vec<SimpleGenotype> = (0..population_size)
        .map(|_| SimpleGenotype { fitness: 0.0, dna: vec![Gene{ id: rand::thread_rng().gen_range(0..255)}; 10], age: 0 })
        .collect();

    // Defines the benchmark
    c.bench_function("random selection", |b| {
        b.iter(|| {
            let _ = random(&individuals);
        });
    });
}

// Create a benchmark group
criterion_group! {
    name = selection_benchmarks;
    config = Criterion::default()
        .with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = benchmark_random_selection
}

criterion_main!(selection_benchmarks);
