mod utils;

use std::convert::TryFrom;
use std::f64;
use std::fmt::{Display, Formatter, Result, Write};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use js_sys::Math::random;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

const STEP: usize = 20;

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

    fn count_neighbors(&self, i: usize, j: usize) -> usize {
        let coords: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        coords
            .iter()
            .filter(|(dx, dy)| {
                if (0 > dx + i as i32) || (dx + i as i32 >= self.size as i32) {
                    false
                } else if (0 > dy + j as i32) || (dy + j as i32 >= self.size as i32) {
                    false
                } else {
                    self.cells[usize::try_from(dx + i as i32).unwrap()]
                        [usize::try_from(dy + j as i32).unwrap()]
                        == Cell::Occupied
                }
            })
            .count()
    }

    pub fn simulate(&mut self) {
        for i in 0..self.size {
            for j in 0..self.size {
                let neighbors = self.count_neighbors(i, j);
                self.cells[i][j] = if neighbors == 3 {
                    Cell::Occupied
                } else if neighbors == 2 && self.cells[i][j] == Cell::Occupied {
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
                context.move_to((x * STEP + 1) as f64, 1 as f64);
                context.line_to((x * STEP + 1) as f64, (1 + STEP * size) as f64);

                context.move_to(1 as f64, (y * STEP + 1) as f64);
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

    pub fn to_string(&self) -> String {
        format!("{}", &self)
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
/*
    simulation.draw_grid(&context);
    // simulation.next_turn();
    simulation.fill_random();
    simulation.draw_cells(&context);
}
*/

pub async fn sleep(ms: i32) -> JsFuture {
    wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    }))
}
