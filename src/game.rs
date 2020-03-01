use rand;
use web_sys::CanvasRenderingContext2d;

use crate::types::{Colours, Dimensions};
use crate::utils::Timer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[derive(Clone)]
pub struct World {
    dimensions: Dimensions,
    colours: Colours,
    cells: Vec<Cell>,
    dirty: bool,
    context: CanvasRenderingContext2d,
}

impl World {
    pub fn new(
        dimensions: Dimensions,
        colours: Colours,
        context: CanvasRenderingContext2d,
    ) -> World {
        console_log!("Initializing the world!");

        let cells = (0..dimensions.width * dimensions.height)
            .map(|_| {
                if rand::random() {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        World {
            dimensions,
            colours,
            dirty: true,
            cells,
            context,
        }
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.dimensions.width + col) as usize
    }

    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        let row_max = self.dimensions.height - 1;
        let col_max = self.dimensions.width - 1;

        for (i, &delta_row) in [row_max, 0, 1].iter().enumerate() {
            for (j, &delta_col) in [col_max, 0, 1].iter().enumerate() {
                // Skip the cell itself
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                // Out of bound checks
                if (row == 0 && i == 0)
                    || (row == row_max && i == 2)
                    || (col == 0 && j == 0)
                    || (col == col_max && j == 2)
                {
                    continue;
                }

                let neighbour_row = (row + delta_row) % self.dimensions.height;
                let neighbour_col = (col + delta_col) % self.dimensions.width;
                let index = self.get_index(neighbour_row, neighbour_col);

                count += self.cells[index] as u8;
            }
        }

        count
    }

    pub fn update_state(&mut self) {
        let _timer = Timer::new("State update");
        let mut next_state = self.cells.clone();

        for row in 0..self.dimensions.height {
            for col in 0..self.dimensions.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let live_neighbours = self.live_neighbour_count(row, col);

                let next_cell = match (cell, live_neighbours) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next_state[index] = next_cell;
            }
        }

        self.cells = next_state;
        self.dirty = true;
    }

    pub fn draw_grid(&self) {
        self.context.set_stroke_style(&self.colours.grid);

        // Vertical lines
        for i in 0..=self.dimensions.width {
            self.context.move_to((i as f64) * self.dimensions.cell, 0.0);
            self.context.line_to(
                (i as f64) * self.dimensions.cell,
                self.dimensions.cell * (self.dimensions.height as f64),
            );
        }

        // Horizontal lines
        for j in 0..=self.dimensions.height {
            self.context.move_to(0.0, (j as f64) * self.dimensions.cell);
            self.context.line_to(
                self.dimensions.cell * (self.dimensions.width as f64),
                (j as f64) * self.dimensions.cell,
            );
        }
    }

    pub fn draw_cells(&self) {
        self.draw_cell_types(Cell::Alive);
        self.draw_cell_types(Cell::Dead)
    }

    fn draw_cell_types(&self, cell_type: Cell) {
        let colour = match cell_type {
            Cell::Dead => &self.colours.dead,
            Cell::Alive => &self.colours.alive,
        };

        self.context.set_fill_style(colour);

        for row in 0..self.dimensions.height {
            for col in 0..self.dimensions.width {
                let index = self.get_index(row, col);
                if self.cells[index] != cell_type {
                    continue;
                }

                self.context.fill_rect(
                    (col as f64) * self.dimensions.cell + 1.0,
                    (row as f64) * self.dimensions.cell + 1.0,
                    self.dimensions.cell - 2.0,
                    self.dimensions.cell - 2.0,
                );
            }
        }
    }

    pub fn render(&mut self) {
        if self.dirty {
            let _timer = Timer::new("Render");

            self.context.begin_path();
            self.draw_grid();
            self.draw_cells();
            self.context.stroke();

            self.dirty = false;
        }
    }
}
