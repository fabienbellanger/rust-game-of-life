mod game;

use minifb::{Key, Window, WindowOptions};

const CELL_SIZE: usize = 16;
const GRID_SIZE: usize = 32;

const CELL_WIDTH: usize = GRID_SIZE;
const CELL_HEIGHT: usize = GRID_SIZE;

const WIDTH: usize = CELL_SIZE * GRID_SIZE;
const HEIGHT: usize = CELL_SIZE * GRID_SIZE;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut cells: Vec<u32> = vec![0; CELL_WIDTH * CELL_HEIGHT];

    cells[0] = 0xFF0000;
    cells[1] = 0x00FF00;
    cells[2] = 0x0000FF;
    cells[23] = 0x0034FF;

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
            let x = index % WIDTH;
            let y = index / WIDTH;

            let cell_x = x / CELL_SIZE;
            let cell_y = y / CELL_SIZE;

            *cell = *cells.get(cell_y * CELL_WIDTH + cell_x)
                .expect(&format!("index out of bound (index={})", index));
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
