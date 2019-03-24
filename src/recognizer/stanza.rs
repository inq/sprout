use crate::common::{Fixed, Object, VertLine};
use crate::recognizer::Bar;

use crate::recognizer::Collector;

#[derive(Debug)]
pub struct Stanza {
    x: Fixed,
    pub y: Fixed,
    width: Fixed,
    pub height: Fixed,
    pub scale: Fixed,
    pub bars: Vec<Bar>,

    head_size: Option<Fixed>, // Config
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
            head_size: None,
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

    pub fn put_object(&mut self, obj: &Object) -> bool {
        if self.y < obj.point.y {
            for bar in self.bars.iter_mut().rev() {
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

    fn is_high(&self, stem: &VertLine) -> bool {
        let mut points = vec![self.y, self.y + self.scale * 5, stem.y1, stem.y2];
        let length = stem.y2 - stem.y1 + self.scale * 5;
        points.sort();
        *points.last().unwrap() - *points.first().unwrap() < length
    }

    fn is_low(&self, stem: &VertLine) -> bool {
        let mut points = vec![
            self.y + self.height,
            self.y + self.height - self.scale * 5,
            stem.y1,
            stem.y2,
        ];
        let length = stem.y2 - stem.y1 + self.scale * 5;
        points.sort();
        *points.last().unwrap() - *points.first().unwrap() < length
    }

    fn is_mid(&self, stem: &VertLine) -> bool {
        let mut points = vec![
            self.y + self.scale * 5,
            self.y + self.height - self.scale * 5,
            stem.y1,
            stem.y2,
        ];
        let length = stem.y2 - stem.y1 + self.scale * 5;
        points.sort();
        *points.last().unwrap() - *points.first().unwrap() < length
    }

    pub fn put_stem(&mut self, stem: &VertLine) -> bool {
        let high = self.is_high(stem);
        let low = self.is_low(stem);
        let mid = self.is_mid(stem);
        assert!(!high || !low, "It cannot be both!");
        if !high && !low && !mid {
            return false;
        }
        for bar in self.bars.iter_mut().rev() {
            if bar.x < stem.x {
                if high {
                    bar.stems.push_high(stem);
                }
                if low {
                    bar.stems.push_low(stem);
                }
                if !high && !low {
                    bar.stems.push_mid(stem);
                }
                return true;
            }
        }
        false
    }

    pub fn process(&mut self) {
        use crate::common::Type;

        let borders = [
            self.y + self.height - self.scale * 6,
            self.y + self.scale * 5,
        ];
        let mut collectors = [Collector::new(), Collector::new()];

        for bar in self.bars.iter_mut() {
            bar.store.sort_by(|a, b| match a.point.x.cmp(&b.point.x) {
                std::cmp::Ordering::Equal => b.point.y.cmp(&a.point.y),
                etc => etc,
            });
            bar.stems.sort();
            collectors[0].prepare();
            collectors[1].prepare();
            for obj in bar.store.iter() {
                let channel = if obj.point.y >= self.y + self.height - self.scale * 5 {
                    0
                } else {
                    1
                };

                match obj.t {
                    Type::Head(size) if size == 2 || size == 4 => {
                        if self.head_size.is_none() {
                            if let Some(head_size) = bar.stems.get_head_size(&obj) {
                                if head_size > self.scale {
                                    self.head_size = Some(head_size);
                                }
                            }
                        }
                        if !bar.stems.attach(obj, self.scale / 2, self.head_size) {
                            panic!("Cannot attach {:?}, head_size: {:?}", obj, self.head_size);
                        }

                        collectors[channel].put_quarter(
                            // TODO: quarter or half
                            obj.point.x,
                            (borders[channel] - obj.point.y) / self.scale,
                        );
                    }
                    Type::Wing(8) => {
                        collectors[channel].put_wing(obj.point.x);
                    }
                    Type::Rest(len) => {
                        collectors[channel].put_rest(obj.point.x, len);
                    }
                    Type::Head(1) => {
                        collectors[channel]
                            .put_whole(obj.point.x, (borders[channel] - obj.point.y) / self.scale);
                    }
                    _ => {
                        // println!("{:?}", obj);
                    }
                }
            }
            bar.debug();
            /*
            i += 1;
            println!("{}", i);
            if i == 4 {
                let mut smf = crate::smf::Smf::new(152);
                smf.write(&collectors);
                panic!("HELLO");
            }
            */
        }
    }
}
