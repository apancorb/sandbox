use crate::finitefield::FieldElement;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        match (x, y) {
            (Some(x_val), Some(y_val)) => {
                let lhs = y_val * y_val;
                let rhs = x_val * x_val * x_val + a * x_val + b;
                if lhs != rhs {
                    return Err("Point is not on the curve".to_string());
                }
            }
            (None, None) => {} // point at infinity
            _ => {
                return Err("Either both x and y must be Some(...) or None".to_string());
            }
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

        let (x1, y1) = (self.x.unwrap(), self.y.unwrap());
        let (x2, y2) = (other.x.unwrap(), other.y.unwrap());

        // Vertical line → point at infinity
        if x1 == x2 && y1 != y2 {
            return Point::new(None, None, self.a, self.b).unwrap();
        }

        // General case: x1 ≠ x2
        if x1 != x2 {
            let s = (y2 - y1) / (x2 - x1);
            let x3 = s * s - x1 - x2;
            let y3 = s * (x1 - x3) - y1;
            return Point::new(Some(x3), Some(y3), self.a, self.b).unwrap();
        }

        // Doubling: P == P
        if self == other {
            if y1 == FieldElement::new(0, y1.prime()).unwrap() {
                return Point::new(None, None, self.a, self.b).unwrap();
            }

            let three = FieldElement::new(3, x1.prime()).unwrap();
            let two = FieldElement::new(2, x1.prime()).unwrap();

            let s = (three * x1 * x1 + self.a) / (two * y1);
            let x3 = s * s - two * x1;
            let y3 = s * (x1 - x3) - y1;
            return Point::new(Some(x3), Some(y3), self.a, self.b).unwrap();
        }

        panic!("Unhandled point addition case");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_192_105_on_curve() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let x = FieldElement::new(192, prime).unwrap();
        let y = FieldElement::new(105, prime).unwrap();
        let p = Point::new(Some(x), Some(y), a, b);
        assert!(p.is_ok());
    }

    #[test]
    fn test_point_17_56_on_curve() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let x = FieldElement::new(17, prime).unwrap();
        let y = FieldElement::new(56, prime).unwrap();
        let p = Point::new(Some(x), Some(y), a, b);
        assert!(p.is_ok());
    }

    #[test]
    fn test_point_200_119_not_on_curve() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let x = FieldElement::new(200, prime).unwrap();
        let y = FieldElement::new(119, prime).unwrap();
        let p = Point::new(Some(x), Some(y), a, b);
        assert!(p.is_err());
    }

    #[test]
    fn test_point_1_193_on_curve() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let x = FieldElement::new(1, prime).unwrap();
        let y = FieldElement::new(193, prime).unwrap();
        let p = Point::new(Some(x), Some(y), a, b);
        assert!(p.is_ok());
    }

    #[test]
    fn test_point_42_99_not_on_curve() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let x = FieldElement::new(42, prime).unwrap();
        let y = FieldElement::new(99, prime).unwrap();
        let p = Point::new(Some(x), Some(y), a, b);
        assert!(p.is_err());
    }

    #[test]
    fn test_add_170_142_and_60_139() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let x1 = FieldElement::new(170, prime).unwrap();
        let y1 = FieldElement::new(142, prime).unwrap();
        let x2 = FieldElement::new(60, prime).unwrap();
        let y2 = FieldElement::new(139, prime).unwrap();
        let p1 = Point::new(Some(x1), Some(y1), a, b).unwrap();
        let p2 = Point::new(Some(x2), Some(y2), a, b).unwrap();
        let result = p1 + p2;
        let expected = Point::new(
            Some(FieldElement::new(220, prime).unwrap()),
            Some(FieldElement::new(181, prime).unwrap()),
            a,
            b,
        )
        .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_47_71_and_17_56() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let x1 = FieldElement::new(47, prime).unwrap();
        let y1 = FieldElement::new(71, prime).unwrap();
        let x2 = FieldElement::new(17, prime).unwrap();
        let y2 = FieldElement::new(56, prime).unwrap();
        let p1 = Point::new(Some(x1), Some(y1), a, b).unwrap();
        let p2 = Point::new(Some(x2), Some(y2), a, b).unwrap();
        let result = p1 + p2;
        let expected = Point::new(
            Some(FieldElement::new(215, prime).unwrap()),
            Some(FieldElement::new(68, prime).unwrap()),
            a,
            b,
        )
        .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_143_98_and_76_66() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();
        let x1 = FieldElement::new(143, prime).unwrap();
        let y1 = FieldElement::new(98, prime).unwrap();
        let x2 = FieldElement::new(76, prime).unwrap();
        let y2 = FieldElement::new(66, prime).unwrap();
        let p1 = Point::new(Some(x1), Some(y1), a, b).unwrap();
        let p2 = Point::new(Some(x2), Some(y2), a, b).unwrap();
        let result = p1 + p2;
        let expected = Point::new(
            Some(FieldElement::new(47, prime).unwrap()),
            Some(FieldElement::new(71, prime).unwrap()),
            a,
            b,
        )
        .unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_order_of_point_15_86_is_7() {
        let prime = 223;
        let a = FieldElement::new(0, prime).unwrap();
        let b = FieldElement::new(7, prime).unwrap();

        let x = FieldElement::new(15, prime).unwrap();
        let y = FieldElement::new(86, prime).unwrap();

        let p = Point::new(Some(x), Some(y), a, b).unwrap();
        let infinity = Point::new(None, None, a, b).unwrap();

        let mut product = p;
        let mut count = 1;

        while product != infinity {
            product = product + p;
            count += 1;
        }

        assert_eq!(count, 7);
    }
}
