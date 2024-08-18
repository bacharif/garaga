pub mod bls12_381_final_exp_witness;
pub mod bn254_final_exp_witness;
pub mod ecip;

use ark_ec::{pairing::Pairing, AffineRepr};
use ark_ff::PrimeField;
use lambdaworks_crypto::hash::poseidon::{starknet::PoseidonCairoStark252, Poseidon};
use lambdaworks_math::{
    field::{
        element::FieldElement, fields::fft_friendly::stark_252_prime_field::Stark252PrimeField,
    },
    traits::ByteConversion,
};
use num_bigint::BigUint;
use pyo3::{
    types::{PyBytes, PyInt, PyList, PyTuple},
    {prelude::*, wrap_pyfunction},
};

use crate::ecip::core::__pyo3_get_function_zk_ecip_hint;

#[pymodule]
fn garaga_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(g2_add, m)?)?;
    m.add_function(wrap_pyfunction!(g2_scalar_mul, m)?)?;
    m.add_function(wrap_pyfunction!(multi_pairing, m)?)?;
    m.add_function(wrap_pyfunction!(multi_miller_loop, m)?)?;
    m.add_function(wrap_pyfunction!(get_final_exp_witness, m)?)?;
    m.add_function(wrap_pyfunction!(hades_permutation, m)?)?;
    m.add_function(wrap_pyfunction!(zk_ecip_hint, m)?)?;
    Ok(())
}

const CURVE_BN254: usize = 0;
const CURVE_BLS12_381: usize = 1;

#[pyfunction]
fn g2_add(
    py: Python,
    curve_id: usize,
    py_tuple_1: &PyTuple,
    py_tuple_2: &PyTuple,
) -> PyResult<PyObject> {
    let a_0: BigUint = py_tuple_1[0].extract()?;
    let a_1: BigUint = py_tuple_1[1].extract()?;
    let a_2: BigUint = py_tuple_1[2].extract()?;
    let a_3: BigUint = py_tuple_1[3].extract()?;
    let b_0: BigUint = py_tuple_2[0].extract()?;
    let b_1: BigUint = py_tuple_2[1].extract()?;
    let b_2: BigUint = py_tuple_2[2].extract()?;
    let b_3: BigUint = py_tuple_2[3].extract()?;

    if curve_id == CURVE_BN254 {
        use ark_bn254::{Fq, Fq2, G2Affine};
        let a = G2Affine::new(
            Fq2::new(Fq::from(a_0), Fq::from(a_1)),
            Fq2::new(Fq::from(a_2), Fq::from(a_3)),
        );
        let b = G2Affine::new(
            Fq2::new(Fq::from(b_0), Fq::from(b_1)),
            Fq2::new(Fq::from(b_2), Fq::from(b_3)),
        );
        let c: G2Affine = (a + b).into();
        let py_tuple = PyTuple::new(
            py,
            [
                BigUint::from(c.x.c0.into_bigint()),
                BigUint::from(c.x.c1.into_bigint()),
                BigUint::from(c.y.c0.into_bigint()),
                BigUint::from(c.y.c1.into_bigint()),
            ],
        );
        return Ok(py_tuple.into());
    }

    if curve_id == CURVE_BLS12_381 {
        use ark_bls12_381::{Fq, Fq2, G2Affine};
        let a = G2Affine::new(
            Fq2::new(Fq::from(a_0), Fq::from(a_1)),
            Fq2::new(Fq::from(a_2), Fq::from(a_3)),
        );
        let b = G2Affine::new(
            Fq2::new(Fq::from(b_0), Fq::from(b_1)),
            Fq2::new(Fq::from(b_2), Fq::from(b_3)),
        );
        let c: G2Affine = (a + b).into();
        let py_tuple = PyTuple::new(
            py,
            [
                BigUint::from(c.x.c0.into_bigint()),
                BigUint::from(c.x.c1.into_bigint()),
                BigUint::from(c.y.c0.into_bigint()),
                BigUint::from(c.y.c1.into_bigint()),
            ],
        );
        return Ok(py_tuple.into());
    }

    panic!("Curve ID {} not supported", curve_id);
}

