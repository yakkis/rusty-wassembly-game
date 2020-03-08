#[macro_use]
mod macros;

mod buttons;
mod game;
mod js;
mod shapes;
mod types;
mod ui;
mod utils;

use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::MouseEvent;

use game::World;
use types::{Area, Point};
use ui::{ButtonType, Interface};
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

    let canvas = js::canvas_element().ok_or("Cannot access canvas")?;

    let ui_area = Area::new(0.0, 0.0, 640.0, 75.0);
    let ui = Interface::new(ui_area).ok_or("Cannot initialize UI")?;
    let ui = Rc::new(RefCell::new(ui));
    let ui_clone_1 = Rc::clone(&ui);
    let ui_clone_2 = Rc::clone(&ui);
    let ui_clone_3 = Rc::clone(&ui);

    let game_area = Area::new(0.0, 75.0, 640.0, 480.0);
    let game = World::new(game_area).ok_or("Cannot initialize game")?;
    let game = Rc::new(RefCell::new(game));
    let game_clone_1 = Rc::clone(&game);
    let game_clone_2 = Rc::clone(&game);
    let game_clone_3 = Rc::clone(&game);

    /*
     * Set up event listeners
     */

    {
        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            let point = Point::from(event);
            ui_clone_1.borrow_mut().mouse_move(&point);
        }) as Box<dyn FnMut(_)>);

        let casted = closure.as_ref().unchecked_ref();
        if js::add_mouse_event("mousemove", &canvas, casted).is_err() {
            return Err(JsValue::from_str("Failed setting mouse event listener"));
        }
        closure.forget();
    }

    {
        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            let point = Point::from(event);

            match ui_clone_2.borrow_mut().mouse_click(&point) {
                ButtonType::ToggleState => {
                    game_clone_1.borrow_mut().toggle_state();
                }
                ButtonType::RandomizeState => {
                    game_clone_1.borrow_mut().randomize_state();
                }
                _ => {}
            }
        }) as Box<dyn FnMut(_)>);

        let casted = closure.as_ref().unchecked_ref();
        if js::add_mouse_event("click", &canvas, casted).is_err() {
            return Err(JsValue::from_str("Failed setting mouse event listener"));
        }
        closure.forget();
    }

    /*
     * Set up state update loop
     */

    let update = Rc::new(RefCell::new(None));
    let update_clone = Rc::clone(&update);

    *update_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        game_clone_2.borrow_mut().update_state();

        let temp = update.borrow();
        let temp = match temp.as_ref() {
            Some(u) => u,
            None => return abort("Failed setting timeout"),
        };

        if js::set_timeout(temp, 1000).is_err() {
            abort("Failed setting timeout");
        }
    }) as Box<dyn FnMut()>));

    let temp = update_clone.borrow();
    let temp = match temp.as_ref() {
        Some(u) => u,
        None => return Err(JsValue::from_str("Failed setting timeout")),
    };

    if js::set_timeout(temp, 1000).is_err() {
        return Err(JsValue::from_str("Failed setting timeout"));
    }

    /*
     * Set up render loop
     */

    let render = Rc::new(RefCell::new(None));
    let render_clone = Rc::clone(&render);

    *render_clone.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        game_clone_3.borrow_mut().render();
        ui_clone_3.borrow_mut().render();

        let temp = render.borrow();
        let temp = match temp.as_ref() {
            Some(r) => r,
            None => return abort("Failed request animation frame"),
        };

        if js::request_animation_frame(temp).is_err() {
            abort("Failed request animation frame");
        }
    }) as Box<dyn FnMut()>));

    let temp = render_clone.borrow();
    let temp = match temp.as_ref() {
        Some(u) => u,
        None => return Err(JsValue::from_str("Failed request animation frame")),
    };

    if js::request_animation_frame(temp).is_err() {
        return Err(JsValue::from_str("Failed request animation frame"));
    }

    Ok(())
}
