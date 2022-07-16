use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../src/bubble_sort.rs"]
mod bubble_sort;
#[path = "../src/main.rs"]
mod main;

fn criterion_benchmark(c: &mut Criterion) {
    const SIZE: usize = 1_000_000_000;

    c.bench_function(&format!("bubble_sort {}", SIZE), |b| {
        let mut nums = main::generate_nums(&SIZE);
        b.iter(|| bubble_sort::bubble_sort(&mut nums))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
