#![allow(unused_imports)]
use iai::{black_box, main};

use vectora::Vector;

const VEC_I32_3D: Vector<i32, 3_usize> = Vector {
    components: [1, 2, 3],
};

const VEC_F64_3D: Vector<f64, 3_usize> = Vector {
    components: [1.0, 2.0, 3.0],
};

// ================================
//
// Unary neg
//
// ================================

fn vector_unary_neg_i32_3d() {
    let _ = -VEC_I32_3D;
}

fn vector_unary_neg_f64_3d() {
    let _ = -VEC_F64_3D;
}

// ================================
//
// Addition
//
// ================================

fn vector_add_i32_3d() {
    let _ = black_box(VEC_I32_3D + VEC_I32_3D);
}

fn vector_add_f64_3d() {
    let _ = black_box(VEC_F64_3D + VEC_F64_3D);
}

// ================================
//
// Subtraction
//
// ================================`

fn vector_subtract_i32_3d() {
    let _ = black_box(VEC_I32_3D - VEC_I32_3D);
}

fn vector_subtract_f64_3d() {
    let _ = black_box(VEC_F64_3D - VEC_F64_3D);
}

// ================================
//
// Scalar multiplication
//
// ================================`

fn vector_multiplication_i32_3d() {
    let _ = black_box(VEC_I32_3D * 10);
}

fn vector_multiplication_f64_3d() {
    let _ = black_box(VEC_F64_3D * 10.0);
}

iai::main!(
    vector_unary_neg_i32_3d,
    vector_unary_neg_f64_3d,
    vector_add_i32_3d,
    vector_add_f64_3d,
    vector_subtract_i32_3d,
    vector_subtract_f64_3d,
    vector_multiplication_i32_3d,
    vector_multiplication_f64_3d,
);
