use ratatui::{
    layout::{Alignment, Constraint, Layout}, style::{Color, Style, Stylize}, symbols::border, text::{Line, Text}, widgets::{block::title, Block, BorderType, Paragraph, Widget}, Frame
};

use crate::application::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    frame.render_widget(
        Paragraph::new(format!(
            "This is a tui template.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            app.cpu.name
        ))
        .block(
            Block::bordered()
                .title("Template")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .centered(),
        frame.area()
    
        
    );
    
    // let title = Line::from("Rtop".bold().black());
    //         let block = Block::bordered()
    //             .border_set(border::PLAIN)
    //             .border_style(Style::new().green())
    //             .title(title.centered());

    //         let cpu_block = Block::bordered()
    //             .border_set(border::PLAIN)
    //             .border_style(Style::new().green());

    //         // application layout
    //         let layout = Layout::default()
    //             .direction(ratatui::layout::Direction::Vertical)
    //             .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
    //             .split(frame.area());

    //         let upper_layout = Layout::default()
    //             .direction(ratatui::layout::Direction::Horizontal)
    //             .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
    //             .split(layout[0]);

    //         let cpu_layout = Layout::default()
    //             .direction(ratatui::layout::Direction::Horizontal)
    //             .constraints([Constraint::Percentage(5)].repeat(8))
    //             .split(layout[1]);


    //         match app.cpu.name.as_str() {
    //             "cpu0" => {
    //                 let a = format!(
    //                     "{}, {}, {}, {}",
    //                     app.cpu.name, app.cpu.brand, app.cpu.cpu_usage, app.cpu.frequency
    //                 );

    //                 Paragraph::new(a)
    //                     .left_aligned()
    //                     .block(cpu_block.clone())
    //                     .render(cpu_layout[0], buf);
    //             }
    //             "cpu1" => {
    //                 let a = format!(
    //                     "{}, {}, {}, {}",
    //                     app.name, app.brand, app.cpu_usage, app.frequency
    //                 );

    //                 Paragraph::new(a)
    //                     .left_aligned()
    //                     .block(cpu_block.clone())
    //                     .render(cpu_layout[1], buf);
    //             }
    //             _ => {}
    //         }
    //         let a = format!(
    //             "{}, {}, {}, {}",
    //             app.name, app.brand, app.cpu_usage, app.frequency
    //         );

    //         Paragraph::new(a)
    //             .left_aligned()
    //             .block(cpu_block.clone())
    //             .render(layout[0], buf);
}