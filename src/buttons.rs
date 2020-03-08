use wasm_bindgen::prelude::*;

use crate::js::canvas_context;
use crate::shapes::rounded_rect;
use crate::types::{Area, Ctx};
use crate::ui::ButtonType;

#[derive(Clone, Debug)]
pub struct Button<'a> {
    pub area: Area,
    pub button_type: ButtonType,
    text: &'a str,
    hover: bool,
    dirty: bool,
    ctx: Ctx,
}

// TODO: Adjust the button area to also contain highlight

impl<'a> Button<'a> {
    pub fn new(button_type: ButtonType, area: Area, text: &'a str) -> Option<Self> {
        let ctx = match canvas_context() {
            Some(c) => c,
            None => return None,
        };

        ctx.set_font("16px Verdana");
        ctx.set_text_align("center");

        Some(Button { button_type, area, text, ctx, hover: false, dirty: true })
    }

    pub fn set_hover(&mut self, state: bool) {
        if self.hover != state {
            self.hover = state;
            self.dirty = true;
        }
    }

    pub fn click(&self) -> ButtonType {
        self.button_type.clone()
    }

    pub fn render(&mut self) {
        if !self.dirty {
            return;
        }

        let area = &self.area;
        let center = self.area.center();

        // TODO: Create these elsewhere
        let white = JsValue::from_str("#FFF");
        let black = JsValue::from_str("#000");
        let grey_light = JsValue::from_str("#DDD");
        let grey_dark = JsValue::from_str("#555");

        // Clear the area
        self.ctx.set_fill_style(&white);
        self.ctx.fill_rect(area.x1 - 1.0, area.y1 - 1.0, area.w + 5.0, area.h + 5.0);

        // Draw the hover shadow
        if self.hover {
            self.ctx.begin_path();
            self.ctx.set_fill_style(&grey_light);
            rounded_rect(&self.ctx, area.x1 + 4.0, area.y1 + 4.0, area.w, area.h, 5.0);
            self.ctx.fill();
        }

        // Fill the button area
        self.ctx.begin_path();
        self.ctx.set_fill_style(&white);
        rounded_rect(&self.ctx, area.x1, area.y1, area.w, area.h, 5.0);
        self.ctx.fill();

        // Draw the button borders
        self.ctx.begin_path();
        self.ctx.set_stroke_style(&grey_dark);
        self.ctx.set_line_width(2.0);
        rounded_rect(&self.ctx, area.x1, area.y1, area.w, area.h, 5.0);
        self.ctx.stroke();

        // Add button text
        self.ctx.set_fill_style(&grey_dark);
        // TODO: What happens if fill_text returns an error?
        let _ = self.ctx.fill_text(self.text, center.x, center.y + 4.0);

        self.dirty = false;
    }

    // fn draw_highligth() {}
}
