extern crate glfw;

use std::env;
use glfw::WindowHint;
use glfw::OpenGlProfileHint;
use glfw::WindowMode;

fn main() {
    let mut fullscreen = false;
    let title = "OpenGL 3.3 Demo";
    
    for arg in env::args() {
        if arg == "f" || arg == "-f" {
            fullscreen = true;
            println!("Fullscreen");
        }
    }
    
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    
    glfw.window_hint(WindowHint::Samples(4));
    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::ContextVersionMinor(3));
    glfw.window_hint(WindowHint::Resizable(false));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    
    let (window, events) = glfw.with_primary_monitor(
        |glfw, m| {
            let width = m.map_or(640,
                |m| {
                    m.get_video_mode().map_or(640,
                        |vm| { vm.width })});
            let height = m.map_or(480,
                |m| {
                    m.get_video_mode().map_or(480,
                        |vm| { vm.height })});
            glfw.create_window(width, height, title,
                m.map_or(WindowMode::Windowed,
                    |m| {
                        match fullscreen {
                            true => WindowMode::FullScreen(m),
                            false => WindowMode::Windowed
                        }
                    }
                )
            )
        }
    ).expect("Failed to create GLFW window.");
    
    glfw.make_context_current(Some(&window));
    glfw.set_swap_interval(1);
}
