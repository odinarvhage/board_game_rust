use std::cmp::PartialEq;
use std::collections::HashMap;
use fastrand;
fn main() {
    let mut player_list: Vec<Player> = Vec::new();
    let mut board = Board::new(100);
}

fn make_player(username: String, piece: String, player_list: Vec<Player>) {
    add_player_to_list(Player::new(username, piece),player_list);

}

fn add_player_to_list(player_to_add: Player, mut player_list: Vec<Player>) {
    player_list.push(player_to_add);
}

fn start (player_list: Vec<Player>, size: u32, snakes: u32, ladders: u32) {
    let board = make_board(size, snakes, ladders);
}

fn make_board(size: u32, snakes: u32, ladders: u32) -> Board {
    let mut board = Board::new(size);
    board
}

struct Player {
    username: String,
    piece: String,
    position: u32,
}

impl Player {

    fn new(username: String, piece: String) -> Player {
        Player {
            username,
            piece,
            position: 0,
        }
    }
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


enum TileType {
    SNAKE,
    LADDER,
    STANDARD
}

impl TileType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TileType::SNAKE, TileType::SNAKE) => true,
            (TileType::LADDER, TileType::LADDER) => true,
            (TileType::STANDARD, TileType::STANDARD) => true,
            _ => false
        }
    }

    fn clone(&self) -> TileType {
        match self {
            TileType::SNAKE => TileType::SNAKE,
            TileType::LADDER => TileType::LADDER,
            TileType::STANDARD => TileType::STANDARD,
        }
    }
}

use std::ops::{Bound, RangeBounds};

fn extract_bounds<R: RangeBounds<u32>>(range: R) -> Option<(u32, u32)> {
    let start = match range.start_bound() {
        Bound::Included(&n) => n,     // Start is inclusive
        Bound::Excluded(&n) => n + 1, // Start is exclusive, so increment
        Bound::Unbounded => return None, // No start bound
    };

    let end = match range.end_bound() {
        Bound::Included(&n) => n,     // End is inclusive
        Bound::Excluded(&n) => n - 1, // End is exclusive, so decrement
        Bound::Unbounded => return None, // No end bound
    };

    Some((start, end))
}
struct Board {
    board: HashMap<u32, TileType>
}



impl PartialEq<TileType> for &TileType {
    fn eq(&self, other: &TileType) -> bool {
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
            board.insert(i, TileType::STANDARD);
        }
        Board { board }
    }

    fn add_event_tiles(&mut self, amount_to_add: u32, tile_type: TileType) {
        if self.board.is_empty() {
            println!("Board is empty. Please add tiles first.");
        } else {
            let positions: Vec<u32> = (0..)
                .filter_map(|_| {
                    let random_number = fastrand::u32(0..self.board.len() as u32);
                    if random_number == 1 || random_number == self.board.len() as u32 {
                        None
                    } else {
                        Some(random_number)
                    }
                })
                .filter(|&pos| self.board.get(&pos).unwrap() == TileType::STANDARD)
                .take(amount_to_add as usize)
                .collect();

            for pos in positions {
                self.board.insert(pos, tile_type.clone());
            }
        }
    }
}