use criterion::{criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration};

use rand::seq::SliceRandom;
use rand::Rng;
use pprof::criterion::{Output, PProfProfiler};

use genetic_algorithms::operations::crossover::multipoint::multipoint_crossover;
use genetic_algorithms::operations::crossover::uniform_crossover::uniform;
use genetic_algorithms::operations::crossover::cycle::cycle;
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

fn setup_population(population_size: usize, gene_length: usize) -> Vec<SimpleGenotype> {
    let mut rng = rand::thread_rng();
    
    // Generate a single set of genes for all individuals
    let base_genes: Vec<Gene> = (0..gene_length)
        .map(|_| Gene { id: rng.gen_range(0..255) })
        .collect();

    (0..population_size)
        .map(|_| {
            // Clone and shuffle base_genes to create individual DNA with the same genes in a different order
            let mut dna = base_genes.clone();
            dna.shuffle(&mut rng);

            SimpleGenotype {
                fitness: rng.gen_range(0.0..1.0),
                dna,
                age: rng.gen_range(0..100),
            }
        })
        .collect()
}

fn benchmark_crossover_methods(c: &mut Criterion) {
    let population_size = 1000;
    let gene_lengths = vec![10, 100, 1000];
    let crossover_points = vec![1, 2, 3]; // Different points for `multipoint_crossover`

    let mut group = c.benchmark_group("crossover_methods");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for &gene_length in &gene_lengths {
        let individuals = setup_population(population_size, gene_length);

        // Benchmark for cycle crossover
        group.bench_with_input(
            BenchmarkId::new("cycle crossover", format!("genes_{}", gene_length)),
            &individuals,
            |b, individuals| {
                let parent_1 = &individuals[0];
                let parent_2 = &individuals[1];
                b.iter(|| {
                    let _ = cycle(parent_1, parent_2);
                });
            },
        );

        // Benchmark for multipoint crossover
        for &points in &crossover_points {
            group.bench_with_input(
                BenchmarkId::new("multipoint crossover", format!("genes_{}_points_{}", gene_length, points)),
                &individuals,
                |b, individuals| {
                    let parent_1 = &individuals[0];
                    let parent_2 = &individuals[1];
                    b.iter(|| {
                        let _ = multipoint_crossover(parent_1, parent_2, &points);
                    });
                },
            );
        }

        // Benchmark for uniform crossover
        group.bench_with_input(
            BenchmarkId::new("uniform crossover", format!("genes_{}", gene_length)),
            &individuals,
            |b, individuals| {
                let parent_1 = &individuals[0];
                let parent_2 = &individuals[1];
                b.iter(|| {
                    let _ = uniform(parent_1, parent_2);
                });
            },
        );
    }
    group.finish();
}

// Configure the benchmark group with Criterion and PProf
criterion_group! {
    name = crossover_benchmarks;
    config = Criterion::default()
        .with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = benchmark_crossover_methods
}

criterion_main!(crossover_benchmarks);