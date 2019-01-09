mod shader;

use self::shader::Shader;

use gl;
use glfw::{Action, Context, Key};

fn main() {
    let mut width = 512.0;
    let mut height = 512.0;

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .with_primary_monitor(|glfw, _| {
            glfw.create_window(width as u32, height as u32, "SDF", glfw::WindowMode::Windowed)
        })
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_all_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Normal);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
        gl::ClearColor(0.2, 0.2, 0.2, 1.0);
    }
    println!("GLFW Version {}", glfw::get_version());

    let shader = Shader::new();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::Size(w, h) => {
                    unsafe {
                        gl::Viewport(0,0,w,h);
                    }
                    width = w as f32;
                    height = h as f32;
                }
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        shader.draw(width, height);

        window.swap_buffers();
    }
}
