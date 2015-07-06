extern crate glfw;
extern crate gl;

use std::env;
use std::str;
use std::ptr;
use std::mem;
use std::ffi::CString;

use glfw::WindowHint;
use glfw::OpenGlProfileHint;
use glfw::WindowMode;
use glfw::Context;

use gl::types::*;

fn compile_shader(shader_type: GLenum, source: &str) -> GLuint {
    let c_str = CString::new(source.as_bytes()).unwrap();
    
    let shader = unsafe { gl::CreateShader(shader_type) };
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);
        
        let mut param: GLint = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut param);
        
        if param != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::<u8>::with_capacity(len as usize);
            buf.set_len((len as usize) - 1);
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}

fn link_program(vert: GLuint, frag: GLuint) -> GLuint {
    let program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(program, vert);
        gl::AttachShader(program, frag);
        gl::LinkProgram(program);
        
        let mut param = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut param);
        
        if param != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::<u8>::with_capacity(len as usize);
            buf.set_len((len as usize) - 1);
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            panic!("{}", str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8"));
        }
    }
    program
}

/* set this up later...
struct graphics_context {
    window: glfw::Window,
    program: GLuint,
    uniform_angle: GLint,
    vbo_point: GLuint,
    vao_point: GLuint,
    angle: f64,
    framecount: i64,
    lastframe: f64,
}
*/

fn render(program: GLuint, uniform_angle: &GLint, angle: &mut f64, vao_point: GLuint, glfw: &glfw::Glfw, lastframe: &mut f64, framecount: &mut i64, window: &mut glfw::Window) {
    unsafe {
        gl::ClearColor(0.15, 0.15, 0.15, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        
        gl::UseProgram(program);
        gl::Uniform1f(*uniform_angle, *angle as GLfloat);
        gl::BindVertexArray(vao_point);
        gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        gl::BindVertexArray(0);
        gl::UseProgram(0);
    }
        
    /* Physics */
    let now = glfw.get_time();
    let udiff = now - *lastframe;
    *angle += 1.0 * udiff;
    if *angle > (2.0 * std::f64::consts::PI) {
        *angle -= 2.0 * std::f64::consts::PI;
    }
    *framecount += 1;
    if (now as i64) != (*lastframe as i64) {
        println!("FPS: {}", framecount);
    }
    *framecount = now as i64;
    
    window.swap_buffers();
}

fn main() {
    /* Options */
    let mut fullscreen = false;
    let title = "OpenGL 3.3 Demo";
    
    for arg in env::args() {
        if arg == "f" || arg == "-f" {
            fullscreen = true;
            println!("Fullscreen");
        }
    }
    
    /* Create window and OpenGL context */
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    
    glfw.window_hint(WindowHint::Samples(4));
    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::ContextVersionMinor(3));
    glfw.window_hint(WindowHint::Resizable(false));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    
    let (mut window, _) = glfw.with_primary_monitor(
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
    
    window.make_current();
    glfw.set_swap_interval(1);
    
    gl::load_with(|s| window.get_proc_address(s));
    
    /* Shader sources */
    static VERT_SHADER: &'static str =
        "#version 330\n\
        layout(location = 0) in vec2 point;\n\
        uniform float angle;\n\
        void main() {\n\
            mat2 rotate = mat2(cos(angle), -sin(angle),\n\
                               sin(angle), cos(angle));\n\
            gl_Position = vec4(0.75 * rotate * point, 0.0, 1.0);\n\
        }\n";
    
    static FRAG_SHADER: &'static str =
        "#version 330\n\
        out vec4 color;\n\
        void main() {\n\
            color = vec4(1, 0.15, 0.15, 0);\n\
        }\n";
    
    /* Compile and link OpenGL program */
    let vert = compile_shader(gl::VERTEX_SHADER, VERT_SHADER);
    let frag = compile_shader(gl::FRAGMENT_SHADER, FRAG_SHADER);
    let program = link_program(vert, frag);
    let uniform_angle = unsafe {
        gl::GetUniformLocation(program, CString::new("angle").unwrap().as_ptr())
    };
    unsafe {
        gl::DeleteShader(frag);
        gl::DeleteShader(vert);
    }
    
    static SQUARE: [GLfloat; 8] = [
        -1.0,  1.0,
        -1.0, -1.0,
         1.0,  1.0,
         1.0, -1.0
    ];
    
    /* Prepare vertex buffer object (VBO) */
    let mut vbo_point = 0 as GLuint;
    unsafe {
        gl::GenBuffers(1, &mut vbo_point);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_point);
        gl::BufferData(gl::ARRAY_BUFFER,
                       mem::size_of_val(&SQUARE) as GLsizeiptr,
                       mem::transmute(&SQUARE[0]), //ugly...
                       gl::STATIC_DRAW);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
    
    /* Prepare vertex array object (VAO) */
    let mut vao_point = 0 as GLuint;
    unsafe {
        gl::GenVertexArrays(1, &mut vao_point);
        gl::BindVertexArray(vao_point);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_point);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    
    let mut angle: f64 = 0.0;
    
    /* Start main loop */
    window.set_key_polling(true);
    let mut lastframe = glfw.get_time();
    let mut framecount = 0;
    while !window.should_close() {
        //render(...);
        render(
            program,
            &uniform_angle,
            &mut angle,
            vao_point,
            &glfw,
            &mut lastframe,
            &mut framecount,
            &mut window
        );
        glfw.poll_events();
    }
    println!("Exiting ...");
    
    /* Cleanup and exit */
    unsafe {
        gl::DeleteVertexArrays(1, &vao_point);
        gl::DeleteBuffers(1, &vbo_point);
        gl::DeleteProgram(program);
    }
    
    /* glfw::init automatically schedules glfw_terminate via libc::atexit,
     * so we don't need to do anything else here.
     */
}
