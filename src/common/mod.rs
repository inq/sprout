mod fixed;

pub use fixed::Fixed;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct HorzLine {
    pub x1: Fixed,
    pub x2: Fixed,
    pub y: Fixed,
}

impl HorzLine {
    pub fn new<X1: Into<Fixed>, X2: Into<Fixed>, Y: Into<Fixed>>(x1: X1, x2: X2, y: Y) -> Self {
        Self {
            x1: x1.into(),
            x2: x2.into(),
            y: y.into(),
        }
    }

    pub fn len(&self) -> Fixed {
        self.x2 - self.x1
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct VertLine {
    pub x: Fixed,
    pub y1: Fixed,
    pub y2: Fixed,
}

impl VertLine {
    pub fn new<X: Into<Fixed>, Y1: Into<Fixed>, Y2: Into<Fixed>>(x: X, y1: Y1, y2: Y2) -> Self {
        Self {
            x: x.into(),
            y1: y1.into(),
            y2: y2.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: Fixed,
    pub y: Fixed,
}

impl Point {
    pub fn new<X: Into<Fixed>, Y: Into<Fixed>>(x: X, y: Y) -> Point {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Head(u8),
    Wing(u8),
    Rest(u8),
}

#[derive(Debug, Clone)]
pub struct Object {
    pub t: Type,
    pub point: Point,
}

impl Object {
    pub fn new(t: Type, point: Point) -> Self {
        Self { t, point }
    }
}