#[allow(dead_code)]
mod event;

use std::io;
use std::{fs,env};
use std::path::Path;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, SelectableList, Borders, Block};
use tui::style::{Color, Modifier, Style};
use tui::layout::{Layout, Constraint, Direction};
use::failure;

use crate::event::{Event, Events};

struct ListState<T> {
    items: Vec<T>,
    selected: usize,
}

// add permissions, current user, showHidden, etc. later
struct App <'a> {
    dir: &'a mut str,
    contents: ListState<&'a str>,
}

impl<T> ListState<T> {
    fn new(items: Vec<T>) -> ListState<T> {
        ListState { items, selected: 0 }
    }
    
    fn select_previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    fn select_next(&mut self) {
        if self.selected < self.items.len() - 1 {
            self.selected += 1;
        }
    }
}

/*
impl<'a> App<'a> {
    fn new(dir: &'a mut str)
}
*/


fn main() -> Result<(), failure::Error> {
    // Initialize Terminal
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();
        loop {
            let current_dir = env::current_dir()?;
            let paths: Vec <_> = fs::read_dir(current_dir).unwrap().map(|res| res.unwrap().file_name().into_string().unwrap()).collect();
            terminal.draw(|mut f| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints(
                        [
                        Constraint::Percentage(100),
                        ].as_ref()
                        )
                    .split(f.size());
                let style = Style::default().fg(Color::Black).bg(Color::White);

                SelectableList::default()
                    .block(Block::default().borders(Borders::ALL).title("rfm"))
                    .items(&paths)
                    .style(style)
                    .highlight_style(style.fg(Color::LightGreen).modifier(Modifier::BOLD))
                    .render(&mut f, chunks[0]);
            })?;

            match events.next()? {
                Event::Input(input) => match input {
                    Key::Char('q') => {
                        break;
                    }
                    _ => {}
                }   
            }
        } 
    Ok(())
}
