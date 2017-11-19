/* jshint esversion: 6, browser: true, devel: true */
/* global BOARD, intToRGB, toScreenX, toScreenY, Block, COLOR_RED, mat4, glMatrix, main, bindMatrix, CUBE_MESH */


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
   hasTouchStart = 'ontouchstart' in window;
   hasOnClick = 'onclick' in window;
   document.addEventListener('click', function (event) {
      cx = event.clientX;
      cy = event.clientY;
      clicks++;
   });
   document.addEventListener('touchstart', function (event) {
      tsx = Math.floor(event.touches[0].clientX);
      tsy = Math.floor(event.touches[0].clientY);
      touchstarts++;
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
   debug.innerHTML =
      "clicks: " + clicks + ", last at " + cx + ", " + cy +
      "<br/>touchstarts: " + touchstarts + ", last at " + tsx + ", " + tsy +
      "<br/>window has onclick? " + hasOnClick +
      "<br/>window has touchstart? " + hasTouchStart;
};

const render = (gl, matrices, programInfo, buffers) => {
   gl.clearColor(0.95, 0.95, 0.95, 1);
   gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

   BOARD.render(gl, programInfo);
};
