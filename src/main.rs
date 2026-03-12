use glfw::{Action, Context, Key, Window, fail_on_errors};


// Game state struct GameState {
struct State<'a> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: (i32, i32),
    window: &'a mut Window
}

impl<'a> State<'a>{
    fn new(window: &'a mut Window) -> Self {
        let size = window.get_size();

        let instanceDescriptor = wgpu::InstanceDescriptor{
            backends: wgpu::Backends::all(), ..Default::default()
        };

        let instance = wgpu::Instance::new(&instanceDescriptor);

        let target = unsafe {
            wgpu::SurfaceTargetUnsafe::from_window(&window)
        }.unwrap();

        let surface = unsafe{
            instance.create_surface_unsafe(target)
        }.unwrap();
        
        
    }
}

fn main() {
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    let (mut window, events) = glfw.create_window(800, 600, "WGPU Begineers", glfw::WindowMode::Windowed).unwrap();

    window.set_all_polling(true);

    window.make_current();

    while !window.should_close() {
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events){
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press,_) => {
                    window.set_should_close(true);
                }

                e => {
                    println!("{:?}", e);
                }
            }
        }
        window.swap_buffers();
    }
}
