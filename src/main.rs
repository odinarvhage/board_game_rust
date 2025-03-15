use std::cmp::PartialEq;
use rand::Rng;
use crate::TileType::{Ladder, Snake, Standard};

fn main() {
    println!("Welcome to Rust!");
    let board = Board::make_board(100, 10, 10);
    println!("WOW!");
    let mut player_one: User = User {
        name: "Thrall".to_string(),
        position: 0,
        class: "Shaman".to_string(),
        level: 1,
        can_level: true,
    };
    while player_one.read_position() < 100 {
        player_one.read_position();
        player_one.change_position(roll_dice());
        println!("{} is at position {}", player_one.read_name(), player_one.read_position());
        if player_one.position > 70 && player_one.can_level {
            player_one.level_up();
        }
    }
}

struct Board {
    tiles: Vec<Tile>
}
struct BoardComponent {
    tiles: Tile
}

impl Board {
    fn new() -> Board {
        Board {
            tiles: Vec::new()
        }
    }
    pub fn add_tile(&mut self, tile: TileType) {
        self.tiles.push(make_tile(tile));
    }

    fn change_player_position(&self, player: &mut User, movement: u16) {
        player.change_position(movement);
    }
    fn make_board(size: u16, snakes: u16, ladders: u16) -> Board {
        println!("Making board");
        let mut board_to_be_made = Board::new();
        println!("Making i");
        let mut i: u16 = 0;
        while i < size {
            println!("{} ", i);
            i += 1;
            board_to_be_made.add_tile(Standard);
        }
        board_to_be_made
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

    fn new(tile_type: TileType) -> Tile {
        Tile {
            tile_type
        }
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
        Tile::new(Snake)
    } else if tile_type == Ladder {
        Tile::new(Ladder)
    }
    else {
        Tile::new(Standard)
    }
}


struct User {
    name: String,
    position: u16,
    class: String,
    level: u16,
    can_level: bool,
}

/*
* Implementations for the User struct.
* The implementations will be used to read the position of the player,
* change the position of the player and level up the player.
*/

impl User {
    fn read_position(&self) -> u16{
        self.position
    }
    fn change_position(&mut self, input: u16){
        self.position += input;
    }
    fn level_up(&mut self) {
        self.level += 1;
        self.can_level = false;
    }
    fn read_level(&self) -> u16{
        self.level
    }

    fn read_name(&self) -> String {
        self.name.clone()
    }
}
fn roll_dice()-> u16 {
    let mut rng = rand::rng();
    rng.random_range(1..=6)
}



