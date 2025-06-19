use num_bigint::BigUint;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub r: BigUint,
    pub s: BigUint,
}

impl Signature {
    pub fn new(r: BigUint, s: BigUint) -> Self {
        Signature { r, s }
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Signature({:x}, {:x})", self.r, self.s)
    }
}
