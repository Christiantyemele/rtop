use rtop::{ui::AppState, utils::SystemState};

fn main() {
    // render ui
    let terminal = ratatui::init();
    let system = SystemState::new();
    AppState::default().run(terminal, system).ok();
    ratatui::restore();
}
