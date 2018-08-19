## SVG Monsters

Generator for Github default icon-like SVG files written in Rust.

This is based on MonsterID project.

## Usage

```rust
extern crate svg_monsters;

use svg_monster::svg_pixel::{Canvas, SvgGenerator, MonsterPattern};

fn main() {
    // Generate a random Monster shape (returns value as Vec<Vec<bool>>).
    let monster_pattern = MonsterPattern::generate_ramdomly();

    // Read Vec<Vec<bool>> data & Convert it to svg-format String value with a random color.
    let svg_content = Canvas::from_vec_with_random_color(monster_pattern).to_svg();

    // Set your svg file path.
    let filepath = "path/to/your/svg/file";

    // Save svg file to the path given as a second argument.
    // And result returns Result<&'static str, &'static str>.
    let result = SvgGenerator::save(svg_content, filepath);

    // You could use result.
    match result {
        Ok(_) => { println!("Success!!!"); },
        Err(_) => { println!("Failure..."); },
    };
}
```

## Sample

![monster00.svg](https://github.com/furuhama/svg_monsters/blob/master/svg/monster_pattern00.svg)
![monster01.svg](https://github.com/furuhama/svg_monsters/blob/master/svg/monster_pattern01.svg)
![monster02.svg](https://github.com/furuhama/svg_monsters/blob/master/svg/monster_pattern02.svg)
![monster03.svg](https://github.com/furuhama/svg_monsters/blob/master/svg/monster_pattern03.svg)
![monster04.svg](https://github.com/furuhama/svg_monsters/blob/master/svg/monster_pattern04.svg)

## TODOs

- move this to web server

## Refs

[MonsterID](https://www.splitbrain.org/projects/monsterid)
