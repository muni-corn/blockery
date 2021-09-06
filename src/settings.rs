struct SettingsScreen {
    reset_button: Button,
}

impl SettingsScreen {
    fn new() -> Self {
        let w = getStatusBarWidth() / 2;
        let h = getStatusBarHeight() / 2;
        Self {
            reset_button: Button::new(VISIBLE_WIDTH / 2 - w / 2, VISIBLE_HEIGHT / 2 - h / 2, w, h, COLOR_RED, "Reset everything", || {
                Data.reset();
            }),
        }
    }

    fn render_settings<Gl, CanvasContext>(&self, delta: f32, gl: Gl, program_info: ProgramInfo, ctx_2d: CanvasContext, y_offset: f32) {
        ctx_2d.fillStyle = "white";
        ctx_2d.textBaseline = "middle";
        ctx_2d.textAlign = "center";
        ctx_2d.font = toBrowserH(24) + "px New Cicle Fina";
        ctx_2d.fillText("(Do not push)", to_browser_x(VISIBLE_WIDTH / 2), to_browser_y(VISIBLE_HEIGHT / 2 + resetButton.h + y_offset));
        resetButton.render(delta, gl, program_info, ctx_2d, y_offset);
    }
}

