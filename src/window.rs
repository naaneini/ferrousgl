extern crate gl;
extern crate glam;
extern crate glfw;

use std::collections::HashSet;
use std::ptr;

use glam::{bool, Vec3};
use glfw::{fail_on_errors, Context, Key, WindowEvent};
use std::time::{Duration, Instant};

use crate::Mesh;

/// A struct to manage an OpenGL context, window, rendering and input!
pub struct GlWindow {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, WindowEvent)>,
    target_frame_time: Duration,
    last_frame_time: Instant,
    rendering_type: RenderingType,
    mouse_wheel_delta: (f64, f64),
    typed_keys: HashSet<char>,
    pressed_keys: HashSet<WindowKey>,
    previous_pressed_keys: HashSet<WindowKey>,
}

impl GlWindow {
    /// Creates a new OpenGL window with the specified width, height, and title.
    pub fn new(width: u32, height: u32, title: &str, decorated: bool, target_fps: u32) -> Self {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::Decorated(decorated));
        glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);
        window.set_char_polling(true);
        window.set_scroll_polling(true);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        GlWindow {
            glfw,
            window,
            events,
            target_frame_time: Duration::from_secs(1) / target_fps,
            last_frame_time: Instant::now(),
            rendering_type: RenderingType::Solid,
            mouse_wheel_delta: (0.0, 0.0),
            typed_keys: HashSet::new(),
            pressed_keys: HashSet::new(),
            previous_pressed_keys: HashSet::new(),
        }
    }

    /// Returns if the window received a close signal. This allows for "Do you really want to exit?" dialogues for example, or shutting down logic.
    pub fn should_window_close(&self) -> bool {
        self.window.should_close()
    }

    /// Set the windows size.
    pub fn set_window_size(&mut self, width: u32, height: u32) {
        self.window.set_size(width as i32, height as i32);
    }

    /// Set the windows position.
    pub fn set_window_position(&mut self, x: u32, y: u32) {
        self.window.set_pos(x as i32, y as i32);
    }

    /// Set the windows title.
    pub fn set_window_title(&mut self, new_title: &str) {
        self.window.set_title(new_title);
    }

    /// Set the target framerate.
    pub fn set_target_fps(&mut self, new_target_fps: u32) {
        self.target_frame_time = Duration::from_secs(1) / new_target_fps;
    }

    /// Set the preferred Rendering Type such as Lines, Points or (the default) Triangles.
    pub fn set_rendering_type(&mut self, rendering_type: RenderingType) {
        self.rendering_type = rendering_type;
    }

    /// Get the current clipboard string.
    pub fn get_clipboard(&mut self) -> Option<String> {
        self.window.get_clipboard_string()
    }

    /// Set a new clipboard string.
    pub fn set_clipboard(&mut self, new_data: &str) {
        self.window.set_clipboard_string(new_data);
    }

    /// Returns the windows size.
    pub fn get_window_size(&self) -> (i32, i32) {
        self.window.get_size()
    }

    /// Returns the windows position.
    pub fn get_window_position(&self) -> (i32, i32) {
        self.window.get_pos()
    }

    /// Set the mouse position.
    pub fn set_mouse_position(&mut self, x: f64, y: f64) {
        self.window.set_cursor_pos(x, y);
    }

    /// Returns the mouse position.
    pub fn get_mouse_position(&self) -> (f64, f64) {
        self.window.get_cursor_pos()
    }

    /// Returns the mouse wheel delta (x, y).
    pub fn get_mouse_wheel_delta(&self) -> (f64, f64) {
        self.mouse_wheel_delta
    }

    /// Clears mouse wheel delta for the next frame.
    fn reset_mouse_wheel_delta(&mut self) {
        self.mouse_wheel_delta = (0.0, 0.0);
    }

    /// Checks if a specific mouse button is pressed.
    pub fn is_mouse_button_pressed(&self, button: glfw::MouseButton) -> bool {
        self.window.get_mouse_button(button) == glfw::Action::Press
    }

    /// Checks if a specific key is pressed.
    pub fn is_key_pressed(&self, key: WindowKey) -> bool {
        self.pressed_keys.contains(&key) && !self.previous_pressed_keys.contains(&key)
    }

    /// Checks if a specific key is released.
    pub fn is_key_released(&self, key: WindowKey) -> bool {
        self.window.get_key(key.into()) == glfw::Action::Release
    }

    /// Checks if a specific key is held.
    pub fn is_key_held(&self, key: WindowKey) -> bool {
        self.window.get_key(key.into()) == glfw::Action::Press
    }

    /// Checks if a specific character key is being typed.
    pub fn is_key_typed(&self, key: char) -> bool {
        self.typed_keys.contains(&key)
    }

    /// Returns all keys that are currently being typed.
    pub fn get_typed_keys(&self) -> Vec<char> {
        self.typed_keys.iter().cloned().collect()
    }

    /// Clears typed keys for the next frame.
    fn clear_typed_keys(&mut self) {
        self.typed_keys.clear();
    }

    /// Updates the state of pressed keys.
    fn update_pressed_keys(&mut self) {
        self.previous_pressed_keys = self.pressed_keys.clone();
        self.pressed_keys.clear();
    }

    /// Polls events (user input, system events) and swaps buffers.
    pub fn update(&mut self) {
        self.clear_typed_keys();
        self.reset_mouse_wheel_delta();
        self.update_pressed_keys();

        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Key(key, _, action, _) => {
                    if action == glfw::Action::Press {
                        self.pressed_keys.insert(key.into());
                    }
                    if key == Key::Escape && action == glfw::Action::Press {
                        self.window.set_should_close(true);
                    }
                }
                WindowEvent::FramebufferSize(width, height) => {
                    self.update_viewport(width, height);
                }
                WindowEvent::Scroll(xoffset, yoffset) => {
                    self.mouse_wheel_delta = (xoffset, yoffset);
                }
                WindowEvent::Char(codepoint) => {
                    self.typed_keys.insert(codepoint);
                }
                _ => {}
            }
        }
        self.window.swap_buffers();

        let now = Instant::now();
        let elapsed = now - self.last_frame_time;

        if elapsed < self.target_frame_time {
            std::thread::sleep(self.target_frame_time - elapsed);
        }

        self.last_frame_time = Instant::now();
    }

    /// Updates the OpenGL viewport to match a new window size. This function typically only needs to be used after a 
    /// render texture (or offscreen texture) that has a different size than the window is unbound.
    pub fn update_viewport(&self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }

    /// Clears the current bound color buffer with the specified color.
    pub fn clear_color(&self, color: Vec3) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    /// Clears the current bound depth buffer.
    pub fn clear_depth(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    /// Renders a mesh using the provided shader and vertex data onto the current bound framebuffer.
    pub fn render_mesh(&self, mesh: &Mesh) {
        unsafe {
            match self.rendering_type {
                RenderingType::Solid => {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                }
                RenderingType::Wireframe => {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                }
                RenderingType::Points => {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::POINT);
                }
            }

            mesh.bind();

            gl::DrawElements(
                gl::TRIANGLES,
                mesh.indices_length as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            );

            mesh.unbind();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderingType {
    Solid,
    Wireframe,
    Points,
}

/// Enum containing all GLFW key codes converted to a custom implementation for easier usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowKey {
    Space,
    Apostrophe,
    Comma,
    Minus,
    Period,
    Slash,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Semicolon,
    Equal,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LeftBracket,
    Backslash,
    RightBracket,
    GraveAccent,
    World1,
    World2,
    Escape,
    Enter,
    Tab,
    Backspace,
    Insert,
    Delete,
    Right,
    Left,
    Down,
    Up,
    PageUp,
    PageDown,
    Home,
    End,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDecimal,
    KpDivide,
    KpMultiply,
    KpSubtract,
    KpAdd,
    KpEnter,
    KpEqual,
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    Menu,
    Unknown,
}

/// Allowing for converting between GLFW keys to Window keys.
impl From<glfw::Key> for WindowKey {
    fn from(glfw_key: glfw::Key) -> Self {
        match glfw_key {
            glfw::Key::Space => WindowKey::Space,
            glfw::Key::Apostrophe => WindowKey::Apostrophe,
            glfw::Key::Comma => WindowKey::Comma,
            glfw::Key::Minus => WindowKey::Minus,
            glfw::Key::Period => WindowKey::Period,
            glfw::Key::Slash => WindowKey::Slash,
            glfw::Key::Num0 => WindowKey::Num0,
            glfw::Key::Num1 => WindowKey::Num1,
            glfw::Key::Num2 => WindowKey::Num2,
            glfw::Key::Num3 => WindowKey::Num3,
            glfw::Key::Num4 => WindowKey::Num4,
            glfw::Key::Num5 => WindowKey::Num5,
            glfw::Key::Num6 => WindowKey::Num6,
            glfw::Key::Num7 => WindowKey::Num7,
            glfw::Key::Num8 => WindowKey::Num8,
            glfw::Key::Num9 => WindowKey::Num9,
            glfw::Key::Semicolon => WindowKey::Semicolon,
            glfw::Key::Equal => WindowKey::Equal,
            glfw::Key::A => WindowKey::A,
            glfw::Key::B => WindowKey::B,
            glfw::Key::C => WindowKey::C,
            glfw::Key::D => WindowKey::D,
            glfw::Key::E => WindowKey::E,
            glfw::Key::F => WindowKey::F,
            glfw::Key::G => WindowKey::G,
            glfw::Key::H => WindowKey::H,
            glfw::Key::I => WindowKey::I,
            glfw::Key::J => WindowKey::J,
            glfw::Key::K => WindowKey::K,
            glfw::Key::L => WindowKey::L,
            glfw::Key::M => WindowKey::M,
            glfw::Key::N => WindowKey::N,
            glfw::Key::O => WindowKey::O,
            glfw::Key::P => WindowKey::P,
            glfw::Key::Q => WindowKey::Q,
            glfw::Key::R => WindowKey::R,
            glfw::Key::S => WindowKey::S,
            glfw::Key::T => WindowKey::T,
            glfw::Key::U => WindowKey::U,
            glfw::Key::V => WindowKey::V,
            glfw::Key::W => WindowKey::W,
            glfw::Key::X => WindowKey::X,
            glfw::Key::Y => WindowKey::Y,
            glfw::Key::Z => WindowKey::Z,
            glfw::Key::LeftBracket => WindowKey::LeftBracket,
            glfw::Key::Backslash => WindowKey::Backslash,
            glfw::Key::RightBracket => WindowKey::RightBracket,
            glfw::Key::GraveAccent => WindowKey::GraveAccent,
            glfw::Key::World1 => WindowKey::World1,
            glfw::Key::World2 => WindowKey::World2,
            glfw::Key::Escape => WindowKey::Escape,
            glfw::Key::Enter => WindowKey::Enter,
            glfw::Key::Tab => WindowKey::Tab,
            glfw::Key::Backspace => WindowKey::Backspace,
            glfw::Key::Insert => WindowKey::Insert,
            glfw::Key::Delete => WindowKey::Delete,
            glfw::Key::Right => WindowKey::Right,
            glfw::Key::Left => WindowKey::Left,
            glfw::Key::Down => WindowKey::Down,
            glfw::Key::Up => WindowKey::Up,
            glfw::Key::PageUp => WindowKey::PageUp,
            glfw::Key::PageDown => WindowKey::PageDown,
            glfw::Key::Home => WindowKey::Home,
            glfw::Key::End => WindowKey::End,
            glfw::Key::CapsLock => WindowKey::CapsLock,
            glfw::Key::ScrollLock => WindowKey::ScrollLock,
            glfw::Key::NumLock => WindowKey::NumLock,
            glfw::Key::PrintScreen => WindowKey::PrintScreen,
            glfw::Key::Pause => WindowKey::Pause,
            glfw::Key::F1 => WindowKey::F1,
            glfw::Key::F2 => WindowKey::F2,
            glfw::Key::F3 => WindowKey::F3,
            glfw::Key::F4 => WindowKey::F4,
            glfw::Key::F5 => WindowKey::F5,
            glfw::Key::F6 => WindowKey::F6,
            glfw::Key::F7 => WindowKey::F7,
            glfw::Key::F8 => WindowKey::F8,
            glfw::Key::F9 => WindowKey::F9,
            glfw::Key::F10 => WindowKey::F10,
            glfw::Key::F11 => WindowKey::F11,
            glfw::Key::F12 => WindowKey::F12,
            glfw::Key::F13 => WindowKey::F13,
            glfw::Key::F14 => WindowKey::F14,
            glfw::Key::F15 => WindowKey::F15,
            glfw::Key::F16 => WindowKey::F16,
            glfw::Key::F17 => WindowKey::F17,
            glfw::Key::F18 => WindowKey::F18,
            glfw::Key::F19 => WindowKey::F19,
            glfw::Key::F20 => WindowKey::F20,
            glfw::Key::F21 => WindowKey::F21,
            glfw::Key::F22 => WindowKey::F22,
            glfw::Key::F23 => WindowKey::F23,
            glfw::Key::F24 => WindowKey::F24,
            glfw::Key::F25 => WindowKey::F25,
            glfw::Key::Kp0 => WindowKey::Kp0,
            glfw::Key::Kp1 => WindowKey::Kp1,
            glfw::Key::Kp2 => WindowKey::Kp2,
            glfw::Key::Kp3 => WindowKey::Kp3,
            glfw::Key::Kp4 => WindowKey::Kp4,
            glfw::Key::Kp5 => WindowKey::Kp5,
            glfw::Key::Kp6 => WindowKey::Kp6,
            glfw::Key::Kp7 => WindowKey::Kp7,
            glfw::Key::Kp8 => WindowKey::Kp8,
            glfw::Key::Kp9 => WindowKey::Kp9,
            glfw::Key::KpDecimal => WindowKey::KpDecimal,
            glfw::Key::KpDivide => WindowKey::KpDivide,
            glfw::Key::KpMultiply => WindowKey::KpMultiply,
            glfw::Key::KpSubtract => WindowKey::KpSubtract,
            glfw::Key::KpAdd => WindowKey::KpAdd,
            glfw::Key::KpEnter => WindowKey::KpEnter,
            glfw::Key::KpEqual => WindowKey::KpEqual,
            glfw::Key::LeftShift => WindowKey::LeftShift,
            glfw::Key::LeftControl => WindowKey::LeftControl,
            glfw::Key::LeftAlt => WindowKey::LeftAlt,
            glfw::Key::LeftSuper => WindowKey::LeftSuper,
            glfw::Key::RightShift => WindowKey::RightShift,
            glfw::Key::RightControl => WindowKey::RightControl,
            glfw::Key::RightAlt => WindowKey::RightAlt,
            glfw::Key::RightSuper => WindowKey::RightSuper,
            glfw::Key::Menu => WindowKey::Menu,
            _ => WindowKey::Unknown,
        }
    }
}

/// Allowing for converting between GLFW keys to Window keys.
impl From<WindowKey> for glfw::Key {
    fn from(window_key: WindowKey) -> Self {
        match window_key {
            WindowKey::Space => glfw::Key::Space,
            WindowKey::Apostrophe => glfw::Key::Apostrophe,
            WindowKey::Comma => glfw::Key::Comma,
            WindowKey::Minus => glfw::Key::Minus,
            WindowKey::Period => glfw::Key::Period,
            WindowKey::Slash => glfw::Key::Slash,
            WindowKey::Num0 => glfw::Key::Num0,
            WindowKey::Num1 => glfw::Key::Num1,
            WindowKey::Num2 => glfw::Key::Num2,
            WindowKey::Num3 => glfw::Key::Num3,
            WindowKey::Num4 => glfw::Key::Num4,
            WindowKey::Num5 => glfw::Key::Num5,
            WindowKey::Num6 => glfw::Key::Num6,
            WindowKey::Num7 => glfw::Key::Num7,
            WindowKey::Num8 => glfw::Key::Num8,
            WindowKey::Num9 => glfw::Key::Num9,
            WindowKey::Semicolon => glfw::Key::Semicolon,
            WindowKey::Equal => glfw::Key::Equal,
            WindowKey::A => glfw::Key::A,
            WindowKey::B => glfw::Key::B,
            WindowKey::C => glfw::Key::C,
            WindowKey::D => glfw::Key::D,
            WindowKey::E => glfw::Key::E,
            WindowKey::F => glfw::Key::F,
            WindowKey::G => glfw::Key::G,
            WindowKey::H => glfw::Key::H,
            WindowKey::I => glfw::Key::I,
            WindowKey::J => glfw::Key::J,
            WindowKey::K => glfw::Key::K,
            WindowKey::L => glfw::Key::L,
            WindowKey::M => glfw::Key::M,
            WindowKey::N => glfw::Key::N,
            WindowKey::O => glfw::Key::O,
            WindowKey::P => glfw::Key::P,
            WindowKey::Q => glfw::Key::Q,
            WindowKey::R => glfw::Key::R,
            WindowKey::S => glfw::Key::S,
            WindowKey::T => glfw::Key::T,
            WindowKey::U => glfw::Key::U,
            WindowKey::V => glfw::Key::V,
            WindowKey::W => glfw::Key::W,
            WindowKey::X => glfw::Key::X,
            WindowKey::Y => glfw::Key::Y,
            WindowKey::Z => glfw::Key::Z,
            WindowKey::LeftBracket => glfw::Key::LeftBracket,
            WindowKey::Backslash => glfw::Key::Backslash,
            WindowKey::RightBracket => glfw::Key::RightBracket,
            WindowKey::GraveAccent => glfw::Key::GraveAccent,
            WindowKey::World1 => glfw::Key::World1,
            WindowKey::World2 => glfw::Key::World2,
            WindowKey::Escape => glfw::Key::Escape,
            WindowKey::Enter => glfw::Key::Enter,
            WindowKey::Tab => glfw::Key::Tab,
            WindowKey::Backspace => glfw::Key::Backspace,
            WindowKey::Insert => glfw::Key::Insert,
            WindowKey::Delete => glfw::Key::Delete,
            WindowKey::Right => glfw::Key::Right,
            WindowKey::Left => glfw::Key::Left,
            WindowKey::Down => glfw::Key::Down,
            WindowKey::Up => glfw::Key::Up,
            WindowKey::PageUp => glfw::Key::PageUp,
            WindowKey::PageDown => glfw::Key::PageDown,
            WindowKey::Home => glfw::Key::Home,
            WindowKey::End => glfw::Key::End,
            WindowKey::CapsLock => glfw::Key::CapsLock,
            WindowKey::ScrollLock => glfw::Key::ScrollLock,
            WindowKey::NumLock => glfw::Key::NumLock,
            WindowKey::PrintScreen => glfw::Key::PrintScreen,
            WindowKey::Pause => glfw::Key::Pause,
            WindowKey::F1 => glfw::Key::F1,
            WindowKey::F2 => glfw::Key::F2,
            WindowKey::F3 => glfw::Key::F3,
            WindowKey::F4 => glfw::Key::F4,
            WindowKey::F5 => glfw::Key::F5,
            WindowKey::F6 => glfw::Key::F6,
            WindowKey::F7 => glfw::Key::F7,
            WindowKey::F8 => glfw::Key::F8,
            WindowKey::F9 => glfw::Key::F9,
            WindowKey::F10 => glfw::Key::F10,
            WindowKey::F11 => glfw::Key::F11,
            WindowKey::F12 => glfw::Key::F12,
            WindowKey::F13 => glfw::Key::F13,
            WindowKey::F14 => glfw::Key::F14,
            WindowKey::F15 => glfw::Key::F15,
            WindowKey::F16 => glfw::Key::F16,
            WindowKey::F17 => glfw::Key::F17,
            WindowKey::F18 => glfw::Key::F18,
            WindowKey::F19 => glfw::Key::F19,
            WindowKey::F20 => glfw::Key::F20,
            WindowKey::F21 => glfw::Key::F21,
            WindowKey::F22 => glfw::Key::F22,
            WindowKey::F23 => glfw::Key::F23,
            WindowKey::F24 => glfw::Key::F24,
            WindowKey::F25 => glfw::Key::F25,
            WindowKey::Kp0 => glfw::Key::Kp0,
            WindowKey::Kp1 => glfw::Key::Kp1,
            WindowKey::Kp2 => glfw::Key::Kp2,
            WindowKey::Kp3 => glfw::Key::Kp3,
            WindowKey::Kp4 => glfw::Key::Kp4,
            WindowKey::Kp5 => glfw::Key::Kp5,
            WindowKey::Kp6 => glfw::Key::Kp6,
            WindowKey::Kp7 => glfw::Key::Kp7,
            WindowKey::Kp8 => glfw::Key::Kp8,
            WindowKey::Kp9 => glfw::Key::Kp9,
            WindowKey::KpDecimal => glfw::Key::KpDecimal,
            WindowKey::KpDivide => glfw::Key::KpDivide,
            WindowKey::KpMultiply => glfw::Key::KpMultiply,
            WindowKey::KpSubtract => glfw::Key::KpSubtract,
            WindowKey::KpAdd => glfw::Key::KpAdd,
            WindowKey::KpEnter => glfw::Key::KpEnter,
            WindowKey::KpEqual => glfw::Key::KpEqual,
            WindowKey::LeftShift => glfw::Key::LeftShift,
            WindowKey::LeftControl => glfw::Key::LeftControl,
            WindowKey::LeftAlt => glfw::Key::LeftAlt,
            WindowKey::LeftSuper => glfw::Key::LeftSuper,
            WindowKey::RightShift => glfw::Key::RightShift,
            WindowKey::RightControl => glfw::Key::RightControl,
            WindowKey::RightAlt => glfw::Key::RightAlt,
            WindowKey::RightSuper => glfw::Key::RightSuper,
            WindowKey::Menu => glfw::Key::Menu,
            WindowKey::Unknown => glfw::Key::Unknown,
        }
    }
}
