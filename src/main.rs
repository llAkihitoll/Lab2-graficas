use std::thread;
use std::time::Duration;

mod framebuffer;
mod organisms;

use framebuffer::Framebuffer;
use organisms::{
    beacon, block, blinker, glider, heavyweight_spaceship, lightweight_spaceship, loaf,
    middleweight_spaceship, pulsar, toad,
};

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

    // patron inicial: organismos clasicos repartidos en tres franjas
    // para que la simulacion ocupe la mayor parte del tablero

    glider(&mut fb, 5, 5);
    lightweight_spaceship(&mut fb, 18, 5);
    middleweight_spaceship(&mut fb, 32, 5);
    heavyweight_spaceship(&mut fb, 46, 5);
    blinker(&mut fb, 63, 5);
    block(&mut fb, 75, 5);

    beacon(&mut fb, 5, 30);
    pulsar(&mut fb, 20, 30);
    toad(&mut fb, 40, 32);
    loaf(&mut fb, 55, 32);
    glider(&mut fb, 70, 30);
    middleweight_spaceship(&mut fb, 85, 28);

    lightweight_spaceship(&mut fb, 5, 60);
    pulsar(&mut fb, 20, 60);
    beacon(&mut fb, 40, 62);
    block(&mut fb, 55, 60);
    toad(&mut fb, 65, 62);
    loaf(&mut fb, 78, 62);
    blinker(&mut fb, 90, 60);

    while !rl.window_should_close() {
        fb.step();

        let mut d = rl.begin_drawing(&thread);
        fb.render(&mut d, CELL_SIZE);
        drop(d);

        thread::sleep(Duration::from_millis(DELAY_MS));
    }
}
