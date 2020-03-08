use js_sys::Function;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Document, HtmlCanvasElement, Window};

use crate::types::Ctx;

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

pub fn canvas_element() -> Option<HtmlCanvasElement> {
    let canvas = document()?.get_element_by_id(CANVAS_ID)?;
    let canvas = match canvas.dyn_into::<HtmlCanvasElement>() {
        Ok(c) => c,
        Err(_) => return None,
    };

    Some(canvas)
}

pub fn canvas_context() -> Option<Ctx> {
    let canvas = match canvas_element() {
        Some(c) => c,
        None => return None,
    };

    let ctx = match canvas.get_context("2d") {
        Ok(opt) => match opt {
            Some(c) => c,
            None => return None,
        },
        Err(_) => return None,
    };

    let ctx = match ctx.dyn_into::<Ctx>() {
        Ok(c) => c,
        Err(_) => return None,
    };

    Some(ctx)
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

// Add mouse event listener with callback to given canvas
pub fn add_mouse_event(
    event: &str,
    canvas: &HtmlCanvasElement,
    callback: &Function,
) -> Result<(), JsValue> {
    canvas.add_event_listener_with_callback(event, callback)?;
    Ok(())
}
