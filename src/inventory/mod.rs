mod wallet;
mod shop;

pub use wallet::*;
pub use shop::*;

#[derive(Debug)]
struct Weapon {
    id: String,
}