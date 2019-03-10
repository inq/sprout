use crate::common::Fixed;

#[derive(Debug)]
pub enum Note {
    Note(Vec<i8>, i32),
    Rest(i32),
}

#[derive(Debug)]
pub struct Collector {
    quarters: Vec<f64>,
    wholes: Vec<f64>,
    x: Option<Fixed>,
    pub notes: Vec<Note>,
}

impl Collector {
    pub fn new() -> Self {
        Collector {
            quarters: vec![],
            wholes: vec![],
            x: None,
            notes: vec![],
        }
    }

    pub fn prepare(&mut self) {
        self.clear();
        self.x = None;
    }

    pub fn put_quarter(&mut self, x: Fixed, note: f64) {
        self.quarters.push(note);
        self.x = Some(x);
    }

    pub fn put_whole(&mut self, x: Fixed, note: f64) {
        self.wholes.push(note);
        self.x = Some(x);
    }

    pub fn put_wing(&mut self, x: Fixed) {
        let note = Note::Note(
            self.quarters
                .iter()
                .map(|h| 0x3c + (*h * 4.) as i8)
                .collect(),
            240,
        );
        self.notes.push(note);
        self.quarters.clear();

        self.x = Some(x);
    }

    fn h2n(h: f64) -> i8 {
        let i = (h * 2.) as i32;
        let o = ((i / 7) * 12) as i8;
        o + match (i % 7 + 7) % 7 {
            0 => 0, // C
            1 => 2,
            2 => 4,
            3 => 5,
            4 => 7,
            5 => 9,
            6 => 11,
            x => panic!("HEHE {}", x),
        }
    }

    fn clear(&mut self) {
        if !self.quarters.is_empty() {
            let note = Note::Note(
                self.quarters.iter().map(|h| 0x3c + Self::h2n(*h)).collect(),
                480,
            );
            self.notes.push(note);
            self.quarters.clear();
        }
        if !self.wholes.is_empty() {
            let note = Note::Note(
                self.wholes.iter().map(|h| 0x3c + Self::h2n(*h)).collect(),
                480 * 4,
            );
            self.notes.push(note);
            self.wholes.clear();
        }
    }

    pub fn put_rest(&mut self, x: Fixed, len: u8) {
        self.clear();
        let note = Note::Rest(1920 / len as i32);
        self.notes.push(note);
        self.x = Some(x);
    }
}
