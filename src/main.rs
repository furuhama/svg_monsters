extern crate svg_monsters;

mod svg_pixel;

use svg_pixel::SvgGenerator;

fn main() {
    let content = String::from("hoge\n");
    let _ = SvgGenerator::save(content, "./hoge.txt");

    println!("SVG Monsters");
}
