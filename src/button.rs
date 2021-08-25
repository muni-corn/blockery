const BUTTON_DEFAULT_TEXT_HEIGHT: f32 = 25;
const BUTTON_DEPTH: f32 = Board.BLOCK_WIDTH;

struct ButtonManager {
    buttons: Vec<Button>,
}

fn reset_visible_button_flags() {
    for button in buttons {
        button.visible = false;
    }
}

impl ButtonManager {
    fn new(&mut self, listeners: &mut Listeners) -> Self {
        Self {
            buttons: Vec::new(),
        }
    }
}

struct Button {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    y_offset: f32,
    color: i32,
    disabled_color: i32,
    text: String,
    action: F,
    typeface: String,
    font_size: f32,
    enabled: bool,

    hovering: bool,
    press_inter: f32,
    press_inter_velocity: f32,
    lift_inter: f32,

    // a flag to tell if the button has been rendered. it is
    // reset_at the beginning of every render loop to false,
    // and set_to true and the end of the button's
    // render() method
    visible: bool,
}

impl Button {
    fn new<F: Fn>(x: f32, y: f32, w: f32, h: f32, color: i32, text: &str, action: F) -> Self {
        Self {
            x,
            y,
            w,
            h,
            y_offset: 0,
            color,
            disabled_color: self.toGrayscale(color),
            text,
            action,
            typeface: "New Cicle Fina",
            font_size: BUTTON_DEFAULT_TEXT_HEIGHT,
            enabled: true,

            hovering: false,
            press_inter: 0,
            press_inter_velocity: -1,
            lift_inter: 0,

            visible: false,
        }
    }

    fn on_click(mx: i32, my: i32) {
        if (!self.visible || !self.enabled) {
            return;
        }

        if (self.coordinateInBounds(mx, my)) {
            self.pressInterVelocity = 1;
            self.action();
        }
    }

    fn coordinate_in_bounds(x: i32, y: i32) {
        x >= self.x
            && x <= self.x + self.w
            && y >= self.y + self.y_offset_
            && y <= self.y + self.y_offset + self.h
    }

    fn on_mouse_move(mx: i32, my: i32) {
        if (!self.visible || !self.enabled) {
            self.hovering = false;
        }

        if (self.coordinateInBounds(mx, my)) {
            self.hovering = true;
        } else {
            self.hovering = false;
        }
    }

    fn to_grayscale(color: i32) {
        let rgb = intToRGB(color);
        let gray = (rgb.r + rgb.g + rgb.b) / 3;

        rgbToInt(gray, gray, gray)
    }

    fn add_highlight_to_color(color: i32) {
        let add_highlight = self.liftInter * 0.05;
        let rgb = intToRGB(color);

        rgb.r += add_highlight;
        if (rgb.r > 1) {
            rgb.r = 1
        };

        rgb.g += add_highlight;
        if (rgb.g > 1) {
            rgb.g = 1
        };

        rgb.b += add_highlight;
        if (rgb.b > 1) {
            rgb.b = 1
        };

        rgbToInt(rgb.r, rgb.g, rgb.b)
    }

    fn render_body<Gl>(gl: Gl, program_info: ProgramInfo, z: f32) {
        cube_mesh.set_color(
            if self.enabled {
                (self.addHighlightToColor(self.color))
            } else {
                self.disabledColor
            },
            gl,
            program_info,
        );
        cube_mesh.render(
            gl,
            self.x,
            self.y + self.y_offset,
            z,
            self.w,
            self.h,
            BUTTON_DEPTH,
        );
    }

    fn render_top_layer<CanvasContext>(
        ctx_2d: CanvasContext,
        button_center_x_2d: f32,
        button_center_y_2d: f32,
        to_new_depth: f32,
    ) {
        if (!self.text) {
            return;
        }
        ctx_2d.fillStyle = "white";
        ctx_2d.font = toBrowserH(toBrowserH(self.fontSize) * to_new_depth) + "px " + self.typeface;
        ctx_2d.textAlign = "center";
        ctx_2d.textBaseline = "middle";
        ctx_2d.fillText(self.text, button_center_x_2d, button_center_y_2d);
    }