#[pyfunction]
fn g2_scalar_mul(
    py: Python,
    curve_id: usize,
    py_tuple_1: &PyTuple,
    py_int_2: &PyInt,
) -> PyResult<PyObject> {
    let a_0: BigUint = py_tuple_1[0].extract()?;
    let a_1: BigUint = py_tuple_1[1].extract()?;
    let a_2: BigUint = py_tuple_1[2].extract()?;
    let a_3: BigUint = py_tuple_1[3].extract()?;
    let k: BigUint = py_int_2.extract()?;

    if curve_id == CURVE_BN254 {
        use ark_bn254::{Fq, Fq2, G2Affine};
        let a = G2Affine::new(
            Fq2::new(Fq::from(a_0), Fq::from(a_1)),
            Fq2::new(Fq::from(a_2), Fq::from(a_3)),
        );
        let c: G2Affine = a.mul_bigint(k.to_u64_digits()).into();
        let py_tuple = PyTuple::new(
            py,
            [
                BigUint::from(c.x.c0.into_bigint()),
                BigUint::from(c.x.c1.into_bigint()),
                BigUint::from(c.y.c0.into_bigint()),
                BigUint::from(c.y.c1.into_bigint()),
            ],
        );
        return Ok(py_tuple.into());
    }

    if curve_id == CURVE_BLS12_381 {
        use ark_bls12_381::{Fq, Fq2, G2Affine};
        let a = G2Affine::new(
            Fq2::new(Fq::from(a_0), Fq::from(a_1)),
            Fq2::new(Fq::from(a_2), Fq::from(a_3)),
        );
        let c: G2Affine = a.mul_bigint(k.to_u64_digits()).into();
        let py_tuple = PyTuple::new(
            py,
            [
                BigUint::from(c.x.c0.into_bigint()),
                BigUint::from(c.x.c1.into_bigint()),
                BigUint::from(c.y.c0.into_bigint()),
                BigUint::from(c.y.c1.into_bigint()),
            ],
        );
        return Ok(py_tuple.into());
    }

    panic!("Curve ID {} not supported", curve_id);
}

