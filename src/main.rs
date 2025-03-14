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

    pub fn add_tile(&mut self, tile: Tile) {
        self.tiles.push(tile);
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
    pos: u16
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
    fn read_position(&self) -> u16 {
        self.pos
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


fn make_tile(snake: bool, ladder: bool, pos: u16) -> Tile {
    Tile {
        snake,
        ladder,
        pos
    }
}

