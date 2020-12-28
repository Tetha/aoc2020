use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time::SystemTime;

use fasthash::city::Hash128;
use itertools::join;

use crate::AdventError;

pub fn part1() -> Result<(), AdventError> {
    let input = include_str!("input");
    let parse_start = SystemTime::now();
    let subject = input.parse::<TileBox>()?;
    let adjacency_start = SystemTime::now();
    let possible_neighbours = build_possible_adjacency_matrix(&subject);
    let precomputation_end = SystemTime::now();
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

    println!("Time parsing: {:?}", adjacency_start.duration_since(parse_start).unwrap());
    println!("Time computing: {:?}", precomputation_end.duration_since(adjacency_start).unwrap());
    Ok(())
}

pub fn part2() -> Result<(), AdventError> {
    let input = include_str!("input");
    let parse_start = SystemTime::now();
    let subject = input.parse::<TileBox>()?;
    let adjacency_start = SystemTime::now();
    let possible_neighbours = build_possible_adjacency_matrix(&subject);
    let precomputation_end = SystemTime::now();

    println!("The long wait starts now");
    //let start = possible_neighbours.possibilities_below.keys().nth(0).unwrap();
    let mut bottom_row_tiles = possible_neighbours.possibilities_below
                                .iter()
                                .filter(|(_, v)| v.is_empty())
                                .map(|(k, _)| k )
                                .collect::<Vec<&PlacedTile>>();

    bottom_row_tiles.sort_by_key(|p| p.id);
    println!("{}", bottom_row_tiles.len());
    /*
    for tile in bottom_row_tiles {
        println!("Bottom Right Corner candidate: {:?}/r {:?}/f {:?}", tile.id, tile.rotation, tile.flipped);
    }*/
    let bottom_left_corner_candidates = bottom_row_tiles.iter()
                                                                        .filter(|p| possible_neighbours.possibilities_left[**p].len() == 0)
                                                                        .copied()
                                                                        .collect::<Vec<&PlacedTile>>();
    println!("{}", bottom_left_corner_candidates.len());

    let mut real_bottom_row: Option<Vec<&PlacedTile>> = None;
    for possible_start in bottom_left_corner_candidates.iter()
                                    .filter(|p| p.flipped == Flipped::Normal )
                                    .copied() {

        let mut current = possible_start;
        let mut bottom_row: Vec<&PlacedTile> = Vec::new();
        loop {
            bottom_row.push(current);
            //println!("{} {}", bottom_row.len(), possible_neighbours.possibilities_right[current].len());
            if possible_neighbours.possibilities_right[current].len() == 0 {
                break
            }
            current = possible_neighbours.possibilities_right[current].iter().nth(0).unwrap();
        }

        if bottom_row.len() == 12 {
            println!("Candidate found\n");
            real_bottom_row = Some(bottom_row);
            break;
        }
    }

    if let Some(br) = real_bottom_row {
        println!("huzza");
        for tile in &br {
            println!("Bottom Right Corner candidate: {:?}/r {:?}/f {:?}", tile.id, tile.rotation, tile.flipped);
        }

        let mut grid: Vec<Option<&PlacedTile>> = vec![None; 12 * 12];
        for (i, tile) in br.iter().enumerate() {
            let y = 11;
            grid[(i + y * 12)] = Some(*tile);
        }

        let width = 12;
        for y in (0..(width-1)).rev() {
            for x in 0..width {
                let tile_below_me = grid[(x + (y + 1) * width)].unwrap();
                let my_possibilities_from_below = &possible_neighbours.possibilities_top[tile_below_me];
                let my_possibilities_from_left = if x == 0 {
                    None
                } else {
                    let tile_left_of_me = grid[(x - 1 + y * width)];
                    tile_left_of_me.map(|t| &possible_neighbours.possibilities_right[t])
                };

                println!("-----");
                println!("{} {} {}", x, y, tile_below_me.id);
                println!("{}", join(my_possibilities_from_below.iter().map(|p| p.id), ", "));
                println!("{}", my_possibilities_from_left.map(|l| join(l.iter().map(|p| p.id), ", ")).unwrap_or("/".to_string()));
                println!("-----");

                if my_possibilities_from_below.len() == 1 {
                    grid[x + y * width] = Some(&my_possibilities_from_below[0]);
                    println!("{} {} deduced based on bottom", x, y);
                } else if let Some(poss) = my_possibilities_from_left {
                    if poss.len() == 1 {
                        println!("{} {} deduced based on left", x, y);
                        grid[x + y * width] = Some(&poss[0]);
                    } else {
                        panic!("Cannot decide based on below & left. Not prepared for this");
                    }
                } else {
                    println!("guessing...");
                    grid[x + y * width] = Some(&my_possibilities_from_below[0]);
                    //panic!("Cannot decide based on below & left. Not prepared for this");
                }
            }
        }
        println!("Unknown remaining grid fields {}", grid.iter().filter(|o| o.is_none()).count());
        println!("Known grid fields {}", grid.iter().filter(|o| o.is_some()).count());

        let tile_width = subject.tiles.values().nth(0).unwrap().width;
        let tile_height = subject.tiles.values().nth(0).unwrap().height;
        
        let pixel_width = 12 * (tile_width - 2);
        let mut pixels: Vec<char> = Vec::new();
        for py in 0..(12 * tile_height) {
            for px in 0..(12 * tile_width) {
                let tile_x = px / tile_width;
                let tile_y = py / tile_height;
                let mut in_tile_x = px % tile_width;
                let mut in_tile_y = py % tile_height;

                if in_tile_x == 0 || in_tile_x == tile_width - 1 {
                    continue;
                }
                if in_tile_y == 0 || in_tile_y == tile_height - 1 {
                    continue;
                }

                let placed_tile = grid[tile_x + tile_y * width].unwrap();


                match placed_tile.rotation {
                    Rotation::None => {}, // nothing to do
                    Rotation::Once => {
                        let old_x = in_tile_x;
                        let old_y = in_tile_y;
                        in_tile_x = old_y;
                        in_tile_y = tile_height - 1 - old_x;
                    },
                    Rotation::Twice => {
                        let old_x = in_tile_x;
                        let old_y = in_tile_y;
                        in_tile_x = tile_width - 1 - old_x;
                        in_tile_y = tile_height - 1 - old_y;
                    },
                    Rotation::Thrice => {
                        let old_x = in_tile_x;
                        let old_y = in_tile_y;
                        in_tile_x = tile_width - 1 - old_y;
                        in_tile_y = old_x;
                    },
                }
                match placed_tile.flipped {
                    Flipped::Normal => {}, // nothing to do
                    Flipped::Horizontally => {
                        in_tile_x = tile_width - 1 - in_tile_x;
                    }
                } 
                let pixel = //if placed_tile.flipped == Flipped::Normal {
                    match placed_tile.content.pixels[in_tile_x + in_tile_y * tile_width] {
                        Pixel::Block => '#',
                        Pixel::Water => '.',
                    };
                /*} else {
                    "?"
                };*/
                //print!("{}", pixel);
                pixels.push(pixel);
            }
            //println!("");
        }

        for y in 0..(pixel_width) {
            for x in 0..(pixel_width) {
                //let tp_x = x;
                //let tp_y = y;

                // 90 degrees clockwise
                //let tp_x = y;
                //let tp_y = pixel_width - 1 - x;

                // 180 degrees clockwise
                //let tp_x = pixel_width - 1 - x;
                //let tp_y = pixel_width - 1 - y;

                // 3 turns clockwise
                let tp_x = pixel_width - 1 - y;
                let tp_y = x;

                print!("{}", pixels[tp_x + tp_y * pixel_width]);
            }
            println!("")
        }
        println!("");

        let pattern_height = 3;
        let pattern_width = 20;
        let pattern = vec![
            ' ', ' ', ' ',  ' ', ' ', ' ', ' ',  ' ', ' ', ' ', ' ',  ' ', ' ', ' ', ' ',  ' ', ' ', ' ', '#', ' ',
            '#', ' ', ' ',  ' ', ' ', '#', '#',  ' ', ' ', ' ', ' ',  '#', '#', ' ', ' ',  ' ', ' ', '#', '#', '#',
            ' ', '#', ' ',  ' ', '#', ' ', ' ',  '#', ' ', ' ', '#',  ' ', ' ', '#', ' ',  ' ', '#', ' ', ' ', ' ',
        ];

        for y in 0..pattern_height {
            for x in 0..pattern_width {
                print!("{}", pattern[x + y * pattern_width]);
            }
            println!("");
        }

        let mut nessies = 0;
        for y in 0..(pixel_width - pattern_height) {
            for x in 0..(pixel_width - pattern_width) {
                let mut is_nessie = true;
                for py in 0..pattern_height {
                    if !is_nessie {
                        break;
                    }
                    for px in 0..pattern_width {
                        if !is_nessie {
                            break;
                        }
                        match pattern[px + py * pattern_width] {
                            '#' => {
                                let tp_x = pixel_width - 1 - (y + py);
                                let tp_y = x + px;
                                match pixels[tp_x + tp_y * pixel_width] {
                                    '#' => {}, // nop
                                    _ => {
                                        is_nessie = false
                                    },
                                }
                            },
                            _ => {} // ignore
                        }
                    }
                }
                if is_nessie {
                    nessies += 1;
                }
            }
        }
        println!("found {} nessies", nessies);
        let blocks = pixels.iter().filter(|p| **p == '#').count();
        println!("there are {} blocks", blocks);
        println!("thus, roughness is {}", blocks - 15 * nessies);
        //           11111111112
        //  12345678901234567890
        //0                   # 
        //1 #    ##    ##    ###
        //2  #  #  #  #  #  #   
        
    }

    /* 
    let mut start = None;
    for tile in possible_neighbours.possibilities_below.keys() {
        if possible_neighbours.possibilities_top[tile].len() == 0 
            && possible_neighbours.possibilities_left[tile].len() == 0 {
                println!("Bottom Right Corner candidate: {:?}/r {:?}/f {:?}", tile.id, tile.rotation, tile.flipped);
            start = Some(tile);
            break;
        }
    }
    */
    /* 
    let start = Some(PlacedTile{
        id: 1543,
        rotation: Rotation::Twice,
        flipped: Flipped::Normal,
        content: &subject.tiles.get(&1543).unwrap(),
    })*/
    /* 
    let offset = 144;
    let placed_tiles = vec![None; 2*15 * 2 * 15];
    let mut recursor = ReconstructionState{
                placement_attempts: 0,
                tilebox: &subject,
                placed_tiles: placed_tiles,
                used_tile_contents: HashSet::new(),
                height: 13,
                width: 13,
                offset: offset as i32,
            };
    println!("{} to go", possible_neighbours.possibilities_below.len());
    for (i, tile) in possible_neighbours.possibilities_below.keys().enumerate() {
        if i % 100 == 0 {
            println!(".");
        }
        recursor.place_tile(&tile, 0, 0, 0, &possible_neighbours);
        panic!()
    }
    let backtracking_end = SystemTime::now();
    println!("Time parsing: {:?}", adjacency_start.duration_since(parse_start).unwrap());
    println!("Time computing: {:?}", precomputation_end.duration_since(adjacency_start).unwrap());
    println!("attempted {} placements", recursor.placement_attempts);
    println!("Time backtracking: {:?}", backtracking_end.duration_since(precomputation_end).unwrap());
    */
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

// PSEUDO CODE
// Input:
//   -> A placed Tile
//   -> an (x,y) where to place it
//   -> State:
//      - other placed tiles
//      - tile-contents already placed
//
//  1. Check if all adjacent placed tiles are compatible with the new tile.
//     if not, abort and return (/)
//  2. Mark the tile content ID as placed (/)
//  3. If all tile contents are placed, finish? Search? unsure. (?)
//  4. For each empty adjacent square, attempt to place a compatible placed tile

struct ReconstructionState<'a> {
    tilebox: &'a TileBox,
    height: i32,
    width: i32,
    offset: i32,
    placed_tiles: Vec<Option<PlacedTile<'a>>>,
    //placed_tiles: HashMap<(i32, i32), &'a PlacedTile<'a>>,
    used_tile_contents: HashSet<usize>,
    placement_attempts: u32,
}

