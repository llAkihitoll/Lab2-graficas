use crate::framebuffer::Framebuffer;

// pinta un patron a partir de filas de texto ('#' = viva, cualquier otro
// caracter = muerta). (x, y) es la esquina superior izquierda del patron.
fn pintar_patron(fb: &mut Framebuffer, x: i32, y: i32, filas: &[&str]) {
    for (dy, fila) in filas.iter().enumerate() {
        for (dx, c) in fila.chars().enumerate() {
            if c == '#' {
                fb.set(x + dx as i32, y + dy as i32, true);
            }
        }
    }
}

// still lifes

pub fn block(fb: &mut Framebuffer, x: i32, y: i32) {
    pintar_patron(fb, x, y, &["##", "##"]);
}

pub fn loaf(fb: &mut Framebuffer, x: i32, y: i32) {
    pintar_patron(fb, x, y, &[".##.", "#..#", ".#.#", "..#."]);
}

// osciladores

pub fn blinker(fb: &mut Framebuffer, x: i32, y: i32) {
    pintar_patron(fb, x, y, &["###"]);
}

pub fn toad(fb: &mut Framebuffer, x: i32, y: i32) {
    pintar_patron(fb, x, y, &[".###", "###."]);
}

pub fn beacon(fb: &mut Framebuffer, x: i32, y: i32) {
    pintar_patron(fb, x, y, &["##..", "##..", "..##", "..##"]);
}

pub fn pulsar(fb: &mut Framebuffer, x: i32, y: i32) {
    pintar_patron(
        fb,
        x,
        y,
        &[
            "..###...###..",
            ".............",
            "#....#.#....#",
            "#....#.#....#",
            "#....#.#....#",
            "..###...###..",
            ".............",
            "..###...###..",
            "#....#.#....#",
            "#....#.#....#",
            "#....#.#....#",
            ".............",
            "..###...###..",
        ],
    );
}

// naves (spaceships)

pub fn glider(fb: &mut Framebuffer, x: i32, y: i32) {
    pintar_patron(fb, x, y, &[".#.", "..#", "###"]);
}

pub fn lightweight_spaceship(fb: &mut Framebuffer, x: i32, y: i32) {
    pintar_patron(fb, x, y, &[".####", "#...#", "....#", "#..#."]);
}

pub fn middleweight_spaceship(fb: &mut Framebuffer, x: i32, y: i32) {
    pintar_patron(
        fb,
        x,
        y,
        &["..#...", "#...#.", ".....#", "#....#", ".#####"],
    );
}

pub fn heavyweight_spaceship(fb: &mut Framebuffer, x: i32, y: i32) {
    pintar_patron(
        fb,
        x,
        y,
        &["..##...", "#....#.", "......#", "#.....#", ".######"],
    );
}
