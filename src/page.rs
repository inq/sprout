pub struct Page {}

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
}

#[derive(Debug)]
struct HorzLine {
    x1: f64, 
    x2: f64,
    y: f64,
}

#[derive(Debug)]
struct VertLine {
    x: f64, 
    y1: f64,
    y2: f64,
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
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

    pub fn read_stream(&mut self, stream: &mut Stream) -> Result<(), failure::Error> {
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

        if let Some(data) = stream.decompressed_content() {
            let mut res = Content::decode(&data)?;
            let mut point_last = Point { x: 0f64, y: 0f64 };
            let mut horz_lines = vec![];
            let mut vert_lines = vec![];

            for op in &mut res.operations {
                match op.operator.as_ref() {
                    "m" => match *Page::read_num_slice(&op.operands)?.as_slice() {
                        [x, y] => {
                            point_last = Point { x, y };
                        }
                        _ => return Err(failure::Error::from(Error::Operand)),
                    },
                    "l" => match *Page::read_num_slice(&op.operands)?.as_slice() {
                        [x, y] => {
                            if point_last.y == y {
                                horz_lines.push(HorzLine{x1: point_last.x, x2: x, y: y});
                            }
                            if point_last.x == x {
                                vert_lines.push(VertLine{x: x, y1: point_last.y, y2: y});
                            }
                            point_last = Point { x, y };
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
        }
        Ok(())
    }
}
