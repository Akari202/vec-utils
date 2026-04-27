use core::hint::black_box;

use bektor::quat::{Quat, QuatFromMatrixAlgorithm};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use rand::SeedableRng;
use rand::rngs::SmallRng;

fn bench_quat_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Quat From Rotation Matrix");

    for i in QuatFromMatrixAlgorithm::ALL {
        group.bench_function(i.to_string(), |b| {
            let mut rng = SmallRng::seed_from_u64(39332);

            b.iter_batched(
                || Quat::random_unit(&mut rng).to_rotation_matrix(),
                |mat| black_box(Quat::from_rotation_matrix(&mat, i)),
                BatchSize::SmallInput
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_quat_conversions);
criterion_main!(benches);
