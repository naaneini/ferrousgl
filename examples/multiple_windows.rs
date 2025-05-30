use ferrousgl::{WindowConfig, GlWindow};
use glam::Vec4;

fn main() {
    let mut window1 = GlWindow::new(
        WindowConfig {
            width: 800,
            height: 600,
            title: "Window ".to_owned(),
            ..Default::default()
        }
    );
    
    let mut window2 = GlWindow::new(
        WindowConfig {
            width: 600,
            height: 400,
            title: "Window 2".to_owned(),
            ..Default::default()
        }
    );
    
    while !window1.should_window_close() || !window2.should_window_close() {
        if !window1.should_window_close() {
            window1.make_current();
            window1.clear_color(Vec4::new(0.2, 0.3, 0.3, 1.0));
            window1.clear_depth();
            
            // Add your rendering code for window 1 here
            
            window1.update();
        }
        
        if !window2.should_window_close() {
            window2.make_current();
            window2.clear_color(Vec4::new(0.3, 0.2, 0.2, 1.0)); // Different color
            window2.clear_depth();
            
            // Add your rendering code for window 2 here
            
            window2.update();
        }
    }
}