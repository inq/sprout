use crate::common::{Fixed, VertLine};
use crate::recognizer::Bar;

#[derive(Debug)]
pub struct Stanza {
    x: Fixed,
    pub y: Fixed,
    width: Fixed,
    height: Fixed,
    scale: Fixed,
    pub bars: Vec<Bar>,
}

impl Stanza {
    pub fn new(x: Fixed, y: Fixed, width: Fixed, height: Fixed, scale: Fixed) -> Self {
        Self {
            x,
            y,
            width,
            height,
            scale,
            bars: vec![],
        }
    }

    pub fn insert_bar(&mut self, vert_line: &VertLine) -> bool {
        if vert_line.y1 == self.y {
            assert!(self.y + self.height == vert_line.y2);
            self.bars.push(Bar::new(vert_line.x));
            true
        } else {
            false
        }
    }

    pub fn sort_bars(&mut self) {
        self.bars.sort();
        self.bars.pop();
    }
}
