/* jshint esversion: 6, browser: true, devel: true */
/* global sendNotification, renderNotifications, dialogs, renderGame, renderDialogs, Button, renderScoreboard, renderStatusBar, toBrowserX, toBrowserY, VISIBLE_HEIGHT, Data, COLOR_BLUE, Board, isMobile, intToRGB, toGLX, toGLY, globalYOffset, Block, COLOR_RED, mat4, glMatrix, main, bindMatrix, CUBE_MESH */

// The interval in which to save in seconds
const SAVE_INTERVAL = 60;

function main(glCanvas, gl, canvas2d, ctx2d, programInfo, matrices, buffers) {
   let lastSave = 0;

   let then = 0;
   const loop = (now) => {
      if (!isNaN(now)) {
         let delta = (now - then) / 1000;
         then = now;
         logic(delta);
         render(delta, gl, matrices, programInfo, buffers, canvas2d, ctx2d);

         if (now - lastSave >= SAVE_INTERVAL * 1000) {
            Data.save();
            lastSave = now;
         }
      }

      requestAnimationFrame(loop);
   };

   requestAnimationFrame(loop);
}

let debug = document.getElementById("debug_text");
let touched = false;
let stx, sty; // Starting touch coordinates
let etx, ety; // Ending touch coordinates
const TOUCH_THRESHOLD = 10; // the start and end coordinates must be within this range to register as a touch
let lastTouchY = 0;
let saveOnBeforeUnload = true;

window.onload = () => {
   // Key listeners
   document.addEventListener('keydown', function (event) {
      if (event.ctrlKey && !event.repeat && event.key.toUpperCase() == "S") {
         event.preventDefault();
         Data.save();
      }
      if (event.ctrlKey && event.altKey && !event.repeat && event.key.toUpperCase() == "Q") {
         event.preventDefault();
         Board.queueFillingEnabled = !Board.queueFillingEnabled;
         sendNotification(Board.queueFillingEnabled ? "Queue filling enabled" : "Queue filling disabled for anti-distraction purposes :)", 4);
      }
   });


   let listenerType = isMobile() ? "touchstart" : "mousedown";

   document.addEventListener(listenerType, function (event) {
      onClickHandler(event);
   });
   // Doesn't work with iPhones >:( >:( >:(
   // document.addEventListener('touchend', function (event) {
   //    // If the ending coordinate is within range of the start coordinate,
   //    // register as a touch
   //    if (Math.sqrt(Math.pow(etx - stx, 2) + Math.pow(ety - sty, 2)) > TOUCH_THRESHOLD)
   //       return;

   //    onClickHandler(event);
   //    touched = true;
   // });

   // mouse move
   document.addEventListener('mousemove', function (event) {
      // Convert to GL space
      let x = toGLX(event.clientX);
      let y = toGLY(event.clientY) - globalYOffset;

      // If a dialog is showing, ignore input
      if (dialogs.length > 0) {
         dialogs[0].onMouseMove(x, y);
         return;
      }

      mouseListeners.forEach(listener => {
         if (listener.onMouseMove)
            listener.onMouseMove(x, y);
      });
   });

   document.addEventListener('touchmove', function (event) {
      let delta = toGLY(event.touches[0].clientY - lastTouchY);
      lastTouchY = event.touches[0].clientY;

      etx = event.touches[0].clientX;
      ety = event.touches[0].clientY - globalYOffset;

      mouseListeners.forEach(listener => {
         if (listener.onMouseMove)
            listener.onMouseMove(toGLX(etx), toGLY(ety));
      });
   });

   // Save data before the window closes
   window.onbeforeunload = function () {
      if (saveOnBeforeUnload)
         Data.save();
   };
};

// An array of objects that have functions called every time a mouse
// event is fired.
const mouseListeners = [];

const onClickHandler = event => {
   // if (touched) {
   //    touched = false;
   //    return;
   // }

   let x, y;
   if (event.type.startsWith("touchstart")) {
      x = event.touches[0].clientX;
      y = event.touches[0].clientY;
   } else if (event.type.startsWith("mouse")) {
      x = event.clientX;
      y = event.clientY;
   }

   // Convert to GL space
   x = toGLX(x || etx);
   y = toGLY(y || ety) - globalYOffset;

   // Ignore input with a dialog showing
   if (dialogs.length > 0) {
      dialogs[0].onClick(x, y);
      return;
   }

   mouseListeners.forEach(listener => {
      if (listener.onClick)
         listener.onClick(x, y);
   });
};

/************************************************
 * Computes game logic. The variable delta is
 * measured in seconds.
 ************************************************/
const logic = delta => {
   Board.logic(delta);
};

const render = (delta, gl, matrices, programInfo, buffers, canvas2d, ctx2d) => {
   gl.clearColor(0.9, 0.9, 0.9, 1);
   gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
   ctx2d.clearRect(0, 0, canvas2d.width, canvas2d.height);

   renderGame(delta, gl, programInfo, matrices, ctx2d);

   // Render UI //

   // Reset the context's transformation to the identity matrix
   ctx2d.setTransform(1, 0, 0, 1, 0, 0);

   // Render special effects that are cool
   renderSpecialEffects(delta, ctx2d);

   // This should always come second to last
   renderDialogs(delta, gl, programInfo, ctx2d);

   // This should always come last
   renderNotifications(delta, ctx2d);
};
