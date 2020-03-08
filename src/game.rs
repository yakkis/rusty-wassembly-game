use rand;

use wasm_bindgen::prelude::*;

use crate::{
    js::canvas_context,
    types::{Area, Colours, Ctx, Dimensions},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Running = 0,
    Paused = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

fn random_cell() -> Cell {
    if rand::random() {
        Cell::Alive
    } else {
        Cell::Dead
    }
}

#[derive(Clone)]
pub struct World {
    pub area: Area,
    dimensions: Dimensions,
    colours: Colours,
    cells: Vec<Cell>,
    dirty: bool,
    state: State,
    ctx: Ctx,
}

impl World {
    pub fn new(area: Area) -> Option<Self> {
        console_log!("Initializing the world!");

        let ctx = match canvas_context() {
            Some(c) => c,
            None => return None,
        };

        let dimensions = Dimensions {
            cells_x: 40,
            cells_y: 30,
            cell_w:  16.0,
            cell_h:  16.0,
        };

        let cells = (0..dimensions.cells_x * dimensions.cells_y).map(|_| random_cell()).collect();

        let colours = Colours {
            grid:  JsValue::from_str("#CCCCCC"),
            alive: JsValue::from_str("#555555"),
            dead:  JsValue::from_str("#FFFFFF"),
        };

        let state = State::Running;
        let dirty = true;

        Some(World {
            area,
            dimensions,
            colours,
            dirty,
            cells,
            state,
            ctx,
        })
    }

    pub fn draw_grid(&self) {
        // Game area offset on the canvas
        let x = self.area.x1;
        let y = self.area.y1;

        self.ctx.begin_path();
        self.ctx.set_stroke_style(&self.colours.grid);

        // Vertical lines
        for i in 0..=self.dimensions.cells_x {
            self.ctx.move_to((i as f64) * self.dimensions.cell_w + x, y);
            self.ctx.line_to((i as f64) * self.dimensions.cell_w + x, self.area.y2);
        }

        // Horizontal lines
        for j in 0..=self.dimensions.cells_y {
            self.ctx.move_to(x, (j as f64) * self.dimensions.cell_h + y);
            self.ctx.line_to(self.area.x2, (j as f64) * self.dimensions.cell_h + y);
        }

        self.ctx.stroke();
    }

    pub fn draw_cells(&self) {
        self.draw_cell_types(Cell::Alive);
        self.draw_cell_types(Cell::Dead)
    }

    pub fn toggle_state(&mut self) {
        self.state = match self.state {
            State::Running => State::Paused,
            State::Paused => State::Running,
        }
    }

    pub fn randomize_state(&mut self) {
        let cells: Vec<Cell> = (0..self.cells.len()).map(|_| random_cell()).collect();

        self.cells = cells;
        self.dirty = true;
    }

    pub fn update_state(&mut self) {
        if self.state == State::Paused {
            return;
        }

        let mut next_state = self.cells.clone();

        for row in 0..self.dimensions.cells_y {
            for col in 0..self.dimensions.cells_x {
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

    pub fn render(&mut self) {
        if !self.dirty {
            return;
        }

        self.draw_grid();
        self.draw_cells();

        self.dirty = false;
    }

    /*
     * Private functions
     */

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.dimensions.cells_x + col) as usize
    }

    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        let row_max = self.dimensions.cells_y - 1;
        let col_max = self.dimensions.cells_x - 1;

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

                let neighbour_row = (row + delta_row) % self.dimensions.cells_y;
                let neighbour_col = (col + delta_col) % self.dimensions.cells_x;
                let index = self.get_index(neighbour_row, neighbour_col);

                count += self.cells[index] as u8;
            }
        }

        count
    }

    fn draw_cell_types(&self, cell_type: Cell) {
        let x = self.area.x1;
        let y = self.area.y1;

        let colour = match cell_type {
            Cell::Dead => &self.colours.dead,
            Cell::Alive => &self.colours.alive,
        };

        self.ctx.set_fill_style(colour);

        for row in 0..self.dimensions.cells_y {
            for col in 0..self.dimensions.cells_x {
                let index = self.get_index(row, col);
                if self.cells[index] != cell_type {
                    continue;
                }

                self.ctx.fill_rect(
                    (col as f64) * self.dimensions.cell_w + 1.0 + x,
                    (row as f64) * self.dimensions.cell_h + 1.0 + y,
                    self.dimensions.cell_w - 2.0,
                    self.dimensions.cell_h - 2.0,
                );
            }
        }
    }
}
