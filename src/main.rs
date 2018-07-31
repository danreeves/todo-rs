extern crate termion;
extern crate tui;

use std::io;
use termion::event;
use termion::input::TermRead;
use tui::backend::RawBackend;
use tui::layout::{Group, Rect, Size};
use tui::widgets::{Block, Borders, Paragraph, Widget};
use tui::Terminal;

const CURSOR: char = '_';

fn main() {
    let stdin = io::stdin();
    let backend = RawBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();
    let size = terminal.size().unwrap();
    let mut body = format!("Hello, planet!{}", CURSOR);

    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    draw(&mut terminal, &size, &body);

    for c in stdin.keys() {
        let key = c.unwrap();
        match key {
            event::Key::Esc => break,
            event::Key::Backspace => backspace(&mut body),
            event::Key::Char(ch) => update_text(&mut body, ch),
            _ => {}
        }
        draw(&mut terminal, &size, &body);
    }

    terminal.clear().unwrap();
    terminal.show_cursor().unwrap();
}

fn draw(terminal: &mut Terminal<RawBackend>, size: &Rect, body: &str) {
    Block::default()
        .title(&format!(" todo-rs: {}x{} ", &size.width, &size.height))
        .borders(Borders::ALL)
        .render(terminal, &size);

    Group::default()
        .margin(2)
        .sizes(&[Size::Percent(20), Size::Percent(80)])
        .render(terminal, &size, |t, chunks| {
            Paragraph::default()
                .wrap(true)
                .text(body)
                .render(t, &chunks[0])
        });

    terminal.draw().expect("Failed to draw");
}

fn update_text(text: &mut String, ch: char) {
    // Remove the cursor
    text.pop();
    // Add the new character
    text.push(ch);
    // Add the cursor again
    text.push(CURSOR);
}

fn backspace(text: &mut String) {
    // Remove the cursor
    text.pop();
    // Delete a character
    text.pop();
    // Add the cursor again
    text.push(CURSOR);
}
