use criterion::{criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration, Throughput};
use rand::Rng;
use pprof::criterion::{Output, PProfProfiler};

use genetic_algorithms::operations::selection::random::random;
use genetic_algorithms::operations::selection::fitness_proportionate::roulette_wheel_selection;
use genetic_algorithms::operations::selection::fitness_proportionate::stochastic_universal_sampling;
use genetic_algorithms::operations::selection::tournament::tournament;
use genetic_algorithms::traits::{GeneT, GenotypeT};

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Gene {
    pub id: i32,
}
impl GeneT for Gene {
    fn get_id(&self) -> i32 {
        self.id
    }
    fn set_id(&mut self, id: i32) -> &mut Self {
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
    fn set_fitness(&mut self, fitness: f64) -> &mut Self {
        self.fitness = fitness;
        self
    }
    fn set_age(&mut self, age: i32) -> &mut Self {
        self.age = age;
        self
    }
    fn get_age(&self) -> i32 {
        self.age
    }
    fn set_dna(&mut self, dna: &[Self::Gene]) -> &mut Self {
        self.dna = dna.to_vec();
        self
    }
    fn calculate_fitness(&mut self) {
        self.fitness = 0.0;
    }
}

// Setup function to create a population with configurable size and gene length
fn setup_population(population_size: usize, gene_length: usize) -> Vec<SimpleGenotype> {
    let mut rng = rand::thread_rng();
    (0..population_size)
        .map(|_| SimpleGenotype {
            fitness: rng.gen_range(0.0..=1.0),
            dna: (0..gene_length)
                .map(|_| Gene { id: rng.gen_range(0..255) })
                .collect(),
            age: rng.gen_range(0..=100),
        })
        .collect()
}

// Benchmark function with parameterized population and gene length
fn benchmark_selection_methods(c: &mut Criterion) {
    let population_sizes = vec![10, 100, 1000];
    let gene_lengths = vec![10, 100, 1000];
    let tournament_threads = vec![1, 2, 4, 8];

    let mut group = c.benchmark_group("selection_methods");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for &population_size in &population_sizes {
        for &gene_length in &gene_lengths {
            let individuals = setup_population(population_size, gene_length);
            
            group.throughput(Throughput::Elements(population_size as u64));
            
            // Benchmark random selection
            group.bench_with_input(
                BenchmarkId::new("random selection", format!("population_{}_genes_{}", population_size, gene_length)),
                &individuals,
                |b, individuals| {
                    b.iter(|| {
                        let _ = random(individuals);
                    });
                },
            );

            // Benchmark roulette wheel selection
            group.bench_with_input(
                BenchmarkId::new("roulette wheel selection", format!("population_{}_genes_{}", population_size, gene_length)),
                &individuals,
                |b, individuals| {
                    b.iter(|| {
                        let _ = roulette_wheel_selection(individuals);
                    });
                },
            );

            // Benchmark stochastic universal sampling
            group.bench_with_input(
                BenchmarkId::new("stochastic universal sampling", format!("population_{}_genes_{}", population_size, gene_length)),
                &individuals,
                |b, individuals| {
                    b.iter(|| {
                        let _ = stochastic_universal_sampling(individuals, 50);
                    });
                },
            );

            // Benchmarks for tournament selection with different threads
            for &threads in &tournament_threads {
                group.bench_with_input(
                    BenchmarkId::new(format!("tournament {} threads", threads), format!("population_{}_genes_{}", population_size, gene_length)),
                    &individuals,
                    |b, individuals| {
                        b.iter(|| {
                            let _ = tournament(individuals, 5, threads);
                        });
                    },
                );
            }

        }
    }
    group.finish();
}

// Create the benchmark group with profiling
criterion_group! {
    name = selection_benchmarks;
    config = Criterion::default()
        .with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = benchmark_selection_methods
}

criterion_main!(selection_benchmarks);