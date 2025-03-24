extern crate gl;
extern crate glam;
extern crate glfw;

use gl::types::*;
use std::mem;

/// A struct to handle a mesh for rendering.
pub struct Mesh {
    vertex_array: u32,
    vertex_buffer: u32,
    index_buffer: u32,
    pub(crate) indices_length: usize,
}

impl Mesh {
    /// Creates a new empty mesh. After creation, you can begin to fill it with data afterwards.
    pub fn new() -> Self {
        let mut vertex_array = 0;
        let mut vertex_buffer = 0;
        let mut index_buffer = 0;

        unsafe {
            // VAO
            gl::GenVertexArrays(1, &mut vertex_array);
            gl::BindVertexArray(vertex_array);

            // VBO
            gl::GenBuffers(1, &mut vertex_buffer);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);

            // EBO
            gl::GenBuffers(1, &mut index_buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer);

            gl::BindVertexArray(0);
        }

        Mesh {
            vertex_array,
            vertex_buffer,
            index_buffer,
            indices_length: 0,
        }
    }

    /// Adds a vertex attribute to the mesh.
    pub fn add_vertex_attributes(
        &self,
        attributes: &[(u32, i32, GLenum, bool)],
    ) {
        unsafe {
            gl::BindVertexArray(self.vertex_array);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);

            let stride = attributes.iter().fold(0, |acc, &(_, size, type_, _)| {
                acc + size as usize
                    * match type_ {
                        gl::FLOAT => std::mem::size_of::<f32>(),
                        gl::UNSIGNED_INT => std::mem::size_of::<u32>(),
                        gl::UNSIGNED_BYTE => std::mem::size_of::<u8>(),
                        _ => panic!("Unsupported attribute type"),
                    }
            });

            let mut offset = 0;
            for &(index, size, type_, normalized) in attributes {
                let type_size = match type_ {
                    gl::FLOAT => std::mem::size_of::<f32>(),
                    gl::UNSIGNED_INT => std::mem::size_of::<u32>(),
                    gl::UNSIGNED_BYTE => std::mem::size_of::<u8>(),
                    _ => panic!("Unsupported attribute type"),
                };

                gl::EnableVertexAttribArray(index);
                gl::VertexAttribPointer(
                    index,
                    size,
                    type_,
                    normalized as u8,
                    stride as GLsizei,
                    offset as *const GLvoid,
                );

                offset += size as usize * type_size;
            }

            gl::BindVertexArray(0);
        }
    }

    /// Updates the vertex data.
    pub fn update_vertices(&self, data: &[f32]) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * mem::size_of::<f32>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    /// Updates the indices.
    pub fn update_indices(&mut self, indices: &[u32]) {
        self.indices_length = indices.len();

        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<u32>()) as GLsizeiptr,
                indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    /// Binds the mesh for rendering.
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array);
        }
    }

    /// Unbinds the mesh.
    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    /// Clears all buffers.
    pub fn remesh(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vertex_array);
            gl::DeleteBuffers(1, &self.vertex_buffer);
            gl::DeleteBuffers(1, &self.index_buffer);
        }

        *self = Mesh::new();
    }
}
