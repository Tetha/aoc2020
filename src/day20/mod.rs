use std::collections::HashMap;
use std::str::FromStr;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let subject = input.parse::<TileBox>()?;
    let possible_neighbours = build_possible_adjacency_matrix(&subject);

    let possible_neighbours = build_possible_adjacency_matrix(&subject);
    for tile in possible_neighbours.possibilities_below.keys() {
        if possible_neighbours.possibilities_top[tile].len() == 0 
            && possible_neighbours.possibilities_left[tile].len() == 0 {
                println!("Top Left Corner candidate: {:?}/r {:?}/f {:?}", tile.id, tile.rotation, tile.flipped);
        }

        if possible_neighbours.possibilities_top[tile].len() == 0 
            && possible_neighbours.possibilities_right[tile].len() == 0 {
                println!("Top Right Corner candidate: {:?}/r {:?}/f {:?}", tile.id, tile.rotation, tile.flipped);
        }

        if possible_neighbours.possibilities_below[tile].len() == 0 
            && possible_neighbours.possibilities_right[tile].len() == 0 {
                println!("Bottom Right Corner candidate: {:?}/r {:?}/f {:?}", tile.id, tile.rotation, tile.flipped);
        }

        if possible_neighbours.possibilities_below[tile].len() == 0 
            && possible_neighbours.possibilities_left[tile].len() == 0 {
                println!("Bottom Left Corner candidate: {:?}/r {:?}/f {:?}", tile.id, tile.rotation, tile.flipped);
        }
    }
    Ok(())
}
pub fn test() -> Result<(), AdventError> {
    let input = include_str!("example_input");
    let subject = input.parse::<TileBox>()?;
    let sample_tile = subject.tiles.iter().nth(0).unwrap();
    println!("Tile {}:", sample_tile.0);
    sample_tile.1.dump_to_stdout();

    println!("Tile {}: (flipped)", sample_tile.0);
    sample_tile.1.dump_to_stdout_flipped();

    /*
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
    */

    let possible_neighbours = build_possible_adjacency_matrix(&subject);

    for tile in possible_neighbours.possibilities_below.keys() {
        if possible_neighbours.possibilities_top[tile].len() == 0 
            && possible_neighbours.possibilities_left[tile].len() == 0 {
                println!("Top Left Corner candidate: {:?}/r {:?}/f {:?}", tile.id, tile.rotation, tile.flipped);
        }

        if possible_neighbours.possibilities_top[tile].len() == 0 
            && possible_neighbours.possibilities_right[tile].len() == 0 {
                println!("Top Right Corner candidate: {:?}/r {:?}/f {:?}", tile.id, tile.rotation, tile.flipped);
        }

        if possible_neighbours.possibilities_below[tile].len() == 0 
            && possible_neighbours.possibilities_right[tile].len() == 0 {
                println!("Bottom Right Corner candidate: {:?}/r {:?}/f {:?}", tile.id, tile.rotation, tile.flipped);
        }

        if possible_neighbours.possibilities_below[tile].len() == 0 
            && possible_neighbours.possibilities_left[tile].len() == 0 {
                println!("Bottom Left Corner candidate: {:?}/r {:?}/f {:?}", tile.id, tile.rotation, tile.flipped);
        }
    }
    /*
    println!("0 - {}", possible_neighbours.possibilities_right.values().filter(|v| v.len() == 0).count());
    println!("1 - {}", possible_neighbours.possibilities_right.values().filter(|v| v.len() == 1).count());
    println!("2 - {}", possible_neighbours.possibilities_right.values().filter(|v| v.len() == 2).count());
    println!("3 - {}", possible_neighbours.possibilities_right.values().filter(|v| v.len() == 3).count());
    println!("4 - {}", possible_neighbours.possibilities_right.values().filter(|v| v.len() == 4).count());
    println!("5 - {}", possible_neighbours.possibilities_right.values().filter(|v| v.len() == 5).count());
    println!("+ - {}", possible_neighbours.possibilities_right.values().filter(|v| v.len() > 5).count());
    */

    /*
    for (top_piece, bottom_pieces) in 
        possible_neighbours.iter() {
        println!("Possible placements: On top {} {:?} {:?}, below {:?}", 
            top_piece.id, top_piece.rotation, top_piece.flipped,
            bottom_pieces.iter()
                         .map(|p| format!("{} {:?} {:?},", p.id, p.rotation, p.flipped))
                         .collect::<Vec<String>>()
        );
    }*/
    Ok(())
}

