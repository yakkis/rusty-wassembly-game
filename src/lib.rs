#[macro_use]
mod macros;
mod game;
mod html;
mod types;
mod utils;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

use html::{create_canvas, request_animation_frame, set_timeout};
use types::{Colours, Dimensions};
use utils::{abort, set_panic_hook};

// TODO: The current division into state update and render closures doesn't
// make sense when rendering can occur only after a state update. This is just a
// demo code, implement a better solution later.

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    set_panic_hook();

    /*
     * Set up variables
     */

    let colours = Colours {
        grid: JsValue::from_str("#CCCCCC"),
        alive: JsValue::from_str("#555555"),
        dead: JsValue::from_str("#FFFFFF"),
    };

    let dimensions = Dimensions {
        width: 40,
        height: 30,
        cell: 16.0,
    };

    let (_canvas, context) = create_canvas().ok_or("Cannot access 'canvas'")?;

    let game = game::World::new(dimensions, colours, context);
    let game = Rc::new(RefCell::new(game));
    let game_clone_1 = Rc::clone(&game);
    let game_clone_2 = Rc::clone(&game);

    /*
     * Set up state update loop
     */

    let update = Rc::new(RefCell::new(None));
    let update_clone = Rc::clone(&update);

    *update_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        game_clone_1.borrow_mut().update_state();

        let temp = update.borrow();
        let temp = match temp.as_ref() {
            Some(u) => u,
            None => return abort("Failed setting timeout"),
        };

        if set_timeout(temp, 1000).is_err() {
            abort("Failed setting timeout");
        }
    }) as Box<dyn FnMut()>));

    let temp = update_clone.borrow();
    let temp = match temp.as_ref() {
        Some(u) => u,
        None => return Err(JsValue::from_str("Failed setting timeout")),
    };

    if set_timeout(temp, 1000).is_err() {
        return Err(JsValue::from_str("Failed setting timeout"));
    }

    /*
     * Set up render loop
     */

    let render = Rc::new(RefCell::new(None));
    let render_clone = Rc::clone(&render);

    *render_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        game_clone_2.borrow_mut().render();

        let temp = render.borrow();
        let temp = match temp.as_ref() {
            Some(r) => r,
            None => return abort("Failed request animation frame"),
        };

        if request_animation_frame(temp).is_err() {
            abort("Failed request animation frame");
        }
    }) as Box<dyn FnMut()>));

    let temp = render_clone.borrow();
    let temp = match temp.as_ref() {
        Some(u) => u,
        None => return Err(JsValue::from_str("Failed request animation frame")),
    };

    if request_animation_frame(temp).is_err() {
        return Err(JsValue::from_str("Failed request animation frame"));
    }

    Ok(())
}
