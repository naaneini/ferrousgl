use ferrousgl::{BlendMode, GlWindow, Mesh, Shader, Texture, WindowConfig, WindowKey};
use glam::{Mat4, Vec3};
use std::path::Path;

fn main() {
    // Create a 800x600 window
    let mut window = GlWindow::new(WindowConfig {
        width: 800,
        height: 600,
        title: "Textured Cube with Transparency (Numbers 1-4 to change Blend Mode)".to_owned(),
        ..Default::default()
    });

    let shader = Shader::new_from_file(
        Path::new("./examples/shaders/textured_cube/vertex.glsl"),
        Path::new("./examples/shaders/textured_cube/fragment.glsl"),
    )
    .unwrap();

    // Load a texture with transparency (e.g., PNG with alpha channel)
    let texture = Texture::new_from_file("examples/assets/transparent_texture.png").unwrap();

    let mut mesh = Mesh::new();

    let vertices = [
        // positions        // texture coords
        // bottom face
        0.5, 0.5, -0.5, 1.0, 1.0, // top right
        0.5, -0.5, -0.5, 1.0, 0.0, // bottom right
        -0.5, -0.5, -0.5, 0.0, 0.0, // bottom left
        -0.5, 0.5, -0.5, 0.0, 1.0, // top left
        // top face
        0.5, 0.5, 0.5, 0.0, 0.0, // top right
        0.5, -0.5, 0.5, 0.0, 1.0, // bottom right
        -0.5, -0.5, 0.5, 1.0, 1.0, // bottom left
        -0.5, 0.5, 0.5, 1.0, 0.0, // top left
    ];

    let indices = [
        // Bottom face (already defined)
        0, 1, 3, // first triangle
        1, 2, 3, // second triangle
        // Top face
        4, 5, 7, // first triangle
        5, 6, 7, // second triangle
        // Front face
        4, 0, 7, // first triangle
        0, 3, 7, // second triangle
        // Back face
        5, 1, 6, // first triangle
        1, 2, 6, // second triangle
        // Right face
        4, 5, 0, // first triangle
        5, 1, 0, // second triangle
        // Left face
        7, 6, 3, // first triangle
        6, 2, 3, // second triangle
    ];

    mesh.update_vertices(&vertices);
    mesh.update_indices(&indices);
    mesh.add_vertex_attributes(&[
        (0, 3, gl::FLOAT, false), // position
        (1, 2, gl::FLOAT, false), // texture coord
    ]);

    // Create perspective projection
    let aspect_ratio = 800.0 / 600.0;
    let projection = Mat4::perspective_rh_gl(45.0f32.to_radians(), aspect_ratio, 0.1, 100.0);

    let view = Mat4::look_at_rh(
        Vec3::new(0.0, 0.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut y_rotation = 0.0f32;
    let mut x_rotation = 0.0f32;

    // Start with no blending (opaque)
    let mut current_blend_mode = BlendMode::None;
    window.set_blend_mode(current_blend_mode);

    while !window.should_window_close() {
        window.clear_color(Vec3::new(0.0, 0.5, 0.5));
        window.clear_depth();

        // Check for key presses to change blend mode
        if window.is_key_pressed(WindowKey::Num1) {
            current_blend_mode = BlendMode::None;
            window.set_blend_mode(current_blend_mode);
            println!("Blend mode: None (Opaque)");
        } else if window.is_key_pressed(WindowKey::Num2) {
            current_blend_mode = BlendMode::Alpha;
            window.set_blend_mode(current_blend_mode);
            println!("Blend mode: Alpha (Standard transparency)");
        } else if window.is_key_pressed(WindowKey::Num3) {
            current_blend_mode = BlendMode::Additive;
            window.set_blend_mode(current_blend_mode);
            println!("Blend mode: Additive");
        } else if window.is_key_pressed(WindowKey::Num4) {
            current_blend_mode = BlendMode::Multiplicative;
            window.set_blend_mode(current_blend_mode);
            println!("Blend mode: Multiplicative");
        }

        // Update rotation
        y_rotation += 0.01;
        x_rotation += 0.005;
        let model = Mat4::from_rotation_y(y_rotation) * Mat4::from_rotation_x(x_rotation);

        texture.bind(0);

        shader.bind_program();
        shader.set_uniform_texture("ourTexture", 0);

        // Set matrices
        shader.set_uniform_matrix_4fv("projection", projection.to_cols_array().as_ref());
        shader.set_uniform_matrix_4fv("view", view.to_cols_array().as_ref());
        shader.set_uniform_matrix_4fv("model", model.to_cols_array().as_ref());

        // Draw the textured cube
        window.render_mesh(&mesh);

        shader.unbind_program();
        texture.unbind();

        window.update();
    }
}
