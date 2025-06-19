use crate::{
    ellipticcurve::Point,
    finitefield::FieldElement,
    secp256k1::{
        constants::{G, N, P},
        signature::Signature,
    },
};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::ops::{Add, Mul};

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

    pub fn verify(&self, z: BigUint, sig: Signature) -> bool {
        let s_inv = sig.s.modpow(&(N.clone() - BigUint::from(2u8)), &N);
        let u = (&z * &s_inv) % N.clone();
        let v = (&sig.r * &s_inv) % N.clone();

        let u_g = G.clone() * u;
        let v_p = self.clone() * v;
        let total = u_g + v_p;

        match total.inner().x {
            Some(ref x) => x.num == sig.r,
            None => false,
        }
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

impl Add for S256Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        S256Point(self.0 + rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;
    use num_traits::Num;

    #[test]
    fn test_signature_1_is_valid() {
        let point = S256Point::new(
            Some(
                BigUint::from_str_radix(
                    "887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c",
                    16,
                )
                .unwrap(),
            ),
            Some(
                BigUint::from_str_radix(
                    "61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34",
                    16,
                )
                .unwrap(),
            ),
        );

        let z = BigUint::from_str_radix(
            "ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60",
            16,
        )
        .unwrap();
        let r = BigUint::from_str_radix(
            "ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395",
            16,
        )
        .unwrap();
        let s = BigUint::from_str_radix(
            "68342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4",
            16,
        )
        .unwrap();

        let sig = Signature { r: r.clone(), s };
        assert!(point.verify(z, sig));
    }

    #[test]
    fn test_signature_2_is_valid() {
        let point = S256Point::new(
            Some(
                BigUint::from_str_radix(
                    "887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c",
                    16,
                )
                .unwrap(),
            ),
            Some(
                BigUint::from_str_radix(
                    "61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34",
                    16,
                )
                .unwrap(),
            ),
        );

        let z = BigUint::from_str_radix(
            "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
            16,
        )
        .unwrap();
        let r = BigUint::from_str_radix(
            "eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c",
            16,
        )
        .unwrap();
        let s = BigUint::from_str_radix(
            "c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6",
            16,
        )
        .unwrap();

        let sig = Signature { r: r.clone(), s };
        assert!(point.verify(z, sig));
    }
}
