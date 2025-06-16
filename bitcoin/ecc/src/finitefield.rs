use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FieldElement {
    num: i32,
    prime: i32,
}

impl FieldElement {
    pub fn new(num: i32, prime: i32) -> Result<Self, String> {
        if num >= prime || num < 0 {
            return Err(format!("Num {} not in field range 0 to {}", num, prime - 1));
        }

        Ok(FieldElement { num, prime })
    }

    pub fn prime(&self) -> i32 {
        self.prime
    }

    pub fn pow(self, exponent: i32) -> Self {
        let mut base = self.num.rem_euclid(self.prime) as u64;
        let mut exp = exponent.abs() as u64;
        let modulus = self.prime as u64;
        let mut result = 1u64;

        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % modulus;
            }
            base = (base * base) % modulus;
            exp /= 2;
        }

        let result_num = if exponent < 0 {
            // Modular inverse: result^-1 ≡ result^(p-2)
            let inv = Self::mod_pow(result, modulus - 2, modulus);
            inv as i32
        } else {
            result as i32
        };

        FieldElement {
            num: result_num,
            prime: self.prime,
        }
    }

    fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
        let mut result = 1;
        base %= modulus;
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % modulus;
            }
            base = (base * base) % modulus;
            exp /= 2;
        }
        result
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        if self.prime != rhs.prime {
            panic!("Cannot add two numbers in different Fields");
        }

        let num = (self.num + rhs.num).rem_euclid(self.prime);
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        if self.prime != rhs.prime {
            panic!("Cannot subtract two numbers in different Fields");
        }

        let num = (self.num - rhs.num).rem_euclid(self.prime);
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        if self.prime != rhs.prime {
            panic!("Cannot multiply two numbers in different Fields");
        }

        let num = (self.num * rhs.num).rem_euclid(self.prime);
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        if self.prime != rhs.prime {
            panic!("Cannot divide two numbers in different Fields");
        }

        // Fermat’s Little Theorem: b^-1 ≡ b^(p-2) mod p
        let inv = rhs.pow((self.prime - 2) as i32);
        self * inv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition_44_plus_33() {
        let a = FieldElement::new(44, 57).expect("init failed");
        let b = FieldElement::new(33, 57).expect("init failed");
        let result = a + b;
        assert_eq!(result, FieldElement::new(20, 57).expect("init failed"));
    }

    #[test]
    fn test_subtraction_9_minus_29() {
        let a = FieldElement::new(9, 57).expect("init failed");
        let b = FieldElement::new(29, 57).expect("init failed");
        let result = a - b;
        assert_eq!(result, FieldElement::new(37, 57).expect("init failed"));
    }

    #[test]
    fn test_addition_17_plus_42_plus_49() {
        let a = FieldElement::new(17, 57).expect("init failed");
        let b = FieldElement::new(42, 57).expect("init failed");
        let c = FieldElement::new(49, 57).expect("init failed");
        let result = a + b + c;
        assert_eq!(result, FieldElement::new(51, 57).expect("init failed"));
    }

    #[test]
    fn test_subtraction_52_minus_30_minus_38() {
        let a = FieldElement::new(52, 57).expect("init failed");
        let b = FieldElement::new(30, 57).expect("init failed");
        let c = FieldElement::new(38, 57).expect("init failed");
        let result = a - b - c;
        assert_eq!(result, FieldElement::new(41, 57).expect("init failed"));
    }

    #[test]
    fn test_multiplication_95_45_31() {
        let a = FieldElement::new(95, 97).expect("init failed");
        let b = FieldElement::new(45, 97).expect("init failed");
        let c = FieldElement::new(31, 97).expect("init failed");

        let result = a * b * c;

        assert_eq!(result, FieldElement::new(23, 97).expect("init failed"));
    }

    #[test]
    fn test_multiplication_17_13_19_44() {
        let a = FieldElement::new(17, 97).expect("init failed");
        let b = FieldElement::new(13, 97).expect("init failed");
        let c = FieldElement::new(19, 97).expect("init failed");
        let d = FieldElement::new(44, 97).expect("init failed");

        let result = a * b * c * d;

        assert_eq!(result, FieldElement::new(68, 97).expect("init failed"));
    }

    #[test]
    fn test_exponentiation_12_pow_7_times_77_pow_49() {
        let a = FieldElement::new(12, 97).expect("init failed").pow(7);
        let b = FieldElement::new(77, 97).expect("init failed").pow(49);
        let result = a * b;
        assert_eq!(result, FieldElement::new(63, 97).expect("init failed"));
    }

    #[test]
    fn test_division_3_div_24() {
        let a = FieldElement::new(3, 31).expect("init failed");
        let b = FieldElement::new(24, 31).expect("init failed");

        let result = a / b;

        assert_eq!(result, FieldElement::new(4, 31).expect("init failed"));
    }

    #[test]
    fn test_inverse_exponent_17_pow_neg3() {
        let base = FieldElement::new(17, 31).expect("init failed");

        let result = base.pow(-3);

        assert_eq!(result, FieldElement::new(29, 31).expect("init failed"));
    }

    #[test]
    fn test_inverse_pow_4_pow_neg4_times_11() {
        let inv = FieldElement::new(4, 31).expect("init failed").pow(-4);
        let eleven = FieldElement::new(11, 31).expect("init failed");

        let result = inv * eleven;

        assert_eq!(result, FieldElement::new(13, 31).expect("init failed"));
    }
}
