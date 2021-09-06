use crate::utils::{self, to_browser_h};

pub const UI_SHADOW: i32 = 20;
pub const UI_PADDING: i32 = 25;

pub const UI_SANS_TEXT_HEIGHT: i32 = 20;

/// The maximum alpha for 2D dialogs, notifications
pub const UI_MAX_ALPHA: f32 = 0.95;

fn get_sans_font() -> String {
   format!("{}px sans-serif", utils::to_browser_h(UI_SANS_TEXT_HEIGHT))
}

fn enable_shadow<CanvasContext>(ctx: CanvasContext) {
   ctx.shadow_color = "rgba(0, 0, 0, 0.2)";
   ctx.shadow_blur = to_browser_h(UI_SHADOW);
   ctx.shadow_offset_y = to_browser_h(UI_SHADOW);
}

fn remove_shadow<CanvasContext>(ctx: CanvasContext) {
   ctx.shadowBlur = 0;
   ctx.shadowOffsetY = 0;
}

struct Rgb(f32, f32, f32);

struct Theme {
    background: Rgb,
    settings_background: Rgb,
}

const THEME: Theme = Theme {
   background: (0.9, 0.9, 0.9),
   settings_background: (0.1, 0.1, 0.2)
};
