pub struct Page {}

use std::collections::HashSet;

use lopdf::content::Content;
use lopdf::{Object, Stream};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid dict")]
    Dict,
    #[fail(display = "invalid object")]
    Object,
    #[fail(display = "invalid operand")]
    Operand,
    #[fail(display = "no content")]
    NoContent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Fixed(i64);

impl From<i64> for Fixed {
    fn from(item: i64) -> Self {
        Self(item * 1000)
    }
}

impl From<f64> for Fixed {
    fn from(item: f64) -> Self {
        Self((item * 1000f64) as i64)
    }
}

impl Fixed {
    fn new<N: Into<Self>>(value: N) -> Fixed {
        value.into()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct HorzLine {
    x1: Fixed, 
    x2: Fixed,
    y: Fixed,
}

impl HorzLine {
    fn new<X1: Into<Fixed>, X2: Into<Fixed>, Y: Into<Fixed>>(x1: X1, x2: X2, y: Y) -> Self {
        Self { x1: x1.into(), x2: x2.into(), y: y.into() }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct VertLine {
    x: Fixed, 
    y1: Fixed,
    y2: Fixed,
}

impl VertLine {
    fn new<X: Into<Fixed>, Y1: Into<Fixed>, Y2: Into<Fixed>>(x: X, y1: Y1, y2: Y2) -> Self {
        Self { x: x.into(), y1: y1.into(), y2: y2.into() }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: Fixed,
    y: Fixed,
}

impl Point {
    fn new<X: Into<Fixed>, Y: Into<Fixed>>(x: X, y: Y) -> Point {
        Self { x: x.into(), y: y.into() }
    }
}

pub struct Parsed {
    horz_lines: HashSet<HorzLine>,
    vert_lines: HashSet<VertLine>,
}

impl Page {
    pub fn new() -> Self {
        Self {}
    }

    fn read_num_slice(data: &[Object]) -> Result<Vec<f64>, failure::Error> {
        let res: Option<Vec<_>> = data
            .iter()
            .map(|obj| obj.as_i64().map(|i| i as f64).or(obj.as_f64()))
            .collect();
        Ok(res.ok_or(Error::Object)?)
    }

    pub fn read_stream(&mut self, stream: &mut Stream) -> Result<Parsed, failure::Error> {
        let dict = &stream.dict;
        let _filter = String::from_utf8(
            dict.get(b"Filter")
                .and_then(Object::as_name)
                .ok_or(Error::Dict)?
                .to_vec(),
        )?;
        let _length = dict
            .get(b"Length")
            .and_then(Object::as_i64)
            .ok_or(Error::Dict)?;
        dbg!(dict);

        let data = stream.decompressed_content().ok_or(Error::NoContent)?;

        let mut res = Content::decode(&data)?;
        let mut point_last = Point { x: Fixed(0), y: Fixed(0) };
        let mut horz_lines = HashSet::new();
        let mut vert_lines = HashSet::new();

        for op in &mut res.operations {
            match op.operator.as_ref() {
                "m" => match *Page::read_num_slice(&op.operands)?.as_slice() {
                    [x, y] => {
                        point_last = Point::new(x, y);
                    }
                    _ => return Err(failure::Error::from(Error::Operand)),
                },
                "l" => match *Page::read_num_slice(&op.operands)?.as_slice() {
                    [x, y] => {
                        if point_last.y == y.into() {
                            horz_lines.insert(HorzLine::new(point_last.x, x, y));
                        }
                        if point_last.x == x.into() {
                            vert_lines.insert(VertLine::new(x, point_last.y, y));
                        }
                        point_last = Point::new(x, y);
                    }
                    _ => return Err(failure::Error::from(Error::Operand)),
                },
                "c" => match *Page::read_num_slice(&op.operands)?.as_slice() {
                    [x1, y1, x2, y2, x3, y3] => {
                        // Curve
                    }
                    _ => return Err(failure::Error::from(Error::Operand)),
                },
                "h" => {
                    // Finish drawing
                }
                "q" => {
                    // Save the current graphics state on the graphics state stack
                }
                "gs" => {
                    // Set the specified parameters in the graphics state
                }
                "cs" => {
                    // Set the current color space to use for non-stroking operations
                    // CSp
                }
                "CS" => {
                    // Set the current color space to use for stroking operations
                    // CSp
                }
                "cm" => {
                    // Modify the current transformation matrix (CTM) by concatenating the specified matrix
                    // [0.06, 0, 0, -0.06, 0, 842]
                }
                "Q" => {
                    // Restore the graphics state by removing the most recently saved state from the stack and making it the current state
                }
                "SCN" => {
                    // Color::??
                    // [0, 0, 0]
                }
                "scn" => {
                    // Color::??
                    // [0, 0, 0]
                }
                "w" => {
                    // Set the line width in the graphics state
                    // [5]
                }
                "J" => {
                    // Set the line cap style in the graphics state
                    // [0]
                }
                "j" => {
                    // Set the line join style in the graphics state
                    // [2]
                }
                "d" => {
                    // Set the line dash pattern in the graphics state
                    // [[], 0]
                }
                "S" => {
                    // PathPainting::??
                    // []
                }
                "B*" => {
                    // PathPainting::??
                    // []
                }
                "n" => {
                    // PathPainting::??
                    // []
                }
                "f*" => {
                    // PathPainting::??
                    // []
                }
                "Do" => {
                    // Paint the specified XObject.
                }

                "W*" => {
                    // ClipingPath::??
                    // []
                }

                "BT" => {
                    // TextObject::??
                    // []
                }
                "Tf" => {
                    // TextObject: Font, Size
                    // [/F7, 128]
                }
                "Tm" => {
                    // TextPos::??
                    // [1, 0, 0, -1, 0, 0]
                }
                "Td" => {
                    // TextPos::??
                    // [0, 0]
                }
                "Tj" => {
                    // TextShowing::??
                    // [()]
                    match &mut op.operands[0] {
                        Object::String(_vec, _format) => {
                            // 1: C
                            // 2: &
                            // 3: rectangle
                            // 4: lower
                            // 5: small rect
                            // 6: >
                            // 7: Black
                            // 8: 8 Wing
                            // 9: 8 rest
                            // 10: White
                            // 11: #
                            // 12: half #
                            // 13: 8 rev. wing
                            // 14: b
                            // 15: 4 rest
                            // 16: White
                            // 17: 16 Wing
                            // 18: Rev. 16 Wing
                            // 19: 16 rest
                            // 20: rect
                            // 21: triangle
                        }
                        etc => {
                            println!("{:?}", dbg!(&etc));
                        }
                    }
                }
                "ET" => {
                    // TextObject::??
                    // []
                }

                _ => {
                    panic!("{:?}", op);
                }
            }
        }

        stream.set_plain_content(res.encode().unwrap());
        stream.compress();
        Ok(Parsed {
            horz_lines,
            vert_lines,
        })
    }
}
