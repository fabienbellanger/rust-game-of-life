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

    pub fn get_index(&self, row: usize, column: usize) -> Option<usize> {
        let index = row * self.width + column;

        match index {
            idx if idx >= self.width * self.height => None,
            idx => Some(idx),
        }
    }

    pub fn get_cell(&self, row: usize, column: usize) -> Option<Cell> {
        match self.get_index(row, column) {
            Some(idx) => Some(self.cells[idx]),
            _ => None,
        }
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

                if let Some(index) = index {
                    count += self.cells[index] as u8;
                }
            }
        }

        count
    }

    pub fn next_generation(&mut self) {
        let mut next_cells = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);

                if let Some(index) = index {
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
        }

        self.cells = next_cells;
    }
}

mod tests {
    #[test]
    fn test_new_universe() {
        let universe = super::Universe::new(8, 8);

        assert_eq!(64, universe.cells.len(), "cells length must be correct");
        assert_eq!(8, universe.width, "width must be correct");
        assert_eq!(8, universe.height, "height must be correct");
        assert_eq!(
            64,
            universe
                .cells
                .iter()
                .filter(|cell| cell == &&super::Cell::Dead)
                .count(),
            "all cells must be dead"
        );
    }

    #[test]
    fn test_get_index() {
        let universe = super::Universe::new(8, 8);

        assert_eq!(Some(0), universe.get_index(0, 0));
        assert_eq!(Some(63), universe.get_index(7, 7));
        assert_eq!(None, universe.get_index(8, 0));
    }

    #[test]
    fn test_get_cell() {
        let mut universe = super::Universe::new(8, 8);
        universe.cells[0] = super::Cell::Alive;

        assert_eq!(Some(super::Cell::Alive), universe.get_cell(0, 0));
        assert_eq!(Some(super::Cell::Dead), universe.get_cell(7, 7));
        assert_eq!(Some(super::Cell::Dead), universe.get_cell(1, 5));
    }

    #[test]
    fn test_alive_neighbor_count() {
        let mut universe = super::Universe::new(8, 8);
        universe.cells[0] = super::Cell::Alive;
        universe.cells[1] = super::Cell::Alive;
        universe.cells[8] = super::Cell::Alive;
        universe.cells[9] = super::Cell::Alive;
        universe.cells[10] = super::Cell::Alive;

        assert_eq!(3, universe.alive_neighbor_count(0, 0));
        assert_eq!(4, universe.alive_neighbor_count(1, 1));
        assert_eq!(1, universe.alive_neighbor_count(2, 3));
        assert_eq!(1, universe.alive_neighbor_count(7, 7));
        assert_eq!(0, universe.alive_neighbor_count(6, 6));
    }

    #[test]
    fn test_next_generation() {
        let mut universe = super::Universe::new(8, 8);
        universe.cells[8] = super::Cell::Alive;
        universe.cells[9] = super::Cell::Alive;
        universe.cells[10] = super::Cell::Alive;

        let mut expected_universe = super::Universe::new(8, 8);
        expected_universe.cells[1] = super::Cell::Alive;
        expected_universe.cells[9] = super::Cell::Alive;
        expected_universe.cells[17] = super::Cell::Alive;

        universe.next_generation();

        assert_eq!(universe.cells, expected_universe.cells);
    }
}
