use map::Map;
mod map;

fn main() {
    let map = Map::new("############");
    map.draw();
}
