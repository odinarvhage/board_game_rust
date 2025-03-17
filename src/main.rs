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

impl Board {
    fn new(size: u32) -> Board {
        let mut board = HashMap::new();
        for i in 1..size {
            board.insert(i, Tile { tile_type: TileType::STANDARD });
        }
        Board { board }
    }

    fn add_snakes(&mut self, snakes: u32) {
        let mut rng = rand::rng();
        for _ in 0..snakes {
            rng.random_range(self.board.len());

        }
    }
}