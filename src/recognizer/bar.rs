use crate::common::{Fixed, Object, VertLine};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Bar {
    pub x: Fixed,
    pub high: Vec<u8>,
    pub low: Vec<u8>,
    pub store: Vec<Object>,
    pub stems: Vec<(bool, VertLine)>,
}

impl Bar {
    pub fn new(x: Fixed) -> Self {
        Self {
            x,
            high: vec![],
            low: vec![],
            store: vec![],
            stems: vec![],
        }
    }

    pub fn debug(&self) {
        println!(
            "Bar x: {:?}, stems: {}, objects: {}",
            self.x,
            self.stems.len(),
            self.store.len()
        );
    }
}

impl PartialOrd for Bar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bar {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x)
    }
}

impl PartialEq for Bar {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

impl Eq for Bar {}
