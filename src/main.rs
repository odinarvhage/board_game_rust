use std::collections::HashMap;
use fastrand;

//TODO: Add tests
//TODO: Add text input for making players
//TODO: Add text input for board size, snakes, and ladders
//TODO: Start making the GUI with egui crate
fn main() {
    let mut player_list: Vec<Player> = Vec::new();
    player_list.push(Player::new("Nick".to_string(),"Hat".to_string()));
    player_list.push(Player::new("Odin".to_string(),"Car".to_string()));
    start(&mut player_list,100,10,10);

}

fn make_player(username: String, piece: String, player_list: Vec<Player>) {
    add_player_to_list(Player::new(username, piece),player_list);

}

fn add_player_to_list(player_to_add: Player, mut player_list: Vec<Player>) {
    player_list.push(player_to_add);
}

fn start(player_list: &mut Vec<Player>, size: u32, snakes: u32, ladders: u32) {
    let mut winner = false;
    let board = make_board(size, snakes, ladders);
    while !winner {
        for player in player_list.iter_mut() {
            println!("{} is at position {}", player.get_username(), player.get_position());
            let dice_roll = fastrand::u32(1..7);
            println!("{} has rolled {}", player.get_username(), dice_roll);
            perform_turn(player, dice_roll);
            match board.get_tile(player.get_position()) {
                Some(2) => {
                    println!("{} has landed on a snake!", player.get_username());
                    let new_position = player.get_position() - 5;
                    player.set_position(new_position);
                }
                Some(1) => {
                    println!("{} has landed on a ladder!", player.get_username());
                    let new_position = player.get_position() + 5;
                    println!("{} is moving up {} spaces!", player.get_username(), 5);
                    player.set_position(new_position);
                }
                _ => {}
            }
            if player.get_position() >= size {
                println!("{} has won the game!", player.get_username());
                winner = true;
                break;
            }
            println!("{} is now at position {}", player.get_username(), player.get_position());
            println!("\n");
        }
    }
}
fn perform_turn(player: &mut Player, roll: u32) {
    let position_change = player.get_position() + roll;
    player.set_position(position_change);
}
fn make_board(size: u32, snakes: u32, ladders: u32) -> Board {
    let mut board = Board::new(size);
    board.add_event_tiles(snakes, 2);
    board.add_event_tiles(ladders, 1);
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

/**
    * The board struct is a map of u32 to u32. The key represents the position on the board and the value represents the type of tile.
    * The board is initialized with a size and all tiles are set to STANDARD.
    * A value of 0 means standard, 1 means ladder, and 2 means snake.
    */
struct Board {
    board: HashMap<u32, u32>
}


impl Board {
    fn new(size: u32) -> Board {
        let mut board = HashMap::new();
        for i in 1..size {
            board.insert(i, 0);
        }
        Board { board }
    }

    fn get_tile(&self, position: u32) -> Option<u32> {
        self.board.get(&position).cloned()
    }

    fn add_event_tiles(&mut self, amount_to_add: u32, tile_type: u32) {
        if self.board.is_empty() {
            println!("Board is empty. Please add tiles first.");
        } else {
            let positions: Vec<u32> = (0..self.board.len() as u32)
                .filter_map(|_| {
                    let random_number = fastrand::u32(0..self.board.len() as u32);
                    if random_number == 1 || random_number == self.board.len() as u32 {
                        None
                    } else {
                        Some(random_number)
                    }
                })
                .take(amount_to_add as usize)
                .collect();

            for pos in positions {
                self.board.insert(pos, tile_type);
            }
        }
    }
}