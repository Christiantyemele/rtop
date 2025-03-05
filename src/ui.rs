use std::{thread::sleep, time::Duration};

use crossterm::style::Stylize;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples

    // let title = Line::from("Rtop".bold());

    // any of the cpu's contain `num_cpu`
    sleep(Duration::from_millis(100));
    let num_cpus = app.cpu.values().nth(0).cloned().unwrap().num_cpus;

    let constraints = vec![Constraint::Percentage(100 / num_cpus as u16); num_cpus];
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints);

    let areas = layout.split(frame.area());

    for (i, cpu) in app.cpu.iter().enumerate() {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(cpu.0.to_string());
        let text = format!(
            "Brand: {}\nUsage: {}%\nFrequency: {} MHz\nTemp: {}°C",
            cpu.1.brand, cpu.1.cpu_usage, cpu.1.frequency, cpu.1.temperature
        );
        let paragraph = Paragraph::new(text).block(block);
        frame.render_widget(paragraph, areas[i]);
    }

    // frame.render_widget(
    //     Paragraph::new(format!(
    //         "This is a tui template.\n\
    //             Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
    //             Press left and right to increment and decrement the counter respectively.\n\
    //             Counter: {}",
    //         app.cpu.name
    //     ))
    //     .block(
    //         Block::bordered()
    //             .title("Template")
    //             .title_alignment(Alignment::Center)
    //             .border_type(BorderType::Rounded),
    //     )
    //     .style(Style::default().fg(Color::Cyan).bg(Color::Black))
    //     .centered(),
    //     frame.area(),
    // );
}
