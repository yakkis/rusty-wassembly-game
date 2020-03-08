use wasm_bindgen::prelude::*;

use crate::js::canvas_context;
use crate::shapes::rounded_rect;
use crate::types::{Area, Ctx};

#[derive(Clone, Debug)]
pub struct Button<'a> {
    pub area: Area,
    text: &'a str,
    hover: bool,
    dirty: bool,
    ctx: Ctx,
}

// TODO: Adjust the button area to also contain highlight

impl<'a> Button<'a> {
    pub fn new(area: Area, text: &'a str) -> Option<Self> {
        let ctx = match canvas_context() {
            Some(c) => c,
            None => return None,
        };

        ctx.set_font("16px Verdana");
        ctx.set_text_align("center");

        Some(Button { area, text, ctx, hover: false, dirty: true })
    }

    pub fn set_hover(&mut self, state: bool) {
        if self.hover != state {
            self.hover = state;
            self.dirty = true;
        }
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
        let grey_dark = JsValue::from_str("#444");

        // Clear the area
        self.ctx.set_fill_style(&white);
        self.ctx.fill_rect(area.x1, area.y1, area.w + 3.0, area.h + 3.0);

        // Draw the hover shadow
        if self.hover {
            self.ctx.begin_path();
            self.ctx.set_fill_style(&grey_light);
            rounded_rect(&self.ctx, area.x1 + 3.0, area.y1 + 3.0, area.w, area.h, 2.0);
            self.ctx.fill();
        }

        // Draw the actual button
        self.ctx.begin_path();
        self.ctx.set_fill_style(&white);
        self.ctx.set_stroke_style(&grey_dark);
        self.ctx.set_line_width(1.0);
        rounded_rect(&self.ctx, area.x1, area.y1, area.w, area.h, 2.0);
        self.ctx.fill();
        self.ctx.stroke();

        // Add button text
        self.ctx.set_fill_style(&black);
        // TODO: What happens if fill_text returns an error?
        let _ = self.ctx.fill_text(self.text, center.x, center.y + 4.0);

        self.dirty = false;
    }
}
