use std::ops::Add;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point {
    x: Option<i64>,
    y: Option<i64>,
    a: i64,
    b: i64,
}

impl Point {
    pub fn new(x: Option<i64>, y: Option<i64>, a: i64, b: i64) -> Result<Self, String> {
        match (x, y) {
            (Some(x_val), Some(y_val)) => {
                // Check curve equation: y^2 = x^3 + ax + b
                let lhs = y_val.pow(2);
                let rhs = x_val.pow(3) + a * x_val + b;
                if lhs != rhs {
                    return Err(format!("Point ({}, {}) is not on the curve", x_val, y_val));
                }
            }
            (None, None) => {
                // point at infinity, always allowed
            }
            _ => {
                return Err("Both x and y must be Some(...) or None".to_string());
            }
        }

        Ok(Point { x, y, a, b })
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // Ensure both points are on the same curve
        if self.a != other.a || self.b != other.b {
            panic!(
                "Points {:?} and {:?} are not on the same curve",
                self, other
            );
        }

        // Handle point at infinity cases
        if self.x.is_none() {
            return other;
        }
        if other.x.is_none() {
            return self;
        }

        // Unwrap safely now that we know both are Some
        let (x1, y1) = (self.x.unwrap(), self.y.unwrap());
        let (x2, y2) = (other.x.unwrap(), other.y.unwrap());

        // If x1 == x2 and y1 != y2 => vertical line => return point at infinity
        if x1 == x2 && y1 != y2 {
            return Point::new(None, None, self.a, self.b).unwrap();
        }

        // Distinct points: x1 ≠ x2
        if x1 != x2 {
            let s = (y2 - y1) as f64 / (x2 - x1) as f64;
            let x3 = s * s - x1 as f64 - x2 as f64;
            let y3 = s * (x1 as f64 - x3) - y1 as f64;
            return Point::new(Some(x3 as i64), Some(y3 as i64), self.a, self.b).unwrap();
        }

        // Point doubling: P1 == P2 and y1 != 0
        if self == other {
            // If y1 == 0, the tangent line is vertical → return point at infinity
            if y1 == 0 {
                return Point::new(None, None, self.a, self.b).unwrap();
            }

            let s = (3 * x1 * x1 + self.a) as f64 / (2 * y1) as f64;
            let x3 = s * s - 2.0 * x1 as f64;
            let y3 = s * (x1 as f64 - x3) - y1 as f64;

            return Point::new(
                Some(x3.round() as i64),
                Some(y3.round() as i64),
                self.a,
                self.b,
            )
            .unwrap();
        }

        // Placeholder for other addition logic (doubling, general case)
        unimplemented!("Point addition not fully implemented yet");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_2_4_not_on_curve() {
        let p = Point::new(Some(2), Some(4), 5, 7);
        assert!(p.is_err());
    }

    #[test]
    fn test_point_minus1_minus1_on_curve() {
        let p = Point::new(Some(-1), Some(-1), 5, 7);
        assert!(p.is_ok());
    }

    #[test]
    fn test_point_18_77_on_curve() {
        let p = Point::new(Some(18), Some(77), 5, 7);
        assert!(p.is_ok());
    }

    #[test]
    fn test_point_5_7_not_on_curve() {
        let p = Point::new(Some(5), Some(7), 5, 7);
        assert!(p.is_err());
    }
}
