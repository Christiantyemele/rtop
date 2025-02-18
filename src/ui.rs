use crate::system::get_ram_info;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::{thread, time::Duration};
use thiserror::Error;

#[derive(Default)]
pub struct AppState {
    exit: bool,
    value: char,
}

#[derive(Error, Debug)]
pub enum UiErrors {
    #[error("error {0}")]
    GenericError(String),
}

impl AppState {
    // Runs the application until the user quits
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<(), UiErrors> {
        while !self.exit {
            terminal
                .draw(|frame| self.draw(frame))
                .map_err(|_| UiErrors::GenericError("error drawing to frame".to_owned()))?;
            self.handle_events()?;
            thread::sleep(Duration::from_secs(1));
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();
        let mut buf = frame.buffer_mut();
        let (total_memory, used_memory) = get_ram_info();
        let memory_percentage = (used_memory as f64 / total_memory as f64 * 100.0).round() as u64;

        let memory_info = format!(
            "RAM Usage: {} / {} ({}%)",
            used_memory, total_memory, memory_percentage
        );

        // Memory display
        let memory_display = Text::from(vec![
            Line::from("Real-Time RAM Monitor".bold().fg(Color::Cyan)),
            Line::from(memory_info),
            Line::from("Press 'q' to quit.".fg(Color::Red)),
        ]);

        // Key press display
        let key_press_display = Text::from(vec![Line::from(vec![
            "Pressed: ".into(),
            self.value.to_string().yellow(),
        ])]);

        // Define layout
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        // Create a block for the memory display
        let memory_block = Block::bordered()
            .border_set(border::PLAIN)
            .border_style(Style::new().green())
            .title("Memory Info".fg(Color::Yellow));

        // Create a block for the key press display
        let key_press_block = Block::bordered()
            .border_set(border::PLAIN)
            .border_style(Style::new().green())
            .title("Key Press".to_string());

        // Render memory info in the UI
        Paragraph::new(memory_display)
            .block(memory_block)
            .render(layout[1], &mut buf);

        // Render key press info in the UI
        Paragraph::new(key_press_display)
            .centered()
            .block(key_press_block)
            .render(layout[0], &mut buf);
    }

    fn handle_events(&mut self) -> Result<(), UiErrors> {
        match event::read() {
            Ok(Event::Key(key_event)) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_events(key_event);
                Ok(())
            }
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
        self.value = value;
    }
}

impl Widget for &AppState {
    fn render(self, _area: ratatui::prelude::Rect, _buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // This implementation is not needed since rendering is handled in draw()
    }
}
