use tui::backend::RawBackend;
use tui::layout::{Group, Rect, Size};
use tui::widgets::{Block, Borders, Paragraph, Widget};
use tui::Terminal;

pub fn draw(terminal: &mut Terminal<RawBackend>, size: &Rect, body: &str) {
    Block::default()
        .title(&format!(" todo-rs: {}x{} ", &size.width, &size.height))
        .borders(Borders::ALL)
        .render(terminal, &size);

    Group::default()
        .margin(1)
        .sizes(&[Size::Percent(30), Size::Percent(70)])
        .render(terminal, &size, |t, chunks| {
            Paragraph::default()
                .block(Block::default().borders(Borders::LEFT))
                .wrap(true)
                .text(body)
                .render(t, &chunks[1])
        });

    terminal.draw().expect("Failed to draw");
}
