/* jshint esversion: 6, browser: true, devel: true */
/* global toBrowserX, toBrowserY, VISIBLE_HEIGHT, DATA, COLOR_BLUE, BOARD, isMobile, intToRGB, toGLX, toGLY, Block, COLOR_RED, mat4, glMatrix, main, bindMatrix, CUBE_MESH */

// The interval in which to save in seconds
const SAVE_INTERVAL = 5

function main(glCanvas, gl, canvas2d, ctx2d, programInfo, matrices, buffers) {
   let lastSave = 0;

   let then = 0;
   const loop = (now) => {
      if (!isNaN(now)) {
         let delta = (now - then) / 1000;
         then = now;

         logic(delta);
         render(gl, matrices, programInfo, buffers, canvas2d, ctx2d);

         if (now - lastSave >= SAVE_INTERVAL * 1000) {
            DATA.save();
            lastSave = now;
         }
      }

      requestAnimationFrame(loop);
   };

   requestAnimationFrame(loop);
}

let pitch, yaw, roll;
let debug = document.getElementById("debug_text");
let clicks = 0,
   touchstarts = 0,
   tsx, tsy, cx, cy, hasTouchStart, hasOnClick;

window.onload = function () {
   let listenerType = isMobile() ? "touchstart" : "click";
   let x, y;
   document.addEventListener(listenerType, function (event) {
      if (listenerType === "touchstart") {
         x = event.touches[0].clientX;
         y = event.touches[0].clientY;
      } else {
         x = event.clientX;
         y = event.clientY;
      }

      BOARD.onClick(x, y);
   });
};

/************************************************
 * LOGIC
 * Computes game logic. The variable delta is
 * measured in seconds. Returns an object of data
 * to be passed to the rendering method.
 ************************************************/
const logic = (delta) => {
   BOARD.logic(delta);
};

const render = (gl, matrices, programInfo, buffers, canvas2d, ctx2d) => {
   gl.clearColor(0.9, 0.9, 0.9, 1);
   gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
   ctx2d.clearRect(0, 0, canvas2d.width, canvas2d.height);

   BOARD.render(gl, programInfo);
   renderScoreboard(gl, programInfo, ctx2d);
};

const renderScoreboard = (gl, programInfo, ctx2d) => {
   CUBE_MESH.setColor(COLOR_BLUE, gl, programInfo);
   let x = BOARD.boardCenter.x - BOARD.width / 2 - BOARD.GRID_PADDING - BOARD.FRAME_THICKNESS;
   let y = BOARD.boardCenter.y + BOARD.height / 2 + BOARD.GRID_PADDING + BOARD.FRAME_THICKNESS * 2;
   let w = BOARD.width + BOARD.FRAME_THICKNESS * 2 + BOARD.GRID_PADDING * 2;
   let h = VISIBLE_HEIGHT - y;
   CUBE_MESH.render(gl, x, y, 0, w, h, BOARD.BLOCK_WIDTH);

   let amountText = DATA.currentBlocks;

   let textHeight = 50;
   let monospaceFont = toBrowserY(72) + "px Digital-7";
   let sansFont = toBrowserY(35) + "px New Cicle Fina";

   ctx2d.font = monospaceFont;

   let blocksTextX = toBrowserX(x + w - BOARD.FRAME_THICKNESS);
   let textY = toBrowserY(y + h / 2) + toBrowserY(textHeight) / 2;


   ctx2d.font = sansFont;
   ctx2d.textAlign = "right";
   ctx2d.fillStyle = "white";
   ctx2d.fillText("blocks", blocksTextX, textY);

   let blocksTextWidth = ctx2d.measureText(" blocks").width;

   ctx2d.font = monospaceFont;
   ctx2d.fillText(amountText, blocksTextX - blocksTextWidth, textY);
};
