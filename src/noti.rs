/* jshint esversion: 6 */
/* global applyShadow, removeShadow, BOARD, getSansFont, UI_SHADOW, UI_MAX_ALPHA, quintEaseOut, getStatusBarX, getStatusBarWidth, getStatusBarHeight, to_browser_x, to_browser_y, to_browser_w, toBrowserH */

struct NotificationManager {
    noti_message: String,
    noti_duration: f32,
    noti_elapsed_time: f32,
}

const NOTIFICATION_ANIMATE_DURATION: f32 = 0.5;
const NOTIFICATION_HEIGHT: f32 = getStatusBarHeight() * 3 / 4;

impl NotificationManager {
    fn render_notifications<CanvasContext>(&self, delta: f32, ctx_2d: CanvasContext) {
        if (self.noti_elapsed_time > self.noti_duration) {
            self.noti_elapsed_time += delta;
        }

        let enter = quintEaseOut(self.noti_elapsed_time / NOTIFICATION_ANIMATE_DURATION);
        enter = 1f32.min(enter);

        let exit = 1 - quintEaseOut(
            (self.noti_duration - self.noti_elapsed_time) / NOTIFICATION_ANIMATE_DURATION,
        );
        exit = 0f32.max(exit);

        let x = to_browser_x(getStatusBarX());
        let y = to_browser_y(-(NOTIFICATION_HEIGHT + UI_SHADOW) * (1 - (enter - exit)));
        let w = to_browser_w(getStatusBarWidth());
        let h = toBrowserH(NOTIFICATION_HEIGHT);

        ctx_2d.save();

        applyShadow(ctx_2d);
        ctx_2d.fillStyle = "rgba(255, 255, 255, " + UI_MAX_ALPHA + ")";
        ctx_2d.fillRect(x, y, w, h);
        removeShadow(ctx_2d);

        ctx_2d.beginPath();
        ctx_2d.rect(x, y, w, h);
        ctx_2d.clip();

        // Render the message
        ctx_2d.fillStyle = "black";
        ctx_2d.font = getSansFont();
        ctx_2d.textAlign = "center";
        ctx_2d.textBaseline = "middle";
        let text_y = (h / 2) + (if enter >= 1 { y } else { y / 2 });
        ctx_2d.fillText(self.noti_message, x + w / 2, text_y);

        ctx_2d.restore();
    }

    fn send_notification(&mut self, message: &str, duration: f32) {
        self.noti_message = message;
        if (duration) {
            self.noti_duration = duration
        } else {
            self.noti_duration = NOTIFICATION_ANIMATE_DURATION * 2
        };
        self.noti_elapsed_time = 0;
    }
}
