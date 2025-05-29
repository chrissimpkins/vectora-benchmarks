#![allow(unused_imports)]
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use vectora::Vector;

const VEC_I32_3D: Vector<i32, 3_usize> = Vector {
    components: [1, 2, 3],
};

const VEC_F64_3D: Vector<f64, 3_usize> = Vector {
    components: [1.0, 2.0, 3.0],
};

// ================================
// Unary neg
// ================================

fn bench_vector_unary_neg_i32_3d(c: &mut Criterion) {
    c.bench_function("unary neg i32 3d", |b| {
        b.iter(|| {
            let _ = -VEC_I32_3D;
        })
    });
}

fn bench_vector_unary_neg_f64_3d(c: &mut Criterion) {
    c.bench_function("unary neg f64 3d", |b| {
        b.iter(|| {
            let _ = -VEC_F64_3D;
        })
    });
}

// ================================
// Addition
// ================================

fn bench_vector_add_i32_3d(c: &mut Criterion) {
    c.bench_function("add i32 3d", |b| {
        b.iter(|| {
            let _ = black_box(VEC_I32_3D + VEC_I32_3D);
        })
    });
}

fn bench_vector_add_f64_3d(c: &mut Criterion) {
    c.bench_function("add f64 3d", |b| {
        b.iter(|| {
            let _ = black_box(VEC_F64_3D + VEC_F64_3D);
        })
    });
}

// ================================
// Subtraction
// ================================

fn bench_vector_subtract_i32_3d(c: &mut Criterion) {
    c.bench_function("subtract i32 3d", |b| {
        b.iter(|| {
            let _ = black_box(VEC_I32_3D - VEC_I32_3D);
        })
    });
}

fn bench_vector_subtract_f64_3d(c: &mut Criterion) {
    c.bench_function("subtract f64 3d", |b| {
        b.iter(|| {
            let _ = black_box(VEC_F64_3D - VEC_F64_3D);
        })
    });
}

// ================================
// Scalar multiplication
// ================================

fn bench_vector_multiplication_i32_3d(c: &mut Criterion) {
    c.bench_function("mul i32 3d", |b| {
        b.iter(|| {
            let _ = black_box(VEC_I32_3D * 10);
        })
    });
}

fn bench_vector_multiplication_f64_3d(c: &mut Criterion) {
    c.bench_function("mul f64 3d", |b| {
        b.iter(|| {
            let _ = black_box(VEC_F64_3D * 10.0);
        })
    });
}

criterion_group!(
    benches,
    bench_vector_unary_neg_i32_3d,
    bench_vector_unary_neg_f64_3d,
    bench_vector_add_i32_3d,
    bench_vector_add_f64_3d,
    bench_vector_subtract_i32_3d,
    bench_vector_subtract_f64_3d,
    bench_vector_multiplication_i32_3d,
    bench_vector_multiplication_f64_3d,
);
criterion_main!(benches);
