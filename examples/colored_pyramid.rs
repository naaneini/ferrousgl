use ferrousgl::{GlWindow, Mesh, Shader, WindowConfig};
use glam::{Mat4, Vec3};
use std::path::Path;

fn main() {
    // Create a 800x600 window
    let mut window = GlWindow::new(WindowConfig {
        width: 800,
        height: 600,
        title: "Colored Pyramid".to_owned(),
        ..Default::default()
    });

    let shader = Shader::new_from_file(
        Path::new("./examples/shaders/colored_pyramid/vertex.glsl"),
        Path::new("./examples/shaders/colored_pyramid/fragment.glsl"),
    ).unwrap();

    let mut mesh = Mesh::new();

    let vertices = [
        // Positions          // Colors
        // Apex (top point)
        0.0, 0.5, 0.0, 1.0, 0.0, 0.0, // red
        // Base vertices (square)
        0.5, -0.5, 0.5, 0.0, 1.0, 0.0, // green - front right
        -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, // blue - front left
        -0.5, -0.5, -0.5, 1.0, 1.0, 0.0, // yellow - back left
        0.5, -0.5, -0.5, 1.0, 0.0, 1.0, // purple - back right
    ];

    let indices = [
        // 4 triangular faces
        0, 1, 2, // front face
        0, 2, 3, // left face
        0, 3, 4, // back face
        0, 4, 1, // right face
        // Square base (2 triangles)
        1, 2, 3, 1, 3, 4,
    ];

    mesh.update_vertices(&vertices);
    mesh.update_indices(&indices);
    mesh.add_vertex_attributes(&[
        (0, 3, gl::FLOAT, false), // position
        (1, 3, gl::FLOAT, false), // color
    ]);

    // Projection
    let aspect_ratio = 800.0 / 600.0;
    let projection = Mat4::perspective_rh_gl(45.0f32.to_radians(), aspect_ratio, 0.1, 100.0);

    // View matrix
    let view = Mat4::look_at_rh(
        Vec3::new(0.0, 0.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    while !window.should_window_close() {
        window.clear_color(Vec3::new(0.0, 0.30, 0.0));
        window.clear_depth();
        let model = Mat4::from_rotation_y(window.get_mouse_position().0 as f32 * 0.01);

        shader.bind_program();

        // Set matrices
        shader.set_uniform_matrix_4fv("projection", projection.to_cols_array().as_ref());
        shader.set_uniform_matrix_4fv("view", view.to_cols_array().as_ref());
        shader.set_uniform_matrix_4fv("model", model.to_cols_array().as_ref());

        // Draw the mesh
        window.render_mesh(&mesh);

        shader.unbind_program();

        window.update();
    }
}
