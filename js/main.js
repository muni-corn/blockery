/* jshint esversion: 6, browser: true, devel: true */
/* global mat4, glMatrix, main, bindMatrix */

function main(canvas, gl, programInfo, matrices, buffers, cubeMesh) {
   bindCubeAttributes(gl, programInfo.attributeLocations, buffers);
   setCubeColor(gl, programInfo, 1, 0, 0);

   let then;

   const loop = (now) => {
      let data = logic((now - then) / 1000);
      render(data, gl, matrices, programInfo, cubeMesh, buffers);

      requestAnimationFrame(loop);
   };

   loop();
}

let pitch, yaw, roll;
let pitchMatrix = new Float32Array(16);
let yawMatrix = new Float32Array(16);
let rollMatrix = new Float32Array(16);
let mx, my;
let debug = document.getElementById("debug_text");

window.onmousemove = (event) => {
   mx = event.clientX;
   my = event.clientY;
};

/************************************************
 * LOGIC
 * Computes game logic. The variable delta is
 * measured in seconds. Returns an object of data
 * to be passed to the rendering method.
 ************************************************/
const logic = (delta) => {
   pitch = performance.now() / 100;
   yaw = performance.now() / 50;
   roll = performance.now() / 25;
   //   document.getElementById("time").innerHTML = performance.now() / 1000;

   return {
      pitch: pitch,
      yaw: yaw,
      roll: roll
   };
};

const toRad = Math.PI / 180;

function render(data, gl, matrices, programInfo, cubeMesh, buffers) {
   gl.clearColor(1, 1, 1, 1);
   gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

   mat4.translate(matrices.world, matrices.identity, [toScreenX(mx), toScreenY(my), 0]);
   mat4.scale(matrices.world, matrices.world, [50, 50, 50]);
   mat4.rotate(pitchMatrix, matrices.identity, pitch * toRad, [1, 0, 0]);
   mat4.rotate(yawMatrix, matrices.identity, yaw * toRad, [0, 1, 0]);
   mat4.rotate(rollMatrix, matrices.identity, roll * toRad, [0, 0, 1]);
   mat4.mul(matrices.world, matrices.world, rollMatrix);
   mat4.mul(matrices.world, matrices.world, yawMatrix);
   mat4.mul(matrices.world, matrices.world, pitchMatrix);
   bindMatrix(matrices.world, programInfo.uniformLocations.worldMatrix);

   gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, buffers.index);
   gl.drawElements(gl.TRIANGLES, cubeMesh.indicies.length, gl.UNSIGNED_SHORT, 0);
}

function bindCubeAttributes(gl, attributeLocations, buffers) {
   gl.bindBuffer(gl.ARRAY_BUFFER, buffers.vertex);
   gl.vertexAttribPointer(attributeLocations.position, 3, gl.FLOAT, false, 0, 0);
   gl.enableVertexAttribArray(attributeLocations.position);

   gl.bindBuffer(gl.ARRAY_BUFFER, buffers.normal);
   gl.vertexAttribPointer(attributeLocations.normal, 3, gl.FLOAT, true, 0, 0);
   gl.enableVertexAttribArray(attributeLocations.normal);
}

const setCubeColor = (gl, programInfo, r, g, b) =>
   gl.uniform3f(programInfo.uniformLocations.color, r, g, b);


const width = () => window.innerWidth;
const height = () => window.innerHeight;

const toScreenX = (clientX) => (clientX - (width() - height()) / 2) * 1000 / height();
const toScreenY = (clientY) => (clientY / height()) * 1000;
