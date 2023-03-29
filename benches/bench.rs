use comparison_counter::*;
use criterion::{
    criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup, Criterion,
};
use rand::distributions::Standard;
use rand::prelude::*;

fn do_bench<T: Ord + From<u64>, M: Measurement>(
    c: &mut BenchmarkGroup<M>,
    name: &str,
    orig: &[u64]
) {
    c.bench_function(name, |b| {
        b.iter_batched(
            || orig.iter().map(|x| T::from(*x)).collect::<Vec<T>>(),
            |mut v| v.sort_unstable(),
            criterion::BatchSize::SmallInput,
        );
    });
}

pub fn comparison_count_sort(c: &mut Criterion) {
    let n = 100000;
    let rng = rand::thread_rng();
    let vals: Vec<u64> = rng.sample_iter(Standard).take(n).collect();

    let mut group = c.benchmark_group("sorting");

    do_bench::<u64, _>(&mut group, "baseline", &vals);
    do_bench::<WrapUnsafeCnt, _>(&mut group, "unsafe", &vals);
    do_bench::<WrapUnsafeToggleableCnt, _>(&mut group, "toggleable unsafe", &vals);
    do_bench::<WrapCellThreadLocal, _>(&mut group, "thread local cell", &vals);
    do_bench::<WrapAtomic, _>(&mut group, "atomic", &vals);
}

criterion_group!(benches, comparison_count_sort);
criterion_main!(benches);
