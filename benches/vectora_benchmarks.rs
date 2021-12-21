#![allow(unused_imports)]
use iai::{black_box, main};

use vectora::Vector;

// ================================
//
// Vector Initialization
//
// ================================

// ~~~~~~~~~~~~~~~~~~~~~~~~~~
// Zero vectors
// ~~~~~~~~~~~~~~~~~~~~~~~~~~

fn vector_initialization_i32_zero() -> Vector<i32, 3> {
    Vector::<i32, 3>::zero()
}

fn vector_initialization_f64_zero() -> Vector<f64, 3> {
    Vector::<f64, 3>::zero()
}



iai::main!(
    vector_initialization_i32_zero,
    vector_initialization_f64_zero,
);