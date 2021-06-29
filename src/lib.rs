mod game;

use minifb::{Key, Window, WindowOptions};

const CELL_SIZE: usize = 16;
const GRID_SIZE: usize = 32;

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

            *cell = *cells
                .get(cell_row * CELL_WIDTH + cell_col)
                .expect(&format!("index out of bound (index={})", index));
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
