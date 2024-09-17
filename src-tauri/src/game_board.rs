use crate::tiles::Tiles;

pub struct GameBoard {
    tiles: Tiles,
}

impl GameBoard {
    pub fn new() -> Self {
        Self {
            tiles: Tiles::new(),
        }
    }
}
