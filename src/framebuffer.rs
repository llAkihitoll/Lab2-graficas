use raylib::prelude::*;

pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    cells: Vec<bool>,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        Framebuffer {
            width,
            height,
            cells: vec![false; (width * height) as usize],
        }
    }

    // convierte x,y a un indice del vector, con wrap toroidal
    fn index(&self, x: i32, y: i32) -> usize {
        let x = x.rem_euclid(self.width);
        let y = y.rem_euclid(self.height);
        (y * self.width + x) as usize
    }

    pub fn get(&self, x: i32, y: i32) -> bool {
        self.cells[self.index(x, y)]
    }

    pub fn set(&mut self, x: i32, y: i32, alive: bool) {
        let i = self.index(x, y);
        self.cells[i] = alive;
    }

    pub fn get_color(&self, x: i32, y: i32) -> Color {
        if self.get(x, y) {
            Color::WHITE
        } else {
            Color::BLACK
        }
    }

    // cuenta las 8 celdas vecinas, con wrap toroidal (get ya lo hace)
    pub fn count_neighbors(&self, x: i32, y: i32) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if self.get(x + dx, y + dy) {
                    count += 1;
                }
            }
        }
        count
    }

    // calcula toda la siguiente generacion en un vector nuevo y recien al
    // final reemplaza self.cells, para no contar vecinos sobre celdas que
    // ya cambiaron en esta misma pasada
    pub fn step(&mut self) {
        let mut new_cells = vec![false; self.cells.len()];

        for y in 0..self.height {
            for x in 0..self.width {
                let vecinos = self.count_neighbors(x, y);
                let viva = self.get(x, y);

                let siguiente = match (viva, vecinos) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };

                let i = self.index(x, y);
                new_cells[i] = siguiente;
            }
        }

        self.cells = new_cells;
    }

    // dibuja la parte de la grilla que entra en la ventana actual, celda
    // por celda, usando point() como unica primitiva de dibujo.
    // visible_cols/visible_rows es cuantas celdas caben en la ventana en
    // este momento (puede ser menor que width/height si la ventana esta
    // chica, o toda la grilla si la ventana es igual o mas grande)
    pub fn render(&self, d: &mut RaylibDrawHandle, cell_size: i32, visible_cols: i32, visible_rows: i32) {
        for y in 0..visible_rows.min(self.height) {
            for x in 0..visible_cols.min(self.width) {
                let color = self.get_color(x, y);
                for dy in 0..cell_size {
                    for dx in 0..cell_size {
                        point(d, x * cell_size + dx, y * cell_size + dy, color);
                    }
                }
            }
        }
    }
}

pub fn point(d: &mut RaylibDrawHandle, x: i32, y: i32, color: Color) {
    d.draw_pixel(x, y, color);
}
