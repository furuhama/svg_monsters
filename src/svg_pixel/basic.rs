pub struct Dot {
    x: usize,
    y: usize,
}

impl Dot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x: x,
            y: y,
        }
    }

    pub fn to_svg(&self) -> String {
        format!("<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"{}\" y=\"{}\" width=\"100\" height=\"100\"/>\n", self.x * 10, self.y * 10)
    }
}

pub struct Canvas {
    // TODO: define canvas size here
    dots: Vec<Dot>,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            dots: Vec::<Dot>::new(),
        }
    }

    pub fn add(&mut self, dot: Dot) -> bool {
        // TODO: check dot position here (dot should not be outside the canvas)
        // TODO: check dot position not duplicate (dot should not be the same position)
        self.dots.push(dot);

        true
    }

    pub fn to_svg(&self) -> String {
        let mut svg_tag = String::from("<svg width=\"80\" height=\"80\" viewPort=\"0 0 80 80\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\">\n");
        for dot in &self.dots {
            svg_tag.push_str(dot.to_svg().as_str());
        }
        svg_tag.push_str("</svg>\n");
        svg_tag
    }
}

mod test {
    #[allow(unused)]
    use super::{Dot, Canvas};

    #[test]
    fn dot() {
        let dot1 = Dot::new(1, 1);
        let dot1_svg = String::from("<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"10\" y=\"10\" width=\"100\" height=\"100\"/>\n");
        assert_eq!(dot1.to_svg(), dot1_svg);

        let dot2 = Dot::new(5, 4);
        let dot2_svg = String::from("<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"50\" y=\"40\" width=\"100\" height=\"100\"/>\n");
        assert_eq!(dot2.to_svg(), dot2_svg);
    }

    #[test]
    fn canvas() {
        let mut canvas = Canvas::new();
        let canvas_svg = String::from("<svg width=\"80\" height=\"80\" viewPort=\"0 0 80 80\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\">\n</svg>\n");
        assert_eq!(canvas.to_svg(), canvas_svg);

        let dot1 = Dot::new(1, 1);
        canvas.add(dot1);
        let canvas_svg = String::from(r#"<svg width="80" height="80" viewPort="0 0 80 80" version="1.1" xmlns="http://www.w3.org/2000/svg">
<rect xmlns="http://www.w3.org/2000/svg" x="10" y="10" width="100" height="100"/>
</svg>
"#);
        assert_eq!(canvas.to_svg(), canvas_svg);

        let dot2 = Dot::new(5, 4);
        canvas.add(dot2);
        let canvas_svg = String::from(r#"<svg width="80" height="80" viewPort="0 0 80 80" version="1.1" xmlns="http://www.w3.org/2000/svg">
<rect xmlns="http://www.w3.org/2000/svg" x="10" y="10" width="100" height="100"/>
<rect xmlns="http://www.w3.org/2000/svg" x="50" y="40" width="100" height="100"/>
</svg>
"#);
        assert_eq!(canvas.to_svg(), canvas_svg);
    }
}
