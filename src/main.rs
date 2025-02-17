use std::{
    sync::{
        mpsc::{self, SyncSender},
        Arc,
    },
    thread,
};

use rtop::{
    system::CpuState,
    ui::{self, AppState, UiState},
};
#[tokio::main]
async fn main() {
    let ui_state = Arc::new(UiState::default());

    let (tx, rx) = mpsc::sync_channel(1000);

    thread::spawn(move || start_worker_thread(ui_state, tx));

    let terminal = ratatui::init();
    AppState::default().run(terminal, rx).await.ok();
    ratatui::restore();
}

fn start_worker_thread(systemstate: Arc<UiState>, tx: SyncSender<CpuState>) {
    let mut cpu_state = systemstate.cpu_state.clone();
    let system_state = systemstate.system.clone();

    cpu_state.cpu_info(system_state, tx).unwrap();
}
