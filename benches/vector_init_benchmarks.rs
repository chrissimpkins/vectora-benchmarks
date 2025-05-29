#![allow(unused_imports)]
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use vectora::Vector;

fn bench_vector_initialization_i32_zero(c: &mut Criterion) {
    c.bench_function("i32 zero", |b| b.iter(|| Vector::<i32, 3>::zero()));
}

fn bench_vector_initialization_f64_zero(c: &mut Criterion) {
    c.bench_function("f64 zero", |b| b.iter(|| Vector::<f64, 3>::zero()));
}

fn bench_vector_initialization_i32_3d_array(c: &mut Criterion) {
    c.bench_function("i32 3d array", |b| {
        b.iter(|| Vector::<i32, 3>::from(black_box([1, 2, 3])))
    });
}

fn bench_vector_initialization_i32_3d_iter_over_array(c: &mut Criterion) {
    c.bench_function("i32 3d iter over array", |b| {
        b.iter(|| black_box([1, 2, 3]).into_iter().collect::<Vector<i32, 3>>())
    });
}

fn bench_vector_initialization_i32_iter_over_vec(c: &mut Criterion) {
    c.bench_function("i32 iter over vec", |b| {
        b.iter(|| {
            black_box(vec![1, 2, 3])
                .into_iter()
                .collect::<Vector<i32, 3>>()
        })
    });
}

fn bench_vector_initialization_i32_try_from_array_slice(c: &mut Criterion) {
    c.bench_function("i32 try_from array slice", |b| {
        b.iter(|| {
            let arr = [1, 2, 3];
            let sli = &arr[..];
            Vector::<i32, 3>::try_from(black_box(sli)).unwrap()
        })
    });
}

fn bench_vector_initialization_i32_try_from_vec_slice(c: &mut Criterion) {
    c.bench_function("i32 try_from vec slice", |b| {
        b.iter(|| {
            let slvec = vec![1, 2, 3];
            let sli = &slvec[..];
            Vector::<i32, 3>::try_from(black_box(black_box(sli))).unwrap()
        })
    });
}

fn bench_vector_initialization_i32_try_from_vec_reference(c: &mut Criterion) {
    c.bench_function("i32 try_from vec reference", |b| {
        b.iter(|| {
            let vect = vec![1, 2, 3];
            Vector::<i32, 3>::try_from(&vect).unwrap()
        })
    });
}

fn bench_vector_initialization_i32_try_from_vec(c: &mut Criterion) {
    c.bench_function("i32 try_from vec", |b| {
        b.iter(|| {
            let vect = vec![1, 2, 3];
            Vector::<i32, 3>::try_from(black_box(vect)).unwrap()
        })
    });
}

fn bench_vector_initialization_f64_3d_array(c: &mut Criterion) {
    c.bench_function("f64 3d array", |b| {
        b.iter(|| Vector::<f64, 3>::from(black_box([1.0, 2.0, 3.0])))
    });
}

fn bench_vector_initialization_f64_3d_iter_over_array(c: &mut Criterion) {
    c.bench_function("f64 3d iter over array", |b| {
        b.iter(|| {
            black_box([1.0, 2.0, 3.0])
                .into_iter()
                .collect::<Vector<f64, 3>>()
        })
    });
}

fn bench_vector_initialization_f64_iter_over_vec(c: &mut Criterion) {
    c.bench_function("f64 iter over vec", |b| {
        b.iter(|| {
            black_box(vec![1.0, 2.0, 3.0])
                .into_iter()
                .collect::<Vector<f64, 3>>()
        })
    });
}

fn bench_vector_initialization_f64_try_from_array_slice(c: &mut Criterion) {
    c.bench_function("f64 try_from array slice", |b| {
        b.iter(|| {
            let arr = [1.0, 2.0, 3.0];
            let sli = &arr[..];
            Vector::<f64, 3>::try_from(black_box(sli)).unwrap()
        })
    });
}

fn bench_vector_initialization_f64_try_from_vec_slice(c: &mut Criterion) {
    c.bench_function("f64 try_from vec slice", |b| {
        b.iter(|| {
            let slvec = vec![1.0, 2.0, 3.0];
            let sli = &slvec[..];
            Vector::<f64, 3>::try_from(black_box(black_box(sli))).unwrap()
        })
    });
}

fn bench_vector_initialization_f64_try_from_vec_reference(c: &mut Criterion) {
    c.bench_function("f64 try_from vec reference", |b| {
        b.iter(|| {
            let vect = vec![1.0, 2.0, 3.0];
            Vector::<f64, 3>::try_from(&vect).unwrap()
        })
    });
}

fn bench_vector_initialization_f64_try_from_vec(c: &mut Criterion) {
    c.bench_function("f64 try_from vec", |b| {
        b.iter(|| {
            let vect = vec![1.0, 2.0, 3.0];
            Vector::<f64, 3>::try_from(black_box(vect)).unwrap()
        })
    });
}

criterion_group!(
    benches,
    bench_vector_initialization_i32_zero,
    bench_vector_initialization_f64_zero,
    bench_vector_initialization_i32_3d_array,
    bench_vector_initialization_i32_3d_iter_over_array,
    bench_vector_initialization_i32_iter_over_vec,
    bench_vector_initialization_i32_try_from_array_slice,
    bench_vector_initialization_i32_try_from_vec_slice,
    bench_vector_initialization_i32_try_from_vec_reference,
    bench_vector_initialization_i32_try_from_vec,
    bench_vector_initialization_f64_3d_array,
    bench_vector_initialization_f64_3d_iter_over_array,
    bench_vector_initialization_f64_iter_over_vec,
    bench_vector_initialization_f64_try_from_array_slice,
    bench_vector_initialization_f64_try_from_vec_slice,
    bench_vector_initialization_f64_try_from_vec_reference,
    bench_vector_initialization_f64_try_from_vec,
);
criterion_main!(benches);
