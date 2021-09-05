const DIALOG_ANIMATE_DURATION: f32 = 0.25;

const DIALOG_TITLE_TEXT_HEIGHT: f32 = 35;
const DIALOG_TITLE_COLOR: Rgba = (0, 0, 0, 1.0);

const DIALOG_MESSAGE_COLOR: Rgba = (0, 0, 0, 0.75);
const DIALOG_MESSAGE_SPACING: f32 = 1.15;

const DIALOG_BUTTON_TEXT_HEIGHT: f32 = 25;
const DIALOG_BUTTON_HEIGHT: f32 = 50;

struct DialogManager {
    dialogs: Vec<Dialog>,
    background_inter: f32,
}

impl DialogManager {
    fn render_dialogs<Gl, CanvasContext>(
        &self,
        delta: f32,
        gl: Gl,
        program_info: ProgramInfo,
        ctx_2d: CanvasContext,
    ) {
        ctx_2d.fillStyle = "rgba(0, 0, 0," + cubicEaseOut(backgroundInter) * 0.5 + ")";
        ctx_2d.fillRect(0, 0, window.innerWidth, window.innerHeight);

        if (dialogs[0]) {
            if (backgroundInter < 1) {
                backgroundInter += delta / 0.25;
                if (backgroundInter > 1) {
                    backgroundInter = 1;
                }
            }

            dialogs[0].render(delta, gl, program_info);
        } else {
            if (backgroundInter > 0) {
                backgroundInter -= delta / 0.25;
                if (backgroundInter < 0) {
                    backgroundInter = 0;
                }
            }
        }
    }
}

struct Dialog {
    title: String,
    message: String,

    background_color: Rgba,
    text_color: Rgb,

    positive_button: Button,
    negative_button: Option<Button>,
    neutral_button: Option<Button>,

    positive_button_background: Rgb,
    negative_button_background: Rgb,
    neutral_button_background: Rgb,
    positive_button_text_color: Rgb,
    negative_button_text_color: Rgb,
    neutral_button_text_color: Rgb,
}

impl Dialog {
    const TITLE_FONT: &str = format!("{}px New Cicle Fina", toBrowserH(DIALOG_TITLE_TEXT_HEIGHT));

    pub fn new<CanvasContext>(ctx_2d: CanvasContext, title: &str, message: &str) -> Self {
        Self {
            title,
            message,

            background_color: Rgba(1, 1, 1, UI_MAX_ALPHA),
            text_color: Rgba(0, 0, 0, 1),

            positive_button: DEFAULT_BUTTON,
            negative_button: None,
            neutral_button: None,

            positive_button_background: Rgba(1, 1, 1, 1),
            negative_button_background: Rgba(1, 1, 1, 1),
            neutral_button_background: Rgba(1, 1, 1, 1),
            positive_button_text_color: intToRGBText(COLOR_GREEN),
            negative_button_text_color: intToRGBText(COLOR_RED),
            neutral_button_text_color: intToRGBText(COLOR_ORANGE),
        }
    }

    fn get_default_button(&self) -> Button {
        DialogButton::new(self, BUTTON_POSITIVE, "Dismiss", |dialog| {
            dialog.dismiss();
        });
    }

    fn on_click(&self, mx: f32, my: f32) {
        self.positive_button.onClick(mx, my);
        if (self.negative_button) {
            self.negative_button.onClick(mx, my);
        }
        if (self.neutral_button) {
            self.neutral_button.onClick(mx, my);
        }
    }

    fn on_mouse_move(mx: f32, my: f32) {
        self.positiveButton.onMouseMove(mx, my);
        if (self.negativeButton) {
            self.negativeButton.onMouseMove(mx, my);
        }
        if (self.neutralButton) {
            self.neutralButton.onMouseMove(mx, my);
        }
    }

    fn set_button<F>(button_type: DialogButtonType, text: &str, action: F) {
        let button = DialogButton::new(self, button_type, text, action);
        match button_type {
            BUTTON_POSITIVE => {
                if (!button) {
                    self.positiveButton = self.DEFAULT_BUTTON;
                } else {
                    self.positiveButton = button;
                }
            }
            BUTTON_NEGATIVE => self.negativeButton = button,
            BUTTON_NEUTRAL => self.neutralButton = button,
        }
        self
    }

    fn show(self) {
        self.ctx.font = self.TITLE_FONT;
        self.titleWrap = getWrappedLines(self.ctx, self.title, self.width - UI_PADDING * 2);

        self.ctx.font = getSansFont();
        self.messageWrap = getWrappedLines(self.ctx, self.message, self.width - UI_PADDING * 2);

        // Reset_all mouse move listeners
        mouse_listeners.forEach(|listener| {
            if (listener.onMouseMove) {
                listener.onMouseMove(NaN, NaN);
            }
        });
        dialogs.push(self);
        self.enterInter = 0;
    }

    fn get_height() {
        UI_PADDING
            + (if self.titleWrap {
                self.titleWrap.length
            } else {
                0
            }) * DIALOG_TITLE_TEXT_HEIGHT
            + UI_PADDING
            + (if self.messageWrap {
                self.messageWrap.length
            } else {
                0
            }) * UI_SANS_TEXT_HEIGHT
                * DIALOG_MESSAGE_SPACING
            + UI_PADDING / 2
            + DIALOG_BUTTON_HEIGHT
            + UI_PADDING;
    }

    fn get_width() {
        VISIBLE_WIDTH * 7 / 8
    }

    fn dismiss() {
        dialogs.splice(dialogs.indexOf(self), 1);
    }

