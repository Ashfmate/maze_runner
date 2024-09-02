use map::{Map, Tiles};
mod map;

fn main() {
    let tiles: Tiles = ["1234", "1234", "1234"].as_slice().into();
    let map = Map::new(tiles);
    map.draw();
}
