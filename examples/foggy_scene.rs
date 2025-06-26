use ferrousgl::{WindowConfig, GlWindow, Mesh, Shader, Texture};
use glam::{Mat4, Vec3, Vec4};
use std::path::Path;

fn main() {
    // Create a 800x600 window
    let mut window = GlWindow::new(
        WindowConfig {
            width: 800,
            height: 600,
            title: "Plane with Stacked Cubes".to_owned(),
            ..Default::default()
        }
    );
    
    let shader = Shader::new_from_file(
        Path::new("./examples/shaders/foggy_scene/vertex.glsl"),
        Path::new("./examples/shaders/foggy_scene/fragment.glsl"),
    ).unwrap();
    
    let texture = Texture::new_from_file(Path::new("examples/assets/wood_texture.png")).unwrap();
    texture.bind(0);
    texture.set_mipmap_and_filtering(ferrousgl::MipmapType::Linear, ferrousgl::FilterMode::Nearest);
    
    // Create cube mesh
    let mut cube_mesh = Mesh::new();
    let cube_vertices = [
        // positions        // texture coords
        // bottom face
         0.5,  0.5, -0.5,   1.0, 1.0,   // top right
         0.5, -0.5, -0.5,   1.0, 0.0,   // bottom right
        -0.5, -0.5, -0.5,   0.0, 0.0,   // bottom left
        -0.5,  0.5, -0.5,   0.0, 1.0,   // top left
        
        // top face
         0.5,  0.5,  0.5,   0.0, 0.0,   // top right
         0.5, -0.5,  0.5,   0.0, 1.0,   // bottom right
        -0.5, -0.5,  0.5,   1.0, 1.0,   // bottom left
        -0.5,  0.5,  0.5,   1.0, 0.0   // top left
    ];
    
    let cube_indices = [
        // Bottom face
        0, 1, 3, 1, 2, 3,
        // Top face
        4, 5, 7, 5, 6, 7,
        // Front face
        4, 0, 7, 0, 3, 7,
        // Back face
        5, 1, 6, 1, 2, 6,
        // Right face
        4, 5, 0, 5, 1, 0,
        // Left face
        7, 6, 3, 6, 2, 3
    ];
    
    cube_mesh.update_vertices(&cube_vertices);
    cube_mesh.update_indices(&cube_indices);
    cube_mesh.add_vertex_attributes(&[
        (0, 3, gl::FLOAT, false),  // position
        (1, 2, gl::FLOAT, false)   // texture coord
    ]);
    
    // Create plane mesh
    let mut plane_mesh = Mesh::new();
    let plane_vertices = [
        // positions        // texture coords (scaled up for tiling)
         5.0, -0.5,  5.0,   10.0,  0.0,  // bottom right
         5.0, -0.5, -5.0,   10.0, 10.0,  // top right
        -5.0, -0.5, -5.0,    0.0, 10.0,  // top left
        -5.0, -0.5,  5.0,    0.0,  0.0   // bottom left
    ];
    
    let plane_indices = [
        0, 1, 3,  // first triangle
        1, 2, 3   // second triangle
    ];
    
    plane_mesh.update_vertices(&plane_vertices);
    plane_mesh.update_indices(&plane_indices);
    plane_mesh.add_vertex_attributes(&[
        (0, 3, gl::FLOAT, false),  // position
        (1, 2, gl::FLOAT, false)   // texture coord
    ]);
    
    // Create perspective projection
    let aspect_ratio = 800.0 / 600.0;
    let projection = Mat4::perspective_rh_gl(45.0f32.to_radians(), aspect_ratio, 0.1, 100.0);
    
    let mut view = Mat4::look_at_rh(
        Vec3::new(5.0, 5.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let rotation = 0.0f32;
    let camera_distance = 10.0;
    
    // Positions for stacked cubes
    let cube_positions = [
        Vec3::new(0.0, 0.0, 0.0),   // On top of plane
    ];
    
    while !window.should_window_close() {
        window.clear_color(Vec4::new(0.52, 0.67, 1.0, 1.0));
        window.clear_depth();
        
        // Update camera position
        let camera_x = camera_distance * rotation.sin();
        let camera_z = camera_distance * rotation.cos();
        view = Mat4::look_at_rh(
            Vec3::new(camera_x, 5.0, camera_z),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        
        texture.bind(0);
        shader.bind_program();
        shader.set_uniform_texture("ourTexture", 0);

        window.get_depth_texture().bind(1);
        shader.set_uniform_texture("depthTexture", 1);

        // Set matrices
        shader.set_uniform_matrix_4fv("projection", projection.to_cols_array().as_ref());
        shader.set_uniform_matrix_4fv("view", view.to_cols_array().as_ref());
        shader.set_uniform_3f("cameraPos", camera_x, 5.0, camera_z);
        
        // Draw plane (no rotation)
        let plane_model = Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0));
        shader.set_uniform_matrix_4fv("model", plane_model.to_cols_array().as_ref());
        window.render_mesh(&plane_mesh);
        
        // Draw all cubes
        for (i, position) in cube_positions.iter().enumerate() {
            let cube_model = Mat4::from_translation(*position);
            
            shader.set_uniform_matrix_4fv("model", cube_model.to_cols_array().as_ref());
            window.render_mesh(&cube_mesh);
        }
        
        shader.unbind_program();
        texture.unbind();

        window.update_framebuffer_textures();
        
        window.update();
    }
}