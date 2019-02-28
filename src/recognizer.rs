use crate::common::Fixed;
use crate::Parser;
use std::collections::{BTreeMap, BTreeSet};

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

#[derive(Debug)]
pub struct Stanza {
    x: Fixed,
    y: Fixed,
    width: Fixed,
    height: Fixed,
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
            .chunks(5)
            .map(|chunk| -> Result<_, failure::Error> {
                let &&first = chunk.first().ok_or(Error::EmptyChunk)?;
                let &&last = chunk.last().ok_or(Error::EmptyChunk)?;
                Ok(Stanza {
                    x,
                    y: first,
                    width: width,
                    height: last - first,
                })
            })
            .collect::<Result<Vec<_>, _>>()?)
    }

    pub fn process(&mut self) -> Result<(), failure::Error> {
        let stanzas = self.detect_stanzas()?;
        println!("{:?}", stanzas);
        Ok(())
    }
}
