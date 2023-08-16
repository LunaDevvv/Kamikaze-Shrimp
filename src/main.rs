// Kamikaze Shrimp
// Rust game engine.

// ! CURRENTLY JUST LEARNING OPENGL RUST
extern crate glfw;

use self::glfw::{ Context, Key, Action };

extern crate gl;
use self::gl::types::*;

use std::sync::mpsc::Receiver;
use std::ffi::CString;
use std::ptr;
use std::str;

mod objects;

// Settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
out vec3 our_color;
void main() {
   gl_Position = vec4(aPos, 1.0);
   our_color = aColor;
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330 core
out vec4 FragColor;
in vec3 our_color;
void main() {
   FragColor = vec4(our_color, 1.0f);
}
"#;

fn main() {
    // Init glfw
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // glfw window creation
    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "Kamikaze Shrimp", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // Load all gl function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (vaos, shader_program) = unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1);
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(vertex_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
        }

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();

        gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(fragment_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
        }

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(shader_program, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        let mut vaos : Vec<u32> = Vec::new();

        vaos.push(objects::triangle::create_triangle([
            0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
            -0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
            0.0, 0.5, 0.0, 0.0, 0.0, 0.0
        ].to_vec(), [
            0, 1, 2,
            1, 2, 3
        ].to_vec()));

        (vaos, shader_program)
    };

    // Render loop
    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);

            for i in 0..vaos.len() {
                gl::BindVertexArray(vaos[i]);
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}
