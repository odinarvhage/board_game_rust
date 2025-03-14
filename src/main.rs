use std::cmp::PartialEq;
use rand::Rng;
use crate::TileType::{Ladder, Snake, Standard};

fn main() {
    let mut board = BoardBuilder::new();
        board.add_tile(Standard);
        board.add_tile(Standard);
        board.add_tile(Standard);
        board.add_tile(Standard);
        board.add_tile(Standard);
        board.add_tile(Standard);
        board.add_tile(Standard);

    let mut player_one: User = User {
        name: "Thrall".to_string(),
        position: 0,
        class: "Shaman".to_string(),
        level: 1,
        can_level: true,
    };
    while player_one.position < 100 {
        player_one.read_position();
        player_one.change_position(roll_dice());
        println!("\n");
        if player_one.position > 70 && player_one.can_level {
            player_one.level_up();
            player_one.read_level()
        }
    }
}

struct BoardBuilder {
    tiles: Vec<Tile>
}

enum BoardComponent {
    Snake,
    Ladder,
    Standard
}

impl BoardBuilder {
    fn new() -> BoardBuilder {
        BoardBuilder {
            tiles: Vec::new()
        }
    }
    pub fn add_tile(&mut self, tile: TileType) {
        self.tiles.push(make_tile(tile));
    }
}


/*
 * Struct to represent a tile on the board.
 * The tile can be a standard, snake or ladder tile.
 * The snake and ladder fields will be used when a player moves to check what tile they are on.
*/
struct Tile {
    tile_type: TileType,
}

enum TileType {
    Snake,
    Ladder,
    Standard
}

/*
 * Implementations for the Tile struct.
 * The implementations will be used to check if the tile is a snake or ladder.
 */
impl Tile {
    fn read_type(self) -> TileType {
        self.tile_type
    }
}

/*
 * Implementations for the TileType enum.
 * The implementations will be used to check if the tile is a snake or ladder.
 */
impl PartialEq for TileType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Snake => {
                match other {
                    Snake => true,
                    _ => false
                }
            }
            Ladder => {
                match other {
                    Ladder => true,
                    _ => false
                }
            }
            Standard => {
                match other {
                    Standard => true,
                    _ => false
                }
            }
        }
    }
}

fn make_tile(tile_type: TileType) -> Tile {
    if tile_type == Snake {
        Tile {
            tile_type
        }
    }
    else if tile_type == Ladder {
        Tile {
            tile_type
        }
    }
    else {
        Tile {
            tile_type
        }
    }
}


struct User {
    name: String,
    position: u8,
    class: String,
    level: u8,
    can_level: bool,
}

/*
* Implementations for the User struct.
* The implementations will be used to read the position of the player,
* change the position of the player and level up the player.
*/

impl User {
    fn read_position(&self) {
        println!("{} is at position {}", self.name, self.position);
    }
    fn change_position(&mut self, input: u8) {
        self.position += input;
    }
    fn level_up(&mut self) {
        self.level += 1;
        self.can_level = false;
    }
    fn read_level(&self) {
        println!("{} has leveled up to level {}!", self.name, self.level);
    }
}
fn roll_dice()-> u8 {
    let mut rng = rand::rng();
    rng.random_range(1..=6)
}



