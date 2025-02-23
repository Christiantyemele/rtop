use std::{
    io,
    sync::{Arc, RwLock},
};

use ratatui::{prelude::CrosstermBackend, Terminal};
use rtop::{application::{App, AppResult}, event::{Event, EventHandler}, handler::handle_key_events, systems::CpuState, tui::Tui, utils::SystemState};




#[derive(Default)]
pub struct UiState {
    pub cpu_state: CpuState,
    pub system: Arc<RwLock<SystemState>>,
}


#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();
    let (tx, rx) = std::sync::mpsc::channel();
    let ui_state = std::sync::Arc::new(UiState::default());

    std::thread::spawn(move || start_worker_thread(ui_state, tx));

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        if let Ok(data) = rx.try_recv() {
            app.handle_cpu(data);
        }
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

fn start_worker_thread(systemstate: Arc<UiState>, tx: std::sync::mpsc::Sender<CpuState>) {
    let mut cpu_state = systemstate.cpu_state.clone();
    let system_state = systemstate.system.clone();

    cpu_state.cpu_info(system_state, tx).unwrap();
}
