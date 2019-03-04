use ghakuf::messages::{Message, MidiEvent, MetaEvent};
use ghakuf::writer::*;
use std::path;

pub struct Smf {
    messages: Vec<Message>
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
            ]
        }
    }

    pub fn write(&mut self) {
        self.messages.push(
            Message::MidiEvent {
                delta_time: 0,
                event: MidiEvent::NoteOn {
                    ch: 0,
                    note: 0x3c,
                    velocity: 96
                },
            },
        );
        self.messages.push(
            Message::MidiEvent {
                delta_time: 0,
                event: MidiEvent::NoteOn {
                    ch: 0,
                    note: 0x40,
                    velocity: 96
                },
            },
        );
        self.messages.push(
            Message::MidiEvent {
                delta_time: 960,
                event: MidiEvent::NoteOff {
                    ch: 0,
                    note: 0x3c,
                    velocity: 64,
                },
            }
        );
        self.messages.push(
            Message::MidiEvent {
                delta_time: 0,
                event: MidiEvent::NoteOff {
                    ch: 0,
                    note: 0x40,
                    velocity: 96
                },
            },
        );
        self.messages.push(
            Message::MetaEvent {
                delta_time: 1024,
                event: MetaEvent::EndOfTrack,
                data: Vec::new(),
            }
        );
        let path = path::Path::new("example.mid");
        let mut writer = Writer::new();
        writer.running_status(true);
        for message in &self.messages {
            writer.push(&message);
        }
        let _ = writer.write(&path);
        println!("HEHEHE");
    }
}