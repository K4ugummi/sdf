// This file is part of Carambolage.

// Carambolage is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Carambolage is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Carambolage.  If not, see <http://www.gnu.org/licenses/>.
use gl;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::mem::size_of;
use std::os::raw::c_void;
use std::time::Instant;

#[repr(C)]
pub struct ScreenVertex {
    pub position: [f32; 2],
}

/// Compiled GLSL Shader Program.
pub struct Shader {
    pub id: u32,

    vao: u32,
    vbo: u32,

    start_time: Instant,
}

impl Shader {
    pub fn new() -> Shader {
        let mut shader = Shader {
            id: 0,
            vao: 0,
            vbo: 0,
            start_time: Instant::now(),
        };

        let vertex_bytes = include_str!("../sdf.vs");
        let vertex_code = CString::new(vertex_bytes).unwrap();

        let fragment_bytes = include_str!("../sdf.fs");
        let fragment_code = CString::new(fragment_bytes).unwrap();

        // Try to compile both shaders.
        unsafe {
            // Compile vertex shader.
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vertex_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.check_compile_errors(vertex, "VertexShader");

            // Compile fragment Shader.
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &fragment_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.check_compile_errors(fragment, "FragmentShader");

            // Create program from vertex and fragment shader.
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            shader.check_compile_errors(id, "ShaderProgram");

            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.id = id;

            let vertices: [ScreenVertex; 6] = [
                ScreenVertex {
                    position: [-1.0, -1.0],
                },
                ScreenVertex {
                    position: [1.0, -1.0],
                },
                ScreenVertex {
                    position: [1.0, 1.0],
                },
                ScreenVertex {
                    position: [1.0, 1.0],
                },
                ScreenVertex {
                    position: [-1.0, 1.0],
                },
                ScreenVertex {
                    position: [-1.0, -1.0],
                },
            ];
            gl::GenVertexArrays(1, &mut shader.vao);
            gl::GenBuffers(1, &mut shader.vbo);
            // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
            gl::BindVertexArray(shader.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, shader.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<ScreenVertex>()) as isize,
                &vertices[0] as *const ScreenVertex as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                size_of::<ScreenVertex>() as i32,
                ptr::null(),
            );

            gl::BindVertexArray(0);
        }

        shader
    }

    /// Bind the shader program.
    pub fn draw(&self, width: f32, height: f32) {
        let run_time = Instant::now() - self.start_time;
        let run_time_s = run_time.as_secs() as f32 + run_time.subsec_nanos() as f32 / 1_000_000_000.0;

        unsafe {
            gl::UseProgram(self.id);
            gl::Uniform1f(0, run_time_s);
            gl::Uniform2f(1, width, height);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }
    }

    ///
    unsafe fn check_compile_errors(&self, shader: u32, shader_type: &str) {
        let mut success = i32::from(gl::FALSE);
        let mut info_log = Vec::with_capacity(1024);
        info_log.set_len(1024 - 1);

        if shader_type != "ShaderProgram" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != i32::from(gl::TRUE) {
                // i8 is a GLchar
                gl::GetShaderInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut i8,
                );
                println!(
                    "Compilation error of type: {}\nInfo log:\n{}",
                    shader_type,
                    str::from_utf8(&info_log).unwrap_or("UNKNOWN")
                );
            }
        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != i32::from(gl::TRUE) {
                gl::GetProgramInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut i8,
                );
                println!(
                    "Linking error of type: {}\nInfo log:\n{}",
                    shader_type,
                    str::from_utf8(&info_log).unwrap_or("UNKNOWN")
                );
            }
        }
    }
}