#[pyfunction]
fn multi_pairing(py: Python, curve_id: usize, py_list_1: &PyList) -> PyResult<PyObject> {
    assert!(py_list_1.len() % 6 == 0, "invalid length");

    if curve_id == CURVE_BN254 {
        use ark_bn254::{Bn254, Fq, Fq12, Fq2, G1Affine, G2Affine};
        let mut a_list = Vec::new();
        let mut b_list = Vec::new();
        for i in (0..py_list_1.len()).step_by(6) {
            let a_0: BigUint = py_list_1[i].extract()?;
            let a_1: BigUint = py_list_1[i + 1].extract()?;
            let b_0: BigUint = py_list_1[i + 2].extract()?;
            let b_1: BigUint = py_list_1[i + 3].extract()?;
            let b_2: BigUint = py_list_1[i + 4].extract()?;
            let b_3: BigUint = py_list_1[i + 5].extract()?;
            let a = G1Affine::new(Fq::from(a_0), Fq::from(a_1));
            let b = G2Affine::new(
                Fq2::new(Fq::from(b_0), Fq::from(b_1)),
                Fq2::new(Fq::from(b_2), Fq::from(b_3)),
            );
            a_list.push(a);
            b_list.push(b);
        }
        let c = Bn254::multi_pairing(a_list, b_list);
        fn to(v: Fq12) -> [BigUint; 12] {
            [
                BigUint::from(v.c0.c0.c0.into_bigint()),
                BigUint::from(v.c0.c0.c1.into_bigint()),
                BigUint::from(v.c0.c1.c0.into_bigint()),
                BigUint::from(v.c0.c1.c1.into_bigint()),
                BigUint::from(v.c0.c2.c0.into_bigint()),
                BigUint::from(v.c0.c2.c1.into_bigint()),
                BigUint::from(v.c1.c0.c0.into_bigint()),
                BigUint::from(v.c1.c0.c1.into_bigint()),
                BigUint::from(v.c1.c1.c0.into_bigint()),
                BigUint::from(v.c1.c1.c1.into_bigint()),
                BigUint::from(v.c1.c2.c0.into_bigint()),
                BigUint::from(v.c1.c2.c1.into_bigint()),
            ]
        }
        let py_list = PyList::new(py, to(c.0));
        return Ok(py_list.into());
    }

    if curve_id == CURVE_BLS12_381 {
        use ark_bls12_381::{Bls12_381, Fq, Fq12, Fq2, G1Affine, G2Affine};
        let mut a_list = Vec::new();
        let mut b_list = Vec::new();
        for i in (0..py_list_1.len()).step_by(6) {
            let a_0: BigUint = py_list_1[i].extract()?;
            let a_1: BigUint = py_list_1[i + 1].extract()?;
            let b_0: BigUint = py_list_1[i + 2].extract()?;
            let b_1: BigUint = py_list_1[i + 3].extract()?;
            let b_2: BigUint = py_list_1[i + 4].extract()?;
            let b_3: BigUint = py_list_1[i + 5].extract()?;
            let a = G1Affine::new(Fq::from(a_0), Fq::from(a_1));
            let b = G2Affine::new(
                Fq2::new(Fq::from(b_0), Fq::from(b_1)),
                Fq2::new(Fq::from(b_2), Fq::from(b_3)),
            );
            a_list.push(a);
            b_list.push(b);
        }
        let c = Bls12_381::multi_pairing(a_list, b_list);
        fn to(v: Fq12) -> [BigUint; 12] {
            [
                BigUint::from(v.c0.c0.c0.into_bigint()),
                BigUint::from(v.c0.c0.c1.into_bigint()),
                BigUint::from(v.c0.c1.c0.into_bigint()),
                BigUint::from(v.c0.c1.c1.into_bigint()),
                BigUint::from(v.c0.c2.c0.into_bigint()),
                BigUint::from(v.c0.c2.c1.into_bigint()),
                BigUint::from(v.c1.c0.c0.into_bigint()),
                BigUint::from(v.c1.c0.c1.into_bigint()),
                BigUint::from(v.c1.c1.c0.into_bigint()),
                BigUint::from(v.c1.c1.c1.into_bigint()),
                BigUint::from(v.c1.c2.c0.into_bigint()),
                BigUint::from(v.c1.c2.c1.into_bigint()),
            ]
        }
        let py_list = PyList::new(py, to(c.0));
        return Ok(py_list.into());
    }

    panic!("Curve ID {} not supported", curve_id);
}

