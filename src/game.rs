//! Game module with game logic.
// See https://rustwasm.github.io/book/game-of-life/implementing.html

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Alive = 0,
    Dead = 1,
}

#[derive(Debug)]
struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Universe {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::Dead; width * height],
        }
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        todo!();
    }
}
