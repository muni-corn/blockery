pub fn int_to_rgb(rgb: i32) -> (u8, u8, u8) {
    let red = (rgb >> 16) / 255;
    let green = ((rgb >> 8) & 0xff) / 255;
    let blue = (rgb & 0xff) / 255;

    (red, green, blue)
}

pub fn int_to_rgba(argb: i32) -> (u8, u8, u8, f32) {
    let (r, g, b) = int_to_rgb(argb);
    let a = (argb >> 24) / 255.0;

    (r, g, b, a)
}

pub fn int_to_rgba_text(argb: i32) -> String {
    let (r, g, b, a) = int_to_rgba(argb);

    format!("rgba({}, {}, {}, {})", r, g, b, a)
}

pub fn int_to_rgb_text(rgb: i32) -> String {
    let (r, g, b) = int_to_rgb(rgb);

    format!("rgba({}, {}, {}, 1)", r, g, b)
}

pub fn rgb_to_int(r: u8, g: u8, b: u8) -> i32 {
    (r << 16) + (g << 8) + (b)
}

pub fn window_width() -> i32 { todo!() }
pub fn window_height() -> i32 { todo!() }
pub fn visible_width() -> i32 { todo!() }
pub fn visible_height() -> i32 { todo!() }

/// Converts an x-coordinate in the browser window
/// to an x-coordinate usable in the WebGL matrix
/// space.
pub fn to_glx(client_x: f32) -> f32 {
    (visible_height() * (2 * client_x - window_width())) / (2 * window_height()) + visible_width() / 2
}

/// Converts an y-coordinate in the browser window
/// to an y-coordinate usable in the WebGL matrix
/// space.
pub fn to_gly(client_y: f32) -> f32 { client_y * (visible_height() / window_height()) }

/// Converts an x-coordinate in the WebGL matrix
/// space to an x-coordinate usable in the browser
/// window.
pub fn to_browser_x(gl_x: f32) -> f32 {
    (2 * window_height() * gl_x - window_height() * visible_width() + window_width() * visible_height()) / (2 * visible_height());
}

/// Converts an y-coordinate in the WebGL matrix
/// space to an y-coordinate usable in the browser
/// window.
pub fn to_browser_y(gl_y: f32) -> f32 {
    gl_y * window_height() / visible_height()
}

/// Converts a width dimension in the WebGL matrix
/// space to a width dimension usable in the browser
/// window.
pub fn to_browser_w(gl_w: f32) -> f32 {
    gl_w * window_height() / visible_height()
}

/// Converts a height dimension in the WebGL matrix
/// space to a height dimension usable in the browser
/// window.
pub fn to_browser_h(gl_h: f32) -> f32 {
    to_browser_y(gl_h)
}

pub fn get_wrapped_lines<CanvasContext>(ctx_2d: CanvasContext, text: &str, max_width: i32) {
    // Convert to browser window space
    max_width = to_browser_w(max_width);
    let result = [];
    let words = text.split(" ");
    let line = "";

    for i in 0..words.length {
        let test = line + words[i];

        if ctx_2d.measure_text(test).width <= max_width {
            line = test + " ";
        } else {
            result.push(line);
            line = words[i] + " ";
        }
    }

    if line.length > 0 {
        result.push(line);
    }

    result
}

pub fn cubic_ease_in(t: f32) -> f32 {
    t * t * t
}

pub fn cubic_ease_out(t: f32) -> f32 {
    t -= 1;

    t * t * t + 1
}

pub fn quint_ease_in(mut t: f32) -> f32 {
    t * t * t * t * t
}


pub fn quint_ease_out(mut t: f32) -> f32 {
    t -= 1;

    t * t * t * t * t + 1
}
