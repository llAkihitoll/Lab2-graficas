use std::thread;
use std::time::Duration;

mod framebuffer;

use framebuffer::Framebuffer;

const GRID_WIDTH: i32 = 100;
const GRID_HEIGHT: i32 = 100;
const CELL_SIZE: i32 = 8;
const DELAY_MS: u64 = 100;

fn main() {
    let (window_width, window_height) = (GRID_WIDTH * CELL_SIZE, GRID_HEIGHT * CELL_SIZE);

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Conway's Game of Life")
        .build();

    let mut fb = Framebuffer::new(GRID_WIDTH, GRID_HEIGHT);

    // patron de prueba: un bloque (no deberia cambiar nunca, es estable)
    // y un blinker (deberia oscilar entre horizontal y vertical cada frame)
    let cx = GRID_WIDTH / 2;
    let cy = GRID_HEIGHT / 2;
    fb.set(cx, cy, true);
    fb.set(cx + 1, cy, true);
    fb.set(cx, cy + 1, true);
    fb.set(cx + 1, cy + 1, true);

    fb.set(cx - 10, cy, true);
    fb.set(cx - 9, cy, true);
    fb.set(cx - 8, cy, true);

    while !rl.window_should_close() {
        fb.step();

        let mut d = rl.begin_drawing(&thread);
        fb.render(&mut d, CELL_SIZE);
        drop(d);

        thread::sleep(Duration::from_millis(DELAY_MS));
    }
}
