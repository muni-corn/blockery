/* jshint esversion: 6, browser: true, devel: true */
/* global BOARD, isMobile, intToRGB, toScreenX, toScreenY, Block, COLOR_RED, mat4, glMatrix, main, bindMatrix, CUBE_MESH */


function main(canvas, gl, programInfo, matrices, buffers) {
   let then = 0;
   const loop = (now) => {
      if (!isNaN(now)) {
         let delta = (now - then) / 1000;
         then = now;

         logic(delta);

         render(gl, matrices, programInfo, buffers);
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

const render = (gl, matrices, programInfo, buffers) => {
   gl.clearColor(0.95, 0.95, 0.95, 1);
   gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

   BOARD.render(gl, programInfo);
};
