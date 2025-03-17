use std::cmp::PartialEq;
use std::collections::HashMap;
use rand::Rng;
fn main() {

}

fn start(player_list: Vec<Player>) {
    //create a new board, then take turns until a player reaches the final tile
}
struct Player {
    username: String,
    piece: String,
    position: u32,
}

impl Player {
    fn get_position(&self) -> u32 {
        self.position
    }
    fn get_username(&self) -> String {
        self.username.clone()
    }
    fn get_piece(&self) -> String {
        self.piece.clone()
    }
    fn set_name(&mut self, name: String) {
        self.username = name;
    }
    fn set_position(&mut self, position: u32) {
        self.position = position;
    }
    fn set_piece(&mut self, piece: String) {
        self.piece = piece;
    }
}

struct Tile {
    tile_type: TileType,
}

enum TileType {
    SNAKE,
    LADDER,
    STANDARD
}

impl Tile {
    fn new(tile_type: TileType) -> Tile {
        Tile { tile_type }
    }
    fn get_tile_type(self) -> TileType {
        self.tile_type
    }
    fn set_tile_type(&mut self, tile_type: TileType) {
        self.tile_type = tile_type;
    }
}

struct Board {
    board: HashMap<u32, Tile>
}

impl PartialEq for TileType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TileType::SNAKE, TileType::SNAKE) => true,
            (TileType::LADDER, TileType::LADDER) => true,
            (TileType::STANDARD, TileType::STANDARD) => true,
            _ => false
        }
    }
}

impl Board {
    fn new(size: u32) -> Board {
        let mut board = HashMap::new();
        for i in 1..size {
            board.insert(i, Tile { tile_type: TileType::STANDARD });
        }
        Board { board }
    }

    fn add_snakes(&mut self, amount_to_add: u32,) {
        let mut rng = rand::rng();
        if self.board.is_empty() {
            println!("Board is empty. Please add tiles first.");
        }
        else {
            for _ in 0..amount_to_add {
                rng.random_range(self.board.len()).unwrap();
                if self.board.get(&rng).unwrap().tile_type == TileType::SNAKE ||
                   self.board.get(&rng).unwrap().tile_type == TileType::LADDER {
                    _ -= 1;
                }
                else {
                    self.board.get_mut(&rng).unwrap().set_tile_type(TileType::SNAKE);
                }
            }
        }
    }

    //I know this is a lot of repeated code, but I'm not sure how to make a function to add both snakes and ladders.
    fn add_ladders(&mut self, amount_to_add: u32,) {
        let mut rng = rand::rng();
        if self.board.is_empty() {
            println!("Board is empty. Please add tiles first.");
        }
        else {
            for _ in 0..amount_to_add {
                rng.random_range(self.board.len()).unwrap();
                if self.board.get(&rng).unwrap().tile_type == TileType::LADDER
                || self.board.get(&rng).unwrap().tile_type == TileType::SNAKE {
                    _ -= 1;
                }
                else {
                    self.board.get_mut(&rng).unwrap().set_tile_type(TileType::LADDER);
                }
            }
        }
    }
}