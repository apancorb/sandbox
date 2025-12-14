use crate::{finitefield::FieldElement, secp256k1::constants::P};
use num_bigint::BigUint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S256Field(pub FieldElement);

impl S256Field {
    pub fn new(n: impl Into<BigUint>) -> Self {
        S256Field(FieldElement::new(n.into(), P.clone()).unwrap())
    }

    pub fn from_u64(n: u64) -> Self {
        Self::new(n)
    }

    pub fn inner(&self) -> &FieldElement {
        &self.0
    }
}

