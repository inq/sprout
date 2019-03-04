mod bar;
mod stanza;

use std::collections::{BTreeMap, BTreeSet};

use crate::common::{Fixed, Object, Type};
use crate::Parser;
pub use bar::Bar;
pub use stanza::Stanza;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "no stanza")]
    NoStanza,
    #[fail(display = "empty chunk")]
    EmptyChunk,
}

#[derive(Debug)]
pub struct Recognizer {
    parser: Parser,
}

impl Recognizer {
    pub fn new(parser: Parser) -> Self {
        Self { parser }
    }

    fn detect_width(&self) -> Result<Fixed, failure::Error> {
        let mut counts = BTreeMap::new();
        for line in self.parser.horz_lines.iter() {
            let len = line.len();
            counts.insert(len, *counts.get(&len).unwrap_or(&0) + 1);
        }
        Ok(*counts
            .iter()
            .rfind(|&(len, count)| count % 5 == 0)
            .ok_or(Error::NoStanza)?
            .0)
    }

    fn detect_stanzas(&mut self) -> Result<Vec<Stanza>, failure::Error> {
        let width = self.detect_width()?;
        let mut x = None;
        let mut ys = BTreeSet::new();
        self.parser.horz_lines.retain(|line| {
            if line.len() == width {
                x = Some(line.x1);
                ys.insert(line.y);
                false
            } else {
                true
            }
        });
        let x = x.ok_or(Error::NoStanza)?;
        Ok(ys
            .iter()
            .collect::<Vec<_>>()
            .chunks(10)
            .map(|chunk| -> Result<_, failure::Error> {
                let &&first = chunk.first().ok_or(Error::EmptyChunk)?;
                let &&last = chunk.last().ok_or(Error::EmptyChunk)?;
                let high = chunk[0..5]
                    .windows(2)
                    .map(|w| *w[1] - *w[0])
                    .collect::<BTreeSet<_>>();
                let low = chunk[0..5]
                    .windows(2)
                    .map(|w| *w[1] - *w[0])
                    .collect::<BTreeSet<_>>();
                assert!(high.len() == 1 && low.len() == 1);
                Ok(Stanza::new(
                    x,
                    first,
                    width,
                    last - first,
                    *high.iter().next().unwrap(),
                ))
            })
            .collect::<Result<Vec<_>, _>>()?)
    }

    fn debug_vert_lines(&self) {
        let mut test = crate::svg::Svg::new();
        let mut count = 0;
        for line in self.parser.vert_lines.iter() {
            test.vert_line(line);
            count += 1;
        }
        for line in self.parser.horz_lines.iter() {
            test.horz_line(line);
            count += 1;
        }
        for obj in self.parser.objects.iter() {
            test.circle(&obj.point)
        }
        test.save("output.svg");
        assert!(count == 0);
    }

    pub fn put_obj_into_stanza(stanza: &mut Stanza, obj: &Object) -> bool {
        if stanza.y < obj.point.y {
            for bar in stanza.bars.iter_mut().rev() {
                if bar.x < obj.point.x {
                    bar.store.push(obj.clone());
                    break;
                }
            }
            true
        } else {
            false
        }
    }

    pub fn process(&mut self) -> Result<(), failure::Error> {
        let mut smf = crate::smf::Smf::new(152);
        smf.write();
        let mut stanzas = self.detect_stanzas()?;
        for stanza in stanzas.iter_mut() {
            self.parser
                .vert_lines
                .retain(|line| !stanza.insert_bar(line));
            stanza.sort_bars();
        }
        for stanza in stanzas.iter_mut().rev() {
            self.parser
                .objects
                .retain(|obj| !Self::put_obj_into_stanza(stanza, obj));
        }

        // TEST
        let res = vec![];

        for stanza in stanzas.iter_mut() {
            let border = stanza.y + stanza.height - stanza.scale * 6;
            for bar in stanza.bars.iter_mut() {
                bar.store.sort_by(|a, b| a.point.x.cmp(&b.point.x));
                for obj in bar.store.iter() {
                    if obj.point.y >= border {
                        match obj.t {
                            Type::Head(4) => {
                                println!("{:?}", (border - obj.point.y) / stanza.scale);
                            }
                            _ => (),
                        }
                        println!("{:?}", obj);
                    }
                }

                panic!("HELLO");
            }
        }

        self.debug_vert_lines();

        Ok(())
    }
}
