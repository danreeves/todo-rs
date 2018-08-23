extern crate chrono;

use chrono::naive::NaiveDate;
use std::fs::{create_dir, read_dir, read_to_string};

const ROOT_DIR: &'static  = "~/.todo-rs"

struct Todo {
    done: bool,
    text: String
}

struct Day {
    date: NaiveDate,
    todos: Vec<Todo>
}

pub fn init() -> Vec<Day> {
    // Create folder if it doesn't exist
    create_dir(ROOT_DIR);

    let files = read_dir(ROOT_DIR);

    if !files.is_ok() {
        // No files
        return vec![]
    }

    for file in files {
        let text = read_to_string(file.path());
    }
}
