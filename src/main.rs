use rtop::ui::AppState;

fn main() {
    // render ui
    let terminal = ratatui::init();
    AppState::new().run(terminal).ok();
    ratatui::restore();
}