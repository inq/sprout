use ghakuf::messages::{Message, MetaEvent, MidiEvent};
use ghakuf::writer::*;
use std::path;

use crate::recognizer::Collector;

pub struct Smf {
    messages: Vec<Message>,
}

impl Smf {
    pub fn new(bpm: u32) -> Self {
        let tempo: u32 = 60 * 1000000 / bpm;
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

    pub fn write(&mut self, collector: &Collector) {
        use crate::recognizer::Note;

        let mut delta = 0;
        for note in collector.notes.iter() {
            match note {
                Note::Note(data, len) => {
                    for tone in data.iter() {
                        self.messages.push(Message::MidiEvent {
                            delta_time: delta,
                            event: MidiEvent::NoteOn {
                                ch: 0,
                                note: *tone as u8,
                                velocity: 96,
                            },
                        });
                        delta = 0;
                    }
                    delta = *len as u32;
                    for tone in data.iter() {
                        self.messages.push(Message::MidiEvent {
                            delta_time: delta,
                            event: MidiEvent::NoteOff {
                                ch: 0,
                                note: *tone as u8,
                                velocity: 64,
                            },
                        });
                        delta = 0;
                    }
                }
                Note::Rest(len) => {
                    delta += *len as u32;
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
