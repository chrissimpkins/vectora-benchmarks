#![allow(unused_imports)]
use iai::{black_box, main};

use vectora::Vector;

// ================================
//
// Vector Initialization
//
// ================================

// ~~~~~~~~~~~~~~~~~~~~~~~~~~
// init Zero vectors
// ~~~~~~~~~~~~~~~~~~~~~~~~~~

fn vector_initialization_i32_zero() -> Vector<i32, 3> {
    Vector::<i32, 3>::zero()
}

fn vector_initialization_f64_zero() -> Vector<f64, 3> {
    Vector::<f64, 3>::zero()
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~
// init i32 3D Vectors
// ~~~~~~~~~~~~~~~~~~~~~~~~~~

fn vector_initialization_i32_3d_array() {
    let _ = Vector::<i32, 3>::from(black_box([1, 2, 3]));
}

fn vector_initialization_i32_3d_iter_over_array() {
    let _ = black_box([1, 2, 3]).into_iter().collect::<Vector<i32, 3>>();
}

fn vector_initialization_i32_iter_over_vec() {
    let _ = black_box(vec![1, 2, 3])
        .into_iter()
        .collect::<Vector<i32, 3>>();
}

fn vector_initialization_i32_try_from_array_slice() {
    let arr = [1, 2, 3];
    let sli = &arr[..];
    let _ = Vector::<i32, 3>::try_from(black_box(sli)).unwrap();
}

fn vector_initialization_i32_try_from_vec_slice() {
    let slvec = vec![1, 2, 3];
    let sli = &slvec[..];
    let _ = Vector::<i32, 3>::try_from(black_box(black_box(sli))).unwrap();
}

fn vector_initialization_i32_try_from_vec_reference() {
    let vect = vec![1, 2, 3];
    let _ = Vector::<i32, 3>::try_from(&vect).unwrap();
}

fn vector_initialization_i32_try_from_vec() {
    let vect = vec![1, 2, 3];
    let _: Vector<i32, 3> = Vector::try_from(black_box(vect)).unwrap();
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~
// init f64 3D Vectors
// ~~~~~~~~~~~~~~~~~~~~~~~~~~

fn vector_initialization_f64_3d_array() {
    let _ = Vector::<f64, 3>::from(black_box([1.0, 2.0, 3.0]));
}

fn vector_initialization_f64_3d_iter_over_array() {
    let _ = black_box([1.0, 2.0, 3.0])
        .into_iter()
        .collect::<Vector<f64, 3>>();
}

fn vector_initialization_f64_iter_over_vec() {
    let _ = black_box(vec![1.0, 2.0, 3.0])
        .into_iter()
        .collect::<Vector<f64, 3>>();
}

fn vector_initialization_f64_try_from_array_slice() {
    let arr = [1.0, 2.0, 3.0];
    let sli = &arr[..];
    let _ = Vector::<f64, 3>::try_from(black_box(sli)).unwrap();
}

fn vector_initialization_f64_try_from_vec_slice() {
    let slvec = vec![1.0, 2.0, 3.0];
    let sli = &slvec[..];
    let _ = Vector::<f64, 3>::try_from(black_box(black_box(sli))).unwrap();
}

fn vector_initialization_f64_try_from_vec_reference() {
    let vect = vec![1.0, 2.0, 3.0];
    let _ = Vector::<f64, 3>::try_from(&vect).unwrap();
}

fn vector_initialization_f64_try_from_vec() {
    let vect = vec![1.0, 2.0, 3.0];
    let _: Vector<f64, 3> = Vector::try_from(black_box(vect)).unwrap();
}

iai::main!(
    vector_initialization_i32_zero,
    vector_initialization_f64_zero,
    vector_initialization_i32_3d_array,
    vector_initialization_i32_3d_iter_over_array,
    vector_initialization_i32_iter_over_vec,
    vector_initialization_i32_try_from_array_slice,
    vector_initialization_i32_try_from_vec_slice,
    vector_initialization_i32_try_from_vec_reference,
    vector_initialization_i32_try_from_vec,
    vector_initialization_f64_3d_array,
    vector_initialization_f64_3d_iter_over_array,
    vector_initialization_f64_iter_over_vec,
    vector_initialization_f64_try_from_array_slice,
    vector_initialization_f64_try_from_vec_slice,
    vector_initialization_f64_try_from_vec_reference,
    vector_initialization_f64_try_from_vec,
);
