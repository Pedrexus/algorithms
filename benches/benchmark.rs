use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::distributions::Uniform;
use rand::Rng;

use algorithms::bs;

fn random_vec(n: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, n);
    (0..n).map(|_| rng.sample(&range)).collect()
}

fn sorted_random_vec(n: usize) -> Vec<usize> {
    let mut v = random_vec(n);
    v.sort();
    v
}

fn criterion_benchmark_bs_baseline(c: &mut Criterion) {
    c.bench_function("baseline", |b| {
        b.iter(|| black_box(sorted_random_vec(1000)).binary_search(&111))
    });
}

fn criterion_benchmark_index1(c: &mut Criterion) {
    c.bench_function("index1", |b| {
        b.iter(|| bs::index(black_box(&sorted_random_vec(1000)), &111, 0, 999))
    });
}

fn criterion_benchmark_index2(c: &mut Criterion) {
    c.bench_function("index2", |b| {
        b.iter(|| bs::index2(black_box(&sorted_random_vec(1000)), &111))
    });
}

criterion_group!(
    benches,
    criterion_benchmark_bs_baseline,
    criterion_benchmark_index1,
    criterion_benchmark_index2
);
criterion_main!(benches);
