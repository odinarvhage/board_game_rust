fn main() {
    let name: &str = "Odin";
    let age: u16 = 20;
    let height: f32 = 182.2;
    print_user_information(name,age,height);
}

fn print_user_information(name: &str, age: u16, height: f32) {
    println!("This persons name is {}, their age is {}, and their height is {}", name, age, height);
}

fn roll_dice() -> i32 {
    let dice_roll: i32 = 42;
    return dice_roll;
}