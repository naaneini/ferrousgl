extern crate gl;

use gl::types::*;
use std::ffi::CString;
use std::ptr;

use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Represents a shader to be used for telling the GPU how to "fill in" a meshes vertices.
pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    /// Creates a new shader using a vertex shader string and fragment string to create the shader.
    /// After this it is ready to be used for rendering.
    pub fn new_from_source(vertex_source: &str, fragment_source: &str) -> Self {
        let vertex_shader = Shader::compile_shader(gl::VERTEX_SHADER, vertex_source).unwrap();
        let fragment_shader = Shader::compile_shader(gl::FRAGMENT_SHADER, fragment_source).unwrap();
        let shader_program = Shader::link_program(vertex_shader, fragment_shader).unwrap();

        unsafe {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Shader { id: shader_program }
    }

    /// Creates a new shader using a vertex shader file path and fragment file path.
    /// After this it is ready to be used for rendering.
    pub fn new_from_file(vertex_path: &Path, fragment_path: &Path) -> Result<Self, String> {
        let mut vertex_file = File::open(vertex_path)
            .map_err(|e| format!("[FerrousGl Error] Failed to open vertex shader file: {}", e))?;
        let mut vertex_source = String::new();
        vertex_file.read_to_string(&mut vertex_source)
            .map_err(|e| format!("[FerrousGl Error] Failed to read vertex shader file: {}", e))?;

        let mut fragment_file = File::open(fragment_path)
            .map_err(|e| format!("[FerrousGl Error] Failed to open fragment shader file: {}", e))?;
        let mut fragment_source = String::new();
        fragment_file.read_to_string(&mut fragment_source)
            .map_err(|e| format!("[FerrousGl Error] Failed to read fragment shader file: {}", e))?;

        Ok(Self::new_from_source(&vertex_source, &fragment_source))
    }

    /// Recompiles the shader from the given vertex and fragment shader files.
    /// Returns Ok(()) on success, or an error message if compilation fails.
    pub fn recompile_from_file(&mut self, vertex_path: &Path, fragment_path: &Path) -> Result<(), String> {
        match Self::new_from_file(vertex_path, fragment_path) {
            Ok(new_shader) => {
                unsafe { gl::DeleteProgram(self.id) };
                self.id = new_shader.id;
                std::mem::forget(new_shader);
                Ok(())
            },
            Err(e) => {
                eprintln!("[Shader Recompile Error] {}", e);
                Err(e)
            }
        }
    }

    /// Internal function to compile a shader.
    fn compile_shader(shader_type: GLenum, source: &str) -> Result<GLuint, String> {
        let shader = unsafe { gl::CreateShader(shader_type) };
        let c_str = CString::new(source.as_bytes()).unwrap();
        unsafe {
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);
        }

        let mut success = 1;
        unsafe {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len = 0;
            unsafe {
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(shader, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(shader)
    }

    /// Internal function to link the shader program.
    fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> Result<GLuint, String> {
        let program = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
        }

        let mut success = 1;
        unsafe {
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len = 0;
            unsafe {
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(program, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(program)
    }

    /// Binds or begins to use the shader program. All rendering after this will use this shader program.
    /// If no shader program is bound, the GPU will now know how to render the vertices, so nothing will show on screen.
    pub fn bind_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    /// Unbinds or stops to use the shader program. All rendering done after this will not use any shader program, 
    /// which means not binding a new shader program and trying to render something will result in nothing being shown on screen.
    /// Always undinding shaders after they are used, will help prevent bugs and is generally good for performance.
    pub fn unbind_program(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    /// Sets a single integer uniform with a name and value. The name of the value should be the same in code and in the shader code.
    pub fn set_uniform_1i(&self, name: &str, value: i32) {
        let cname = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.id, cname.as_ptr());
            if location != -1 {
                gl::Uniform1i(location, value);
            }
        }
    }

    /// Set a single float uniform with a name and value. The name of the value should be the same in code and in the shader code.
    pub fn set_uniform_1f(&self, name: &str, value: f32) {
        let cname = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.id, cname.as_ptr());
            if location != -1 {
                gl::Uniform1f(location, value);
            }
        }
    }

    /// Set a vector of 2 float uniforms with a name and value. The name of the value should be the same in code and in the shader code.
    pub fn set_uniform_2f(&self, name: &str, value_0: f32, value_1: f32) {
        let cname = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.id, cname.as_ptr());
            if location != -1 {
                gl::Uniform2f(location, value_0, value_1);
            }
        }
    }

    /// Set a vector of 3 float uniforms with a name and value. The name of the value should be the same in code and in the shader code.
    pub fn set_uniform_3f(&self, name: &str, v0: f32, v1: f32, v2: f32) {
        let cname = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.id, cname.as_ptr());
            if location != -1 {
                gl::Uniform3f(location, v0, v1, v2);
            }
        }
    }

    /// Set a vector of 4 float uniforms with a name and value. The name of the value should be the same in code and in the shader code.
    pub fn set_uniform_4f(&self, name: &str, v0: f32, v1: f32, v2: f32, v3: f32) {
        let cname = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.id, cname.as_ptr());
            if location != -1 {
                gl::Uniform4f(location, v0, v1, v2, v3);
            }
        }
    }

    // Sets a 4x4 matrix uniform with a name and value. The name of the value should be the same in code and in the shader code.
    pub fn set_uniform_matrix_4fv(&self, name: &str, matrix: &[f32]) {
        let cname = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.id, cname.as_ptr());
            if location != -1 {
                gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
            }
        }
    }

    /// Sets a texture sampler uniform with a name and value. The name of the value should be the same in code and in the shader code.
    pub fn set_uniform_texture(&self, name: &str, texture_unit: u32) {
        let cname = CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.id, cname.as_ptr());
            if location != -1 {
                gl::Uniform1i(location, texture_unit as i32);
            }
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
