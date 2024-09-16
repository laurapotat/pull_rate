use std::time::Instant;

use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::EventLoop, window::Window};

struct App {
    window: Window,

    times: Vec<u128>,
    timer: Instant,
}

impl App {
    fn new(window: Window) -> Self {
        Self {
            window: window,
            times: vec![],
            timer: Instant::now(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // dont need to do anything in here on linux i belive?
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        window_event: winit::event::WindowEvent,
    ) {
        if let WindowEvent::KeyboardInput { 
            device_id, 
            event, 
            is_synthetic 
        } = &window_event {
            let elapsed = self.timer.elapsed().as_millis();
            // just store the fact that teh key was presesd 
            self.times.push(elapsed);
            self.timer = Instant::now();
        }

        if let WindowEvent::CloseRequested = &window_event {
            self.times.sort();
            let min = self.times[0];
            let median = self.times[self.times.len() / 2];
            let mean = self.times.iter().fold(0, |acc, n| acc + n) 
                / self.times.len() as u128;

            println!("\n");
            println!("whole times:\n {:?}", self.times); // <- peperchinho de codigo gwuehheh
            println!("\n");

            println!("median: {:>5}ms", median);
            println!("mean:   {:>5}ms", mean);
            println!("min:    {:>5}ms", min);

            println!("\n");

            println!("median /  8 (128hz): {:>5.2}", median as f64 / 8.);
            println!("median /  4 (256hz): {:>5.2}", median as f64 / 4.);
            println!("median /  2 (512hz): {:>5.2}", median as f64 / 2.);
            event_loop.exit();
        }
    }
}

const INIT_MESSAGE: &'static str = 
"
spam your keyboard as much as u can for few seconds then close 
the window to print the stats. 
the meadian delay should be the pullrate ur keybord has. 

you can also tell if most of the values in `whole times` (-+1) devide 
by your expected wait period (eg 8ms for 128hz 4 ms for 256 etc) 
without any remainder

(i assume the 0s happen when you press the key on the same pull row and 
microcontroller sends them at the same time or something)
";

fn main() {
    println!("{}", INIT_MESSAGE);

    let event_loop = EventLoop::new().expect("cant create event loop");
    let window_attributes = Window::default_attributes()
        .with_title("pull rate sucker");

    let window = event_loop
        .create_window(window_attributes)
        .expect("cant create window");

    let mut app = App::new(window);

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    event_loop.run_app(&mut app).expect("cant run app");
}
