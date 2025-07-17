use ferrousgl::{GlWindow, Mesh, Shader, WindowConfig};
use glam::{Mat4, Vec3, Vec4};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::time::{Duration, Instant};

fn main() {
    // Create a 800x600 window
    let mut window = GlWindow::new(WindowConfig {
        width: 800,
        height: 600,
        title: "Shader Reloading Example (change the shaders in ./shaders/shader_reloading/ while this is running!)".to_owned(),
        target_framerate: 144,
        ..Default::default()
    });

    let vertex_path = Path::new("./examples/shaders/shader_reloading/vertex.glsl");
    let fragment_path = Path::new("./examples/shaders/shader_reloading/fragment.glsl");

    // Create initial shader
    let mut shader =
        Shader::new_from_file(vertex_path, fragment_path).expect("Failed to load initial shader");

    // Set up file watcher
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher: RecommendedWatcher = Watcher::new(
        tx,
        notify::Config::default()
            .with_poll_interval(Duration::from_secs(1)) // Check for changes every second
            .with_compare_contents(true), // Compare file contents to avoid false positives
    )
    .expect("Failed to create file watcher");

    watcher
        .watch(vertex_path, RecursiveMode::NonRecursive)
        .unwrap();
    watcher
        .watch(fragment_path, RecursiveMode::NonRecursive)
        .unwrap();

    let mut last_reload_time = Instant::now();
    let reload_cooldown = Duration::from_millis(500); // Prevent rapid reloads

    let mut mesh = Mesh::new();

    let vertices = [
        // positions        // texture coords
        // bottom face
        0.5, 0.5, -0.5, 1.0, 1.0, 0.0, // top right
        0.5, -0.5, -0.5, 1.0, 0.0, 0.0, // bottom right
        -0.5, -0.5, -0.5, 0.0, 0.0, 0.0, // bottom left
        -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, // top left
        // top face
        0.5, 0.5, 0.5, 0.0, 0.0, 0.0, // top right
        0.5, -0.5, 0.5, 0.0, 1.0, 0.0, // bottom right
        -0.5, -0.5, 0.5, 1.0, 1.0, 0.0, // bottom left
        -0.5, 0.5, 0.5, 1.0, 0.0, 0.0, // top left
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

    // Add these before the main loop
    let mut rotation_x = 0.0f32;
    let mut rotation_y = 0.0f32;

    while !window.should_window_close() {
        // recompile the shader if the file changed according to the watcher
        if let Ok(event) = rx.try_recv() {
        match event {
            Ok(Event { kind: notify::EventKind::Modify(_), .. }) => {
                if last_reload_time.elapsed() > reload_cooldown {
                    match shader.recompile_from_file(vertex_path, fragment_path) {
                        Ok(_) => println!("Shader reloaded successfully!"),
                        Err(e) => eprintln!("Shader reload failed: {}", e),
                    }
                    last_reload_time = Instant::now();
                }
            }
            _ => {}
        }
    }

        window.clear_color(Vec4::new(0.4, 0.0, 0.6, 1.0));
        window.clear_depth();

        // Get mouse delta and accumulate rotation
        let (dx, dy) = window.get_mouse_delta();
        rotation_y += dx as f32 * 0.01;
        rotation_x += dy as f32 * 0.01;

        // Clamp rotation_x to avoid flipping
        rotation_x = rotation_x.clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);

        // Build model matrix from both rotations
        let model = Mat4::from_rotation_y(rotation_y) * Mat4::from_rotation_x(rotation_x);

        shader.bind_program();

        // Set matrices
        shader.set_uniform_matrix_4fv("projection", projection.to_cols_array().as_ref());
        shader.set_uniform_matrix_4fv("view", view.to_cols_array().as_ref());
        shader.set_uniform_matrix_4fv("model", model.to_cols_array().as_ref());

        window.render_mesh(&mesh);

        shader.unbind_program();

        window.update();
    }
}