struct PlacementMatrixes<'a> {
    possibilities_below: HashMap<PlacedTile<'a>, Vec<PlacedTile<'a>>>,
    possibilities_right: HashMap<PlacedTile<'a>, Vec<PlacedTile<'a>>>,
    possibilities_left: HashMap<PlacedTile<'a>, Vec<PlacedTile<'a>>>,
    possibilities_top: HashMap<PlacedTile<'a>, Vec<PlacedTile<'a>>>,
}

fn format_pixel_vec(pixels: &Vec<Pixel>) -> String {
    pixels.iter()
          .map(|p| match p {
              Pixel::Block => "#",
              Pixel::Water => ".",
          })
          .collect::<String>()
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Pixel {
    Block,
    Water,
}

#[derive(Debug)]
struct TileBox {
    tiles: HashMap<usize, TileContent>,
}

fn build_possible_adjacency_matrix(tiles: &TileBox) -> PlacementMatrixes {

    let mut possibilities_below: HashMap<PlacedTile, Vec<PlacedTile>> = HashMap::new();
    let mut possibilities_right: HashMap<PlacedTile, Vec<PlacedTile>> = HashMap::new();
    for tile_a_idx in tiles.tiles.keys() {
        for first_tile_placement in tiles.get_all_placements(tile_a_idx) {
            let mut possible_below: Vec<PlacedTile> = Vec::new();
            let mut possible_right: Vec<PlacedTile> = Vec::new();
            for second_tile_idx in tiles.tiles.keys() {
                if second_tile_idx == tile_a_idx {
                    continue;
                }
                for second_tile_placement in tiles.get_all_placements(second_tile_idx) {
                    if second_tile_placement.can_be_placed_below_of(&first_tile_placement) {
                        possible_below.push(second_tile_placement.clone());
                    }
                    if second_tile_placement.can_be_placed_right_of(&first_tile_placement) {
                        possible_right.push(second_tile_placement);
                    }
                }
            }
            possibilities_below.insert(first_tile_placement.clone(), possible_below);
            possibilities_right.insert(first_tile_placement, possible_right);
        }
    }
    let mut result = PlacementMatrixes{
        possibilities_below,
        possibilities_right,
        possibilities_left: HashMap::new(),
        possibilities_top: HashMap::new(),
    } ;
    reverse_placement_matrixes(&mut result);
    result
}

fn reverse_placement_matrixes(matrixes: &mut PlacementMatrixes) {
    for key in matrixes.possibilities_below.keys() {
        matrixes.possibilities_left.insert(key.clone(), Vec::new());
        matrixes.possibilities_top.insert(key.clone(), Vec::new());
    }

    for (tile, right_possibilities) in matrixes.possibilities_right.iter() {
        for rp in right_possibilities {
            matrixes.possibilities_left.get_mut(&rp).unwrap().push(tile.clone());
        }
    }

    for (tile, bottom_possibilities) in matrixes.possibilities_below.iter() {
        for bp in bottom_possibilities {
            matrixes.possibilities_top.get_mut(&bp).unwrap().push(tile.clone());
        }
    }
}
impl TileBox {
    fn get_all_placements(&self, idx: &usize) -> Vec<PlacedTile> {
        let content = &self.tiles[idx];
        vec![
            PlacedTile{
                id: *idx,
                flipped: Flipped::Normal,
                rotation: Rotation::None,
                content: content,
            },
            PlacedTile{
                id: *idx,
                flipped: Flipped::Normal,
                rotation: Rotation::Once,
                content: content,
            },
            PlacedTile{
                id: *idx,
                flipped: Flipped::Normal,
                rotation: Rotation::Twice,
                content: content,
            },
            PlacedTile{
                id: *idx,
                flipped: Flipped::Normal,
                rotation: Rotation::Thrice,
                content: content,
            },
            PlacedTile{
                id: *idx,
                flipped: Flipped::Horizontally,
                rotation: Rotation::None,
                content: content,
            },
            PlacedTile{
                id: *idx,
                flipped: Flipped::Horizontally,
                rotation: Rotation::Once,
                content: content,
            },
            PlacedTile{
                id: *idx,
                flipped: Flipped::Horizontally,
                rotation: Rotation::Twice,
                content: content,
            },
            PlacedTile{
                id: *idx,
                flipped: Flipped::Horizontally,
                rotation: Rotation::Thrice,
                content: content,
            },
        ]
    }
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

#[derive(Debug, PartialEq, Eq, Hash)]
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
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Rotation {
    None,
    Once,
    Twice,
    Thrice,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Flipped {
    Normal,
    Horizontally,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
            content: &content,
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