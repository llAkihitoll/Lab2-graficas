use raylib::prelude::*;
use std::thread;
use std::time::Duration;

mod framebuffer;
mod organisms;

use framebuffer::Framebuffer;
use organisms::{
    beacon, block, blinker, glider, heavyweight_spaceship, lightweight_spaceship, loaf,
    middleweight_spaceship, pulsar, toad,
};

// el tablero es mas grande que la ventana inicial: al agrandar la ventana
// (es redimensionable) se ve mas tablero y aparecen mas organismos
const GRID_WIDTH: i32 = 200;
const GRID_HEIGHT: i32 = 200;
const CELL_SIZE: i32 = 8;
const DELAY_MS: u64 = 100;
const INITIAL_WINDOW_WIDTH: i32 = 800;
const INITIAL_WINDOW_HEIGHT: i32 = 800;

// coloca la franja de organismos clasicos con esquina superior izquierda
// en (ox, oy). Se reutiliza para llenar cada cuadrante del tablero.
fn poblar_franja(fb: &mut Framebuffer, ox: i32, oy: i32) {
    glider(fb, ox + 5, oy + 5);
    lightweight_spaceship(fb, ox + 18, oy + 5);
    middleweight_spaceship(fb, ox + 32, oy + 5);
    heavyweight_spaceship(fb, ox + 46, oy + 5);
    blinker(fb, ox + 63, oy + 5);
    block(fb, ox + 75, oy + 5);

    beacon(fb, ox + 5, oy + 30);
    pulsar(fb, ox + 20, oy + 30);
    toad(fb, ox + 40, oy + 32);
    loaf(fb, ox + 55, oy + 32);
    glider(fb, ox + 70, oy + 30);
    middleweight_spaceship(fb, ox + 85, oy + 28);

    lightweight_spaceship(fb, ox + 5, oy + 60);
    pulsar(fb, ox + 20, oy + 60);
    beacon(fb, ox + 40, oy + 62);
    block(fb, ox + 55, oy + 60);
    toad(fb, ox + 65, oy + 62);
    loaf(fb, ox + 78, oy + 62);
    blinker(fb, ox + 90, oy + 60);
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT)
        .title("Conway's Game of Life")
        .resizable()
        .build();

    let mut fb = Framebuffer::new(GRID_WIDTH, GRID_HEIGHT);

    // una franja de organismos por cuadrante, para llenar los 200x200
    poblar_franja(&mut fb, 0, 0);
    poblar_franja(&mut fb, 100, 0);
    poblar_franja(&mut fb, 0, 100);
    poblar_franja(&mut fb, 100, 100);

    while !rl.window_should_close() {
        fb.step();

        // cuantas celdas caben en la ventana ahora mismo (cambia si el
        // usuario la redimensiono), sin pasarse del tamano del tablero
        let visible_cols = (rl.get_screen_width() / CELL_SIZE).min(GRID_WIDTH);
        let visible_rows = (rl.get_screen_height() / CELL_SIZE).min(GRID_HEIGHT);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        fb.render(&mut d, CELL_SIZE, visible_cols, visible_rows);
        drop(d);

        thread::sleep(Duration::from_millis(DELAY_MS));
    }
}
