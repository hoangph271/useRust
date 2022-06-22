#[path = "../src/handlers/handlers.rs"]
mod handlers;
use criterion::{criterion_group, criterion_main, Criterion};
use std::path::Path;

// TODO: async_travel_benchmark...?

fn single_threaded_travel_benchmark(criterion: &mut Criterion) {
    criterion.bench_function("single_threaded_travel_benchmark", |bencher| {
        bencher.iter(|| {
            handlers::single_threaded_travel(&Path::new("."));
        });
    });
}

fn multiple_threaded_travel_benchmark(criterion: &mut Criterion) {
    criterion.bench_function("multiple_threaded_travel_benchmark", |bencher| {
        bencher.iter(|| {
            handlers::multiple_threaded_travel(&Path::new("."));
        });
    });
}

criterion_group!(
    benches,
    single_threaded_travel_benchmark,
    multiple_threaded_travel_benchmark
);
criterion_main!(benches);
