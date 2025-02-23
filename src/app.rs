use std::error;

use crate::systems::CpuState;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
/// Holds application state
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    /// cpu
    pub cpu: CpuState,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = true;
    }

    /// update cpu info
    pub fn cpu_handle(&mut self, cpu: CpuState) {
        self.cpu = cpu
    }
}
