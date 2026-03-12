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
    async fn new(window: &'a mut Window) {
        let size = window.get_size();

        let instance_descriptor = wgpu::InstanceDescriptor{
            backends: wgpu::Backends::all(), ..Default::default()
        };

        let instance = wgpu::Instance::new(&instance_descriptor);

        let target = unsafe {
            wgpu::SurfaceTargetUnsafe::from_window(&window)
        }.unwrap();

        let surface = unsafe{
            instance.create_surface_unsafe(target)
        }.unwrap();
        
        let adapter_descriptor = wgpu::RequestAdapterOptionsBase{
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };

        let adapter = instance.request_adapter(&adapter_descriptor).await.unwrap();

        let device_descriptor = wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("Device"),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            memory_hints: wgpu::MemoryHints::default(),
            trace: wgpu::Trace::default(),
        };

        let (device, queue) = adapter.request_device(&device_descriptor).await.unwrap();
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
