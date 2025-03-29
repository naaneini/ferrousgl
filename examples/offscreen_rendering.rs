use ferrousgl::{WindowConfig, GlWindow, Mesh, RenderTexture, Shader, Texture};
use glam::{Vec3, Mat4};

fn main() {
    // Create a 800x600 window
    let mut window = GlWindow::new(WindowConfig {
        width: 800,
        height: 600,
        title: "Offscreen Rendering Example".to_owned(),
        ..Default::default()
    });
    
    let vertex_shader = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec2 aTexCoord;
        
        out vec2 TexCoord;
        
        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 projection;
        
        void main() {
            gl_Position = projection * view * model * vec4(aPos, 1.0);
            TexCoord = aTexCoord;
        }
    "#;
    
    let fragment_shader = r#"
        #version 330 core
        in vec2 TexCoord;
        out vec4 FragColor;
        uniform sampler2D ourTexture;
        void main() {
            FragColor = texture(ourTexture, TexCoord);
        }
    "#;
    
    let shader = Shader::new_from_source(vertex_shader, fragment_shader);
    
    let texture = Texture::new_from_file("examples/assets/wood_texture.png").unwrap();
    
    let mut mesh = Mesh::new();
    
    let vertices = [
        // positions        // texture coords
        // bottom face
         0.5,  0.5, -0.5,   1.0, 1.0,   // top right
         0.5, -0.5, -0.5,   1.0, 0.0,   // bottom right
        -0.5, -0.5, -0.5,   0.0, 0.0,   // bottom left
        -0.5,  0.5, -0.5,   0.0, 1.0,    // top left
        
        // top face
         0.5,  0.5,  0.5,   0.0, 0.0,   // top right
         0.5, -0.5,  0.5,   0.0, 1.0,   // bottom right
        -0.5, -0.5,  0.5,   1.0, 1.0,   // bottom left
        -0.5,  0.5,  0.5,   1.0, 0.0    // top left
    ];
    
    let indices = [
        // Bottom face (already defined)
        0, 1, 3,  // first triangle
        1, 2, 3,  // second triangle
        
        // Top face
        4, 5, 7,  // first triangle
        5, 6, 7,  // second triangle
        
        // Front face
        4, 0, 7,  // first triangle
        0, 3, 7,  // second triangle
        
        // Back face
        5, 1, 6,  // first triangle
        1, 2, 6,  // second triangle
        
        // Right face
        4, 5, 0,  // first triangle
        5, 1, 0,  // second triangle
        
        // Left face
        7, 6, 3,  // first triangle
        6, 2, 3   // second triangle
    ];
    
    mesh.update_vertices(&vertices);
    mesh.update_indices(&indices);
    mesh.add_vertex_attributes(&[
        (0, 3, gl::FLOAT, false),  // position
        (1, 2, gl::FLOAT, false)   // texture coord
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

    let render_texture = RenderTexture::new(256, 256, true).unwrap();
    
    while !window.should_window_close() {
        // Update rotation
        y_rotation += 0.01;
        x_rotation += 0.005;
        let model = Mat4::from_rotation_y(y_rotation) * Mat4::from_rotation_x(x_rotation);
        
        // Set the render texture to be rendered on
        render_texture.bind();

        // Clears the render textures color and depth buffers
        window.clear_color(Vec3::new(0.0, 0.0, 0.0));
        window.clear_depth();

        // Sets the texture
        texture.bind(0);
        shader.bind_program();
        shader.set_uniform_texture("ourTexture", 0);
        
        // Set matrices
        shader.set_uniform_matrix_4fv("projection", projection.to_cols_array().as_ref());
        shader.set_uniform_matrix_4fv("view", view.to_cols_array().as_ref());
        shader.set_uniform_matrix_4fv("model", model.to_cols_array().as_ref());
        
        // Draw the texture
        window.render_mesh(&mesh);

        // Unbinds the render texture and sets the default viewport size
        render_texture.unbind();
        window.update_viewport(800, 600);

        // Clears the default viewports color and depth buffers
        window.clear_color(Vec3::new(0.1, 0.1, 0.1));
        window.clear_depth();

        // Sets the render textures color texture as the cubes texture
        render_texture.texture().bind(0);
        shader.set_uniform_texture("ourTexture", 0);

        window.render_mesh(&mesh);
        
        shader.unbind_program();
        texture.unbind();
        
        window.update();
    }
}