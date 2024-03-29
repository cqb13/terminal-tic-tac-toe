use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal, ExecutableCommand,
};
use std::io;

pub struct OptionSelect {
    title: String,
    options: Vec<String>,
}

impl OptionSelect {
    pub fn new() -> Self {
        OptionSelect {
            title: String::new(),
            options: Vec::new(),
        }
    }

    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn add_option(mut self, option: String) -> Self {
        self.options.push(option);
        self
    }

    pub fn ask(&self) -> String {
        let mut current_option = 0;
        println!("{}:", self.title);

        loop {
            for (i, option) in self.options.iter().enumerate() {
                if i == current_option {
                    println!("> [{}] {}", i + 1, option);
                    continue;
                }
                println!("  [{}] {}", i + 1, option);
            }
            terminal::enable_raw_mode().expect("Failed to enable raw mode");

            let event = read().unwrap();
            match event {
                Event::Key(KeyEvent {
                    code,
                    kind: KeyEventKind::Press,
                    ..
                }) => match code {
                    KeyCode::Char('q') => {
                        terminal::disable_raw_mode().unwrap();
                        std::process::exit(0);
                    }
                    KeyCode::Up => {
                        if current_option > 0 {
                            current_option -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if current_option < self.options.len() - 1 {
                            current_option += 1;
                        }
                    }
                    KeyCode::Enter => {
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");
                        return self.options[current_option].to_string();
                    }
                    _ => {}
                },
                _ => {}
            }
            terminal::disable_raw_mode().expect("Failed to disable raw mode");
            clear_lines(self.options.len() as u16);
        }
    }
}

fn clear_lines(lines: u16) {
    for _ in 0..lines {
        io::stdout().execute(cursor::MoveUp(1)).unwrap();
        io::stdout()
            .execute(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();
    }
}
