use ferrousgl::{WindowConfig, GlWindow, Mesh, Shader, Texture};
use glam::{Mat4, Vec3, Vec4};
use rand::Rng;
use std::path::Path;

struct Particle {
    position: Vec3,
    velocity: Vec3,
    lifetime: f32,
    color: Vec4,
}

fn main() {
    // Configurable variables
    let base_color = Vec4::new(1.0, 1.0, 1.0, 1.0); // Base color of particles
    let velocity_range_x = -0.005..0.005; // X velocity range
    let velocity_range_y = 0.02..0.03;  // Y velocity range
    let lifetime_range = 0.0..2.0;      // Lifetime range of particles
    let fade_duration = 1.0;           // Duration for particles to fade out
    let spawn_rate = 32;               // Number of particles to spawn per frame

    let mut window = GlWindow::new(
        WindowConfig {
            width: 800,
            height: 600,
            title: "Particle Generator".to_owned(),
            ..Default::default()
        },
    );

    window.set_blend_mode(ferrousgl::BlendMode::Alpha);
    window.set_depth_testing(ferrousgl::DepthType::None);

    let shader = Shader::new_from_file(
        Path::new("./examples/shaders/particles/vertex.glsl"),
        Path::new("./examples/shaders/particles/fragment.glsl"),
    )
    .unwrap();

    let texture = Texture::new_from_file(Path::new("examples/assets/particle.png")).unwrap();
    texture.bind(0);
    texture.set_mipmap_and_filtering(ferrousgl::MipmapType::Linear, ferrousgl::FilterMode::Nearest);

    let mut mesh = Mesh::new();

    let vertices = [
        // positions        // texture coords
        -0.05, -0.05, 0.0,    0.0, 0.0, // bottom left
         0.05, -0.05, 0.0,    1.0, 0.0, // bottom right
         0.05,  0.05, 0.0,    1.0, 1.0, // top right
        -0.05,  0.05, 0.0,    0.0, 1.0, // top left
    ];

    let indices = [
        0, 1, 2, // first triangle
        2, 3, 0, // second triangle
    ];

    mesh.update_vertices(&vertices);
    mesh.update_indices(&indices);
    mesh.add_vertex_attributes(&[
        (0, 3, gl::FLOAT, false), // position
        (1, 2, gl::FLOAT, false), // texture coord
    ]);

    let mut particles: Vec<Particle> = Vec::new();
    let mut rng = rand::rng();

    while !window.should_window_close() {
        let (width, height) = window.get_window_size();
        let aspect_ratio = width as f32 / height as f32;

        let projection = Mat4::orthographic_rh_gl(
            -aspect_ratio, aspect_ratio,
            -1.0, 1.0,
            -1.0, 1.0,
        );

        window.clear_color(Vec4::new(0.2, 0.3, 0.3, 1.0));
        window.clear_depth();

        // Spawn new particles
        for _ in 0..spawn_rate {
            particles.push(Particle {
                position: Vec3::new(0.0, 0.0, 0.0),
                velocity: Vec3::new(
                    rng.random_range(velocity_range_x.clone()),
                    rng.random_range(velocity_range_y.clone()),
                    0.0,
                ),
                lifetime: rng.random_range(lifetime_range.clone()),
                color: base_color,
            });
        }

        // Update particles
        particles.iter_mut().for_each(|particle| {
            particle.position += particle.velocity;
            particle.lifetime -= 0.016; // Assuming ~60 FPS
            particle.color.w = particle.lifetime / fade_duration; // Fade out
        });

        // Remove expired particles
        particles.retain(|particle| particle.lifetime > 0.0);

        // Render particles
        texture.bind(0);
        shader.bind_program();
        shader.set_uniform_matrix_4fv("projection", projection.to_cols_array().as_ref());
        shader.set_uniform_matrix_4fv("view", Mat4::IDENTITY.to_cols_array().as_ref());

        for particle in &particles {
            let model = Mat4::from_translation(particle.position);
            shader.set_uniform_matrix_4fv("model", model.to_cols_array().as_ref());
            shader.set_uniform_4f("particleColor", particle.color.x, particle.color.y, particle.color.z, particle.color.w);
            window.render_mesh(&mesh);
        }

        shader.unbind_program();
        texture.unbind();

        window.update();
    }
}