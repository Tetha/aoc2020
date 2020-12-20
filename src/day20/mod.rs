use std::collections::HashMap;
use std::str::FromStr;

use crate::AdventError;

pub fn test() -> Result<(), AdventError> {
    let input = include_str!("example_input");
    let subject = input.parse::<TileBox>()?;
    let sample_tile = subject.tiles.iter().nth(0).unwrap();
    println!("Tile {}:", sample_tile.0);
    sample_tile.1.dump_to_stdout();

    println!("Tile {}: (flipped)", sample_tile.0);
    sample_tile.1.dump_to_stdout_flipped();

    let subject = PlacedTile{
        id: *sample_tile.0,
        content: sample_tile.1,
        flipped: Flipped::Horizontally,
        rotation: Rotation::Thrice,
    };

    println!("-----");
    println!("Rotation = {:?}", subject.rotation);
    println!("Top: {:?}", format_pixel_vec(&subject.top_border()));
    println!("Right: {:?}", format_pixel_vec(&subject.right_border()));
    println!("Bottom: {:?}", format_pixel_vec(&subject.bottom_border()));
    println!("Left: {:?}", format_pixel_vec(&subject.left_border()));
    Ok(())
}

fn format_pixel_vec(pixels: &Vec<Pixel>) -> String {
    pixels.iter()
          .map(|p| match p {
              Pixel::Block => "#",
              Pixel::Water => ".",
          })
          .collect::<String>()
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum Pixel {
    Block,
    Water,
}

#[derive(Debug)]
struct TileBox {
    tiles: HashMap<usize, TileContent>,
}

impl FromStr for TileBox {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut result = TileBox{tiles: HashMap::new()};
        loop {
            let tile_line = lines.next();
            if tile_line.is_none() {
                return Ok(result);
            }
            let tile_line = tile_line.unwrap();
            if !tile_line.starts_with("Tile") || !tile_line.ends_with(":") {
                return Err(AdventError{cause: format!("tile line malformed: <{}>", tile_line)});
            }
            let unparsed_tile_id = tile_line.strip_prefix("Tile ").and_then(|s| s.strip_suffix(":")).unwrap();
            let tile_id = unparsed_tile_id.parse::<usize>()?;

            let mut tile_content = String::new();
            loop {
                let tile_line = lines.next().unwrap_or("");
                if tile_line == "" {
                    break;
                }
                tile_content.push_str(tile_line);
                tile_content.push('\n');
            }
            let tile = tile_content.parse::<TileContent>()?;
            result.tiles.insert(tile_id, tile);
        }
    }
}

#[derive(Debug)]
struct TileContent {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

impl FromStr for TileContent {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let mut width = 0;
        let mut pixels: Vec<Pixel> = Vec::new();
        for l in s.lines() {
            width = l.chars().count();
            for c in l.chars() {
                match c {
                    '#' => pixels.push(Pixel::Block),
                    '.' => pixels.push(Pixel::Water),
                    _ => return Err(AdventError{cause: format!("unknown char {}", c)}),
                }
            }
        }
        assert_eq!(width * height, pixels.len());
        Ok(TileContent{ height, width, pixels })
    }
}

impl TileContent {
    fn get_pixel(&self, x: usize, y: usize) -> Pixel {
        self.pixels[x + y * self.width]
    }

    fn dump_to_stdout(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get_pixel(x, y) {
                    Pixel::Block => print!("#"),
                    Pixel::Water => print!("."),
                }
            }
            println!("");
        }
    }
    fn dump_to_stdout_flipped(&self) {
        for y in 0..self.height {
            for x in (0..self.width).rev() {
                match self.get_pixel(x, y) {
                    Pixel::Block => print!("#"),
                    Pixel::Water => print!("."),
                }
            }
            println!("");
        }
    }
    fn top_border(&self, reversed: bool) -> Vec<Pixel> {
        let mut result = Vec::new();
        for x in 0..self.height {
            result.push(self.get_pixel(x, 0));
        }
        if reversed {
            result.reverse();
        }
        result
    }

    fn bottom_border(&self, reversed: bool) -> Vec<Pixel> {
        let mut result = Vec::new();
        for x in 0..self.height {
            result.push(self.get_pixel(x, self.height-1));
        }
        if reversed {
            result.reverse();
        }
        result
    }

    fn left_border(&self, reversed: bool) -> Vec<Pixel> {
        let mut result = Vec::new();
        for y in 0..self.width {
            result.push(self.get_pixel(0, y));
        }
        if reversed {
            result.reverse();
        }
        result
    }

    fn right_border(&self, reversed: bool) -> Vec<Pixel> {
        let mut result = Vec::new();
        for y in 0..self.width {
            result.push(self.get_pixel(self.width-1, y));
        }
        if reversed {
            result.reverse();
        }
        result
    }
}

// clockwise rotation
#[derive(Debug)]
enum Rotation {
    None,
    Once,
    Twice,
    Thrice,
}

enum Flipped {
    Normal,
    Horizontally,
}
struct PlacedTile<'a> {
    id: usize,
    rotation: Rotation,
    flipped: Flipped,
    content: &'a TileContent,
}

