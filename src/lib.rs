mod utils;

#[cfg(test)]
mod test;

use wasm_bindgen::prelude::*;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        return (row * self.width + column) as usize;
    }

    fn get_position(&self, index: u32) -> (u32, u32) {
        let row = index / self.width;
        let column = index - (row * self.width);
        return (row, column);
    }

    fn count_live_neighbours(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1].iter() {
            for delta_column in [self.width - 1, 0, 1].iter() {
                let delta_row = *delta_row;
                let delta_column = *delta_column;
                if delta_column == 0 && delta_row == 0 {
                    continue;
                }
                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (column + delta_column) % self.width;
                count += self.cells[self.get_index(neighbour_row, neighbour_col)] as u8;
            }
        }

        return count;
    }

    pub fn set_live_cells(&mut self, cells: &Vec<(u32, u32)>) -> () {
        for (cell_row, cell_column) in cells {
            let indx = self.get_index(*cell_row, *cell_column);
            self.cells[indx] = Cell::Alive;
        }
    }

    pub fn get_live_cells(&self) -> Vec<(u32, u32)> {
        let mut live_cells = Vec::new();
        for (index, cell) in self.cells.iter().enumerate() {
            if *cell == Cell::Alive {
                let position = self.get_position(index as u32);
                live_cells.push(position);
            }
        }
        return live_cells;
    }

    pub fn reset(&mut self) {
        self.cells = self.cells.iter().map(|_| Cell::Dead).collect();
    }

    pub fn cells(&self) -> &Vec<Cell> {
        return &self.cells;
    }
}

#[wasm_bindgen]
impl Universe {

    pub fn new(width: u32, height: u32) -> Universe {

        let cells: Vec<Cell> = vec![Cell::Dead; width as usize * height as usize];

        return Universe {
            width: width,
            height: height,
            cells: cells
        };
    }

    pub fn width(&self) -> u32 {
        return self.width;
    }

    pub fn height(&self) -> u32 {
        return self.height;
    }

    pub fn cells_pointer(&self) -> *const Cell {
        return self.cells.as_ptr();
    }

    pub fn my_tick(&mut self) {
        let mut new_cells: Vec<Cell> = Vec::with_capacity(self.cells.len());
        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.get_index(row, column);
                let live_neighbours = self.count_live_neighbours(row, column);
                let cell = self.cells[index];

                let new_cell = match (cell, live_neighbours) {
                    // if less then 2 neighbours, dies
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // if 2 or 3 neighbours, lives
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // more then 3, dies
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // exactly 3, cell is borned
                    (Cell::Dead, 3) => Cell::Alive,

                    // rest
                    (current_state, _) => current_state
                };

                new_cells[index] = new_cell;
            }
        }
        self.cells = new_cells;
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.count_live_neighbours(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn render(&self) -> () {
        print!("\n\n - ");
        for col in 0..self.width {
            print!(" {} ", col);
        }
        print!("\n");
        for row in 0..self.height {
            print!(" {} ", row);
            for col in 0..self.width {
                let indx = self.get_index(row, col);
                print!(" {} ", if self.cells[indx] == Cell::Dead { '◻' } else { '◼' });
            }
            print!("\n");
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}