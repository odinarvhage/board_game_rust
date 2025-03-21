mod test;
use std::collections::BTreeMap;
use eframe::egui;
use std::io;

//TODO: Add text input for board size, snakes, and ladders

/**
* The main function creates a vector of players and adds two players to the list.
* The start function is then called with the player list, the size of the board, the number of snakes, and the number of ladders.
*/
fn main() {
    let board = make_board(100,10,10);
    let mut player_list = PlayerList::new();
    player_list.add_player();
    player_list.add_player();
    start(&mut player_list.get_players(), board);
}

/**
 * The PlayerList struct represents a list of players in the game.
 * It has a vector of players.
 * The new function creates a new PlayerList with an empty vector of players.
 * The add_player function adds a new player to the player list.
 * The get_players function returns the list of players.
 */
struct PlayerList {
    players: Vec<Player>,
}

/**
 * The PlayerList struct represents a list of players in the game.
 * It has a vector of players.
 * The new function creates a new PlayerList with an empty vector of players.
 * The add_player function adds a new player to the player list.
 * The get_players function returns the list of players.
 */
impl PlayerList {
    /**
     * The new function creates a new PlayerList with an empty vector of players.
     * The add_player function adds a new player to the player list.
     * The get_players function returns the list of players.
     */
    fn new() -> PlayerList {
        PlayerList {
            players: Vec::new(),
        }
    }

    /**
     * The add_player function adds a new player to the player list.
     */
    fn add_player(&mut self) {
        let player = Player::new();
        self.players.push(player);
    }

    /**
     * The get_players function returns the list of players.
     */
    fn get_players(self) -> Vec<Player> {
        self.players
    }
}
/**
* The start function is the main function that runs the game. It takes a mutable reference to a vector of players, the size of the board, the number of snakes, and the number of ladders.
* The function loops through the player list and performs a turn for each player. The player rolls a die and moves that many spaces.
* If the player lands on a snake, they move back 5 spaces. If they land on a ladder, they move up 5 spaces.
* The game continues until a player reaches the end of the board.
*/

fn start(player_list: &mut Vec<Player>, board: Board) {
    let mut winner = false;
    while !winner {
        for player in player_list.iter_mut() {
            println!("{} is at position {}", player.username, player.position);
            let dice_roll = fastrand::u32(1..7);
            println!("{} has rolled {}", player.username, dice_roll);
            perform_turn(player, dice_roll);
            match board.get_tile(player.position) {
                Some(2) => {
                    println!("{} has landed on a snake!", player.username);
                    let new_position = player.position - 5;
                    if new_position >= 0 {
                        player.set_position(0);
                    }

                    player.set_position(new_position);
                }
                Some(1) => {
                    println!("{} has landed on a ladder!", player.username);
                    let new_position = player.position + 5;
                    println!("{} is moving up {} spaces!", player.username, 5);
                    player.set_position(new_position);
                }
                _ => {}
            }
            if player.position >= board.board.len() as u32 {
                println!("{} has won the game!", player.username);
                winner = true;
                break;
            }
            println!("{} is now at position {}", player.username, player.position);
            println!("\n");
        }
    }
}

/**
 * The perform_turn function takes a mutable reference to a player and a u32 representing the roll of the die.
 * The function adds the roll to the player's current position.
 */
fn perform_turn(player: &mut Player, roll: u32) {
    let position_change = player.position + roll;
    player.set_position(position_change);
}

/**
* The make_board function takes a u32 representing the size of the board, a u32 representing the number of snakes, and a u32 representing the number of ladders.
* The function creates a new board with the given size and adds the specified number of snakes and ladders to the board.
* The snakes and ladders are randomly placed on the board.
*/
fn make_board(size: u32, snakes: u32, ladders: u32) -> Board {
    let mut board = Board::new(size);
    board.add_event_tiles(snakes, 2);
    board.add_event_tiles(ladders, 1);
    board
}

/**
* The Player struct represents a player in the game. It has a username, a piece, and a position.
* The position represents the player's current position on the board.
*/
struct Player {
    username: String,
    position: u32,
}

/**
* The Board struct represents the game board.
* It's a map of u32 to u32, where the key represents the position on the board,
* and the value represents the type of tile.
* The board is initialized with a size and all tiles are set to STANDARD.
* A value of 0 means standard, 1 means ladder, and 2 means snake.
*/
impl Player {
    fn new() -> Player {
        let mut name_input = String::new();
        println!("Enter the name of the player: ");
        io::stdin().read_line(&mut name_input).expect("Failed to read line");
        let mut piece_input = String::new();
        println!("Enter the piece for the player: ");
        io::stdin().read_line(&mut piece_input).expect("Failed to read line");
        Player {
            username: name_input,
            position: 0,
        }
    }

    fn set_position(&mut self, new_position: u32) {
        self.position = new_position;
    }

}

/**
 * The board struct is a map of u32 to u32.
 * The key represents the position on the board,and the value represents the type of tile.
 * The board is initialized with a size and all tiles are set to STANDARD.
 * A value of 0 means standard, 1 means ladder, and 2 means snake.
 */

#[derive(Default)]
struct Board {
    board: BTreeMap<u32, u32>,
}

/**
 * The board struct is a map of u32 to u32.
 * The key represents the position on the board,and the value represents the type of tile.
 * The board is initialized with a size and all tiles are set to STANDARD.
 * A value of 0 means standard, 1 means ladder, and 2 means snake.
 */
impl eframe::App for Board {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for (key, value) in &self.board {
                let entry = format!("pos {}, tileType: {}", key, value); // Format the tuple into a String
                ui.label(entry); // Pass the String to the label
            }
        });
    }
}



/**
 * The Board struct has a new function that takes a u32 representing the size of the board and returns a new Board with the given size.
 * The get_tile function takes a u32 representing the position on the board and returns the type of tile at that position.
 * The add_event_tiles function takes a u32 representing the number of tiles to add and a u32 representing the type of tile to add.
 * The function randomly places the specified number of tiles on the board.
 */
impl Board {

    /**
     * The new function takes a u32 representing the size of the board and returns a new Board with the given size.
     */
    fn new(size: u32) -> Board {
        let mut board = BTreeMap::new();
        for i in 1..size {
            board.insert(i, 0);
        }

        Board { board }
    }
    /**
     * The get_tile function takes a u32 representing the position on the board and returns the type of tile at that position.
     */
    fn get_tile(&self, position: u32) -> Option<u32> {
        self.board.get(&position).cloned()
    }

    /**
     * The add_event_tiles function takes a u32 representing the number of tiles to add and a u32 representing the type of tile to add.
     * The function randomly places the specified number of tiles on the board.
     */
    fn add_event_tiles(&mut self, amount_to_add: u32, tile_type: u32) {
        if self.board.is_empty() {
            println!("Board is empty. Please add tiles first.");
        } else {
            let mut counter: u32 = 0;
            while counter < amount_to_add {
                let position = fastrand::u32(0..self.board.len() as u32);
                if self.board.get(&position) == Some(&1) || self.board.get(&position) == Some(&2) {
                    continue;
                } else {
                    self.board.insert(position, tile_type);
                    counter += 1;
                }
            }
        }
    }
}
