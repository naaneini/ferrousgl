use ferrousgl::{GlWindow, Mesh, Shader, WindowConfig};
use glam::{Mat4, Vec4};
use image::ImageReader;
use rand::Rng;
use std::path::Path;

fn main() {
    let mut window = GlWindow::new(WindowConfig {
        width: 800,
        height: 600,
        title: "Triangle Pointillism".to_owned(),
        ..Default::default()
    });

    let shader = Shader::new_from_file(
        Path::new("./examples/shaders/triangle_pointillism/vertex.glsl"),
        Path::new("./examples/shaders/triangle_pointillism/fragment.glsl"),
    ).unwrap();

    // Load image
    let img = ImageReader::open("examples/assets/cool_image.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8();
    let (img_w, img_h) = img.dimensions();

    // Generate triangles
    let num_triangles = 5000;
    let mut meshes = Vec::new();
    let mut rng = rand::rng();

    for _ in 0..num_triangles {
        // Random center in [-1, 1] (NDC)
        let cx = rng.random_range(-1.0..1.0);
        let cy = rng.random_range(-1.0..1.0);
        let size = rng.random_range(0.02..0.09);

        // Triangle vertices (equilateral)
        let angle = rng.random_range(0.0..std::f32::consts::TAU);
        let mut verts = Vec::new();
        for i in 0..3 {
            let theta = angle + i as f32 * std::f32::consts::TAU / 3.0;
            let x = cx + size * theta.cos();
            let y = cy + size * theta.sin();
            verts.push((x, y));
        }

        // Sample color from image
        let img_x = (((cx + 1.0) / 2.0) * img_w as f32).clamp(0.0, img_w as f32 - 1.0) as u32;
        let img_y = (((1.0 - cy) / 2.0) * img_h as f32).clamp(0.0, img_h as f32 - 1.0) as u32;
        let px = img.get_pixel(img_x, img_y);
        let color = Vec4::new(
            px[0] as f32 / 255.0,
            px[1] as f32 / 255.0,
            px[2] as f32 / 255.0,
            0.3, // semi-transparent
        );

        // Interleaved: pos(x,y,0), color(r,g,b)
        let mut vertex_data = Vec::new();
        for (x, y) in verts {
            vertex_data.extend_from_slice(&[x, y, 0.0, color.x, color.y, color.z, color.w]);
        }

        let indices = [0u32, 1, 2];

        let mut mesh = Mesh::new();
        mesh.update_vertices(&vertex_data);
        mesh.update_indices(&indices);
        mesh.add_vertex_attributes(&[
            (0, 3, gl::FLOAT, false), // position
            (1, 4, gl::FLOAT, false), // color
        ]);
        meshes.push(mesh);
    }

    // Set up projection and view for NDC
    let projection = Mat4::IDENTITY;
    let view = Mat4::IDENTITY;

    while !window.should_window_close() {
        window.clear_color(Vec4::new(1.0, 1.0, 1.0, 1.0));
        window.clear_depth();
        window.set_depth_testing(ferrousgl::DepthType::None);
        window.set_blend_mode(ferrousgl::BlendMode::Alpha);

        shader.bind_program();
        shader.set_uniform_matrix_4fv("projection", projection.to_cols_array().as_ref());
        shader.set_uniform_matrix_4fv("view", view.to_cols_array().as_ref());
        shader.set_uniform_matrix_4fv("model", Mat4::IDENTITY.to_cols_array().as_ref());

        for mesh in &meshes {
            window.render_mesh(mesh);
        }

        shader.unbind_program();
        window.update();
    }
}