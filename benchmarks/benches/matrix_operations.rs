use core::hint::black_box;

use bektor::matrix::generic::GMatrix;
use criterion::{Criterion, criterion_group, criterion_main};

fn bench_adjoint(c: &mut Criterion) {
    let m = GMatrix::<3, 3, f64>::from_flat_arr([1.0, 2.0, 3.0, 0.0, 1.0, 4.0, 5.0, 6.0, 0.0]);
    c.bench_function("adjoint", |b| b.iter(|| black_box(m.adjoint())));
}

fn bench_cofactor_matrix(c: &mut Criterion) {
    let m = GMatrix::<3, 3, f64>::from_flat_arr([1.0, 2.0, 3.0, 0.0, 1.0, 4.0, 5.0, 6.0, 0.0]);
    c.bench_function("cofactor matrix", |b| {
        b.iter(|| black_box(m.cofactor_matrix()))
    });
}

fn bench_transpose(c: &mut Criterion) {
    let m = GMatrix::<3, 3, f64>::from_flat_arr([1.0, 2.0, 3.0, 0.0, 1.0, 4.0, 5.0, 6.0, 0.0]);
    c.bench_function("transpose", |b| b.iter(|| black_box(m.transpose())));
}

fn bench_determinant(c: &mut Criterion) {
    let m = GMatrix::<3, 3, f64>::from_flat_arr([1.0, 2.0, 3.0, 0.0, 1.0, 4.0, 5.0, 6.0, 0.0]);
    c.bench_function("determinant", |b| b.iter(|| black_box(m.determinant())));
}

criterion_group!(
    benches,
    bench_adjoint,
    bench_cofactor_matrix,
    bench_transpose,
    bench_determinant
);
criterion_main!(benches);
