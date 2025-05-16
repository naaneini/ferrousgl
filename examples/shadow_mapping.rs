use ferrousgl::{texture::FilterMode, DepthType, GlWindow, Mesh, MipmapType, RenderTexture, Shader, Texture, WindowConfig};
use glam::{Vec3, Vec4, Mat4};
use std::{path::Path, time::Instant};

fn main() {
    // Window setup
    let mut window = GlWindow::new(WindowConfig {
        width: 800,
        height: 600,
        title: "Shadow Mapping Example".to_owned(),
        target_framerate: 144,
        ..Default::default()
    });
    
    // Shaders
    let shader = Shader::new_from_file(
        Path::new("./examples/shaders/shadow_mapping/vertex.glsl"),
        Path::new("./examples/shaders/shadow_mapping/fragment.glsl"),
    ).unwrap();

    let depth_shader = Shader::new_from_file(
        Path::new("./examples/shaders/shadow_mapping/depth_vertex.glsl"),
        Path::new("./examples/shaders/shadow_mapping/depth_fragment.glsl"),
    ).unwrap();

    let quad_shader = Shader::new_from_file(
        Path::new("./examples/shaders/shadow_mapping/quad_vertex.glsl"),
        Path::new("./examples/shaders/shadow_mapping/quad_fragment.glsl"),
    ).unwrap();

    // Textures
    let cube_texture = Texture::new_from_file(Path::new("examples/assets/wood_texture.png")).unwrap();
    cube_texture.bind(0);
    cube_texture.set_mipmap_and_filtering(MipmapType::Linear, FilterMode::Nearest);
    
    let floor_texture = Texture::new_from_file(Path::new("examples/assets/plank_texture.jpg")).unwrap();
    floor_texture.bind(0);
    floor_texture.set_mipmap_and_filtering(MipmapType::Linear, FilterMode::Nearest);

    let depth_texture = RenderTexture::new(512, 512, true).unwrap();
    depth_texture.depth_texture().unwrap().bind(0);
    depth_texture.depth_texture().unwrap().set_mipmap_and_filtering(MipmapType::None, FilterMode::Linear);

    // Meshes
    let mut quad_mesh = Mesh::new();
    
    let quad_vertices = [
        // positions   // texture coords
        -1.0, -1.0,   0.0, 0.0,  // bottom-left
        -0.25, -1.0,   1.0, 0.0,  // bottom-right
        -0.25, -0.25,   1.0, 1.0,  // top-right
        -1.0, -0.25,   0.0, 1.0   // top-left
    ];

    let quad_indices = [0, 1, 3, 1, 2, 3];

    quad_mesh.update_vertices(&quad_vertices);
    quad_mesh.update_indices(&quad_indices);
    quad_mesh.add_vertex_attributes(&[
        (0, 2, gl::FLOAT, false),  // position
        (1, 2, gl::FLOAT, false)   // texture coord
    ]);

    let mut cube_mesh = Mesh::new();

    let cube_vertices = [
        // positions        // texture coords
        // bottom face
         0.5,  0.5, -0.5,   1.0, 1.0,
         0.5, -0.5, -0.5,   1.0, 0.0,
        -0.5, -0.5, -0.5,   0.0, 0.0,
        -0.5,  0.5, -0.5,   0.0, 1.0,
        // top face
         0.5,  0.5,  0.5,   0.0, 0.0,
         0.5, -0.5,  0.5,   0.0, 1.0,
        -0.5, -0.5,  0.5,   1.0, 1.0,
        -0.5,  0.5,  0.5,   1.0, 0.0
    ];

    let cube_indices = [
        0, 1, 3,  1, 2, 3,  // bottom
        4, 5, 7,  5, 6, 7,  // top
        4, 0, 7,  0, 3, 7,  // front
        5, 1, 6,  1, 2, 6,  // back
        4, 5, 0,  5, 1, 0,  // right
        7, 6, 3,  6, 2, 3   // left
    ];

    cube_mesh.update_vertices(&cube_vertices);
    cube_mesh.update_indices(&cube_indices);
    cube_mesh.add_vertex_attributes(&[
        (0, 3, gl::FLOAT, false),  // position
        (1, 2, gl::FLOAT, false)   // texture coord
    ]);

    let mut floor_mesh = Mesh::new();

    let floor_vertices = [
        // positions          // texture coords
        -5.0, -0.5, -5.0,    0.0, 0.0,
         5.0, -0.5, -5.0,    5.0, 0.0,
         5.0, -0.5,  5.0,    5.0, 5.0,
        -5.0, -0.5,  5.0,    0.0, 5.0,
    ];

    let floor_indices = [0, 1, 3, 1, 2, 3];

    floor_mesh.update_vertices(&floor_vertices);
    floor_mesh.update_indices(&floor_indices);
    floor_mesh.add_vertex_attributes(&[
        (0, 3, gl::FLOAT, false),  // position
        (1, 2, gl::FLOAT, false)   // texture coord
    ]);

    // Camera setup
    let camera_pos = Vec3::new(0.0, 1.0, 3.0);
    let camera_target = Vec3::new(1.0, -1.0, 0.0);
    let camera_up = Vec3::new(0.0, 1.0, 0.0);
    let view = Mat4::look_at_rh(camera_pos, camera_target, camera_up);

    // Light setup
    let ortho_projection = Mat4::orthographic_rh(-10.0, 10.0, -10.0, 10.0, 7.5, 20.0);
    let light_pos = Vec3::new(5.0, 10.0, 2.0);
    let light_target = Vec3::new(1.0, 0.0, 0.0);
    let light_up = Vec3::new(0.0, 1.0, 0.0);
    let light_view = Mat4::look_at_rh(light_pos, light_target, light_up);

    let mut y_rotation = 0.0f32;
    let mut x_rotation = 0.0f32;

    let mut previous_frame_time = Instant::now();
    
    // Main render loop
    while !window.should_window_close() {
        let current_frame_time = Instant::now();
        let frame_time = current_frame_time.duration_since(previous_frame_time);
        previous_frame_time = current_frame_time;

        let frame_time_secs = frame_time.as_secs_f32();

        println!("Frame time: {:.6} seconds", frame_time_secs);

        let fps = 1.0 / frame_time_secs;
        println!("FPS: {:.0}", fps); // Prints "FPS: 100"


        window.clear_color(Vec4::new(0.2, 0.3, 0.3, 1.0));
        window.clear_depth();
        window.set_depth_testing(DepthType::LessOrEqual);

        let aspect_ratio = window.get_window_size().0 as f32 / window.get_window_size().1 as f32;
        let projection = Mat4::perspective_rh_gl(45.0f32.to_radians(), aspect_ratio, 0.1, 100.0);

        // Depth pass
        depth_texture.bind();
        window.clear_color(Vec4::new(1.0, 1.0, 1.0, 1.0));
        window.clear_depth();
        
        y_rotation += 0.01;
        x_rotation += 0.005;
        let cube_model = Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0)) * 
                         Mat4::from_rotation_y(y_rotation) * 
                         Mat4::from_rotation_x(x_rotation);
        let podest_model = Mat4::from_translation(Vec3::new(1.0, -1.25, 0.0)) *
        Mat4::from_scale(Vec3::new(2.0, 0.5, 2.0));
        let floor_model = Mat4::from_translation(Vec3::new(0.0, -1.0, 0.0)) * 
                          Mat4::from_scale(Vec3::new(2.0, 1.0, 2.0));
        
        depth_shader.bind_program();
        depth_shader.set_uniform_matrix_4fv("lightSpaceMatrix", 
            (ortho_projection * light_view).to_cols_array().as_ref());

        depth_shader.set_uniform_matrix_4fv("model", floor_model.to_cols_array().as_ref());
        window.render_mesh(&floor_mesh);

        depth_shader.set_uniform_matrix_4fv("model", cube_model.to_cols_array().as_ref());
        window.render_mesh(&cube_mesh);

        depth_shader.set_uniform_matrix_4fv("model", podest_model.to_cols_array().as_ref());
        window.render_mesh(&cube_mesh);

        depth_shader.unbind_program();
        depth_texture.unbind();

        // Main render pass
        window.update_viewport(window.get_window_size().0, window.get_window_size().1);
        
        // Draw floor with shadows
        floor_texture.bind(0);
        depth_texture.depth_texture().unwrap().bind(1);
        shader.bind_program();
        shader.set_uniform_texture("diffuseTexture", 0);
        shader.set_uniform_texture("shadowMap", 1);
        shader.set_uniform_matrix_4fv("projection", projection.to_cols_array().as_ref());
        shader.set_uniform_matrix_4fv("view", view.to_cols_array().as_ref());
        shader.set_uniform_matrix_4fv("model", floor_model.to_cols_array().as_ref());
        shader.set_uniform_matrix_4fv("lightSpaceMatrix", 
            (ortho_projection * light_view).to_cols_array().as_ref());
        shader.set_uniform_3f("lightPos", light_pos.x, light_pos.y, light_pos.z);
        shader.set_uniform_3f("viewPos", camera_pos.x, camera_pos.y, camera_pos.z);
        shader.set_uniform_1i("shadowBlurKernelSize", 4);
        shader.set_uniform_3f("lightColor", 1.0, 1.0, 1.0);
        shader.set_uniform_3f("ambientColor", 0.8, 0.85, 0.95);
        window.render_mesh(&floor_mesh);
        
        // Draw cube with shadows
        cube_texture.bind(0);
        shader.set_uniform_texture("diffuseTexture", 0);
        shader.set_uniform_matrix_4fv("model", cube_model.to_cols_array().as_ref());
        window.render_mesh(&cube_mesh);

        // Draw podest
        shader.set_uniform_matrix_4fv("model", podest_model.to_cols_array().as_ref());
        window.render_mesh(&cube_mesh);

        shader.unbind_program();

        // Debug quad render
        quad_shader.bind_program();
        depth_texture.depth_texture().unwrap().bind(0);
        quad_shader.set_uniform_texture("screenTexture", 0);
        window.set_depth_testing(DepthType::None);
        window.render_mesh(&quad_mesh);
        depth_texture.depth_texture().unwrap().unbind();

        let fps = 1_000_000.0 / window.get_frame_time() as f64;
        println!("Window reports potential FPS: {:.2}", fps);
        
        window.update();
    }
}