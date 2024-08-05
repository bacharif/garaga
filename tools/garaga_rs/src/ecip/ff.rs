use lambdaworks_math::field::element::FieldElement;
use crate::ecip::polynomial::Polynomial;
use lambdaworks_math::field::traits::IsPrimeField;
use std::ops::{Add, Mul, Neg};
use crate::ecip::curve::{CURVES, CurveID};

#[derive(Debug, Clone)]
pub struct FF<F: IsPrimeField> {
    pub coeffs: Vec<Polynomial<FieldElement<F>>>,
    pub y2: Polynomial<FieldElement<F>>,
    pub p: u64,
    pub curve_id: CurveID,
}

impl<F: IsPrimeField> FF<F> {
    pub fn new(coeffs: Vec<Polynomial<FieldElement<F>>>, curve_id: CurveID) -> Self {
        let p = coeffs[0].coefficients()[0].field();
        let field = FieldElement::<F>::zero().field();

        let a = field.from(CURVES[curve_id as usize].a);
        let b = field.from(CURVES[curve_id as usize].b);

        let y2 = Polynomial::new(&[b, a, field.zero(), field.one()]);

        FF {
            coeffs,
            y2,
            p,
            curve_id,
        }
    }

    pub fn degree(&self) -> usize {
        self.coeffs.len() - 1
    }

    pub fn get(&self, i: usize) -> Polynomial<FieldElement<F>> {
        self.coeffs.get(i).cloned().unwrap_or_else(|| Polynomial::zero())
    }

    pub fn reduce(&self) -> FF<F> {
        let mut deg_0_coeff = self.coeffs[0].clone();
        let mut deg_1_coeff = self.coeffs[1].clone();
        let mut y2 = self.y2.clone();

        for (i, poly) in self.coeffs.iter().enumerate().skip(2) {
            if i % 2 == 0 {
                deg_0_coeff = deg_0_coeff + poly.clone() * y2.clone();
            } else {
                deg_1_coeff = deg_1_coeff + poly.clone() * y2.clone();
                y2 = y2.clone() * y2.clone();
            }
        }
        FF {
            coeffs: vec![deg_0_coeff, deg_1_coeff],
            y2: self.y2.clone(),
            p: self.p,
            curve_id: self.curve_id,
        }
    }

    pub fn neg_y(self) -> FF<F> {
        // Implement the neg_y logic
        self
    }
    
    pub fn normalize(&self) -> FF<F> {
        let coeff = self.coeffs[0].coefficients()[0].clone();
        FF {
            coeffs: self.coeffs.iter().map(|c| c.clone() * coeff.inv().unwrap()).collect(),
            y2: self.y2.clone(),
            p: self.p,
            curve_id: self.curve_id,
        }
    }
}

impl<F: IsPrimeField> Add for FF<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let max_degree = self.coeffs.len().max(other.coeffs.len());
        let mut coeffs = vec![Polynomial::zero(); max_degree];

        for i in 0..self.coeffs.len() {
            coeffs[i] = coeffs[i].clone() + self.coeffs[i].clone();
        }

        for i in 0..other.coeffs.len() {
            coeffs[i] = coeffs[i].clone() + other.coeffs[i].clone();
        }

        FF::new(coeffs, self.curve_id)
    }
}

impl<F: IsPrimeField> Mul for FF<F> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let max_degree = self.coeffs.len() + other.coeffs.len() - 1;
        let mut coeffs = vec![Polynomial::zero(); max_degree];

        for i in 0..self.coeffs.len() {
            for j in 0..other.coeffs.len() {
                coeffs[i + j] = coeffs[i + j].clone() + (self.coeffs[i].clone() * other.coeffs[j].clone());
            }
        }

        FF::new(coeffs, self.curve_id)
    }
}

impl<F: IsPrimeField> Neg for FF<F> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        FF::new(self.coeffs.iter().map(|c| -c.clone()).collect(), self.curve_id)
    }
}
