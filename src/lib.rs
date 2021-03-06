mod utils;

use std::f64;
use std::fmt::{Display, Formatter, Result, Write};

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use js_sys::Math::random;
use web_sys::CanvasRenderingContext2d;

const STEP: usize = 20;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Clone, Eq, PartialEq)]
pub enum Cell {
    Empty,
    Occupied,
}

#[wasm_bindgen]
pub struct Simulation {
    size: usize,
    cells: Vec<Vec<Cell>>,
    pub turn: usize,
}

fn coord_saturating_add(a: usize, b: usize, max_value: usize) -> usize {
    if a + b > max_value {
        max_value
    } else {
        a + b
    }
}

fn count_neighbors(cells: &[Vec<Cell>], x: usize, y: usize) -> usize {
    (x.saturating_sub(1)..=coord_saturating_add(x, 1, cells.len() - 1))
        .map(|i| {
            (y.saturating_sub(1)..=coord_saturating_add(y, 1, cells.len() - 1))
                .map(|j| {
                    if i == x && j == y {
                        0
                    } else {
                        match cells[i][j] {
                            Cell::Empty => 0,
                            Cell::Occupied => 1,
                        }
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

#[wasm_bindgen]
impl Simulation {
    fn new_with_size(size: usize) -> Self {
        let cells = vec![vec![Cell::Empty; size]; size];
        Simulation {
            size,
            cells,
            turn: 0,
        }
    }

    pub fn simulate(&mut self) {
        for i in 0..self.size {
            for j in 0..self.size {
                let neighbors = count_neighbors(&self.cells, i, j);
                self.cells[i][j] =
                    if (neighbors == 3) || (neighbors == 2 && self.cells[i][j] == Cell::Occupied) {
                        Cell::Occupied
                    } else {
                        Cell::Empty
                    }
            }
        }
        self.turn += 1;
    }

    pub fn next_turn(&mut self, context: &CanvasRenderingContext2d) {
        self.simulate();
        self.draw_cells(context);
    }

    pub fn draw_cells(&self, context: &CanvasRenderingContext2d) {
        for i in 0..self.size {
            for j in 0..self.size {
                if self.cells[i][j] == Cell::Occupied {
                    context.set_fill_style(&"rgb(64,64,64)".into());
                } else {
                    context.set_fill_style(&"rgb(255,255,255)".into());
                }
                context.fill_rect((STEP * i + 2) as f64, (STEP * j + 2) as f64, 18.0, 18.0);
            }
        }
    }

    pub fn draw_grid(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();

        let size = self.size;
        for x in 0..=size {
            for y in 0..=size {
                context.move_to((x * STEP + 1) as f64, 1_f64);
                context.line_to((x * STEP + 1) as f64, (1 + STEP * size) as f64);

                context.move_to(1_f64, (y * STEP + 1) as f64);
                context.line_to((1 + STEP * size) as f64, (y * STEP + 1) as f64);
            }
        }

        context.stroke();
    }

    pub fn fill_random(&mut self) {
        for i in 0..self.size {
            for j in 0..self.size {
                self.cells[i][j] = if random() * 10.0 > 4.0 {
                    Cell::Empty
                } else {
                    Cell::Occupied
                }
            }
        }
    }

    pub fn debug_grid(&self) {
        log(&format!("{}", self));
    }
}

impl Display for Simulation {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for i in 0..self.size {
            for j in 0..self.size {
                f.write_char(match self.cells[i][j] {
                    Cell::Empty => '_',
                    Cell::Occupied => 'X',
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[wasm_bindgen]
pub fn init() -> CanvasRenderingContext2d {
    set_panic_hook();

    let document = web_sys::window()
        .expect("window")
        .document()
        .expect("document");
    let canvas = document.get_element_by_id("canvas").expect("canvas");
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .expect("get canvas");

    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

#[wasm_bindgen]
pub fn create_simulation() -> Simulation {
    Simulation::new_with_size(15)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_neighbors() {
        let cells = vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Occupied, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty],
        ];
        assert_eq!(count_neighbors(&cells, 1, 1), 0);
        assert_eq!(count_neighbors(&cells, 0, 0), 1);
        assert_eq!(count_neighbors(&cells, 2, 2), 1);
    }

    #[test]
    fn test_coord_saturating_add() {
        assert_eq!(coord_saturating_add(2, 1, 2), 2);
        assert_eq!(coord_saturating_add(2, 1, 3), 3);
    }
}
