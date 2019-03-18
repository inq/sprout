use crate::common::Fixed;
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    data: [Fixed; 3],
}

impl Vector {
    pub fn new<A: Into<Fixed>, B: Into<Fixed>, C: Into<Fixed>>(a: A, b: B, c: C) -> Self {
        Self {
            data: [a.into(), b.into(), c.into()],
        }
    }

    pub fn get(&self, i: usize) -> Fixed {
        self.data[i]
    }
}

impl Mul<&Vector> for Vector {
    type Output = Fixed;

    fn mul(self, rhs: &Self) -> Fixed {
        (0..3).map(|i| self.data[i] * rhs.data[i]).sum()
    }
}
