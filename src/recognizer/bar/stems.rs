use crate::common::{Fixed, Object, VertLine};

#[derive(Default, Debug)]
struct Store {
    stems: Vec<VertLine>,
    current: Option<VertLine>,
}

impl Store {
    fn push(&mut self, stem: &VertLine) {
        self.stems.push(stem.clone());
    }

    fn sort(&mut self) {
        assert!(self.current.is_none());
        self.stems.sort_by(|a, b| b.x.cmp(&a.x));
    }

    fn get_head_size(&self, obj: &Object) -> Option<Fixed> {
        self.stems.last().map(|stem| stem.x - obj.point.x)
    }

    fn attachable(stem: &VertLine, obj: &Object, flexibility: Fixed, width: Option<Fixed>) -> bool {
        (obj.point.x <= stem.x && width.map_or(true, |w| obj.point.x + w >= stem.x))
            && (obj.point.y > stem.y1 - flexibility && obj.point.y < stem.y2 + flexibility)
    }

    fn attach(&mut self, obj: &Object, flexibility: Fixed, width: Option<Fixed>) -> bool {
        let mut res = false;
        // Try current
        if let Some(current) = &self.current {
            println!("T existing {:?} + {:?}", current, obj);
            if Self::attachable(current, obj, flexibility, width) {
                println!("Attached existing {:?} + {:?}", current, obj);
                res = true;
            }
        }
        // Try new one
        if let Some(new) = self.stems.last() {
            println!("T new {:?} + {:?}", new, obj);
            if Self::attachable(new, obj, flexibility, width) {
                println!("Attached new {:?} + {:?}", new, obj);
                self.current = self.stems.pop();
                res = true;
            }
        }
        return res;
    }
}

#[derive(Default, Debug)]
pub struct Stems {
    high: Store,
    low: Store,
    mid: Store,
}

impl Stems {
    pub fn sort(&mut self) {
        self.high.sort();
        self.low.sort();
        self.mid.sort();
    }

    pub fn push_high(&mut self, stem: &VertLine) {
        self.high.push(stem);
    }

    pub fn push_low(&mut self, stem: &VertLine) {
        self.low.push(stem);
    }

    pub fn push_mid(&mut self, stem: &VertLine) {
        self.mid.push(stem);
    }

    pub fn get_head_size(&self, obj: &Object) -> Option<Fixed> {
        [
            self.high.get_head_size(obj),
            self.low.get_head_size(obj),
            self.mid.get_head_size(obj),
        ]
        .iter()
        .flatten()
        .min()
        .map(|res| *res * 1.1)
    }

    pub fn attach(&mut self, obj: &Object, flexibility: Fixed, width: Option<Fixed>) -> bool {
        let high = self.high.attach(obj, flexibility, width);
        let low = self.low.attach(obj, flexibility, width);
        let mid = self.mid.attach(obj, flexibility, width);
        high || low || mid
    }
}
