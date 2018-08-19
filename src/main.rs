extern crate svg_monsters;

mod svg_pixel;

fn main() {
    let content = String::from("hoge\n");
    let _ = svg_pixel::svg_generator::SvgGenerator::save(content, "./hoge.txt");

    println!("SVG Monsters");
}
