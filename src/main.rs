#[allow(dead_code)]
mod event;

use std::io;
use std::{fs,env};
use std::path;
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
    selected: Option<usize>,
}
impl<T> ListState<T> {
    fn new(items: Vec<T>) -> ListState<T> {
        ListState { items, selected: Some(0)}
    }

    fn select_previous(&mut self) {
        if self.selected == None {
            self.selected = Some(0);
        }
        if self.selected.unwrap() > 0 {
            self.selected = Some(self.selected.unwrap() - 1);
        }
    }

    fn select_next(&mut self) {
        if self.selected == None {
            self.selected = Some(0);
        }
        if self.selected.unwrap() < self.items.len() - 1  {
            self.selected = Some(self.selected.unwrap() + 1);
        }
    }
}

// add permissions, current user, showHidden, etc. later
struct App<'a> {
    dir: &'a mut std::path::PathBuf,
    contents: ListState<String>,
}

impl<'a> App<'a> {
    fn new(dir: &'a mut path::PathBuf) -> App<'a> {
        let paths: Vec <_> = fs::read_dir(&dir).unwrap().map(|res| res.unwrap().file_name().into_string().unwrap()).collect();
        let ls = ListState::new(paths);
        App { dir: dir, contents: ls }
    }

    fn previous_dir(&mut self) {
        self.dir.pop();
        let paths: Vec <_> = fs::read_dir(&self.dir).unwrap().map(|res| res.unwrap().file_name().into_string().unwrap()).collect();
        self.contents.items = paths;
    }

    fn next_dir(&mut self, path: &path::PathBuf) {
        self.dir.push(path);
        let paths: Vec <_> = fs::read_dir(&self.dir).unwrap().map(|res| res.unwrap().file_name().into_string().unwrap()).collect();
        self.contents.items = paths;

    }
}


fn main() -> Result<(), failure::Error> {
    // Initialize Terminal
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();
            let mut dir = env::current_dir()?;
            let mut app = App::new(&mut dir);
        loop {
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
                let style = Style::default();

                SelectableList::default()
                    .block(Block::default().borders(Borders::ALL).title(&app.dir.to_str().unwrap()))
                    .select(app.contents.selected)
                    .items(&app.contents.items)
                    .style(style)
                    .highlight_style(style.bg(Color::White).fg(Color::Black).modifier(Modifier::BOLD))
                    .render(&mut f, chunks[0]);
            })?;

            match events.next()? {
                Event::Input(input) => match input {
                    Key::Char('q') => {
                        break;
                    }
                    Key::Char('j') => {
                        app.contents.select_next();
                    }
                    Key::Char('k') => {
                        app.contents.select_previous();
                        if app.contents.selected.unwrap() >  app.contents.items.len() {
                            app.contents.selected = Some(app.contents.items.len() - 1);
                        }
                    }
                    Key::Char('l') => {
                        match app.contents.selected {
                            Some(ref p) => {
                                if p >= &app.contents.items.len() { continue; }
                                let mut full_path: path::PathBuf = app.dir.clone();
                                full_path.push(&app.contents.items[app.contents.selected.unwrap()]);
                                let dir_metadata = path::PathBuf::from(&full_path).metadata()?;
                                
                                if dir_metadata.is_dir() {
                                    app.next_dir(&full_path);
                                }
                                if app.contents.selected.unwrap() >  app.contents.items.len() {
                                    app.contents.selected = Some(app.contents.items.len() - 1);
                                }
                            }
                            None => {}
                            
                        }
                    }
                    Key::Char('h') => {
                        app.previous_dir();
                    }
                    _ => {}
                }
            }
        } 
    Ok(())
}
