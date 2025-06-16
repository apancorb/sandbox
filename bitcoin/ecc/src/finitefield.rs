use num_bigint::BigUint;
use num_traits::One;
use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElement {
    pub num: BigUint,
    pub prime: BigUint,
}

impl FieldElement {
    pub fn new<N: Into<BigUint>, P: Into<BigUint>>(num: N, prime: P) -> Result<Self, String> {
        let num = num.into();
        let prime = prime.into();
        if num >= prime {
            return Err(format!(
                "Num {} not in field range 0 to {}",
                num,
                &prime - BigUint::one()
            ));
        }
        Ok(FieldElement { num, prime })
    }

    pub fn prime(&self) -> BigUint {
        self.prime.clone()
    }

    pub fn pow(&self, exponent: i64) -> Self {
        let n = if exponent < 0 {
            let inv = self
                .num
                .modpow(&(self.prime.clone() - BigUint::from(2u32)), &self.prime);
            inv.modpow(&BigUint::from((-exponent) as u64), &self.prime)
        } else {
            self.num
                .modpow(&BigUint::from(exponent as u64), &self.prime)
        };
        FieldElement {
            num: n,
            prime: self.prime.clone(),
        }
    }

    fn check_same_field(&self, other: &Self) {
        if self.prime != other.prime {
            panic!("Cannot operate on two numbers in different Fields");
        }
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.check_same_field(&rhs);
        let num = (self.num + rhs.num) % &self.prime;
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.check_same_field(&rhs);
        let num = (self.num + &self.prime - rhs.num) % &self.prime;
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.check_same_field(&rhs);
        let num = (self.num * rhs.num) % &self.prime;
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.check_same_field(&rhs);
        let inv = rhs.pow(-1);
        self * inv
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement({} mod {})", self.num, self.prime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;

    fn fe(num: u32, prime: u32) -> FieldElement {
        FieldElement::new(BigUint::from(num), BigUint::from(prime)).unwrap()
    }

    #[test]
    fn test_addition_44_plus_33() {
        assert_eq!(fe(44, 57) + fe(33, 57), fe(20, 57));
    }

    #[test]
    fn test_subtraction_9_minus_29() {
        assert_eq!(fe(9, 57) - fe(29, 57), fe(37, 57));
    }

    #[test]
    fn test_addition_17_plus_42_plus_49() {
        assert_eq!(fe(17, 57) + fe(42, 57) + fe(49, 57), fe(51, 57));
    }

    #[test]
    fn test_subtraction_52_minus_30_minus_38() {
        assert_eq!(fe(52, 57) - fe(30, 57) - fe(38, 57), fe(41, 57));
    }

    #[test]
    fn test_multiplication_95_45_31() {
        assert_eq!(fe(95, 97) * fe(45, 97) * fe(31, 97), fe(23, 97));
    }

    #[test]
    fn test_multiplication_17_13_19_44() {
        assert_eq!(
            fe(17, 97) * fe(13, 97) * fe(19, 97) * fe(44, 97),
            fe(68, 97)
        );
    }

    #[test]
    fn test_exponentiation_12_pow_7_times_77_pow_49() {
        let a = fe(12, 97).pow(7);
        let b = fe(77, 97).pow(49);
        assert_eq!(a * b, fe(63, 97));
    }

    #[test]
    fn test_division_3_div_24() {
        assert_eq!(fe(3, 31) / fe(24, 31), fe(4, 31));
    }

    #[test]
    fn test_inverse_exponent_17_pow_neg3() {
        assert_eq!(fe(17, 31).pow(-3), fe(29, 31));
    }

    #[test]
    fn test_inverse_pow_4_pow_neg4_times_11() {
        let inv = fe(4, 31).pow(-4);
        let eleven = fe(11, 31);
        assert_eq!(inv * eleven, fe(13, 31));
    }
}