    fn render<Gl>(delta: f32, gl: Gl, program_info: ProgramInfo) {
        if (self.enterInter < 1) {
            self.enterInter += delta / DIALOG_ANIMATE_DURATION;
            if (self.enterInter > 1) {
                self.enterInter = 1
            };
        }

        let ctx = self.ctx;

        // Browser padding
        let p = toBrowserH(UI_PADDING);

        let w = self.width;
        let h = quintEaseOut(self.enterInter) * self.height;
        let x = VISIBLE_WIDTH / 2 - w / 2;
        let y = VISIBLE_HEIGHT / 2 - h / 2;

        // Convert to browser window space
        w = to_browser_w(w);
        x = to_browser_x(x);
        y = to_browser_y(y);
        h = toBrowserH(h);

        // Draw the dialog card
        ctx.fillStyle = self.backgroundColor;

        applyShadow(ctx);
        ctx.fillRect(x, y, w, h);
        removeShadow(ctx);

        // Clip the dialog space so that text is not rendered outside of it
        ctx.save();

        ctx.beginPath();
        ctx.rect(x, y, w, h);
        ctx.clip();

        ctx.textBaseline = "top";
        ctx.textAlign = "left";
        ctx.fillStyle = self.textColor;

        // Draw text! //

        // Start drawing text at self dialog's browser-based y plus padding
        let draw_y = y + p;

        // Render the title of the dialog
        if (self.titleWrap) {
            ctx.font = self.TITLE_FONT;
            for i in 0..self.titleWrap.length {
                ctx.fillText(self.titleWrap[i], x + p, draw_y);
                draw_y += toBrowserH(DIALOG_TITLE_TEXT_HEIGHT);
            }
        }

        draw_y += p;

        // Render the message of the dialog
        if (self.messageWrap) {
            ctx.font = getSansFont();
            for i in 0..self.messageWrap.length {
                ctx.fillText(self.messageWrap[i], x + p, draw_y);
                draw_y += toBrowserH(UI_SANS_TEXT_HEIGHT * DIALOG_MESSAGE_SPACING);
            }
        }
        // Reset_clipping
        ctx.restore();

        let button_x = x + w - p / 2 - self.positiveButton.width;
        let button_y = y + h - p / 2 - toBrowserH(DIALOG_BUTTON_HEIGHT);
        self.positiveButton.render(
            button_x,
            button_y,
            self.positiveButtonBackground,
            self.positiveButtonTextColor,
        );
        if (self.negativeButton) {
            button_x -= self.negativeButton.width;
            self.negativeButton.render(
                button_x,
                button_y,
                self.negativeButtonBackground,
                self.negativeButtonTextColor,
            );
        }
        if (self.neutralButton) {
            button_x = x + p / 2;
            self.neutralButton.render(
                button_x,
                button_y,
                self.neutralButtonBackground,
                self.neutralButtonTextColor,
            );
        }
    }
}

enum DialogButtonType {
    Positive,
    Negative,
    Neutral,
}

impl DialogButton<F> {
    pub fn new(dialog: &Dialog, button_type: DialogButtonType, text: &str, action: F) -> Self {
        Self {
            dialog,
            button_type,
            text,
            action,
            hovering: false,
        }
    }

    fn coordinate_in_range(&self, mx: i32, my: i32) -> f32 {
        mx >= to_gl_x(self.x)
            && mx <= to_gl_x(self.x + self.width)
            && my >= to_gl_y(self.y)
            && my <= to_gl_y(self.y) + DIALOG_BUTTON_HEIGHT
    }

    fn on_mouse_move(&self, mx: i32, my: i32) {
        self.hovering = self.coordinateInRange(mx, my);
    }

    fn on_click(&self, mx: i32, my: i32) {
        if (self.coordinateInRange(mx, my)) {
            self.action(self.dialog);
        }
    }

    /// Returns a button width usable in browser window space.
    fn get_width(&self) {
        let ctx = self.dialog.ctx;
        ctx.font = DialogButton.FONT;
        to_browser_w(UI_PADDING * 2) + ctx.measure_text(self.text.toUpperCase()).width
    }

    fn get_font() -> String {
        toBrowserH(DIALOG_BUTTON_TEXT_HEIGHT) + "px New Cicle Fina"
    }

    fn render(&self, browser_x: i32, browser_y: i32, background_color: Rgb, text_color: Rgba) {
        let ctx = self.dialog.ctx;
        self.x = browser_x;
        self.y = browser_y;
        if (self.hovering) {
            ctx.fillStyle = background_color;
            ctx.fillRect(
                browser_x,
                browser_y,
                self.width,
                toBrowserH(DIALOG_BUTTON_HEIGHT),
            );
        }
        ctx.fillStyle = text_color;
        ctx.textBaseline = "middle";
        ctx.textAlign = "center";
        ctx.fillText(
            self.text.toUpperCase(),
            browser_x + self.width / 2,
            browser_y + to_browser_y(DIALOG_BUTTON_HEIGHT / 2),
        );
    }
}

struct RedDialog {
    inner: Dialog,
}

impl RedDialog {
    pub fn new(ctx_2d: CanvasContext, title: &str, message: &str) {
        let inner = Dialog::new(ctx_2d, title, message);
        self.backgroundColor = intToRGBText(COLOR_RED);
        self.textColor = "white";

        self.negativeButtonTextColor = self.positiveButtonTextColor = "white";
        self.negativeButtonBackground = self.positiveButtonBackground = "rgba(0, 0, 0, 0.2)";
    }
}
