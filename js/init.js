/* jshint esversion: 6, browser: true, devel: true */
/* global BOARD, mat4, glMatrix, main, CUBE_MESH */

window.canvas = document.getElementById("glCanvas");
window.gl = window.canvas.getContext("webgl");
if (!window.gl) window.gl = window.canvas.getContext("experimental-webgl");
if (!window.gl) alert("This browser does not support WebGL.");

const UPDATES_PER_SECOND = 60;

function init() {
   let gl = window.gl;
   let canvas = window.canvas;

   let shaders;
   let buffers;
   let matrices;
   let programInfo;

   window.onresize = () => {
      window.canvas.width = window.innerWidth;
      window.canvas.height = window.innerHeight;
      gl.viewport(0, 0, canvas.width, canvas.height);
      getPerspectiveMatrix(matrices.proj);
      bindMatrix(matrices.proj, programInfo.uniformLocations.projMatrix);
   };

   window.onclick = (event) => {
      BOARD.onClick(event);
   };

   getShaders((result) => {
      shaders = result;
      programInfo = getShaderProgramInfo(gl, shaders);

      gl.useProgram(programInfo.program);

      buffers = getBuffers(gl);
      initCulling(gl);
      matrices = getMatrices(gl, programInfo);
      initLighting(gl, programInfo);
      CUBE_MESH.init(gl, matrices, programInfo);

      window.onresize();

      main(canvas, gl, programInfo, matrices, buffers);
   });

   BOARD.init();
}

function error(error) {
   let textBox = document.getElementById("error_text");
   textBox.innerHTML = textBox.innerHTML + error + "<br/><br/>";
}

function getBuffers(gl) {
   return {
      vertex: gl.createBuffer(),
      normal: gl.createBuffer(),
      index: gl.createBuffer()
   };
}

function getShaders(callback) {
   loadTextResource("shaders/vertex.glsl", function (vsError, vsResult) {
      if (vsError)
         error(vsError);
      else
         loadTextResource("shaders/fragment.glsl", function (fsError, fsResult) {
            if (fsError)
               error(fsError);
            else {
               callback({
                  vertexShader: vsResult,
                  fragmentShader: fsResult
               });
            }
         });
   });
}

/************************************************
 * LOAD TEXT RESOURCE
 * Passes an error or, if no error, null and a 
 * result into callback.
 ************************************************/
function loadTextResource(url, callback) {
   let request = new XMLHttpRequest();
   request.open("GET", url, true);
   request.onload = function () {
      if (request.status < 200 || request.status > 299) {
         callback("Error: HTTP Status " + request.status + " on resource " + url);
      } else {
         callback(null, request.responseText);
      }
   };
   request.send();
}

/************************************************
 * GET SHADER PROGRAM
 * Compiles, links, and validates a shader program
 * to be used through the entire application.
 ************************************************/
function getShaderProgramInfo(gl, shaders) {
   let prog = gl.createProgram();
   let vertex = gl.createShader(gl.VERTEX_SHADER);
   let fragment = gl.createShader(gl.FRAGMENT_SHADER);

   gl.shaderSource(vertex, shaders.vertexShader);
   gl.shaderSource(fragment, shaders.fragmentShader);

   compileShaderOrError(gl, vertex);
   compileShaderOrError(gl, fragment);

   gl.attachShader(prog, vertex);
   gl.attachShader(prog, fragment);

   gl.linkProgram(prog);
   if (!gl.getProgramParameter(prog, gl.LINK_STATUS))
      error(gl.getProgramInfoLog(prog));

   gl.validateProgram(prog);
   if (!gl.getProgramParameter(prog, gl.VALIDATE_STATUS))
      error(gl.getProgramInfoLog(prog));

   let vertexPosAttrib = gl.getAttribLocation(prog, "vertPos");
   let vertexNormalAttrib = gl.getAttribLocation(prog, "vertNormal");

   let wmLocation = gl.getUniformLocation(prog, "mWorld");
   let vmLocation = gl.getUniformLocation(prog, "mView");
   let pmLocation = gl.getUniformLocation(prog, "mProj");
   let colorLocation = gl.getUniformLocation(prog, "color");

   return {
      program: prog,
      uniformLocations: {
         worldMatrix: wmLocation,
         viewMatrix: vmLocation,
         projMatrix: pmLocation,
         color: colorLocation
      },
      attributeLocations: {
         position: vertexPosAttrib,
         normal: vertexNormalAttrib
      }
   };
}

function compileShaderOrError(gl, shader) {
   gl.compileShader(shader);
   if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
      error(gl.getShaderInfoLog(shader));
   }
}

function initCulling(gl) {
   gl.enable(gl.DEPTH_TEST);
   gl.enable(gl.CULL_FACE);
   gl.cullFace(gl.BACK);
   gl.frontFace(gl.CCW);
}

const VISIBLE_HEIGHT = 1000;

function getMatrices(gl, programInfo) {
   let i = new Float32Array(16);
   let w = new Float32Array(16);
   let v = new Float32Array(16);
   let p = new Float32Array(16);

   mat4.identity(i);
   mat4.identity(w);
   mat4.lookAt(v, [VISIBLE_HEIGHT / 2, VISIBLE_HEIGHT / 2, -cameraZ], [VISIBLE_HEIGHT / 2, VISIBLE_HEIGHT / 2, 0], [0, -1, 0]); // out, eye, center, upAxis
   getPerspectiveMatrix(p);

   bindMatrix(w, programInfo.uniformLocations.worldMatrix);
   bindMatrix(v, programInfo.uniformLocations.viewMatrix);
   bindMatrix(p, programInfo.uniformLocations.projMatrix);

   return {
      world: w,
      view: v,
      proj: p,
      identity: i
   };
}

function bindMatrix(matrix, uniformLocation) {
   window.gl.uniformMatrix4fv(uniformLocation, false, matrix);
}

const fieldOfView = 45;
let cameraZ = VISIBLE_HEIGHT / 2 / Math.tan(fieldOfView * Math.PI / 180 / 2);

function getPerspectiveMatrix(output) {
   mat4.perspective(
      output,
      fieldOfView * Math.PI / 180,
      window.canvas.width / window.canvas.height,
      0.1,
      cameraZ * -1 + 100
   );
}

function initLighting(gl, programInfo) {
   let ambientLightIntensityLocation = gl.getUniformLocation(programInfo.program, "ambientLightIntensity");
   let sunIntensityLocation = gl.getUniformLocation(programInfo.program, "sun.color");
   let sunPositionLocation = gl.getUniformLocation(programInfo.program, "sun.position");

   gl.uniform3f(ambientLightIntensityLocation, 0.85, 0.85, 0.85);
   gl.uniform3f(sunIntensityLocation, 0.15, 0.15, 0.15);
   gl.uniform3f(sunPositionLocation, 0, 0, -1);
}
