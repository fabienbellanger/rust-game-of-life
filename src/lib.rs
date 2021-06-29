mod game;

use game::{Cell, Universe};
use minifb::{Key, Window, WindowOptions};

const CELL_SIZE: usize = 32;
const GRID_SIZE: usize = 8;

const CELL_WIDTH: usize = GRID_SIZE;
const CELL_HEIGHT: usize = GRID_SIZE;

const WIDTH: usize = CELL_SIZE * GRID_SIZE;
const HEIGHT: usize = CELL_SIZE * GRID_SIZE;

pub fn start_game() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut cells: Vec<u32> = vec![0; CELL_WIDTH * CELL_HEIGHT];

    cells.iter_mut().enumerate().for_each(|(i, c)| {
        *c = i as u32 * 16_000;
    });

    let mut universe = Universe::new(GRID_SIZE, GRID_SIZE);
    universe.cells[8] = Cell::Alive;
    universe.cells[9] = Cell::Alive;
    universe.cells[10] = Cell::Alive;
    universe.next_generation();
    // dbg!(universe.cells);

    let mut window = Window::new(
        "Game Of Life - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (index, cell) in buffer.iter_mut().enumerate() {
            let row = index / WIDTH;
            let col = index % WIDTH;

            let cell_row = row / CELL_SIZE;
            let cell_col = col / CELL_SIZE;

            *cell = match universe.get_cell(cell_row, cell_col) {
                Cell::Alive => 0xFFFFFF,
                Cell::Dead => 0x000000,
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
