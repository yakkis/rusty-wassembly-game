use wasm_bindgen::prelude::*;

#[derive(Clone, Debug)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
    pub cell: f64,
}

#[derive(Clone, Debug)]
pub struct Colours {
    pub grid: JsValue,
    pub alive: JsValue,
    pub dead: JsValue,
}
