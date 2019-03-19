use crate::common::Fixed;
use crate::parser::Vector;
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    data: [Vector; 2],
}

impl Matrix {
    pub fn identity() -> Self {
        Self {
            data: [Vector::new(1, 0, 0), Vector::new(0, -1, 900)],
        }
    }

    pub fn get(&self, i: usize, j: usize) -> Fixed {
        self.data[i].get(j)
    }

    pub fn new<A: Into<Fixed>>(a: A, b: A, c: A, d: A, e: A, f: A) -> Self {
        Self {
            data: [Vector::new(a, c, e), Vector::new(b, d, f)],
        }
    }

    pub fn transform<A: Into<Fixed> + Copy>(&self, x: A, y: A) -> (Fixed, Fixed) {
        let xp = Vector::new(x, y, 1) * &self.data[0];
        let yp = Vector::new(x, y, 1) * &self.data[1];
        (xp, yp)
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Matrix {
        Self {
            data: [
                Vector::new(
                    self.get(0, 0) * rhs.get(0, 0) + self.get(0, 1) * rhs.get(1, 0),
                    self.get(0, 0) * rhs.get(0, 1) + self.get(0, 1) * rhs.get(1, 1),
                    self.get(0, 0) * rhs.get(0, 2)
                        + self.get(0, 1) * rhs.get(1, 2)
                        + self.get(0, 2),
                ),
                Vector::new(
                    self.get(1, 0) * rhs.get(0, 0) + self.get(1, 1) * rhs.get(1, 0),
                    self.get(1, 0) * rhs.get(0, 1) + self.get(1, 1) * rhs.get(1, 1),
                    self.get(1, 0) * rhs.get(0, 2)
                        + self.get(1, 1) * rhs.get(1, 2)
                        + self.get(1, 2),
                ),
            ],
        }
    }
}
