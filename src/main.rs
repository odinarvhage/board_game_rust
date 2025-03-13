use rand::Rng;

fn main() {
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

struct Board {
    board: Vec<Tile>,
    snakes: u8,
    ladders: u8,
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

fn make_tile(snake: bool, ladder: bool) -> Tile {
    Tile {
        snake,
        ladder
    }
}

fn add_tile_to_board(snake: bool, ladder: bool) {
    let tile = make_tile(snake, ladder);
    //Add tile to board
}
/*
* Function to create a board based on the size and the number of snakes and ladders.
* The return type will be a tuple of tiles.
@param size: u16 size of the board, how many tiles will be needed to cross to win
@param snakes: u8 amount of snake tiles on the board, must be an even number
@param ladders: u8 amount of ladder tiles on the board, must be an even number
*/
fn make_board(size: u16, snakes: u8, ladders: u8)  {
//Add tiles to the board with random positions for the snakes and ladders
}
