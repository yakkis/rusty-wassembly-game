use crate::types::Ctx;
use crate::utils::abort;

pub fn rounded_rect(ctx: &Ctx, x: f64, y: f64, w: f64, h: f64, radius: f64) {
    ctx.move_to(x, y + radius);
    ctx.line_to(x, y + h - radius);
    if ctx.arc_to(x, y + h, x + radius, y + h, radius).is_err() {
        abort("Failed arc to");
    }
    ctx.line_to(x + w - radius, y + h);
    if ctx.arc_to(x + w, y + h, x + w, y + h - radius, radius).is_err() {
        abort("Failed arc to");
    }
    ctx.line_to(x + w, y + radius);
    if ctx.arc_to(x + w, y, x + w - radius, y, radius).is_err() {
        abort("Failed arc to");
    }
    ctx.line_to(x + radius, y);
    if ctx.arc_to(x, y, x, y + radius, radius).is_err() {
        abort("Failed arc to");
    }
}
