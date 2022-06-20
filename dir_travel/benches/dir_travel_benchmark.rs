// use crate::handlers;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// TODO: Benchmark 'em functions

fn single_threaded_travel_benchmark(criterion: &mut Criterion) {
    // criterion.bench_function("single_threaded_travel_benchmark", |bencher| {
    //     bencher.iter(|| )
    // })
}

criterion_group!(benches, single_threaded_travel_benchmark);
criterion_main!(benches);
