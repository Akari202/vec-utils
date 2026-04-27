#![feature(iter_intersperse)]
use core::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use itertools::Itertools;

fn bench_intersperse(c: &mut Criterion) {
    let mut group = c.benchmark_group("Intersperse methods");
    let it = (0..100000).map(|_| "hey");

    group.bench_function("Collect and Join", |b| {
        b.iter(|| {
            let iter = it.clone();
            let result: Vec<&str> = iter.collect();
            black_box(result.join("\n"))
        })
    });

    group.bench_function("Fold with Addition", |b| {
        b.iter(|| {
            let iter = it.clone();
            let s = iter.fold(String::new(), |a, b| a + b + "\n");
            black_box(s.trim_end().to_string())
        })
    });

    group.bench_function("Stdlib Intersperse", |b| {
        b.iter(|| {
            let iter = it.clone();
            black_box(Iterator::intersperse(iter, "\n").collect::<String>())
        })
    });

    group.bench_function("Itertools Intersperse", |b| {
        b.iter(|| {
            let iter = it.clone();
            black_box(Itertools::intersperse(iter, "\n").collect::<String>())
        })
    });

    group.finish();
}

criterion_group!(benches, bench_intersperse);
criterion_main!(benches);
