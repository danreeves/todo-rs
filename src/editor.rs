use termion::event;

pub const CURSOR: char = '_';

pub fn handle_key(key: &event::Key, body: &mut String) {
    match key {
        event::Key::Backspace => backspace(body),
        event::Key::Char(ch) => update_text(body, ch),
        _ => {}
    }
}

fn update_text(text: &mut String, ch: &char) {
    // Remove the cursor
    text.pop();
    // Add the new character
    text.push(*ch);
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
