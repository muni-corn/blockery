/* jshint esversion: 6, browser: true, devel: true */
/* global mat4, glMatrix, main, aFunctionInMain */
function init() {
   window.canvas = document.getElementById("glCanvas");
   window.gl = window.canvas.getContext("webgl");


   let gl = window.gl;
   let canvas = window.canvas;

   let shaders;
   let buffers;
   let matrices;
   let programInfo;
   let cubeMesh;

   window.onresize = () => {
      window.canvas.width = window.innerWidth;
      window.canvas.height = window.innerHeight;
      gl.viewport(0, 0, canvas.width, canvas.height);
      getPerspectiveMatrix(matrices.proj);
      bindMatrix(matrices.proj, programInfo.uniformLocations.projMatrix);
   };

   getShaders((result) => {
      shaders = result;
      programInfo = getShaderProgramInfo(gl, shaders);

      gl.useProgram(programInfo.program);

      buffers = getBuffers(gl);
      initCulling(gl);
      matrices = getMatrices(gl, programInfo);
      initLighting(gl, programInfo);
      cubeMesh = getCubeMesh();
      fillBuffers(gl, buffers, cubeMesh);

      window.onresize();

      main(canvas, gl, programInfo, matrices, buffers, cubeMesh);
   });

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
   loadTextResource("/shaders/vertex.glsl", function (vsError, vsResult) {
      if (vsError)
         error(vsError);
      else
         loadTextResource("/shaders/fragment.glsl", function (fsError, fsResult) {
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

function getMatrices(gl, programInfo) {
   let i = new Float32Array(16);
   let w = new Float32Array(16);
   let v = new Float32Array(16);
   let p = new Float32Array(16);

   mat4.identity(i);
   mat4.identity(w);
   let z = 500 / Math.tan(fieldOfView * Math.PI / 180 / 2);
   mat4.lookAt(v, [500, 500, -z], [500, 500, 0], [0, -1, 0]); // out, eye, center, upAxis
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

const fieldOfView = 70;

function getPerspectiveMatrix(output) {
   mat4.perspective(output, fieldOfView * Math.PI / 180, window.canvas.width / window.canvas.height, 10, 800);
}

function initLighting(gl, programInfo) {
   let ambientLightIntensityLocation = gl.getUniformLocation(programInfo.program, "ambientLightIntensity");
   let sunIntensityLocation = gl.getUniformLocation(programInfo.program, "sun.color");
   let sunPositionLocation = gl.getUniformLocation(programInfo.program, "sun.position");

   gl.uniform3f(ambientLightIntensityLocation, 0.85, 0.85, 0.85);
   gl.uniform3f(sunIntensityLocation, 0.15, 1.15, 0.15);
   gl.uniform3f(sunPositionLocation, 0, 0, -1);
}

function getCubeMesh() {
   let v = [
      1, 1, -1,
      1, -1, -1,
      -1, -1, -1,
      -1, 1, -1,

      1, 1, 1,
      -1, 1, 1,
      -1, -1, 1,
      1, -1, 1,

      1, 1, -1,
      1, 1, 1,
      1, -1, 1,
      1, -1, -1,

      1, -1, -1,
      1, -1, 1,
      -1, -1, 1,
      -1, -1, -1,

      -1, -1, -1,
      -1, -1, 1,
      -1, 1, 1,
      -1, 1, -1,

      1, 1, 1,
      1, 1, -1,
      -1, 1, -1,
      -1, 1, 1
   ];

   let i = [
      // Top
      0, 1, 2,
      0, 2, 3,

      // Left
      4, 5, 6,
      4, 6, 7,

      // Right
      8, 9, 10,
      8, 10, 11,

      // Front
      12, 13, 14,
      12, 14, 15,

      // Back
      16, 17, 18,
      16, 18, 19,

      // Bottom
      20, 21, 22,
      20, 22, 23
   ];

   let n = [
      0, 0, -1,
      0, 0, -1,
      0, 0, -1,
      0, 0, -1,

      0, 0, 1,
      0, 0, 1,
      0, 0, 1,
      0, 0, 1,

      1, 0, 0,
      1, 0, 0,
      1, 0, 0,
      1, 0, 0,

      0, -1, 0,
      0, -1, 0,
      0, -1, 0,
      0, -1, 0,

      -1, 0, 0,
      -1, 0, 0,
      -1, 0, 0,
      -1, 0, 0,

      0, 1, 0,
      0, 1, 0,
      0, 1, 0,
      0, 1, 0,
   ];

   return {
      vertices: v,
      indicies: i,
      normals: n
   };
}

function fillBuffers(gl, buffers, mesh) {
   gl.bindBuffer(gl.ARRAY_BUFFER, buffers.vertex);
   gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(mesh.vertices), gl.STATIC_DRAW);

   gl.bindBuffer(gl.ARRAY_BUFFER, buffers.normal);
   gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(mesh.normals), gl.STATIC_DRAW);

   // Buffer indices into an element array buffer
   gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, buffers.index);
   gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Uint16Array(mesh.indicies), gl.STATIC_DRAW);

   // Make sure that any buffer calls after this do not affect anything here
   gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null);
   gl.bindBuffer(gl.ARRAY_BUFFER, null);
}