impl<'a, 'b> ReconstructionState<'a> {
    fn place_tile(&'b mut self, placed_tile: &'a PlacedTile<'a>, x: i32, y: i32, depth: i32, adjacencies: &'a PlacementMatrixes<'a>) {
        if x <= -self.width || x >= self.width { // breaks the data structure
            return;
        }
        if y <= - self.height || y >= self.height {
            return;
        }
        self.placement_attempts += 1;
        //if self.placement_attempts % 10000 == 0 {
            //println!("{} attempts, current {} at {}/{}, {} tiles placed", "*".repeat(depth as usize), placed_tile.id, x, y, self.used_tile_contents.len());
        //}
        //if self.placement_attempts == 100 {
        //    panic!("Too much output");
        //}
        // Check left
        if let Some(Some(left_neighbour_tile)) = self.placed_tiles.get(((x + self.width -1) + (y + self.height) * self.width) as usize) {
        //if let Some(left_neighbour_tile) = self.placed_tiles.get(&(x-1, y)) {
            if !adjacencies.possibilities_left[&placed_tile].contains(left_neighbour_tile) {
                // invalid placement, abort
                //println!("{} - looked at {}", "*".repeat(depth as usize), (self.offset + x-1 + y * self.width));
                return;
            }
        } 
        // Check right
        if let Some(Some(right_neighbour_tile)) = self.placed_tiles.get(((self.width + x+1) + (y+self.height) *  self.width) as usize) {
        //if let Some(right_neighbour_tile) = self.placed_tiles.get(&(x+1, y)) {
            if !adjacencies.possibilities_right[&placed_tile].contains(right_neighbour_tile) {
                // invalid placement, abort
                //println!("{}placing {} at {}/{} failed: right adjacency failed against {:#?}", "*".repeat(depth as usize), placed_tile.id, x, y, right_neighbour_tile.id);
                return;
            }
        } 
        // Check up
        if let Some(Some(up_neighbour_tile)) = self.placed_tiles.get((self.offset + (self.width + x) + (self.height + y+1) * self.width) as usize) {
        //if let Some(up_neighbour_tile) = self.placed_tiles.get(&(x, y+1)) {
            if !adjacencies.possibilities_top[&placed_tile].contains(up_neighbour_tile) {
                // invalid placement, abort
                //println!("{}placing {} at {}/{} failed: up adjacency failed against {:#?}", "*".repeat(depth as usize), placed_tile.id, x, y, up_neighbour_tile.id);
                return;
            }
        } 
        // Check down
        if let Some(Some(below_neighbour_tile)) = self.placed_tiles.get(((x + self.width) + (self.height + y-1) * self.width) as usize) {
        //if let Some(below_neighbour_tile) = self.placed_tiles.get(&(x, y-1)) {
            if !adjacencies.possibilities_below[&placed_tile].contains(below_neighbour_tile) {
                // invalid placement, abort
                //println!("{}placing {} at {}/{} failed: below adjacency failed against {:#?}", "*".repeat(depth as usize), placed_tile.id, x, y, below_neighbour_tile.id);
                return;
            }
        } 

        self.used_tile_contents.insert(placed_tile.id);
        assert_eq!(None, self.placed_tiles[(self.offset + self.width + x + (self.height + y) * self.width) as usize]);
        self.placed_tiles[(self.offset + self.width + x + (self.height + y) * self.width) as usize] = Some(placed_tile.clone());
        //self.placed_tiles.insert((x, y), placed_tile);
        let marked_tiles = self.used_tile_contents.len();
        let placed_tiles_in_vector = self.placed_tiles.iter().filter(|s| s.is_some()).count();
        assert_eq!(marked_tiles, placed_tiles_in_vector);

        if self.used_tile_contents.len() >= self.tilebox.tiles.len() - 5 {
            println!("{} is close to {}", self.used_tile_contents.len(), self.tilebox.tiles.len());
        }


        if self.used_tile_contents.len() == self.tilebox.tiles.len() {
            println!("Possible reassembly found.");
            panic!("too happy");
        }

        if self.placed_tiles[(self.offset + (self.width + x - 1) + (self.height + y) * self.width) as usize].is_none() {
        //if !self.placed_tiles.contains_key(&(x-1, y)) {
            let possibilities = &adjacencies.possibilities_left[&placed_tile];
            //println!("{}Options left: {}", "*".repeat(depth as usize), possibilities.len());
            for possibility in possibilities {
                // skip already used tiles
                if self.used_tile_contents.contains(&possibility.id) {
                    continue;
                }
                self.place_tile(possibility, x - 1, y, depth + 1, adjacencies);
            }
        }

        if self.placed_tiles[(self.offset + (self.width + x + 1) + (self.height + y) * self.width) as usize].is_none() {
        //if !self.placed_tiles.contains_key(&(x+1, y)) {
            let possibilities = &adjacencies.possibilities_right[&placed_tile];
            //println!("{}Options right: {}", "*".repeat(depth as usize), possibilities.len());
            for possibility in possibilities {
                // skip already used tiles
                if self.used_tile_contents.contains(&possibility.id) {
                    continue;
                }
                self.place_tile(possibility, x + 1, y, depth + 1, adjacencies);
            }
        }

        if self.placed_tiles[(self.offset + (self.width + x) + (self.height + y - 1) * self.width) as usize].is_none() {
        //if !self.placed_tiles.contains_key(&(x, y-11)) {
            let possibilities = &adjacencies.possibilities_below[&placed_tile];
            //println!("{}Options below: {}", "*".repeat(depth as usize), possibilities.len());
            for possibility in possibilities {
                // skip already used tiles
                if self.used_tile_contents.contains(&possibility.id) {
                    continue;
                }
                self.place_tile(possibility, x, y-1, depth + 1, adjacencies);
            }
        }

        /* 
        if self.placed_tiles[(self.offset + (self.width + x) + (self.height + y + 1) * self.width) as usize].is_none() {
        //if !self.placed_tiles.contains_key(&(x, y+1)) {
            let possibilities = &adjacencies.possibilities_top[&placed_tile];
            //println!("{}Options above: {}", "*".repeat(depth as usize), possibilities.len());
            for possibility in possibilities {
                // skip already used tiles
                if self.used_tile_contents.contains(&possibility.id) {
                    continue;
                }
                self.place_tile(possibility, x, y+1, depth + 1, adjacencies);
            }
        }*/


        // backtrack, cleanup
        self.placed_tiles[(self.offset + (self.width + x) + (self.height + y) * self.width) as usize] = None;
        //self.placed_tiles.remove(&(x, y));
        self.used_tile_contents.remove(&placed_tile.id);
    }
}

