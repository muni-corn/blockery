struct CubeMesh {
    pitch_matrix: [f32; 16],
    yaw_matrix: [f32; 16],
    roll_matrix: [f32; 16],
}

impl CubeMesh {
   fn get_vertices() -> [f32; 72] {
       [
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
   }

   fn get_indices() -> [i32; 36] {
       [
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
      ]
   }

   fn get_normals() -> [f32; 72] {
       [
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
   }

   fn set_color<Gl>(rgb: i32, gl: Gl, program_info: ProgramInfo) {
      let color = intToRGB(rgb);
      // console.log(color);
      gl.uniform3f(program_info.uniformLocations.color, color.r, color.g, color.b);
   }

   fn init(gl: Gl, matrices: Matrices, program_info: ProgramInfo) {
      self.matrices = matrices;
      self.matrixUniformLocation = program_info.uniformLocations.viewMatrix;
      self.buffers = Buffers {
         vertex: gl.createBuffer(),
         normal: gl.createBuffer(),
         index: gl.createBuffer()
      };

      gl.bindBuffer(gl.ARRAY_BUFFER, self.buffers.vertex);
      gl.bufferData(gl.ARRAY_BUFFER, Float32Array::new(self.vertices), gl.STATIC_DRAW);
      gl.vertexAttribPointer(program_info.attributeLocations.position, 3, gl.FLOAT, false, 0, 0);
      gl.enableVertexAttribArray(program_info.attributeLocations.position);

      gl.bindBuffer(gl.ARRAY_BUFFER, self.buffers.normal);
      gl.bufferData(gl.ARRAY_BUFFER, Float32Array::new(self.normals), gl.STATIC_DRAW);
      gl.vertexAttribPointer(program_info.attributeLocations.normal, 3, gl.FLOAT, true, 0, 0);
      gl.enableVertexAttribArray(program_info.attributeLocations.normal);

      // Buffer indices into an element array buffer
      gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, self.buffers.index);
      gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, Uint16Array::new(self.indices), gl.STATIC_DRAW);
   }

   fn render(gl: Gl, x: f32, y: f32, z: f32, w: f32, h: f32, d: f32, pit: f32, yaw: f32, rol: f32) {
      mat4.translate(self.matrices.view, self.matrices.identity, [x + w / 2, y + h / 2, z + d / 2]);

      if (w != 2 || h != 2 || d != 2) {
         mat4.scale(self.matrices.view, self.matrices.view, [w / 2, h / 2, d / 2]);
      }

      if (rol != 0) {
         mat4.rotate(self.rollMatrix, self.matrices.identity, rol * TO_RAD, [0, 0, 1]);
         mat4.mul(self.matrices.view, self.matrices.view, self.rollMatrix);
      }

      if (yaw != 0) {
         mat4.rotate(self.yawMatrix, self.matrices.identity, yaw * TO_RAD, [0, 1, 0]);
         mat4.mul(self.matrices.view, self.matrices.view, self.yawMatrix);
      }

      if (pit != 0) {
         mat4.rotate(self.pitchMatrix, self.matrices.identity, pit * TO_RAD, [1, 0, 0]);
         mat4.mul(self.matrices.view, self.matrices.view, self.pitchMatrix);
      }

      bindMatrix(self.matrices.view, self.matrixUniformLocation);

      gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, self.buffers.index);
      gl.drawElements(gl.TRIANGLES, self.indices.length, gl.UNSIGNED_SHORT, 0);
   }
}

const TO_RAD: f32 = f32::consts::PI / 180;
