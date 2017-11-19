/* jshint esversion: 6, browser: true, devel: true */
/* globals UPDATES_PER_SECOND, CUBE_MESH, VISIBLE_HEIGHT, BOARD, toScreenX, toScreenY, getBoardX, getBoardY */
const COLOR_RED = 0xff004c;
const COLOR_ORANGE = 0xffa530;
const COLOR_GREEN = 0x50ec8c;
const COLOR_BLUE = 0x117cff;
const BOUNCE_FACTOR = 0.25;
const GRAVITY = 1500;

class Block {
   constructor() {
      let r = Math.floor(Math.random() * 4);
      switch (r) {
         case 0:
            this.color = COLOR_RED;
            break;
         case 1:
            this.color = COLOR_ORANGE;
            break;
         case 2:
            this.color = COLOR_GREEN;
            break;
         case 3:
            this.color = COLOR_BLUE;
            break;
      }
      this._fill = 0; // 0 - 100
      this.x = 0;
      this.y = -75;
      this.yv = 0;
      this.z = 0;
      this.zv = Math.random() * 1000;
      this.falling = false;
      this.pit = 0;
      this.yaw = 0;
      this.rol = 0;
      this.pitV = (Math.random() - 0.5) * 360 * 2;
      this.yawV = (Math.random() - 0.5) * 360 * 2;
      this.rolV = (Math.random() - 0.5) * 360 * 2;
   }

   fall() {
      this.falling = true;
      this.yv = -500 * Math.random();
   }

   blockLogic(delta, row, col) {
      this.row = row;
      this.col = col;
      this.x = BOARD.getBoardX(this.col);
      let maxY = BOARD.getBoardY(this.row);

      if (this.row == -1 && !this.falling) {
         this.y = maxY + 10 *
            (Math.sin(performance.now() / 1000 + (this.col * -Math.PI / BOARD.COLUMNS)) - 1);
      } else {
         while (delta > 0) {
            let spu = 1 / UPDATES_PER_SECOND;
            let timeSlice = delta < spu ? delta : spu;

            this.yv += GRAVITY * timeSlice;
            this.y += this.yv * timeSlice;

            if (!this.falling && this.y >= maxY) {
               this.yv *= -BOUNCE_FACTOR;
               this.y = maxY;
            } else if (this.falling) {
               this.pit += this.pitV * timeSlice;
               this.yaw += this.yawV * timeSlice;
               this.rol += this.rolV * timeSlice;
               this.z += this.zv * timeSlice;
            }
            delta -= timeSlice;
         }
      }
   }

   set fill(val) {
      this._fill = val > 100 ? 100 : val;
   }

   get fill() {
      return this._fill;
   }

   isFull() {
      return this._fill >= 100;
   }

   get gone() {
      return this.y > VISIBLE_HEIGHT;
   }

   renderBlock(gl, programInfo) {
      let f = this._fill / 100;
      // Re-assign the fill variable for a cool cubic-easing animation effect ;)
      f--;
      f = (f * f * f + 1);
      let bw = BOARD.BLOCK_WIDTH;
      let w = BOARD.BLOCK_WIDTH * ((f * 5 > 1) ? 1 : (f * 5));
      let h = bw * f;

      CUBE_MESH.setColor(this.color, gl, programInfo);
      CUBE_MESH.render(gl, this.x + (bw - w) / 2, this.y + bw - h, this.z, w, h, w, this.pit, this.yaw, this.rol);
   }
}