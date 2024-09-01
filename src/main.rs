use map::{Map, Tiles};
mod map;

fn main() {
    let tiles = Tiles::new(3, "******");
    let two_vecs: Vec<Vec<char>> = tiles.into();
    println!("{two_vecs:?}");
}
