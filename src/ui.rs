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
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<(), UiErrors> {
        while !self.exit {
            terminal
                .draw(|frame| self.draw(frame, AppState::default()))
                .map_err(|_| UiErrors::GenericError("error drawing to frame".to_owned()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame, mut state: AppState) {
        frame.render_stateful_widget(self, frame.area(), &mut state);
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
}

impl StatefulWidget for &AppState {
    type State = AppState;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
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

        Paragraph::new("render cpus e.g")
            .centered()
            .block(cpu_block)
            .render(layout[0], buf);
    }
}
