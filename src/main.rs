use rand::Rng;

fn main() {
    println!("The dice has rolled: {}!", roll_dice());

}


/**
 *
 * Returns:
 *  u8: The number rolled on the dice
 */
fn roll_dice() -> u8 {
    let dice_roll: u8 = rand::rng().random_range(1..=6);
    dice_roll
}
