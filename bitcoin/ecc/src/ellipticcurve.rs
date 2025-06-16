use crate::finitefield::FieldElement;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: Option<FieldElement>,
    pub y: Option<FieldElement>,
    pub a: FieldElement,
    pub b: FieldElement,
}

impl Point {
    pub fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> Result<Self, String> {
        let x_cloned = x.clone();
        let y_cloned = y.clone();

        match (x_cloned, y_cloned) {
            (Some(x_val), Some(y_val)) => {
                let lhs = y_val.clone() * y_val.clone();
                let rhs = x_val.clone() * x_val.clone() * x_val.clone()
                    + a.clone() * x_val.clone()
                    + b.clone();

                if lhs != rhs {
                    return Err("Point is not on the curve".to_string());
                }
            }
            (None, None) => {}
            _ => return Err("Either both x and y must be Some(...) or None".to_string()),
        }

        Ok(Point { x, y, a, b })
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.a != other.a || self.b != other.b {
            panic!(
                "Points {:?} and {:?} are not on the same curve",
                self, other
            );
        }

        // Identity
        if self.x.is_none() {
            return other;
        }
        if other.x.is_none() {
            return self;
        }

        // Clone all values needed more than once
        let x1 = self.x.clone().unwrap();
        let y1 = self.y.clone().unwrap();
        let x2 = other.x.clone().unwrap();
        let y2 = other.y.clone().unwrap();

        // Vertical line
        if x1 == x2 && y1 != y2 {
            return Point::new(None, None, self.a.clone(), self.b.clone()).unwrap();
        }

        // General case: x1 ≠ x2
        if x1 != x2 {
            let s = (y2.clone() - y1.clone()) / (x2.clone() - x1.clone());
            let x3 = s.clone() * s.clone() - x1.clone() - x2;
            let y3 = s * (x1 - x3.clone()) - y1;
            return Point::new(Some(x3), Some(y3), self.a.clone(), self.b.clone()).unwrap();
        }

        // Doubling: P == P
        if self == other {
            if y1 == FieldElement::new(0u32, y1.prime()).unwrap() {
                return Point::new(None, None, self.a.clone(), self.b.clone()).unwrap();
            }

            let three = FieldElement::new(3u32, x1.prime()).unwrap();
            let two = FieldElement::new(2u32, x1.prime()).unwrap();

            let s = (three * x1.clone() * x1.clone() + self.a.clone()) / (two.clone() * y1.clone());
            let x3 = s.clone() * s.clone() - two * x1.clone();
            let y3 = s * (x1 - x3.clone()) - y1;

            return Point::new(Some(x3), Some(y3), self.a.clone(), self.b.clone()).unwrap();
        }

        panic!("Unhandled point addition case");
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, coefficient: i32) -> Self::Output {
        let mut coef = coefficient;
        let mut current = self.clone();
        let mut result = Point::new(None, None, self.a.clone(), self.b.clone()).unwrap();

        while coef > 0 {
            if coef & 1 == 1 {
                result = result + current.clone();
            }
            current = current.clone() + current;
            coef >>= 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_192_105_on_curve() {
        let prime = 223u32;
        let a = FieldElement::new(0u32, prime).unwrap();
        let b = FieldElement::new(7u32, prime).unwrap();
        let x = FieldElement::new(192u32, prime).unwrap();
        let y = FieldElement::new(105u32, prime).unwrap();
        let p = Point::new(Some(x), Some(y), a, b);
        assert!(p.is_ok());
    }

    #[test]
    fn test_point_17_56_on_curve() {
        let prime = 223u32;
        let a = FieldElement::new(0u32, prime).unwrap();
        let b = FieldElement::new(7u32, prime).unwrap();
        let x = FieldElement::new(17u32, prime).unwrap();
        let y = FieldElement::new(56u32, prime).unwrap();
        let p = Point::new(Some(x), Some(y), a, b);
        assert!(p.is_ok());
    }

    #[test]
    fn test_point_200_119_not_on_curve() {
        let prime = 223u32;
        let a = FieldElement::new(0u32, prime).unwrap();
        let b = FieldElement::new(7u32, prime).unwrap();
        let x = FieldElement::new(200u32, prime).unwrap();
        let y = FieldElement::new(119u32, prime).unwrap();
        let p = Point::new(Some(x), Some(y), a, b);
        assert!(p.is_err());
    }

    #[test]
    fn test_point_1_193_on_curve() {
        let prime = 223u32;
        let a = FieldElement::new(0u32, prime).unwrap();
        let b = FieldElement::new(7u32, prime).unwrap();
        let x = FieldElement::new(1u32, prime).unwrap();
        let y = FieldElement::new(193u32, prime).unwrap();
        let p = Point::new(Some(x), Some(y), a, b);
        assert!(p.is_ok());
    }

    #[test]
    fn test_point_42_99_not_on_curve() {
        let prime = 223u32;
        let a = FieldElement::new(0u32, prime).unwrap();
        let b = FieldElement::new(7u32, prime).unwrap();
        let x = FieldElement::new(42u32, prime).unwrap();
        let y = FieldElement::new(99u32, prime).unwrap();
        let p = Point::new(Some(x), Some(y), a, b);
        assert!(p.is_err());
    }

    #[test]
    fn test_add_170_142_and_60_139() {
        let prime = 223u32;
        let a = FieldElement::new(0u32, prime).unwrap();
        let b = FieldElement::new(7u32, prime).unwrap();
        let x1 = FieldElement::new(170u32, prime).unwrap();
        let y1 = FieldElement::new(142u32, prime).unwrap();
        let x2 = FieldElement::new(60u32, prime).unwrap();
        let y2 = FieldElement::new(139u32, prime).unwrap();
        let p1 = Point::new(Some(x1), Some(y1), a.clone(), b.clone()).unwrap();
        let p2 = Point::new(Some(x2), Some(y2), a.clone(), b.clone()).unwrap();
        let result = p1 + p2;
        let expected = Point::new(
            Some(FieldElement::new(220u32, prime).unwrap()),
            Some(FieldElement::new(181u32, prime).unwrap()),
            a.clone(),
            b.clone(),
        )
        .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_47_71_and_17_56() {
        let prime = 223u32;
        let a = FieldElement::new(0u32, prime).unwrap();
        let b = FieldElement::new(7u32, prime).unwrap();
        let x1 = FieldElement::new(47u32, prime).unwrap();
        let y1 = FieldElement::new(71u32, prime).unwrap();
        let x2 = FieldElement::new(17u32, prime).unwrap();
        let y2 = FieldElement::new(56u32, prime).unwrap();
        let p1 = Point::new(Some(x1), Some(y1), a.clone(), b.clone()).unwrap();
        let p2 = Point::new(Some(x2), Some(y2), a.clone(), b.clone()).unwrap();
        let result = p1 + p2;
        let expected = Point::new(
            Some(FieldElement::new(215u32, prime).unwrap()),
            Some(FieldElement::new(68u32, prime).unwrap()),
            a.clone(),
            b.clone(),
        )
        .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_143_98_and_76_66() {
        let prime = 223u32;
        let a = FieldElement::new(0u32, prime).unwrap();
        let b = FieldElement::new(7u32, prime).unwrap();
        let x1 = FieldElement::new(143u32, prime).unwrap();
        let y1 = FieldElement::new(98u32, prime).unwrap();
        let x2 = FieldElement::new(76u32, prime).unwrap();
        let y2 = FieldElement::new(66u32, prime).unwrap();
        let p1 = Point::new(Some(x1), Some(y1), a.clone(), b.clone()).unwrap();
        let p2 = Point::new(Some(x2), Some(y2), a.clone(), b.clone()).unwrap();
        let result = p1 + p2;
        let expected = Point::new(
            Some(FieldElement::new(47u32, prime).unwrap()),
            Some(FieldElement::new(71u32, prime).unwrap()),
            a.clone(),
            b.clone(),
        )
        .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_order_of_point_15_86_is_7() {
        let prime = 223u32;
        let a = FieldElement::new(0u32, prime).unwrap();
        let b = FieldElement::new(7u32, prime).unwrap();

        let x = FieldElement::new(15u32, prime).unwrap();
        let y = FieldElement::new(86u32, prime).unwrap();

        let p = Point::new(Some(x), Some(y), a.clone(), b.clone()).unwrap();
        let infinity = Point::new(None, None, a.clone(), b.clone()).unwrap();

        let mut product = p.clone();
        let mut count = 1;

        while product != infinity {
            product = product + p.clone();
            count += 1;
        }

        assert_eq!(count, 7);
    }
}
