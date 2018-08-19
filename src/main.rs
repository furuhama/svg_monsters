extern crate svg_monsters;

mod svg_pixel;

use svg_pixel::{Canvas, SvgGenerator, MonsterPattern};

fn main() {
    // let dot_pattern = vec![
    //     vec![1, 0, 0, 0],
    //     vec![0, 1, 0, 0],
    //     vec![0, 0, 1, 0],
    //     vec![0, 0, 0, 1]
    // ];
    // let svg_content = Canvas::from_vec(dot_pattern).to_svg();
    // let _ = SvgGenerator::save(svg_content, "svg/simple_line.svg");

    let monster_pattern = MonsterPattern::generate_ramdomly();
    let svg_content = Canvas::from_vec_with_random_color(monster_pattern).to_svg();
    let _ = SvgGenerator::save(svg_content, "svg/monster_pattern03.svg");

    println!("SVG Monsters");
}
