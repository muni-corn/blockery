use rand::prelude::*;

use crate::block::BlockColor;

/// The radius of a firework
const FIREWORK_SIZE: u32 = 100;

/// The size of each firework particle
const PARTICLE_SIZE: u32 = 10;

/// The duration of a firework in seconds
const FIREWORK_DURATION: u32 = 1.0;

/// The number of particles in a firework
const NUM_FIREWORK_PARTICLES: u32 = 16;

struct FireworkManager {
    fireworks: Vec<Firework>
}

impl FireworkManager {
    pub fn push_firework(&mut self, firework: Firework) {
        self.fireworks.push(firework);
    }
}

pub struct Firework {
    pos: (f32, f32),
    interpolation: f32, // 0..=1
    color: BlockColor,
}

impl Firework {
   pub fn new(x: f32, y: f32) -> Self {
       Self {
           pos: (x, y),
           interpolation: 0.0,
           color: BlockColor::random(),
       }
   }

   fn is_gone(&self) {
       self.interpolation >= 1
   }

   fn render<CanvasContext>(delta: f32, ctx_2d: CanvasContext) {
      // interpolate cuz we're cool
      if (self.interpolation < 1) {
         self.interpolation += delta / FIREWORK_DURATION;
         self.interpolation = self.interpolation.min(1);
      }

      // get_the quintic value of the animation interpolation
      let t = quintEaseOut(self.interpolation);
      // the calculated particle size (in browser space)
      let s = toBrowserH((1 - t * t * t) * PARTICLE_SIZE);
      // distance of a particle from the center of the firework
      let d = t * FIREWORK_SIZE;

      // draw all the particles yay
      ctx_2d.fillStyle = intToRGBText(self.color);
      for i in 0..NUM_FIREWORK_PARTICLES {
         let angle = 2 * Math.PI * i / NUM_FIREWORK_PARTICLES;
         let particle_x = self.x + Math.sin(angle) * d;
         let particle_y = self.y + Math.cos(angle) * d;
         ctx_2d.fillRect(to_browser_x(particle_x - s / 2), to_browser_y(particle_y - s / 2), s, s);
      }
   }
}

fn render_special_effects<CanvasContext>(delta: f32, ctx_2d: CanvasContext) {
   // render fireworks
   fireworkManager.forEach(|firework| {
      firework.render(delta, ctx_2d);
      if (firework.isGone()) {
         fireworkManager.splice(fireworkManager.indexOf(firework), 1);
      }
   });
}
