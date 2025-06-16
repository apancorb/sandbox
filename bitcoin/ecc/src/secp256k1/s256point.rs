use crate::{
    ellipticcurve::Point,
    finitefield::FieldElement,
    secp256k1::constants::{N, P},
};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::ops::Mul;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S256Point(pub Point);

impl S256Point {
    pub fn new(x: Option<BigUint>, y: Option<BigUint>) -> Self {
        let a = FieldElement::new(BigUint::zero(), P.clone()).unwrap();
        let b = FieldElement::new(BigUint::from(7u8), P.clone()).unwrap();

        let px = x.map(|v| FieldElement::new(v, P.clone()).unwrap());
        let py = y.map(|v| FieldElement::new(v, P.clone()).unwrap());

        let point = Point::new(px, py, a, b).unwrap();
        S256Point(point)
    }

    pub fn inner(&self) -> &Point {
        &self.0
    }
}

impl Mul<BigUint> for S256Point {
    type Output = Self;

    fn mul(self, coefficient: BigUint) -> Self {
        let coef = coefficient % N.clone();
        let mut result = Point::new(None, None, self.0.a.clone(), self.0.b.clone()).unwrap();
        let mut current = self.0.clone();
        let mut k = coef;

        while k > BigUint::zero() {
            if &k & BigUint::one() == BigUint::one() {
                result = result + current.clone();
            }
            current = current.clone() + current;
            k >>= 1;
        }

        S256Point(result)
    }
}
