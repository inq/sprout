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

    pub fn put_stem(&mut self, vert_line: &VertLine) -> bool {
        if self.y < vert_line.y2 {
            for bar in self.bars.iter_mut().rev() {
                if bar.x < vert_line.x {
                    bar.stems.push((false, vert_line.clone()));
                    break;
                }
            }
            true
        } else {
            false
        }
    }

    fn is_attached(obj: &Object, stem: &VertLine, flexibility: Fixed, width: Fixed) -> bool {
        (obj.point.x <= stem.x && obj.point.x + width >= stem.x)
            && (obj.point.y > stem.y1 - flexibility && obj.point.y < stem.y2 + flexibility)
    }

    pub fn process(&mut self) {
        use crate::common::Type;

        let borders = [
            self.y + self.height - self.scale * 6,
            self.y + self.scale * 5,
        ];
        let mut collectors = [Collector::new(), Collector::new()];

        let mut i = 0;

        for bar in self.bars.iter_mut() {
            bar.store.sort_by(|a, b| a.point.x.cmp(&b.point.x));
            bar.stems.sort_by(|a, b| b.1.x.cmp(&a.1.x)); // Reversed order
            collectors[0].prepare();
            collectors[1].prepare();
            for obj in bar.store.iter() {
                let channel = if obj.point.y >= self.y + self.height - self.scale * 5 {
                    0
                } else {
                    1
                };

                match obj.t {
                    Type::Head(4) => {
                        if self.head_size.is_none() {
                            let head_size = (bar.stems.last().unwrap().1.x - obj.point.x) * 1.1;
                            self.head_size = Some(head_size);
                        }
                        if let Some(head_size) = self.head_size {
                            let (used, stem) = &bar.stems.last().unwrap();
                            let attached = Self::is_attached(obj, stem, self.scale / 2, head_size);
                            if !attached {
                                assert!(*used);
                                bar.stems.pop();
                                assert!(bar.stems.len() > 0, "Not attached {:?}", obj);
                                let (used, stem) = &bar.stems.last().unwrap();
                                let attached =
                                    Self::is_attached(obj, stem, self.scale / 2, head_size);
                                assert!(
                                    attached,
                                    "Not attached {:?} + {:?}",
                                    obj,
                                    bar.stems.last()
                                );
                            }
                            bar.stems.last_mut().unwrap().0 = true; // Used!

                            println!("Attached {:?} + {:?}", obj, bar.stems.last());
                        } else {
                            panic!("No way");
                        }

                        collectors[channel].put_quarter(
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
