

// Kamikaze Shrimp
// Rust game engine.

// ! CURRENTLY JUST LEARNING OPENGL RUST
extern crate glfw;

use self::glfw::{ Context, Key, Action };

use std::sync::mpsc::Receiver;
use std::ptr;

extern crate gl;

mod objects;
mod shader_class;
mod texture_functions;

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

    let (vaos, used_shaders, textures) = unsafe {
        let mut used_shaders: Vec<Shader> = Vec::new();
        let mut vaos: Vec<VAO_STRUCT> = Vec::new();

        used_shaders.push(Shader::new(
            "src/shaders/textured_vector_shader.vs",
            "src/shaders/textured_fragment_shader.fs"
        ));


        vaos.push(VAO_STRUCT {
            vao_index: 0,
            vao: objects::triangle::create_2d_texture_triangle([
                // positions       // colors        // texture coords
                 0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
                 0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
                -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
                -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left
            ].to_vec(), [
                0, 1, 3,
                1, 2, 3
            ].to_vec()),
            load_style: "drawElements".to_string()
        });

        used_shaders.push(Shader::new(
            "src/shaders/color_vector_shader.vs",
            "src/shaders/color_fragment_shader.fs"
        ));

        vaos.push(VAO_STRUCT {
            vao_index: 1,
            vao: objects::triangle::create_2d_color_triangle([
                0.25, -0.25,0.0,   1.0, 0.0, 0.0,  // bottom right
               -0.25, -0.25, 0.0,  0.0, 1.0, 0.0,  // bottom left
                0.0,  0.25, 0.0,  0.0, 0.0, 1.0   // top
            ].to_vec(), [
                0, 1, 3
            ].to_vec()),
            load_style: "drawArrays".to_string()
        });

        let mut textures : Vec<u32> = Vec::new();

        textures.push(texture_functions::new_texture("src/textures/Maurice.jpg"));

        (vaos, used_shaders, textures)
    };

    // Render loop
    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            for i in 0..vaos.len() {
                if let Some(_value) = textures.get(i) {
                    gl::BindTexture(gl::TEXTURE_2D, textures[i]);
                }

                let vao = &vaos[i].vao;
                let shader = &used_shaders[vaos[i].vao_index];

                shader.use_program();

                gl::BindVertexArray(*vao);

                if vaos[i].load_style == "drawArrays".to_string() {
                    gl::DrawArrays(gl::TRIANGLES, 0, 3);
                }
                else if vaos[i].load_style == "drawElements".to_string() {
                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
                }
                else {
                    panic!("Invalid load_style");
                }
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

#[allow(non_camel_case_types)]
struct VAO_STRUCT {
    pub vao_index: usize,
    pub vao: u32,
    pub load_style: String,
}
