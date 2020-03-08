use crate::buttons::Button;
use crate::js::canvas_context;
use crate::types::{Area, Ctx};

#[derive(Clone, Debug)]
pub struct Interface<'a> {
    pub area: Area,
    buttons: Vec<Button<'a>>,
    ctx: Ctx,
}

impl<'a> Interface<'a> {
    pub fn new(area: Area) -> Option<Self> {
        let ctx = match canvas_context() {
            Some(c) => c,
            None => return None,
        };

        /*
         * Initialize UI buttons
         */

        // TODO: Add callback to button

        // Button for toggling the game state (running/paused)
        let x = ((area.w as f64) / 2.0) - 50.0;
        let y = ((area.h as f64) / 2.0) - 20.0;
        let ba = Area::new(x, y, 100.0, 32.0);
        let toggle_state_button = match Button::new(ba, "Toggle") {
            Some(b) => b,
            None => return None,
        };

        let buttons: Vec<Button> = vec![toggle_state_button];

        Some(Interface { area, buttons, ctx })
    }

    pub fn mouse_move(&mut self, x: f64, y: f64) {
        if !self.area.in_bounds(x, y) {
            return;
        }

        for btn in self.buttons.iter_mut() {
            if btn.area.in_bounds(x, y) {
                btn.set_hover(true);
                break;
            } else {
                btn.set_hover(false);
            }
        }
    }

    // pub fn mouse_click(&mut self, x: f64, y: f64) {}

    pub fn render(&mut self) {
        for btn in self.buttons.iter_mut() {
            btn.render();
        }
    }
}
