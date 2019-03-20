mod fixed;

pub use fixed::Fixed;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Line {
    pub x1: Fixed,
    pub y1: Fixed,
    pub x2: Fixed,
    pub y2: Fixed,
}

impl Line {
    pub fn new<X1: Into<Fixed>, X2: Into<Fixed>, Y1: Into<Fixed>, Y2: Into<Fixed>>(
        x1: X1,
        y1: Y1,
        x2: X2,
        y2: Y2,
    ) -> Self {
        Self {
            x1: x1.into(),
            x2: x2.into(),
            y1: y1.into(),
            y2: y2.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Quadrangle {
    pub points: [Point; 4],
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Polygon {
    pub lines: Vec<Line>,
    pub point: Point,
}

pub enum PolygonRes {
    HorzLine(HorzLine),
    VertLine(VertLine),
    Line(Line),
    Quadrangle(Quadrangle),
    Empty,
}

impl Polygon {
    pub fn new<X: Into<Fixed>, Y: Into<Fixed>>(x: X, y: Y) -> Self {
        Self {
            point: Point::new(x, y),
            lines: vec![],
        }
    }

    pub fn push<X: Copy + Into<Fixed>, Y: Copy + Into<Fixed>>(&mut self, x: X, y: Y) {
        let point = std::mem::replace(&mut self.point, Point::new(x, y));
        self.lines.push(Line::new(point.x, point.y, x, y));
    }

    pub fn build(&self) -> PolygonRes {
        match self.lines.len() {
            1 => {
                let line = &self.lines[0];
                if line.y1 == line.y2 {
                    PolygonRes::HorzLine(HorzLine::new(line.x1, line.x2, line.y1))
                } else if line.x1 == line.x2 {
                    PolygonRes::VertLine(VertLine::new(line.x1, line.y1, line.y2))
                } else {
                    PolygonRes::Line(line.clone())
                }
            }
            4 => PolygonRes::Quadrangle(Quadrangle {
                points: [
                    Point::new(self.lines[0].x1, self.lines[0].y1),
                    Point::new(self.lines[1].x1, self.lines[1].y1),
                    Point::new(self.lines[2].x1, self.lines[2].y1),
                    Point::new(self.lines[3].x1, self.lines[3].y1),
                ],
            }),
            0 => PolygonRes::Empty,
            _ => {
                panic!("{:?}", self.lines);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct HorzLine {
    pub x1: Fixed,
    pub x2: Fixed,
    pub y: Fixed,
}

impl HorzLine {
    pub fn new<X1: Into<Fixed>, X2: Into<Fixed>, Y: Into<Fixed>>(x1: X1, x2: X2, y: Y) -> Self {
        let x1 = x1.into();
        let x2 = x2.into();
        Self {
            x1: std::cmp::min(x1, x2),
            x2: std::cmp::max(x1, x2),
            y: y.into(),
        }
    }

    pub fn len(&self) -> Fixed {
        self.x2 - self.x1
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct VertLine {
    pub x: Fixed,
    pub y1: Fixed,
    pub y2: Fixed,
}

impl VertLine {
    pub fn new<X: Into<Fixed>, Y1: Into<Fixed>, Y2: Into<Fixed>>(x: X, y1: Y1, y2: Y2) -> Self {
        let y1 = y1.into();
        let y2 = y2.into();
        Self {
            x: x.into(),
            y1: std::cmp::min(y1, y2),
            y2: std::cmp::max(y1, y2),
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
