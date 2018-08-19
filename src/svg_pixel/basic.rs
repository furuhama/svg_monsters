extern crate rand;

use self::rand::{thread_rng, Rng};

pub struct Dot {
    x: usize,
    y: usize,
    color: &'static str,
}

impl Dot {
    fn new(x: usize, y: usize, color: &'static str) -> Self {
        Self {
            x: x,
            y: y,
            color: color,
        }
    }

    fn new_with_random_color(x: usize, y: usize) -> Self {
        let mut rng = thread_rng();

        Self {
            x: x,
            y: y,
            color: COLORS[rng.gen_range(0, 8)],
        }
    }

    fn to_svg(&self) -> String {
        let x_pos = self.x * 10 + 10;
        let y_pos = self.y * 10 + 10;
        format!("<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"{}\" y=\"{}\" width=\"10\" height=\"10\" fill=\"{}\"/>\n", x_pos, y_pos, self.color)
    }
}

pub const COLORS: [&'static str; 8] = ["#D27A7F", "#99D480", "#7E9EAF", "#19A1B0", "#F05030", "#FBB82C", "#86BC24", "#9E47FB"];

pub struct Canvas {
    // TODO: define canvas size here
    dots: Vec<Dot>,
}

impl Canvas {
    fn new() -> Self {
        Self {
            dots: Vec::<Dot>::new(),
        }
    }

    // vec should be like
    // [
    //      [1, 0, 0, 0],
    //      [1, 1, 0, 0],
    //      [0, 0, 1, 1],
    //      [1, 0, 0, 1]
    // ]
    pub fn from_vec(vec: Vec<Vec<usize>>) -> Self {
        let mut dots = Vec::<Dot>::new();

        for y in 0..vec.len() {
            for x in 0..vec[y].len() {
                if vec[y][x] == 1 {
                    dots.push(Dot::new(x, y, COLORS[0]));
                }
            }
        }

        Self {
            dots: dots,
        }
    }

    fn add(&mut self, dot: Dot) -> bool {
        // TODO: check dot position here (dot should not be outside the canvas)
        // TODO: check dot position not duplicate (dot should not be the same position)
        self.dots.push(dot);

        true
    }

    pub fn to_svg(&self) -> String {
        let mut svg_tag = String::from(r##"<svg width="140" height="140" viewPort="10 10 130 130" version="1.1" xmlns="http://www.w3.org/2000/svg">
<rect xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" fill="#ECECEC"/>
"##);
        for dot in &self.dots {
            svg_tag.push_str(dot.to_svg().as_str());
        }
        svg_tag.push_str("</svg>\n");
        svg_tag
    }
}

mod test {
    #[allow(unused)]
    use super::{Dot, Canvas, COLORS};

    #[test]
    fn dot() {
        let dot1 = Dot::new(1, 1, COLORS[0]);
        let dot1_svg = String::from("<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"20\" y=\"20\" width=\"10\" height=\"10\" fill=\"#D27A7F\"/>\n");
        assert_eq!(dot1.to_svg(), dot1_svg);

        let dot2 = Dot::new(5, 4, COLORS[0]);
        let dot2_svg = String::from("<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"60\" y=\"50\" width=\"10\" height=\"10\" fill=\"#D27A7F\"/>\n");
        assert_eq!(dot2.to_svg(), dot2_svg);
    }

    #[test]
    fn canvas() {
        let canvas = Canvas::new();
        let canvas_svg = String::from(r##"<svg width="140" height="140" viewPort="10 10 130 130" version="1.1" xmlns="http://www.w3.org/2000/svg">
<rect xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" fill="#ECECEC"/>
</svg>
"##);
        assert_eq!(canvas.to_svg(), canvas_svg);
    }

    #[test]
    fn canvas_with_dots() {
        let mut canvas = Canvas::new();

        let dot1 = Dot::new(1, 1, COLORS[0]);
        canvas.add(dot1);
        let canvas_svg = String::from(r##"<svg width="140" height="140" viewPort="10 10 130 130" version="1.1" xmlns="http://www.w3.org/2000/svg">
<rect xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" fill="#ECECEC"/>
<rect xmlns="http://www.w3.org/2000/svg" x="20" y="20" width="10" height="10" fill="#D27A7F"/>
</svg>
"##);
        assert_eq!(canvas.to_svg(), canvas_svg);

        let dot2 = Dot::new(5, 4, COLORS[0]);
        canvas.add(dot2);
        let canvas_svg = String::from(r##"<svg width="140" height="140" viewPort="10 10 130 130" version="1.1" xmlns="http://www.w3.org/2000/svg">
<rect xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" fill="#ECECEC"/>
<rect xmlns="http://www.w3.org/2000/svg" x="20" y="20" width="10" height="10" fill="#D27A7F"/>
<rect xmlns="http://www.w3.org/2000/svg" x="60" y="50" width="10" height="10" fill="#D27A7F"/>
</svg>
"##);
        assert_eq!(canvas.to_svg(), canvas_svg);
    }

    #[test]
    fn canvas_from_vec() {
        let vec = vec![
            vec![0, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 1],
        ];
        let canvas = Canvas::from_vec(vec);
        let canvas_svg = String::from(r##"<svg width="140" height="140" viewPort="10 10 130 130" version="1.1" xmlns="http://www.w3.org/2000/svg">
<rect xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" fill="#ECECEC"/>
<rect xmlns="http://www.w3.org/2000/svg" x="20" y="20" width="10" height="10" fill="#D27A7F"/>
<rect xmlns="http://www.w3.org/2000/svg" x="40" y="40" width="10" height="10" fill="#D27A7F"/>
</svg>
"##);
        assert_eq!(canvas.to_svg(), canvas_svg);
    }
}