#[pyfunction]
fn multi_miller_loop(py: Python, curve_id: usize, py_list_1: &PyList) -> PyResult<PyObject> {
    assert!(py_list_1.len() % 6 == 0, "invalid length");

    if curve_id == CURVE_BN254 {
        use ark_bn254::{Bn254, Fq, Fq12, Fq2, G1Affine, G2Affine};
        let mut a_list = Vec::new();
        let mut b_list = Vec::new();
        for i in (0..py_list_1.len()).step_by(6) {
            let a_0: BigUint = py_list_1[i].extract()?;
            let a_1: BigUint = py_list_1[i + 1].extract()?;
            let b_0: BigUint = py_list_1[i + 2].extract()?;
            let b_1: BigUint = py_list_1[i + 3].extract()?;
            let b_2: BigUint = py_list_1[i + 4].extract()?;
            let b_3: BigUint = py_list_1[i + 5].extract()?;
            let a = G1Affine::new(Fq::from(a_0), Fq::from(a_1));
            let b = G2Affine::new(
                Fq2::new(Fq::from(b_0), Fq::from(b_1)),
                Fq2::new(Fq::from(b_2), Fq::from(b_3)),
            );
            a_list.push(a);
            b_list.push(b);
        }
        let c = Bn254::multi_miller_loop(a_list, b_list);
        fn to(v: Fq12) -> [BigUint; 12] {
            [
                BigUint::from(v.c0.c0.c0.into_bigint()),
                BigUint::from(v.c0.c0.c1.into_bigint()),
                BigUint::from(v.c0.c1.c0.into_bigint()),
                BigUint::from(v.c0.c1.c1.into_bigint()),
                BigUint::from(v.c0.c2.c0.into_bigint()),
                BigUint::from(v.c0.c2.c1.into_bigint()),
                BigUint::from(v.c1.c0.c0.into_bigint()),
                BigUint::from(v.c1.c0.c1.into_bigint()),
                BigUint::from(v.c1.c1.c0.into_bigint()),
                BigUint::from(v.c1.c1.c1.into_bigint()),
                BigUint::from(v.c1.c2.c0.into_bigint()),
                BigUint::from(v.c1.c2.c1.into_bigint()),
            ]
        }
        let py_list = PyList::new(py, to(c.0));
        return Ok(py_list.into());
    }

    if curve_id == CURVE_BLS12_381 {
        use ark_bls12_381::{Bls12_381, Fq, Fq12, Fq2, G1Affine, G2Affine};
        let mut a_list = Vec::new();
        let mut b_list = Vec::new();
        for i in (0..py_list_1.len()).step_by(6) {
            let a_0: BigUint = py_list_1[i].extract()?;
            let a_1: BigUint = py_list_1[i + 1].extract()?;
            let b_0: BigUint = py_list_1[i + 2].extract()?;
            let b_1: BigUint = py_list_1[i + 3].extract()?;
            let b_2: BigUint = py_list_1[i + 4].extract()?;
            let b_3: BigUint = py_list_1[i + 5].extract()?;
            let a = G1Affine::new(Fq::from(a_0), Fq::from(a_1));
            let b = G2Affine::new(
                Fq2::new(Fq::from(b_0), Fq::from(b_1)),
                Fq2::new(Fq::from(b_2), Fq::from(b_3)),
            );
            a_list.push(a);
            b_list.push(b);
        }
        let c = Bls12_381::multi_miller_loop(a_list, b_list);
        fn to(v: Fq12) -> [BigUint; 12] {
            [
                BigUint::from(v.c0.c0.c0.into_bigint()),
                BigUint::from(v.c0.c0.c1.into_bigint()),
                BigUint::from(v.c0.c1.c0.into_bigint()),
                BigUint::from(v.c0.c1.c1.into_bigint()),
                BigUint::from(v.c0.c2.c0.into_bigint()),
                BigUint::from(v.c0.c2.c1.into_bigint()),
                BigUint::from(v.c1.c0.c0.into_bigint()),
                BigUint::from(v.c1.c0.c1.into_bigint()),
                BigUint::from(v.c1.c1.c0.into_bigint()),
                BigUint::from(v.c1.c1.c1.into_bigint()),
                BigUint::from(v.c1.c2.c0.into_bigint()),
                BigUint::from(v.c1.c2.c1.into_bigint()),
            ]
        }
        let py_list = PyList::new(py, to(c.0));
        return Ok(py_list.into());
    }

    panic!("Curve ID {} not supported", curve_id);
}

