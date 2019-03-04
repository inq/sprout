use std::collections::HashSet;

use lopdf::content::Content;
use lopdf::{Object, Stream};

use crate::common::{Fixed, HorzLine, Point, VertLine};

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

#[derive(Debug)]
pub struct Parser {
    pub horz_lines: HashSet<HorzLine>,
    pub vert_lines: HashSet<VertLine>,
    pub objects: Vec<crate::common::Object>,
}

impl Parser {
    fn read_num_slice(data: &[Object]) -> Result<Vec<f64>, failure::Error> {
        Ok(data
            .iter()
            .map(|obj| obj.as_i64().map(|i| i as f64).or(obj.as_f64()))
            .collect::<Option<Vec<_>>>()
            .ok_or(Error::Object)?)
    }

    pub fn new(stream: &mut Stream) -> Result<Self, failure::Error> {
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

        let data = stream.decompressed_content().ok_or(Error::NoContent)?;

        let mut res = Content::decode(&data)?;
        let mut point_last = Point::new(0, 0);
        let mut horz_lines = HashSet::new();
        let mut vert_lines = HashSet::new();
        let mut tm = [Fixed::new(0); 6];
        let mut td = [Fixed::new(0); 2];
        let mut heads = vec![];

        for op in &mut res.operations {
            match op.operator.as_ref() {
                "m" => match *Self::read_num_slice(&op.operands)?.as_slice() {
                    [x, y] => {
                        point_last = Point::new(x, y);
                    }
                    _ => return Err(failure::Error::from(Error::Operand)),
                },
                "l" => match *Self::read_num_slice(&op.operands)?.as_slice() {
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
                "c" => match *Self::read_num_slice(&op.operands)?.as_slice() {
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
                    // Text Matrix
                    // [1, 0, 0, -1, 0, 0]
                    for (i, v) in op.operands.iter().enumerate() {
                        tm[i] = Fixed::from(v.as_i64().unwrap());
                    }
                }
                "Td" => {
                    // Text Position
                    // [0, 0]
                    for (i, v) in op.operands.iter().enumerate() {
                        td[i] = Fixed::from(v.as_i64().map(|i| i as f64).or(v.as_f64()).unwrap());
                    }
                }
                "Tj" => {
                    // TextShowing::??
                    // [()]
                    match &mut op.operands[0] {
                        Object::String(vec, _format) => {
                            use crate::common::Type;
                            // 1: C
                            // 2: &
                            // 3: rectangle
                            // 4: lower
                            // 5: small rect
                            // 6: >
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
                            let x = td[0] * tm[0] + td[1] * tm[2] + Fixed::new(1) * tm[4];
                            let y = td[0] * tm[1] + td[1] * tm[3] + Fixed::new(1) * tm[5];

                            let t = match vec.as_slice() {
                                [0, 7] => Some(Type::Head(4)),
                                [0, 8] => Some(Type::Wing(8)),
                                [0, 9] => Some(Type::Rest(8)),
                                [0, 10] => Some(Type::Head(1)),
                                [0, 16] => Some(Type::Head(2)),
                                [0, 3] => Some(Type::Rest(1)),
                                _ => None,
                            };
                            if let Some(t) = t {
                                heads.push(crate::common::Object::new(t, Point::new(x, y)))
                            }
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
        Ok(Self {
            horz_lines,
            vert_lines,
            objects: heads,
        })
    }
}
