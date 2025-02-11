use std::{
    sync::{Arc, RwLock},
    thread,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, StatefulWidget, Widget},
    DefaultTerminal, Frame,
};
use thiserror::Error;

use crate::{system::CpuState, utils::SystemState};

#[derive(Default)]
pub struct UiState {
    cpu_state: Arc<RwLock<CpuState>>, // Store CPU state
}

impl UiState {
    pub fn start_worker_thread(&self, system: SystemState) {
        let system_state = Arc::new(RwLock::new(system)); // Clone the Arc
        let cpu_state = self.cpu_state.clone(); // Clone the Arc, NOT the lock guard

        thread::spawn(move || {
            let mut cpu_state_guard = cpu_state.write().unwrap(); // Lock inside thread
            let _ = cpu_state_guard.cpu_info(system_state);
        });
    }
}

#[derive(Default)]
pub struct AppState {
    exit: bool,
    // dummy
    value: char,
    system: Arc<RwLock<SystemState>>,
}

#[derive(Error, Debug)]
pub enum UiErrors {
    #[error("error {0}")]
    GenericError(String),
}

impl AppState {
    // runs the application untill the user quits
    pub fn run(
        &mut self,
        mut terminal: DefaultTerminal,
        system: SystemState,
    ) -> Result<(), UiErrors> {
        
        // handle this should not be set when exit is true

        let mut ui_state = UiState::default();
        ui_state.start_worker_thread(system);

        while !self.exit {
            terminal
                .draw(|frame| self.draw(frame, &mut ui_state))
                .map_err(|_| UiErrors::GenericError("error drawing to frame".to_owned()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame, ui_state: &mut UiState) {
        frame.render_stateful_widget(self, frame.area(), ui_state);
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

    fn set_system(&mut self, system: SystemState) {
        self.system = Arc::new(RwLock::new(system))
    }
}

impl StatefulWidget for &AppState {
    type State = UiState;
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

            let cpu_state = state.cpu_state.read().unwrap();

            let a = format!(
                "{}, {}, {}",
                cpu_state.brand, cpu_state.cpu_usage, cpu_state.frequency
            );

            Paragraph::new(a)
                .centered()
                .block(cpu_block.clone())
                .render(layout[0], buf);
        }
    }
}
