//! Game module with game logic.

#[derive(Debug)]
enum Cell {
    Alive,
    Died,
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Cell>,
}

impl Grid {
    fn new() -> Self {
        Self { cells: vec![] }
    }
}