impl<'a> PlacedTile<'a> {
    fn left_border(&self) -> Vec<Pixel> {
        match self.flipped {
            Flipped::Normal => {
                match self.rotation {
                    Rotation::None => self.content.left_border(false),
                    Rotation::Once => self.content.bottom_border(false),
                    Rotation::Twice => self.content.right_border(true),
                    Rotation::Thrice => self.content.top_border(true),
                }
            }
            Flipped::Horizontally => {
                match self.rotation {
                    Rotation::None => self.content.right_border(false),
                    Rotation::Once => self.content.bottom_border(true),
                    Rotation::Twice => self.content.left_border(true),
                    Rotation::Thrice => self.content.top_border(false),
                }
            }
        }
    }

    fn right_border(&self) -> Vec<Pixel> {
        match self.flipped {
            Flipped::Normal => {
                match self.rotation {
                    Rotation::None => self.content.right_border(false),
                    Rotation::Once => self.content.top_border(false),
                    Rotation::Twice => self.content.left_border(true),
                    Rotation::Thrice => self.content.bottom_border(true),
                }
            }
            Flipped::Horizontally => {
                match self.rotation {
                    Rotation::None => self.content.left_border(false),
                    Rotation::Once => self.content.top_border(true),
                    Rotation::Twice => self.content.right_border(true),
                    Rotation::Thrice => self.content.bottom_border(false),
                }
            }
        }
    }

    fn top_border(&self) -> Vec<Pixel> {
        match self.flipped {
            Flipped::Normal => {
                match self.rotation {
                    Rotation::None => self.content.top_border(false),
                    Rotation::Once => self.content.left_border(true),
                    Rotation::Twice => self.content.bottom_border(true),
                    Rotation::Thrice => self.content.right_border(false),
                }
            }
            Flipped::Horizontally => {
                match self.rotation {
                    Rotation::None => self.content.top_border(true),
                    Rotation::Once => self.content.right_border(true),
                    Rotation::Twice => self.content.bottom_border(false),
                    Rotation::Thrice => self.content.left_border(false),
                }

            }
        }
    }

    fn bottom_border(&self) -> Vec<Pixel> {
        match self.flipped {
            Flipped::Normal => {
                match self.rotation {
                    Rotation::None => self.content.bottom_border(false),
                    Rotation::Once => self.content.right_border(false),
                    Rotation::Twice => self.content.top_border(true),
                    Rotation::Thrice => self.content.left_border(false),
                }
            }
            Flipped::Horizontally => {
                match self.rotation {
                    Rotation::None => self.content.bottom_border(true),
                    Rotation::Once => self.content.left_border(true),
                    Rotation::Twice => self.content.top_border(false),
                    Rotation::Thrice => self.content.right_border(false),
                }
            }
        }
    }

    fn can_be_placed_right_of(&self, other: &PlacedTile) -> bool {
        self.left_border().iter()
                          .zip(other.right_border())
                          .all(|(&p1, p2)| p1 == p2)
    }

    fn can_be_placed_below_of(&self, other: &PlacedTile) -> bool {
        self.top_border().iter()
                          .zip(other.bottom_border())
                          .all(|(&p1, p2)| p1 == p2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotations_and_stuff() {
        let content = TileContent{
            width: 3,
            height: 3,
            pixels: vec![
                Pixel::Block, Pixel::Block, Pixel::Water,
                Pixel::Water, Pixel::Water, Pixel::Block,
                Pixel::Block, Pixel::Water, Pixel::Block,
            ]
        };

        let mut subject = PlacedTile{
            id: 1234,
            flipped: Flipped::Normal,
            rotation: Rotation::None,
            content,
        };

        subject.flipped = Flipped::Horizontally;
        subject.rotation = Rotation::Once;
        
        assert_eq!(subject.top_border(), vec![Pixel::Block, Pixel::Block, Pixel::Water]);
        assert_eq!(subject.right_border(), vec![Pixel::Water, Pixel::Block, Pixel::Block]);
        assert_eq!(subject.bottom_border(), vec![Pixel::Block, Pixel::Water, Pixel::Block]);
        assert_eq!(subject.left_border(), vec![Pixel::Block, Pixel::Water, Pixel::Block]);

        subject.flipped = Flipped::Normal;
        subject.rotation = Rotation::None;

        assert_eq!(subject.top_border(), vec![Pixel::Block, Pixel::Block, Pixel::Water]);
        assert_eq!(subject.right_border(), vec![Pixel::Water, Pixel::Block, Pixel::Block]);
        assert_eq!(subject.bottom_border(), vec![Pixel::Block, Pixel::Water, Pixel::Block]);
        assert_eq!(subject.left_border(), vec![Pixel::Block, Pixel::Water, Pixel::Block]);

        subject.flipped = Flipped::Horizontally;
        subject.rotation = Rotation::None;
        
        assert_eq!(subject.top_border(), vec![Pixel::Water, Pixel::Block, Pixel::Block]);
        assert_eq!(subject.right_border(), vec![Pixel::Block, Pixel::Water, Pixel::Block]);
        assert_eq!(subject.bottom_border(), vec![Pixel::Block, Pixel::Water, Pixel::Block]);
        assert_eq!(subject.left_border(), vec![Pixel::Water, Pixel::Block, Pixel::Block]);
    }
}