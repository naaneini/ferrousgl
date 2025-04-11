use ferrousgl::{GlWindow, WindowConfig, WindowKey, Shader, Mesh};
use glam::{Mat4, Vec3};
use std::path::Path;

fn main() {
    // Create a default window with custom title
    let config = WindowConfig {
        title: "Realistic Water Example".to_string(),
        ..Default::default()
    };
    let mut window = GlWindow::new(config);

    // Load shaders
    let water_shader = Shader::new_from_file(
        Path::new("./examples/shaders/simple_water/vertex.glsl"),
        Path::new("./examples/shaders/simple_water/fragment.glsl"),
    ).unwrap();

    let floor_shader = Shader::new_from_file(
        Path::new("./examples/shaders/simple_water/vertex_floor.glsl"),
        Path::new("./examples/shaders/simple_water/fragment_floor.glsl"),
    ).unwrap();

    // Create water mesh (same as before)
    let grid_size = 40;  // Increased size for better effect
    let grid_spacing = 0.5;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Generate grid vertices and normals
    for z in 0..=grid_size {
        for x in 0..=grid_size {
            let x_pos = (x as f32 - grid_size as f32 / 2.0) * grid_spacing;
            let z_pos = (z as f32 - grid_size as f32 / 2.0) * grid_spacing;
            
            vertices.push(x_pos);
            vertices.push(0.0);
            vertices.push(z_pos);
            
            vertices.push(0.0);
            vertices.push(1.0);
            vertices.push(0.0);
        }
    }

    // Generate indices
    for z in 0..grid_size {
        for x in 0..grid_size {
            let top_left = z * (grid_size + 1) + x;
            let top_right = top_left + 1;
            let bottom_left = (z + 1) * (grid_size + 1) + x;
            let bottom_right = bottom_left + 1;
            
            indices.push(top_left as u32);
            indices.push(bottom_left as u32);
            indices.push(top_right as u32);
            
            indices.push(top_right as u32);
            indices.push(bottom_left as u32);
            indices.push(bottom_right as u32);
        }
    }

    let mut water_mesh = Mesh::new();
    water_mesh.add_vertex_attributes(&[
        (0, 3, gl::FLOAT, false), // position
        (1, 3, gl::FLOAT, false), // normal
    ]);
    water_mesh.update_vertices(&vertices);
    water_mesh.update_indices(&indices);

    // Create floor mesh (simple plane below the water)
    let floor_size = grid_size as f32 * grid_spacing * 1.5;
    let floor_vertices = vec![
        -floor_size, -1.0, -floor_size, 0.0, 1.0, 0.0, 0.0, 0.0,
        floor_size, -1.0, -floor_size, 0.0, 1.0, 0.0, 1.0, 0.0,
        floor_size, -1.0, floor_size, 0.0, 1.0, 0.0, 1.0, 1.0,
        -floor_size, -1.0, floor_size, 0.0, 1.0, 0.0, 0.0, 1.0,
    ];
    let floor_indices = vec![0, 1, 2, 0, 2, 3];

    let mut floor_mesh = Mesh::new();
    floor_mesh.add_vertex_attributes(&[
        (0, 3, gl::FLOAT, false), // position
        (1, 3, gl::FLOAT, false), // normal
        (2, 2, gl::FLOAT, false), // tex coords
    ]);
    floor_mesh.update_vertices(&floor_vertices);
    floor_mesh.update_indices(&floor_indices);

    // Camera setup
    let mut camera_pos = Vec3::new(0.0, 2.0, 5.0);
    let camera_target = Vec3::new(0.0, 0.0, 0.0);
    let camera_up = Vec3::new(0.0, 1.0, 0.0);

    // Light setup
    let light_pos = Vec3::new(2.0, 5.0, 2.0);
    let light_color = Vec3::new(1.0, 1.0, 1.0);
    let water_color = Vec3::new(0.1, 0.3, 0.8);
    let floor_color = Vec3::new(0.3, 0.2, 0.1);

    // Enable blending for transparency
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::DEPTH_TEST);
    }

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

        window.clear_color(Vec3::new(0.53, 0.81, 0.92)); // Sky blue background
        window.clear_depth();

        // Set up matrices
        let view = Mat4::look_at_rh(camera_pos, camera_target, camera_up);
        let (width, height) = window.get_window_size();
        let projection = Mat4::perspective_rh(45.0f32.to_radians(), width as f32 / height as f32, 0.1, 100.0);
        
        // Render floor first
        let floor_model = Mat4::IDENTITY;
        floor_shader.bind_program();
        floor_shader.set_uniform_matrix_4fv("model", &floor_model.to_cols_array());
        floor_shader.set_uniform_matrix_4fv("view", &view.to_cols_array());
        floor_shader.set_uniform_matrix_4fv("projection", &projection.to_cols_array());
        floor_shader.set_uniform_3f("lightPos", light_pos.x, light_pos.y, light_pos.z);
        floor_shader.set_uniform_3f("viewPos", camera_pos.x, camera_pos.y, camera_pos.z);
        floor_shader.set_uniform_3f("lightColor", light_color.x, light_color.y, light_color.z);
        floor_shader.set_uniform_3f("objectColor", floor_color.x, floor_color.y, floor_color.z);
        window.render_mesh(&floor_mesh);

        // Render water with transparency
        let water_model = Mat4::IDENTITY;
        water_shader.bind_program();
        water_shader.set_uniform_matrix_4fv("model", &water_model.to_cols_array());
        water_shader.set_uniform_matrix_4fv("view", &view.to_cols_array());
        water_shader.set_uniform_matrix_4fv("projection", &projection.to_cols_array());
        water_shader.set_uniform_1f("time", time);
        water_shader.set_uniform_3f("lightPos", light_pos.x, light_pos.y, light_pos.z);
        water_shader.set_uniform_3f("viewPos", camera_pos.x, camera_pos.y, camera_pos.z);
        water_shader.set_uniform_3f("lightColor", light_color.x, light_color.y, light_color.z);
        water_shader.set_uniform_3f("objectColor", water_color.x, water_color.y, water_color.z);
        window.render_mesh(&water_mesh);

        window.update();
    }
}