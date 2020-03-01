use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};

/*
* HTML/JavaScript helper functions
*/

pub const CANVAS_ID: &str = "rusty-wassembly-game";

pub fn window() -> Option<Window> {
    web_sys::window()
}

pub fn document() -> Option<Document> {
    web_sys::window()?.document()
}

pub fn create_canvas() -> Option<(HtmlCanvasElement, CanvasRenderingContext2d)> {
    let canvas = document()?.get_element_by_id(CANVAS_ID)?;
    let canvas = match canvas.dyn_into::<HtmlCanvasElement>() {
        Ok(c) => c,
        Err(_) => return None,
    };

    let context = match canvas.get_context("2d") {
        Ok(opt) => match opt {
            Some(ctx) => ctx,
            None => return None,
        },
        Err(_) => return None,
    };

    let context = match context.dyn_into::<CanvasRenderingContext2d>() {
        Ok(ctx) => ctx,
        Err(_) => return None,
    };

    Some((canvas, context))
}

pub fn request_animation_frame(func: &Closure<dyn FnMut()>) -> Result<i32, &str> {
    let animation_frame_id = window()
        .ok_or("Cannot access window")?
        .request_animation_frame(func.as_ref().unchecked_ref())
        .or(Err("Error requesting animation frame"))?;

    Ok(animation_frame_id)
}

pub fn set_timeout(func: &Closure<dyn FnMut()>, timeout: i32) -> Result<i32, &str> {
    let set_timeout_id = window()
        .ok_or("Cannot access window")?
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            func.as_ref().unchecked_ref(),
            timeout,
        )
        .or(Err("Error setting timeout"))?;

    Ok(set_timeout_id)
}
