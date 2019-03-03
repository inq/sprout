#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fixed(i64);

impl From<i64> for Fixed {
    fn from(item: i64) -> Self {
        Self(item * 100)
    }
}

impl From<f64> for Fixed {
    fn from(item: f64) -> Self {
        Self((item * 100f64) as i64)
    }
}

impl From<Fixed> for f64 {
    fn from(item: Fixed) -> Self {
        item.0 as f64 * 0.01f64
    }
}

impl std::ops::Sub for Fixed {
    type Output = Fixed;

    fn sub(self, rhs: Self) -> Fixed {
        Fixed(self.0 - rhs.0)
    }
}

impl std::ops::Add for Fixed {
    type Output = Fixed;

    fn add(self, rhs: Self) -> Fixed {
        Fixed(self.0 + rhs.0)
    }
}

impl std::fmt::Debug for Fixed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Fixed({:.02})", self.0 as f64 / 100.0)
    }
}

impl Fixed {
    pub fn new<N: Into<Self>>(value: N) -> Fixed {
        value.into()
    }
}
