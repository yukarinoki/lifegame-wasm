mod utils;
use rand::prelude::*;
use wasm_bindgen::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
extern crate web_sys;
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}


impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Universe{
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count (&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height-1, 0, 1].iter().cloned() {
            for delta_column in [self. width-1, 0, 1].iter().cloned(){
                if delta_column == 0 && delta_row == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_column = (column + delta_column) % self.width;
                count += self.cells[self.get_index(neighbor_row, neighbor_column)] as u8;
            }
        }
        count
    }
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }
    pub fn set_glider(&mut self, row: u32, column: u32) {
        // □□□□□
        // □□■□□
        // □□□■□
        // □■■■□
        // □□□□□
        const GLIDER: [Cell; 9] = [Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Alive, Cell::Alive, Cell::Alive, Cell::Alive];
        for delta_row in [0, 1, 2].iter().cloned() {
            for delta_column in [0, 1, 2].iter().cloned(){
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_column = (column + delta_column) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_column);
                self.cells[idx] = GLIDER[(3 * delta_row + delta_column) as usize];
            }
        }
    }

    pub fn set_pulser(&mut self, row: u32, column: u32){
        // □□□□□□□
        // □■□□□■□
        // □■□■□■□
        // □■□□□■□
        // □□□□□□□
        const PULSER: [Cell; 15] = [Cell::Alive, Cell::Dead, Cell::Dead, Cell::Dead,Cell::Alive,Cell::Alive, Cell::Dead, Cell::Alive, Cell::Dead,Cell::Alive,Cell::Alive, Cell::Dead, Cell::Dead, Cell::Dead,Cell::Alive];
        for delta_row in [0, 1, 2].iter().cloned() {
            for delta_column in [0, 1, 2, 3, 4].iter().cloned(){
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_column = (column + delta_column) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_column);
                self.cells[idx] = PULSER[(5 * delta_row + delta_column) as usize];
            }
        }
    }

    pub fn set_random_cells(&mut self){
        self.cells = (0..self.width * self.height)
                    .map(|i| {
                        if random() {
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    })
                    .collect();
    }
    pub fn set_initial_cells(&mut self){
        self.cells = (0..self.width * self.height)
                    .map(|i| {
                        if i % 2 == 0 || i % 7 == 0 {
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    })
                    .collect();
    }

    pub fn set_all_dead_cells(&mut self){
        self.cells = (0..self.width * self.height).map(|_i| Cell::Dead).collect();
    }
}

impl Universe {
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}