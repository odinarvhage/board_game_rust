use rand::Rng;

fn main() {
    let mut player_one: User = User{
        name: "Thrall".to_string(),
        position: 0,
        class: "Shaman".to_string()
    };
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


