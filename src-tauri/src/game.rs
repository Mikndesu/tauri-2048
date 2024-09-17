use crate::game_board::GameBoard;

pub struct Game {
    game_board: GameBoard,
}

impl Game {
    pub fn new() -> Self {
        Self {
            game_board: GameBoard::new(),
        }
    }

    pub fn start(&mut self) {
        loop {}
    }
}
