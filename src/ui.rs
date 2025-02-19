use crate::system::get_ram_info;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{BarChart, Block, Borders, Paragraph, StatefulWidget, Widget},
    DefaultTerminal, Frame,
};
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;

#[derive(Clone)]
pub struct AppState {
    exit: bool,
    value: char,
    ram_usage: RamUsage,
    last_update: Instant,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            exit: false,
            value: ' ',
            ram_usage: RamUsage::default(),
            last_update: Instant::now(),
        }
    }
}

#[derive(Default, Clone)]
pub struct RamUsage {
    total_memory: u64,
    used_memory: u64,
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
    // runs the application until the user quits
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<(), UiErrors> {
        // Load initial RAM info.
        self.update_ram_usage();
        self.last_update = Instant::now();

        while !self.exit {
            terminal
                .draw(|frame| {
                    let mut state = self.clone();
                    self.draw(frame, &mut state);
                })
                .map_err(|_| UiErrors::GenericError("error drawing to frame".to_owned()))?;

            self.handle_events()?;
            self.update_ram_usage();
            self.update_value();

            thread::sleep(Duration::from_millis(16));
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame, state: &mut AppState) {
        frame.render_stateful_widget(self, frame.area(), state);
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

    fn update_ram_usage(&mut self) {
        let (total_memory, used_memory) = get_ram_info();
        self.ram_usage = RamUsage {
            total_memory,
            used_memory,
        };
    }

    fn update_value(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);

        if elapsed >= Duration::from_secs(2) {
            let chars = ['A', 'B', 'C', 'D'];
            let index = (now.elapsed().as_secs() % chars.len() as u64) as usize;
            self.value = chars[index];
            self.last_update = now;
        }
    }
}

impl StatefulWidget for &AppState {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .split(area);

        // Key press display
        let key_press_block = Block::default()
            .title("Key Press")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green));

        let key_press_display = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw("Pressed: "),
            Span::styled(self.value.to_string(), Style::default().fg(Color::Yellow)),
        ])]))
        .block(key_press_block)
        .alignment(ratatui::layout::Alignment::Center);

        key_press_display.render(main_layout[0], buf);

        // RAM Usage Bar Chart
        let free_memory = state.ram_usage.total_memory - state.ram_usage.used_memory;
        let data = vec![("Used", state.ram_usage.used_memory), ("Free", free_memory)];

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
        bar_chart.render(main_layout[1], buf);

        // RAM Statistics Paragraph
        let formatted_total = format_bytes(state.ram_usage.total_memory);
        let formatted_used = format_bytes(state.ram_usage.used_memory);
        let formatted_free = format_bytes(free_memory);
        let memory_percentage = (state.ram_usage.used_memory as f64
            / state.ram_usage.total_memory as f64
            * 100.0) as u64;

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

        ram_stats_paragraph.render(main_layout[2], buf);
    }
}