#[pyfunction]
fn get_final_exp_witness(py: Python, curve_id: usize, py_list: &PyList) -> PyResult<PyObject> {
    let f_0: BigUint = py_list[0].extract()?;
    let f_1: BigUint = py_list[1].extract()?;
    let f_2: BigUint = py_list[2].extract()?;
    let f_3: BigUint = py_list[3].extract()?;
    let f_4: BigUint = py_list[4].extract()?;
    let f_5: BigUint = py_list[5].extract()?;
    let f_6: BigUint = py_list[6].extract()?;
    let f_7: BigUint = py_list[7].extract()?;
    let f_8: BigUint = py_list[8].extract()?;
    let f_9: BigUint = py_list[9].extract()?;
    let f_10: BigUint = py_list[10].extract()?;
    let f_11: BigUint = py_list[11].extract()?;

    if curve_id == CURVE_BN254 {
        use ark_bn254::{Fq, Fq12, Fq2, Fq6};
        let f = Fq12::new(
            Fq6::new(
                Fq2::new(Fq::from(f_0), Fq::from(f_1)),
                Fq2::new(Fq::from(f_2), Fq::from(f_3)),
                Fq2::new(Fq::from(f_4), Fq::from(f_5)),
            ),
            Fq6::new(
                Fq2::new(Fq::from(f_6), Fq::from(f_7)),
                Fq2::new(Fq::from(f_8), Fq::from(f_9)),
                Fq2::new(Fq::from(f_10), Fq::from(f_11)),
            ),
        );
        let (c, wi) = bn254_final_exp_witness::get_final_exp_witness(f);
        fn to(v: Fq12) -> [BigUint; 12] {
            [
                BigUint::from(v.c0.c0.c0.into_bigint()),
                BigUint::from(v.c0.c0.c1.into_bigint()),
                BigUint::from(v.c0.c1.c0.into_bigint()),
                BigUint::from(v.c0.c1.c1.into_bigint()),
                BigUint::from(v.c0.c2.c0.into_bigint()),
                BigUint::from(v.c0.c2.c1.into_bigint()),
                BigUint::from(v.c1.c0.c0.into_bigint()),
                BigUint::from(v.c1.c0.c1.into_bigint()),
                BigUint::from(v.c1.c1.c0.into_bigint()),
                BigUint::from(v.c1.c1.c1.into_bigint()),
                BigUint::from(v.c1.c2.c0.into_bigint()),
                BigUint::from(v.c1.c2.c1.into_bigint()),
            ]
        }
        let py_tuple = PyTuple::new(py, [PyList::new(py, to(c)), PyList::new(py, to(wi))]);
        return Ok(py_tuple.into());
    }

    if curve_id == CURVE_BLS12_381 {
        use ark_bls12_381::{Fq, Fq12, Fq2, Fq6};
        let f = Fq12::new(
            Fq6::new(
                Fq2::new(Fq::from(f_0), Fq::from(f_1)),
                Fq2::new(Fq::from(f_2), Fq::from(f_3)),
                Fq2::new(Fq::from(f_4), Fq::from(f_5)),
            ),
            Fq6::new(
                Fq2::new(Fq::from(f_6), Fq::from(f_7)),
                Fq2::new(Fq::from(f_8), Fq::from(f_9)),
                Fq2::new(Fq::from(f_10), Fq::from(f_11)),
            ),
        );
        let (c, wi) = bls12_381_final_exp_witness::get_final_exp_witness(f);
        fn to(v: Fq12) -> [BigUint; 12] {
            [
                BigUint::from(v.c0.c0.c0.into_bigint()),
                BigUint::from(v.c0.c0.c1.into_bigint()),
                BigUint::from(v.c0.c1.c0.into_bigint()),
                BigUint::from(v.c0.c1.c1.into_bigint()),
                BigUint::from(v.c0.c2.c0.into_bigint()),
                BigUint::from(v.c0.c2.c1.into_bigint()),
                BigUint::from(v.c1.c0.c0.into_bigint()),
                BigUint::from(v.c1.c0.c1.into_bigint()),
                BigUint::from(v.c1.c1.c0.into_bigint()),
                BigUint::from(v.c1.c1.c1.into_bigint()),
                BigUint::from(v.c1.c2.c0.into_bigint()),
                BigUint::from(v.c1.c2.c1.into_bigint()),
            ]
        }
        let py_tuple = PyTuple::new(py, [PyList::new(py, to(c)), PyList::new(py, to(wi))]);
        return Ok(py_tuple.into());
    }

    panic!("Curve ID {} not supported", curve_id);
}

#[pyfunction]
fn hades_permutation(
    py: Python,
    py_value_1: &PyBytes,
    py_value_2: &PyBytes,
    py_value_3: &PyBytes,
) -> PyResult<PyObject> {
    let byte_slice_1: &[u8] = py_value_1.as_bytes();
    let byte_slice_2: &[u8] = py_value_2.as_bytes();
    let byte_slice_3: &[u8] = py_value_3.as_bytes();

    let mut state: Vec<FieldElement<Stark252PrimeField>> = vec![
        FieldElement::<Stark252PrimeField>::from_bytes_be(byte_slice_1)
            .expect("Unable to convert first param from bytes to FieldElement"),
        FieldElement::<Stark252PrimeField>::from_bytes_be(byte_slice_2)
            .expect("Unable to convert second param from bytes to FieldElement"),
        FieldElement::<Stark252PrimeField>::from_bytes_be(byte_slice_3)
            .expect("Unable to convert third param from bytes to FieldElement"),
    ];

    PoseidonCairoStark252::hades_permutation(&mut state);

    let py_tuple = PyTuple::new(
        py,
        state.iter().map(|fe| {
            let fe_bytes = fe.to_bytes_be();
            PyBytes::new(py, &fe_bytes)
        }),
    );

    Ok(py_tuple.into())
}