#[derive(Debug)]
struct PlacementMatrixes<'a> {
    possibilities_below: HashMap<PlacedTile<'a>, Vec<PlacedTile<'a>>, Hash128>,
    possibilities_right: HashMap<PlacedTile<'a>, Vec<PlacedTile<'a>>, Hash128>,
    possibilities_left: HashMap<PlacedTile<'a>, Vec<PlacedTile<'a>>, Hash128>,
    possibilities_top: HashMap<PlacedTile<'a>, Vec<PlacedTile<'a>>, Hash128>,
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

    let mut possibilities_below: HashMap<PlacedTile, Vec<PlacedTile>, Hash128> = HashMap::with_hasher(Hash128);
    let mut possibilities_right: HashMap<PlacedTile, Vec<PlacedTile>, Hash128> = HashMap::with_hasher(Hash128);
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
        possibilities_left: HashMap::with_hasher(Hash128),
        possibilities_top: HashMap::with_hasher(Hash128),
    } ;
    reverse_placement_matrixes(&mut result);
    print_stats(&result);
    result
}

fn print_stats(matrixes: &PlacementMatrixes) {
    println!("Possibilities below");
    print_adjacency_stats(&matrixes.possibilities_below);
    println!("Possibilities right");
    print_adjacency_stats(&matrixes.possibilities_right);
    println!("Possibilities top");
    print_adjacency_stats(&matrixes.possibilities_top);
    println!("Possibilities left");
    print_adjacency_stats(&matrixes.possibilities_left);
}

fn print_adjacency_stats(map: &HashMap<PlacedTile, Vec<PlacedTile>, Hash128>) {
    println!("Elements: {}", map.len());
    println!("Tiles with 0 possible neighbours: {}", map.values().filter(|l| l.len() == 0).count());
    println!("Tiles with 1 possible neighbours: {}", map.values().filter(|l| l.len() == 1).count());
    println!("Tiles with 2 possible neighbours: {}", map.values().filter(|l| l.len() == 2).count());

    for (tile, adjacent) in map.iter().filter(|(_, l)| l.len() == 2) {
        println!("({}, {:?}, {:?}) - {:?}", tile.id, tile.flipped, tile.rotation, adjacent.iter().map(|p| (p.id, p.flipped, p.rotation)).collect::<Vec<(usize, Flipped, Rotation)>>());
    }
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
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Rotation {
    None,
    Once,
    Twice,
    Thrice,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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