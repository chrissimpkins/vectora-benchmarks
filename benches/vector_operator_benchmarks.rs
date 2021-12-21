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
// Addition
//
// ================================

fn vector_add_i32() {
    let _ = black_box(VEC_I32_3D + VEC_I32_3D);
}

fn vector_add_f64() {
    let _ = black_box(VEC_F64_3D + VEC_F64_3D);
}

iai::main!(vector_add_i32, vector_add_f64,);
