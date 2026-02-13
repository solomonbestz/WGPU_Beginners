use glfw::{Action, Context, Key, fail_on_errors};

fn main() {
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    let (mut window, events) = glfw.create_window(800, 600, "WGPU Begineers", glfw::WindowMode::Windowed).unwrap();

    window.make_current();

    while !window.should_close() {
        glfw.poll_events();
        window.swap_buffers();
    }
}
