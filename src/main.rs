extern crate svg_monsters;

mod svg_pixel;

use svg_pixel::{Canvas, SvgGenerator, MonsterPattern};

fn main() {
    let monster_pattern = MonsterPattern::generate_ramdomly();
    let svg_content = Canvas::from_vec_with_random_color(monster_pattern).to_svg();
    let result = SvgGenerator::save(svg_content, "svg/monster_pattern04.svg");

    println!("SVG Monsters: {}", result.unwrap());
}
