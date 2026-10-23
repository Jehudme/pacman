use std::io;
use super::canvas::Canvas;
use std::time::{Duration, Instant};
use crate::engine::entity::EntityBehavior;

// 1. Make the trait object-safe
pub trait ApplicationBehavior: EntityBehavior {
    // Optional setup hook called by the engine before the main loop starts
    fn init(&mut self) {}
}

pub struct Engine {
    stop: bool,
    canvas: Canvas,
    delta_time: f32,
    application: Box<dyn ApplicationBehavior>,
}

impl Engine {
    // Constructor takes ownership of the application and configuration values
    pub fn new<T: ApplicationBehavior + 'static>(width: usize, height: usize, application: T) -> Self {
        Self {
            stop: false,
            canvas: Canvas::new(width, height),
            delta_time: 0.0,
            application: Box::new(application),
        }
    }

    // Main run loop
    pub fn run(&mut self) -> io::Result<()> {
        self.application.init();

        let mut last_frame = Instant::now();

        loop {
            if self.stop {
                return Ok(());
            }

            let now = Instant::now();
            self.delta_time = now.duration_since(last_frame).as_secs_f32();
            last_frame = now;

            self.application.update(self.delta_time);

            self.canvas.clear();
            self.application.draw(&mut self.canvas);

            self.canvas.render()?;

            std::thread::sleep(Duration::from_millis(16));
        }
    }
}