    fn render<Gl, CanvasContext>(
        delta: f32,
        gl: Gl,
        program_info: ProgramInfo,
        ctx_2d: CanvasContext,
        y_offset: f32,
    ) {
        self.y_offset_ = y_offset || 0;

        // Interpolation properties for animation //

        self.pressInter += self.pressInterVelocity * delta * 10;
        if (self.pressInter < 0) {
            self.pressInter = 0;
        } else if (self.pressInter > 1) {
            self.pressInterVelocity *= -1;
            self.pressInter = 1;
        }

        self.liftInter += (if self.hovering { 1 } else { -1 }) * delta * 2;
        if (self.liftInter < 0) {
            self.liftInter = 0;
        } else if (self.liftInter > 1) {
            self.liftInter = 1;
        }

        let max_press = BUTTON_DEPTH * 3 / 4;
        let max_lift = -10;

        // 3D button rendering //

        // get_z offset_of pressing the button
        let z = (if self.pressInterVelocity > 0 {
            cubicEaseOut(self.pressInter)
        } else {
            cubicEaseIn(self.pressInter)
        }) * max_press;
        // and displace further by the mouse-hover effect
        z += (if self.hovering {
            cubicEaseOut(self.liftInter)
        } else {
            cubicEaseIn(self.liftInter)
        }) * max_lift;

        // Render the 3D button body
        self.renderBody(gl, program_info, z);

        // 2D text rendering //

        // get_the proportionality constant (height on screen = k * (height of mesh / depth from viewer))
        // v can really be any number but 0
        let v = 300;
        let k = toBrowserH(v) * CAMERA_Z / v;
        let to_new_depth = (CAMERA_Z - z) / k;

        // Yes, the following lines of code convert to *gl space*
        // Calculate the new x coordinate in gl space
        let x_distance_from_center =
            to_browser_y(VISIBLE_WIDTH / 2 - (self.x + self.w / 2)) * to_new_depth;
        let button_center_x = VISIBLE_WIDTH / 2 - x_distance_from_center;

        // Calculate the new y coordinate in gl space
        let y_distance_from_center =
            to_browser_y(VISIBLE_HEIGHT / 2 - (self.y + self.y_offset_ + self.h / 2))
                * to_new_depth;
        let button_center_y = VISIBLE_HEIGHT / 2 - y_distance_from_center;

        // Render the text!
        self.renderTopLayer(
            ctx_2d,
            to_browser_x(button_center_x),
            to_browser_y(button_center_y),
            to_new_depth,
        );

        self.visible = true;
    }
}

struct ImageButton {
    inner_button: Button,

    enabled: bool,
    img_src: String,
}

impl ImageButton {
    fn new<F>(x: f32, y: f32, w: f32, h: f32, color: i32, img_src: String, action: F) -> Self {
        let inner_button = Button::new(x, y, w, h, color, String::new(), action);
        self.enabled = true;

        Self {
            inner_button,
            enabled: true,
            img_src,
        }
    }

    fn set_enabled(&mut self, val: bool) {
        self.enabled = val;
    }

    fn get_enabled(&self) -> bool {
        self.enabled
    }

    fn set_img_src(&mut self, val: &str) {
        todo!()
    }

    fn get_img_src() {
        todo!()
    }

    fn render_top_layer<CanvasContext>(
        ctx_2d: CanvasContext,
        button_center_x_2d: f32,
        button_center_y_2d: f32,
        to_new_depth: f32,
    ) {
        if (!self.img_src) {
            return;
        }
        let w = to_browser_w(to_browser_w(self.w - UI_PADDING) * toNewDepth);
        let h = toBrowserH(toBrowserH(self.h - UI_PADDING) * toNewDepth);
        ctx_2d.drawImage(
            self.img,
            buttonCenterX2D - w / 2,
            buttonCenterY2D - h / 2,
            w,
            h,
        );
        super.renderTopLayer(ctx_2d, buttonCenterX2D, buttonCenterY2D, toNewDepth);
    }
}

struct ProgressButton {
    inner_button: Button,
    color_fill: i32,
    color_empty: i32,

    /// An indicator from 0 to 1
    progress: f32,
}

impl ProgressButton {
    fn new<F>(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color_fill: i32,
        color_empty: i32,
        text: &str,
        action: F,
    ) -> Self {
        let inner_button = Button::new(x, y, w, h, null, text, action);
        Self {
            inner_button,
            color_fill,
            color_empty,
            progress: 0.75,
        }
    }

    fn render_body<Gl>(&self, gl: Gl, program_info: ProgramInfo, z: f32, cube_mesh: CubeMesh) {
        self.y_offset_ = y_offset || 0;

        cube_mesh.set_color(self.addHighlightToColor(self.color_fill), gl, program_info);
        cube_mesh.render(
            gl,
            self.x,
            self.y + self.y_offset,
            z,
            self.w * self.progress,
            self.h,
            BUTTON_DEPTH,
        );

        cube_mesh.set_color(
            if self.enabled {
                self.addHighlightToColor(self.color_empty)
            } else {
                self.toGrayscale(self.color_empty)
            },
            gl,
            program_info,
        );
        cube_mesh.render(
            gl,
            self.x + self.w * self.progress,
            self.y + self.y_offset,
            z,
            self.w * (1 - self.progress),
            self.h,
            BUTTON_DEPTH,
        );
    }
}
