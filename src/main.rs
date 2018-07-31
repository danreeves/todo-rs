extern crate termion;
extern crate tui;

use std::io;
use termion::event;
use termion::input::TermRead;
use tui::backend::RawBackend;
use tui::Terminal;

mod editor;
mod renderer;

fn main() {
    let stdin = io::stdin();
    let backend = RawBackend::new().unwrap();
    let mut terminal = Terminal::new(backend).unwrap();
    let size = terminal.size().unwrap();
    let mut body = format!("Hello, planet!{}", editor::CURSOR);

    terminal.clear().unwrap();
    terminal.hide_cursor().unwrap();

    renderer::draw(&mut terminal, &size, &body);

    for c in stdin.keys() {
        let key = c.unwrap();
        match key {
            event::Key::Esc => break,
            _ => editor::handle_key(&key, &mut body),
        }
        renderer::draw(&mut terminal, &size, &body);
    }

    terminal.clear().unwrap();
    terminal.show_cursor().unwrap();
}
