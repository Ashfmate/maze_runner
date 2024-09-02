use core::panic;
use std::borrow::Borrow;

pub struct Map {
    tiles: Tiles,
}

impl Map {
    pub fn new(tiles: impl Into<Tiles>) -> Self {
        Self {
            tiles: tiles.into(),
        }
    }

    pub fn draw(&self) {
        let mut buf = String::new();
        let h_line = format!("+{}+", "-".repeat(self.tiles.width));
        buf.push_str(&h_line);
        buf.push('\n');
        let matrix: Vec<Vec<char>> = self.tiles.borrow().into();
        for row in matrix {
            buf.push('|');
            buf.push_str(row.iter().collect::<String>().as_str());
            buf.push_str("|\n");
        }
        buf.push_str(&h_line);
        println!("{buf}");
    }
}

pub struct Tiles {
    width: usize,
    tiles: String,
}

impl Tiles {
    pub fn new(width: usize, tiles: impl Into<String>) -> Self {
        let tiles = tiles.into();
        if tiles.len() % width != 0 {
            panic!("[tiles] argument's length must be a multiple of [width] argument");
        }
        Self { width, tiles }
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
        let mut row = vec!['\0'; value.width];
        for tile in value.tiles.chars().enumerate() {
            let r#mod = tile.0 % value.width;
            row[r#mod] = tile.1;
            if r#mod == value.width - 1 {
                res.push(row.clone().into());
            }
        }
        res
    }
}

impl From<Vec<Vec<char>>> for Tiles {
    fn from(value: Vec<Vec<char>>) -> Self {
        if value.len() == 0 {
            panic!("The number of rows must not be 0");
        }
        let width = value[0].len();
        if value.iter().any(|item| item.len() != width) {
            panic!("Matrix must have a uniform width");
        }
        Self::new(
            width,
            value
                .into_iter()
                .flatten()
                .fold("".to_string(), |acc, item| format!("{acc}{item}")),
        )
    }
}

impl From<Tiles> for String {
    fn from(value: Tiles) -> Self {
        value.tiles.chars().collect()
    }
}

impl From<Vec<String>> for Tiles {
    fn from(value: Vec<String>) -> Self {
        if value.len() == 0 {
            panic!("The number of rows must not be 0");
        }
        let width = value[0].len();
        if value.iter().any(|item| item.len() != width) {
            panic!("Matrix must have a uniform width");
        }
        Self::new(
            width,
            value.iter().fold("".to_string(), |acc, item| acc + item),
        )
    }
}

impl From<Vec<&str>> for Tiles {
    fn from(value: Vec<&str>) -> Self {
        if value.len() == 0 {
            panic!("The number of rows must not be 0");
        }
        let width = value[0].len();
        if value.iter().any(|item| item.len() != width) {
            panic!("Matrix must have a uniform width");
        }
        Self::new(
            width,
            value.iter().fold("".to_string(), |acc, item| acc + item),
        )
    }
}

impl From<&Tiles> for Vec<char> {
    fn from(value: &Tiles) -> Self {
        value.tiles.chars().collect()
    }
}

impl From<&Tiles> for Vec<Vec<char>> {
    fn from(value: &Tiles) -> Self {
        let mut res = vec![];
        let mut row = vec!['\0'; value.width];
        for tile in value.tiles.chars().enumerate() {
            row[tile.0 % value.width] = tile.1;
            if (tile.0 + 1) % value.width == 0 {
                res.push(row.clone().into());
            }
        }
        res
    }
}

impl From<&Tiles> for String {
    fn from(value: &Tiles) -> Self {
        value.tiles.chars().collect()
    }
}

impl From<&[&str]> for Tiles {
    fn from(value: &[&str]) -> Self {
        if value.len() == 0 {
            panic!("The number of rows must not be 0");
        }
        let width = value[0].len();
        if value.iter().any(|item| item.len() != width) {
            panic!("Matrix must have a uniform width");
        }
        Self::new(
            width,
            value.iter().fold("".to_string(), |acc, item| acc + item),
        )
    }
}

impl From<&[String]> for Tiles {
    fn from(value: &[String]) -> Self {
        if value.len() == 0 {
            panic!("The number of rows must not be 0");
        }
        let width = value[0].len();
        if value.iter().any(|item| item.len() != width) {
            panic!("Matrix must have a uniform width");
        }
        Self::new(
            width,
            value.iter().fold("".to_string(), |acc, item| acc + item),
        )
    }
}
