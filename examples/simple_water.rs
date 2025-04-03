use ferrousgl::{GlWindow, WindowConfig, WindowKey, Shader, Mesh};
use glam::{Mat4, Vec3};
use std::path::Path;

fn main() {
    // Create a default window with custom title
    let config = WindowConfig {
        title: "Water Example with simple camera movement".to_string(),
        ..Default::default()
    };
    let mut window = GlWindow::new(config);

    let shader = Shader::new_from_file(
        Path::new("./examples/shaders/simple_water/vertex.glsl"),
        Path::new("./examples/shaders/simple_water/fragment.glsl"),
    ).unwrap();

    // Create a grid mesh
    let grid_size = 20;
    let grid_spacing = 0.5;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Generate grid vertices and normals (initially flat)
    for z in 0..=grid_size {
        for x in 0..=grid_size {
            let x_pos = (x as f32 - grid_size as f32 / 2.0) * grid_spacing;
            let z_pos = (z as f32 - grid_size as f32 / 2.0) * grid_spacing;
            
            // Positions
            vertices.push(x_pos);
            vertices.push(0.0);   // y will be displaced in shader
            vertices.push(z_pos);
            
            // Normals
            vertices.push(0.0);
            vertices.push(1.0);
            vertices.push(0.0);
        }
    }

    // Generate indices for triangles
    for z in 0..grid_size {
        for x in 0..grid_size {
            let top_left = z * (grid_size + 1) + x;
            let top_right = top_left + 1;
            let bottom_left = (z + 1) * (grid_size + 1) + x;
            let bottom_right = bottom_left + 1;
            
            // First triangle
            indices.push(top_left as u32);
            indices.push(bottom_left as u32);
            indices.push(top_right as u32);
            
            // Second triangle
            indices.push(top_right as u32);
            indices.push(bottom_left as u32);
            indices.push(bottom_right as u32);
        }
    }

    let mut mesh = Mesh::new();
    mesh.add_vertex_attributes(&[
        (0, 3, gl::FLOAT, false), // position
        (1, 3, gl::FLOAT, false), // normal
    ]);
    mesh.update_vertices(&vertices);
    mesh.update_indices(&indices);

    // Camera setup
    let mut camera_pos = Vec3::new(0.0, 2.0, 5.0);
    let camera_target = Vec3::new(0.0, 0.0, 0.0);
    let camera_up = Vec3::new(0.0, 1.0, 0.0);

    // Light setup
    let light_pos = Vec3::new(2.0, 5.0, 2.0);
    let light_color = Vec3::new(1.0, 1.0, 1.0);
    let object_color = Vec3::new(0.1, 0.3, 0.8);

    // Main loop
    let mut time = 0.0;
    while !window.should_window_close() {
        time += 0.01;

        // Input for camera
        if window.is_key_held(WindowKey::W) {
            camera_pos.z -= 0.1;
        }
        if window.is_key_held(WindowKey::S) {
            camera_pos.z += 0.1;
        }
        if window.is_key_held(WindowKey::A) {
            camera_pos.x -= 0.1;
        }
        if window.is_key_held(WindowKey::D) {
            camera_pos.x += 0.1;
        }
        if window.is_key_held(WindowKey::Q) {
            camera_pos.y -= 0.1;
        }
        if window.is_key_held(WindowKey::E) {
            camera_pos.y += 0.1;
        }

        window.clear_color(Vec3::new(0.1, 0.1, 0.1));
        window.clear_depth();

        // Set up matrices
        let view = Mat4::look_at_rh(camera_pos, camera_target, camera_up);
        let (width, height) = window.get_window_size();
        let projection = Mat4::perspective_rh(45.0f32.to_radians(), width as f32 / height as f32, 0.1, 100.0);
        let model = Mat4::IDENTITY;

        // Set the shaders uniforms
        shader.bind_program();
        shader.set_uniform_matrix_4fv("model", &model.to_cols_array());
        shader.set_uniform_matrix_4fv("view", &view.to_cols_array());
        shader.set_uniform_matrix_4fv("projection", &projection.to_cols_array());
        shader.set_uniform_1f("time", time);
        shader.set_uniform_3f("lightPos", light_pos.x, light_pos.y, light_pos.z);
        shader.set_uniform_3f("viewPos", camera_pos.x, camera_pos.y, camera_pos.z);
        shader.set_uniform_3f("lightColor", light_color.x, light_color.y, light_color.z);
        shader.set_uniform_3f("objectColor", object_color.x, object_color.y, object_color.z);

        // Render the water mesh
        window.render_mesh(&mesh);

        window.update();
    }
}