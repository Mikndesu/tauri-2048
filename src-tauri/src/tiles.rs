use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

pub struct Tiles {
    tiles: [[i32; 4]; 4],
    randomiser: ThreadRng,
}

impl Tiles {
    pub fn new() -> Self {
        let mut instance = Self {
            tiles: [[0; 4]; 4],
            randomiser: rand::thread_rng(),
        };
        instance.initialise_tiles();
        instance
    }

    fn initialise_tiles(&mut self) {
        let mut vec: Vec<(usize, usize)> = vec![];
        self.tiles.iter().enumerate().for_each(|(y, arr)| {
            arr.iter().enumerate().for_each(|(x, _)| {
                if !self.is_initialised(y, x) {
                    vec.push((y, x));
                }
            });
        });
        if let Some(&(y, x)) = vec.choose(&mut self.randomiser) {
            let new_value = self.new_tile();
            self.update_tile(y, x, new_value);
        }
    }

    fn update_tile(&mut self, y: usize, x: usize, new_value: i32) {
        self.tiles[y][x] = new_value;
    }

    fn new_tile(&mut self) -> i32 {
        if self.randomiser.gen_range(1..10) == 1 {
            4
        } else {
            2
        }
    }

    fn is_initialised(&self, y: usize, x: usize) -> bool {
        self.tiles[y][x] != 0
    }
}
