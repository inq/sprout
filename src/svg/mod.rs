pub struct Svg {
    document: svg::Document,
    minx: f64,
    maxx: f64,
    miny: f64,
    maxy: f64,
}

impl Svg {
    pub fn new() -> Self {
        Self {
            document: svg::Document::new(),
            minx: 1e10,
            miny: 1e10,
            maxx: -1e10,
            maxy: -1e10,
        }
    }

    pub fn put_point(&mut self, x: f64, y: f64) {
        if self.minx > x {
            self.minx = x;
        }
        if self.miny > y {
            self.miny = y;
        }
        if self.maxx < x {
            self.maxx = x;
        }
        if self.maxy < y {
            self.maxy = y;
        }
    }

    pub fn horz_line(&mut self, horz_line: &crate::common::HorzLine) {
        use svg::node::element::path::Data;
        use svg::node::element::Path;

        self.put_point(f64::from(horz_line.x1), f64::from(horz_line.y));
        self.put_point(f64::from(horz_line.x2), f64::from(horz_line.y));

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

    pub fn quadra(&mut self, quadra: &crate::common::Quadrangle) {
        use svg::node::element::path::Data;
        use svg::node::element::Path;

        self.put_point(f64::from(quadra.points[0].x), f64::from(quadra.points[0].y));
        self.put_point(f64::from(quadra.points[2].x), f64::from(quadra.points[2].y));

        let data = Data::new()
            .move_to((f64::from(quadra.points[0].x), f64::from(quadra.points[0].y)))
            .line_to((f64::from(quadra.points[1].x), f64::from(quadra.points[1].y)))
            .line_to((f64::from(quadra.points[2].x), f64::from(quadra.points[2].y)))
            .line_to((f64::from(quadra.points[3].x), f64::from(quadra.points[3].y)))
            .close();
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "green")
            .set("stroke-width", 1)
            .set("d", data);
        svg::node::Node::append(&mut self.document, path);
    }

    pub fn line(&mut self, line: &crate::common::Line) {
        use svg::node::element::path::Data;
        use svg::node::element::Path;

        self.put_point(f64::from(line.x1), f64::from(line.y1));
        self.put_point(f64::from(line.x2), f64::from(line.y2));

        let data = Data::new()
            .move_to((f64::from(line.x1), f64::from(line.y1)))
            .line_to((f64::from(line.x2), f64::from(line.y2)))
            .close();
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "blue")
            .set("stroke-width", 1)
            .set("d", data);
        svg::node::Node::append(&mut self.document, path);
    }

    pub fn vert_line(&mut self, vert_line: &crate::common::VertLine) {
        use svg::node::element::path::Data;
        use svg::node::element::Path;

        self.put_point(f64::from(vert_line.x), f64::from(vert_line.y1));
        self.put_point(f64::from(vert_line.x), f64::from(vert_line.y2));

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

    pub fn circle(&mut self, point: &crate::common::Point) {
        use svg::node::element::Circle;

        self.put_point(f64::from(point.x), f64::from(point.y));

        let circle = Circle::new()
            .set("cx", f64::from(point.x))
            .set("cy", f64::from(point.y))
            .set("r", 1f64)
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("fill", "red");
        svg::node::Node::append(&mut self.document, circle);
    }

    pub fn save(&mut self, filename: &str) -> Result<(), failure::Error> {
        let doc = self.document.clone().set(
            "viewBox",
            (
                self.minx,
                self.miny,
                self.maxx - self.minx,
                self.maxy - self.miny,
            ),
        );
        svg::save(filename, &doc).unwrap();
        Ok(())
    }
}
