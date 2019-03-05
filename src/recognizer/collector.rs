use crate::common::Fixed;

#[derive(Debug)]
pub enum Note {
    Note(Vec<i8>, i32),
    Rest(i32),
}

#[derive(Debug)]
pub struct Collector {
    quarters: Vec<f64>,
    x: Option<Fixed>,
    pub notes: Vec<Note>,
}

impl Collector {
    pub fn new() -> Self {
        Collector {
            quarters: vec![],
            x: None,
            notes: vec![],
        }
    }

    pub fn prepare(&mut self) {
        self.x = None;
    }

    pub fn put_quarter(&mut self, x: Fixed, note: f64) {
        self.quarters.push(note);
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

    pub fn put_rest(&mut self, x: Fixed, len: u8) {
        if !self.quarters.is_empty() {
            let note = Note::Note(
                self.quarters
                    .iter()
                    .map(|h| 0x3c + (*h * 4.) as i8)
                    .collect(),
                480,
            );
            self.quarters.clear();
        }
        let note = Note::Rest(1920 / len as i32);
        self.notes.push(note);
        self.x = Some(x);
    }
}
