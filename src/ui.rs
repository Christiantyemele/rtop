use crate::system::get_ram_info;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{BarChart, Block, Borders, Paragraph, Widget},
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

// Function to format bytes into human-readable format
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

impl AppState {
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
        let (total_memory, used_memory) = get_ram_info();
        let free_memory = total_memory - used_memory;

        let formatted_total = format_bytes(total_memory);
        let formatted_used = format_bytes(used_memory);
        let formatted_free = format_bytes(free_memory);

        // Calculate percentage for the bar chart
        let memory_percentage = (used_memory as f64 / total_memory as f64 * 100.0) as u64;

        // Data for the bar chart
        let data = vec![("Used", used_memory), ("Free", free_memory)];

        // Bar chart widget
        let bar_chart = BarChart::default()
            .block(
                Block::default()
                    .title("RAM Usage")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            .data(
                &data
                    .iter()
                    .map(|(label, value)| (&**label, *value))
                    .collect::<Vec<(&str, u64)>>(),
            )
            .bar_width(3)
            .bar_style(Style::default().fg(Color::Green))
            .value_style(
                Style::default()
                    .bg(Color::Green)
                    .add_modifier(Modifier::ITALIC),
            );

        // Text for detailed memory statistics
        let ram_stats_text = Text::from(vec![
            Line::from(vec![
                Span::raw("Total Memory: "),
                Span::styled(formatted_total, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::raw("Used Memory: "),
                Span::styled(formatted_used, Style::default().fg(Color::Red)),
            ]),
            Line::from(vec![
                Span::raw("Free Memory: "),
                Span::styled(formatted_free, Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::raw(format!("Usage: {}%", memory_percentage)).fg(Color::White)
            ]),
            Line::from(vec![Span::raw("Press 'q' to quit.").fg(Color::Red)]),
        ]);

        let ram_stats_paragraph = Paragraph::new(ram_stats_text)
            .block(
                Block::default()
                    .title("Statistics")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            .alignment(ratatui::layout::Alignment::Left);

        // Key press block
        let key_press_block = Block::default()
            .title("Key Press")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green));

        // Key press display
        let key_press_display = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw("Pressed: "),
            Span::styled(self.value.to_string(), Style::default().fg(Color::Yellow)),
        ])]))
        .block(key_press_block)
        .alignment(ratatui::layout::Alignment::Center);

        // Main layout dividing the screen
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .split(area);

        // Render key press display
        frame.render_widget(key_press_display, main_layout[0]);

        // Render RAM bar chart
        frame.render_widget(bar_chart, main_layout[1]);

        // Render RAM statistics
        frame.render_widget(ram_stats_paragraph, main_layout[2]);
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
    }
}
