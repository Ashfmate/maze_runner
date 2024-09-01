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
