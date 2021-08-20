#![allow(unused_variables)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num_derive::{FromPrimitive, ToPrimitive};

use tokidator::rbac::json_discriminant_array_to_vec;

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    num_derive::FromPrimitive,
    num_derive::ToPrimitive,
    strum::Display,
    strum::EnumCount,
)]
enum TestPolicy {
    Policy0,
    Policy1,
    Policy2,
    Policy3,
    Policy4,
    Policy5,
    Policy6,
    Policy7,
    Policy8,
    Policy9,
    Policy10,
    Policy11,
    Policy12,
    Policy13,
    Policy14,
    Policy15,
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("json_discriminant_array_to_vec", |b| {
        b.iter(|| {
            json_discriminant_array_to_vec::<TestPolicy>(black_box(
                "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]",
            ))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
