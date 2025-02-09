use rtop::ui::AppState;


fn main() {
    // render ui
    let terminal = ratatui::init();
    AppState::default().run(terminal).ok();
    ratatui::restore();
}