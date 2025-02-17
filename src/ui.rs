use std::{
    self,
    os::linux::raw::stat,
    sync::{
        mpsc::{channel, Receiver},
        Arc, RwLock,
    },
    thread,
    time::Duration,
};

use crate::{system::CpuState, ui, utils::SystemState};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, StatefulWidget, Widget},
    DefaultTerminal, Frame,
};
use sysinfo::Cpu;
use thiserror::Error;
use tokio::time;

#[derive(Default)]
pub struct UiState {
    pub cpu_state: CpuState,
    pub system: Arc<RwLock<SystemState>>,
}

// impl UiState {
//     pub fn start_worker_thread(&self, system: Arc<RwLock<SystemState>>) {
//         let system_state = system;
//         let cpu_state = self.cpu_state.clone();
//         let shared_cpu_state = cpu_state;

//         let mut cpu_state_guard = shared_cpu_state;
//         let _ =cpu_state_guard.cpu_info(system_state);
//     }
// }

#[derive(Default)]
pub struct AppState {
    exit: bool,
    // dummy
    value: char,
}

#[derive(Error, Debug)]
pub enum UiErrors {
    #[error("error {0}")]
    GenericError(String),
}

impl AppState {
    // runs the application untill the user quits
    pub async fn run(
        &mut self,
        mut terminal: DefaultTerminal,
        rx: Receiver<CpuState>,
    ) -> Result<(), UiErrors> {
        let (timer_tx, mut timer_rx) = tokio::sync::mpsc::channel(10);

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(1));
            loop {
                interval.tick().await;
                if timer_tx.send(()).await.is_err() {
                    break;
                }
            }
        });

        while !self.exit {
            tokio::select! {
            _ = timer_rx.recv() => {
                let cpu = rx.recv().unwrap();
                                terminal
                                    .draw(|frame| self.draw(frame, cpu)) // clonining and arc
                                    .map_err(|_| UiErrors::GenericError("error drawing to frame".to_owned())).unwrap();
                                self.handle_events().unwrap();
                            }


                    };
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame, mut rx: CpuState) {
        frame.render_stateful_widget(self, frame.area(), &mut rx);
    }

    fn handle_events(&mut self) -> Result<(), UiErrors> {
        // use blocking event for now
        match event::read() {
            Ok(Event::Key(key_event)) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(key_event);
                Ok(())
            }
            // ignore mouse events for now
            _ => Ok(()),
        }
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char(char) => self.handle_char(char),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_char(&mut self, value: char) {
        self.value = value
    }
    // Ok(())
}

impl StatefulWidget for &AppState {
    type State = CpuState;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        {
            let title = Line::from("Rtop".bold().black());
            let block = Block::bordered()
                .border_set(border::PLAIN)
                .border_style(Style::new().green())
                .title(title.centered());

            let cpu_block = Block::bordered()
                .border_set(border::PLAIN)
                .border_style(Style::new().green());

            // application layout
            let layout = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(area);

            let display = Text::from(vec![Line::from(vec![
                "pressed ".into(),
                self.value.to_string().yellow(),
            ])]);

            Paragraph::new(display)
                .centered()
                .block(block)
                .render(layout[1], buf);

            let num_block = state.num_cpus;
            let sections = 100 / num_block;
            // iterate to create cpu sections

            let upper_layout = Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(layout[0]);

            let cpu_layout = Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .constraints([Constraint::Percentage(5)].repeat(8))
                .split(layout[1]);

            Paragraph::new("cpu")
                .left_aligned()
                .block(cpu_block.clone())
                .render(cpu_layout[0], buf);

            match state.name.as_str() {
                "cpu0" => {
                    let a = format!(
                        "{}, {}, {}, {}",
                        state.name, state.brand, state.cpu_usage, state.frequency
                    );

                    Paragraph::new(a)
                        .left_aligned()
                        .block(cpu_block.clone())
                        .render(cpu_layout[0], buf);
                }
                "cpu1" => {
                    let a = format!(
                        "{}, {}, {}, {}",
                        state.name, state.brand, state.cpu_usage, state.frequency
                    );

                    Paragraph::new(a)
                        .left_aligned()
                        .block(cpu_block.clone())
                        .render(cpu_layout[1], buf);
                }
                _ => {}
            }
            let a = format!(
                "{}, {}, {}, {}",
                state.name, state.brand, state.cpu_usage, state.frequency
            );

            Paragraph::new(a)
                .left_aligned()
                .block(cpu_block.clone())
                .render(layout[0], buf);
        }
    }
}
