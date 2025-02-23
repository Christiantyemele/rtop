use std::{
    io,
    sync::{Arc, RwLock},
};

use ratatui::{prelude::CrosstermBackend, Terminal};
use rtop::{
    app::{App, AppResult},
    event::{Event, EventHandler},
    handler::handle_key_events,
    systems::CpuState,
    tui::Tui,
    utils::SystemState,
};

#[derive(Default)]
pub struct Global {
    pub cpu_state: CpuState,
    pub system: Arc<RwLock<SystemState>>,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();
    let (tx, rx) = std::sync::mpsc::channel();
    let state = std::sync::Arc::new(Global::default());
    tokio::task::spawn(async move { cpu_worker_task(state, tx) });

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while !app.running {
        // Render the user interface.
        tui.draw(&mut app)?;

        // handle cpu events without blocking
        if let Ok(data) = rx.try_recv() {
            app.cpu_handle(data);
        }

        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

fn cpu_worker_task(systemstate: Arc<Global>, tx: std::sync::mpsc::Sender<CpuState>) -> Result<(), String> {
    let mut cpu_state = systemstate.cpu_state.clone();
    let system_state = systemstate.system.clone();

    cpu_state.cpu_info(system_state, tx).map_err(|e| e)
}
