//! Game module with game logic.
// See https://rustwasm.github.io/book/game-of-life/implementing.html

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Alive = 1,
    Dead = 0,
}

#[derive(Debug)]
pub struct Universe {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::Dead; width * height],
        }
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    pub fn get_cell(&self, row: usize, column: usize) -> Cell {
        self.cells[self.get_index(row, column)]
    }

    pub fn alive_neighbor_count(&self, row: usize, column: usize) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let index = self.get_index(neighbor_row, neighbor_col);

                count += self.cells[index] as u8;
            }
        }

        count
    }

    pub fn next_generation(&mut self) {
        let mut next_cells = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let alive_neighbors = self.alive_neighbor_count(row, col);

                let next_cell = match (cell, alive_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, n) if n < 2 => Cell::Dead,

                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,

                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, n) if n > 3 => Cell::Dead,

                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,

                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next_cells[index] = next_cell;
            }
        }

        self.cells = next_cells;
    }
}

// TODO: Add tests!
