use crate::{
    buttons::Button,
    js::canvas_context,
    types::{Area, Ctx, Point},
};

#[derive(Clone, Debug)]
pub enum ButtonType {
    ToggleState,
    RandomizeState,
    None,
}

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

        let center = area.center();

        // Button for toggling the game state (running/paused)
        let bt = ButtonType::ToggleState;
        let ba = Area::new(center.x - 110.0, center.y - 16.0, 100.0, 32.0);
        let toggle_state_button = match Button::new(bt, ba, "Toggle") {
            Some(b) => b,
            None => return None,
        };

        // Button for randomizing the game state
        let bt = ButtonType::RandomizeState;
        let ba = Area::new(center.x + 10.0, center.y - 16.0, 100.0, 32.0);
        let randomize_button = match Button::new(bt, ba, "Randomize") {
            Some(b) => b,
            None => return None,
        };

        let buttons: Vec<Button> = vec![toggle_state_button, randomize_button];

        Some(Interface {
            area,
            buttons,
            ctx,
        })
    }

    pub fn mouse_move(&mut self, point: &Point) {
        if !self.area.in_bounds(point) {
            return;
        }

        for btn in self.buttons.iter_mut() {
            if btn.area.in_bounds(point) {
                btn.set_hover(true);
                break;
            } else {
                btn.set_hover(false);
            }
        }
    }

    pub fn mouse_click(&mut self, point: &Point) -> ButtonType {
        if !self.area.in_bounds(point) {
            return ButtonType::None;
        }

        for btn in self.buttons.iter_mut() {
            if btn.area.in_bounds(point) {
                return btn.click();
            }
        }

        ButtonType::None
    }

    pub fn render(&mut self) {
        for btn in self.buttons.iter_mut() {
            btn.render();
        }
    }
}
