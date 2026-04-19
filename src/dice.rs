//TODO: Dice roll in rust
//
use rand::Rng;
pub fn dice_roll() -> u32 {
    let mut rng = rand::rng();
    rng.random_range(1..=6)
}
