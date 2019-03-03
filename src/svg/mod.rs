pub struct Svg {
    document: svg::Document,
}

impl Svg {
    pub fn new() -> Self {
        Self {
            document: svg::Document::new().set("viewBox", (0, 0, 5000, 7000)),
        }
    }

    pub fn horz_line(&mut self, horz_line: &crate::common::HorzLine) {
        use svg::node::element::path::Data;
        use svg::node::element::Path;

        let data = Data::new()
            .move_to((f64::from(horz_line.x1), f64::from(horz_line.y)))
            .line_to((f64::from(horz_line.x2), f64::from(horz_line.y)))
            .close();
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("d", data);
        svg::node::Node::append(&mut self.document, path);
    }

    pub fn vert_line(&mut self, vert_line: &crate::common::VertLine) {
        use svg::node::element::path::Data;
        use svg::node::element::Path;

        let data = Data::new()
            .move_to((f64::from(vert_line.x), f64::from(vert_line.y1)))
            .line_to((f64::from(vert_line.x), f64::from(vert_line.y2)))
            .close();
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("d", data);
        svg::node::Node::append(&mut self.document, path);
    }

    pub fn save(&self, filename: &str) -> Result<(), failure::Error> {
        svg::save(filename, &self.document).unwrap();
        Ok(())
    }
}
