mod game;

use game::{Cell, Universe};
use minifb::{Key, MouseMode, Window, WindowOptions};
use std::time::{Duration, SystemTime};

const CELL_SIZE: usize = 16;
const GRID_SIZE: usize = 32;

const CELL_WIDTH: usize = GRID_SIZE;
const CELL_HEIGHT: usize = GRID_SIZE;

const WIDTH: usize = CELL_SIZE * GRID_SIZE;
const HEIGHT: usize = CELL_SIZE * GRID_SIZE;

const REFRESH_INTERVAL: u128 = 100;

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
    window.limit_update_rate(Some(Duration::from_micros(16_600)));

    let mut universe = Universe::new(GRID_SIZE, GRID_SIZE);
    let mut now = SystemTime::now();
    let mut generation = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Start to the next generation on space key down
        // ----------------------------------------------
        if window.is_key_down(Key::Space) {
            if let Ok(duration_elapsed) = now.elapsed() {
                if duration_elapsed.as_millis() >= REFRESH_INTERVAL {
                    universe.next_generation();

                    now = SystemTime::now();

                    generation += 1;
                    println!("Generation: {}", generation);
                }
            }
        }

        // Draw cells
        // ----------
        window
            .get_unscaled_mouse_pos(MouseMode::Discard)
            .map(|mouse| {
                let x = mouse.0 as usize;
                let y = mouse.1 as usize;

                let index = universe.get_index(y / CELL_SIZE, x / CELL_SIZE);
                if window.get_mouse_down(minifb::MouseButton::Left) {
                    universe.cells[index] = Cell::Alive;
                } else if window.get_mouse_down(minifb::MouseButton::Right) {
                    universe.cells[index] = Cell::Dead;
                }
            });

        // Display cells
        // -------------
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
