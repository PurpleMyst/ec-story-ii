use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

#[rustfmt::skip]
macro_rules! problems {
    ($($problem:ident),*$(,)?) => {
        pub fn ec_benchmark_full(c: &mut Criterion) {
            $(c.bench_function(stringify!($problem), |b| b.iter(|| black_box($problem::solve())));)+
            c.bench_function("all", |b| b.iter(|| ($(black_box($problem::solve())),+)));
        }

        pub fn ec_benchmark_parts(c: &mut Criterion) {
            $(c.bench_function(concat!(stringify!($problem), "/part1"), |b| b.iter(|| black_box($problem::solve_part1())));)+
            $(c.bench_function(concat!(stringify!($problem), "/part2"), |b| b.iter(|| black_box($problem::solve_part2())));)+
            $(c.bench_function(concat!(stringify!($problem), "/part3"), |b| b.iter(|| black_box($problem::solve_part3())));)+
        }

        criterion_group! {
            name = benches;
            config = Criterion::default();
            targets = ec_benchmark_full, ec_benchmark_parts
        }

        criterion_main!{
            benches
        }
    };
}

#[rustfmt::skip]
problems!(
    problem01,
);
