use crate::common::Fixed;

#[derive(Debug)]
pub struct Stanza {
    x: Fixed,
    y: Fixed,
    width: Fixed,
    height: Fixed,
}

impl Stanza {
    pub fn new(x: Fixed, y: Fixed, width: Fixed, height: Fixed) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}
