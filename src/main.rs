// Kamikaze Shrimp
// Rust game engine.

// ! CURRENTLY JUST LEARNING OPENGL RUST
extern crate glfw;

use self::glfw::{ Context, Key, Action };

use std::sync::mpsc::Receiver;

extern crate gl;

mod objects;
mod shader_class;

use shader_class::Shader;

// Settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

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

    let (vaos, current_shaders) = {
        let current_shaders = Shader::new(
            "src/shaders/colorful_triangle.vs",
            "src/shaders/colorful_triangle.fs"
        );

        let mut vaos : Vec<u32> = Vec::new();

        vaos.push(objects::triangle::create_triangle([
            0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
            -0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
            0.0, 0.5, 0.0, 0.0, 0.0, 0.0
        ].to_vec(), [
            0, 1, 2,
            1, 2, 3
        ].to_vec()));

        (vaos, current_shaders)
    };

    // Render loop
    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            current_shaders.use_program();

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
