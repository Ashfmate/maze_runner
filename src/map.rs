use core::panic;

pub struct Map {
    tiles: &'static str,
}

impl Map {
    pub fn new(tiles: &'static str) -> Self {
        if tiles.len() != Map::width() * Map::height() {
            panic!(
                "Tiles amount should be {} by {}",
                Map::width(),
                Map::height()
            );
        }
        Self { tiles }
    }

    pub fn draw(&self) {
        let mut buf = String::new();
        let h_line = format!("+{}+", "-".repeat(Map::width()));
        buf.push_str(&h_line);
        buf.push_str("\n|");
        let mut col_num = 0;
        for tile in self.tiles.chars() {
            if col_num >= Map::width() {
                col_num = 0;
                buf.push_str("|\n|");
            }
            buf.push(tile);
            col_num += 1;
        }
        buf.push_str("|\n");
        buf.push_str(&h_line);
        println!("{buf}");
    }

    pub const fn width() -> usize {
        4
    }

    pub const fn height() -> usize {
        3
    }
}

pub struct Tiles {
    width: usize,
    height: usize,
    tiles: String,
}

impl Tiles {
    pub fn new(width: usize, tiles: impl Into<String>) -> Self {
        let tiles = tiles.into();
        if tiles.len() % width != 0 {
            panic!("[tiles] argument's length must be a multiple of [width] argument");
        }
        let height = tiles.len() / width;

        Self {
            width,
            height,
            tiles,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl From<Tiles> for Vec<char> {
    fn from(value: Tiles) -> Self {
        value.tiles.chars().collect()
    }
}

impl From<Tiles> for Vec<Vec<char>> {
    fn from(value: Tiles) -> Self {
        let mut res = vec![];
        let mut row = vec![];
        for tile in value.tiles.chars().enumerate() {
            row.push(tile.1);
            if (tile.0 + 1) % value.width() == 0 {
                res.push(row.drain(..).collect());
            }
        }
        res
    }
}
