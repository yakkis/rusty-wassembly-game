use web_sys::{CanvasRenderingContext2d, MouseEvent};

use wasm_bindgen::prelude::*;

// A shorter alias type for canvas context
pub type Ctx = CanvasRenderingContext2d;

#[derive(Clone, Debug)]
pub struct Dimensions {
    pub cells_x: u32,
    pub cells_y: u32,
    pub cell_w:  f64,
    pub cell_h:  f64,
}

#[derive(Clone, Debug)]
pub struct Colours {
    pub grid:  JsValue,
    pub alive: JsValue,
    pub dead:  JsValue,
}

#[derive(Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl From<MouseEvent> for Point {
    fn from(event: MouseEvent) -> Self {
        Point {
            x: (event.offset_x() as f64),
            y: (event.offset_y() as f64),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Area {
    // Top left coordinate
    pub x1: f64,
    pub y1: f64,
    // Bottom right coordinate
    pub x2: f64,
    pub y2: f64,
    // Center coordinates
    pub cx: f64,
    pub cy: f64,
    // Width and height
    pub w:  f64,
    pub h:  f64,
}

impl Area {
    #[rustfmt::skip]
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        let x2 = x + w;
        let y2 = y + h;
        let cx = x + (w / 2.0);
        let cy = y + (h / 2.0);

        Area { x1: x, y1: y, x2, y2, cx, cy, w, h }
    }

    pub fn in_bounds(&self, point: &Point) -> bool {
        point.x >= self.x1 && point.x <= self.x2 && point.y >= self.y1 && point.y <= self.y2
    }

    pub fn center(&self) -> Point {
        Point {
            x: self.cx,
            y: self.cy,
        }
    }
}
