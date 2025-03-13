use rand::Rng;

fn main() {
    let mut player_one: User = User {
        name: "Thrall".to_string(),
        position: 0,
        class: "Shaman".to_string()
    };
    while(player_one.position < 100) {
        println!("The position of {} is {}.\n",player_one.name,player_one.position);
        player_one.change_position(roll_dice())
    }
}

/*
 * Struct to represent a tile on the board.
 * The tile can be a standard, snake or ladder tile.
 * The snake and ladder fields will be used when a player moves to check what tile they are on.
*/
struct Tile {
    snake: bool,
    ladder: bool,
}

/*
 * Implementations for the Tile struct.
 * The implementations will be used to check if the tile is a snake or ladder.
 */
impl Tile{
    fn is_snake(&self) -> bool {
        self.snake
    }
    fn is_ladder(&self) -> bool {
        self.ladder
    }
}

struct User {
    name: String,
    position: u8,
    class: String,
}
impl User {
    fn read_position(&self) {
        println!("Player is at position {}", self.position);
    }
    fn change_position(&mut self, input: u8) {
        self.position += input;
    }

}
fn roll_dice()-> u8 {
    let mut rng = rand::rng();
    rng.random_range(1..=6)
}

fn make_tile(snake: bool, ladder: bool) -> Tile {
    Tile {
        snake: false,
        ladder: false,
    }
}

/*
* Function to create a board based on the size and the number of snakes and ladders.
* The return type will be a tuple of tiles.
@param size: u16 size of the board, how many tiles will be needed to cross to win
@param snakes: u8 amount of snake tiles on the board, must be an even number
@param ladders: u8 amount of ladder tiles on the board, must be an even number
*/
fn make_board(size: u16, snakes: u8, ladders: u8) -> ()  {

}

