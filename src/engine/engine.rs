use std::io;
use super::canvas::Canvas;
use std::time::{Duration, Instant};

// 1. Make the trait object-safe
pub trait IApplication {
    // Optional setup hook called by the engine before the main loop starts
    fn on_init(&mut self) {}

    fn on_update(&mut self, delta_time: f32);
    fn on_draw(&mut self, canvas: &mut Canvas);
}

pub struct Engine {
    stop: bool,
    canvas: Canvas,
    delta_time: f32,
    application: Box<dyn IApplication>,
}

impl Engine {
    // Constructor takes ownership of the application and configuration values
    pub fn new<T: IApplication + 'static>(width: usize, height: usize, application: T) -> Self {
        Self {
            stop: false,
            canvas: Canvas::new(width, height),
            delta_time: 0.0,
            application: Box::new(application),
        }
    }

    // Main run loop
    pub fn run(&mut self) -> io::Result<()> {
        self.application.on_init();

        let mut last_frame = Instant::now();

        loop {
            if self.stop {
                return Ok(());
            }

            let now = Instant::now();
            self.delta_time = now.duration_since(last_frame).as_secs_f32();
            last_frame = now;

            self.application.on_update(self.delta_time);

            self.canvas.clear();
            self.application.on_draw(&mut self.canvas);

            self.canvas.render()?;

            std::thread::sleep(Duration::from_millis(16));
        }
    }
}