use ghakuf::messages::{Message, MetaEvent, MidiEvent};
use ghakuf::writer::*;
use std::path;

use crate::recognizer::Collector;

pub struct Smf {
    messages: Vec<Message>,
}

pub enum Action {
    NoteOn {
        time: u32,
        ch: u8,
        note: u8,
        velocity: u8,
    },
    NoteOff {
        time: u32,
        ch: u8,
        note: u8,
        velocity: u8,
    },
}

impl Action {
    fn time(&self) -> u32 {
        match self {
            Action::NoteOn { time, .. } => *time,
            Action::NoteOff { time, .. } => *time,
        }
    }
}

impl Smf {
    pub fn new(bpm: u32) -> Self {
        let tempo: u32 = 60 * 1_000_000 / bpm;
        Self {
            messages: vec![
                Message::MetaEvent {
                    delta_time: 0,
                    event: MetaEvent::SetTempo,
                    data: [(tempo >> 16) as u8, (tempo >> 8) as u8, tempo as u8].to_vec(),
                },
                Message::MetaEvent {
                    delta_time: 0,
                    event: MetaEvent::EndOfTrack,
                    data: Vec::new(),
                },
                Message::TrackChange,
            ],
        }
    }

    pub fn write(&mut self, collectors: &[Collector]) {
        use crate::recognizer::Note;

        let mut actions = vec![];
        for (i, collector) in collectors.iter().enumerate() {
            let mut time = 0;
            for note in collector.notes.iter() {
                match note {
                    Note::Note(data, len) => {
                        for tone in data.iter() {
                            actions.push(Action::NoteOn {
                                time,
                                ch: i as u8,
                                note: *tone as u8,
                                velocity: 96,
                            });
                            actions.push(Action::NoteOff {
                                time: time + *len as u32,
                                ch: i as u8,
                                note: *tone as u8,
                                velocity: 64,
                            });
                        }
                        time += *len as u32;
                    }
                    Note::Rest(len) => {
                        time += *len as u32;
                    }
                }
            }
        }
        actions.sort_by(|a, b| Action::time(a).cmp(&Action::time(b)));
        let mut clock = 0;
        for action in actions.iter() {
            match action {
                Action::NoteOn {
                    time,
                    ch,
                    note,
                    velocity,
                } => {
                    let delta = time - clock;
                    clock = *time;
                    self.messages.push(Message::MidiEvent {
                        delta_time: delta,
                        event: MidiEvent::NoteOn {
                            ch: *ch,
                            note: *note,
                            velocity: *velocity,
                        },
                    });
                }
                Action::NoteOff {
                    time,
                    ch,
                    note,
                    velocity,
                } => {
                    let delta = time - clock;
                    clock = *time;
                    self.messages.push(Message::MidiEvent {
                        delta_time: delta,
                        event: MidiEvent::NoteOff {
                            ch: *ch,
                            note: *note,
                            velocity: *velocity,
                        },
                    });
                }
            }
        }

        self.messages.push(Message::MetaEvent {
            delta_time: 1024,
            event: MetaEvent::EndOfTrack,
            data: Vec::new(),
        });
        let path = path::Path::new("example.mid");
        let mut writer = Writer::new();
        writer.running_status(true);
        for message in &self.messages {
            writer.push(&message);
        }
        let _ = writer.write(&path);
    }
}
