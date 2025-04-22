mod app;
mod ui;
mod tools;

use std::{io, time::Duration};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::app::{App, AppState, MenuItem};

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();
    
    // Main loop
    let tick_rate = Duration::from_millis(100);
    let res = run_app(&mut terminal, &mut app, tick_rate);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    tick_rate: Duration,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                match app.state {
                    AppState::MainMenu => {
                        match key.code {
                            KeyCode::Char('q') => {
                                return Ok(());
                            }
                            KeyCode::Down => {
                                app.next();
                            }
                            KeyCode::Up => {
                                app.previous();
                            }
                            KeyCode::Enter => {
                                match app.selected_item {
                                    MenuItem::MusicPlayer => app.state = AppState::MusicPlayer,
                                    MenuItem::VideoPlayer => app.state = AppState::VideoPlayer,
                                    MenuItem::FileTools => app.state = AppState::FileTools,
                                    MenuItem::ComingSoon => app.state = AppState::ComingSoon,
                                    MenuItem::Quit => return Ok(()),
                                }
                            }
                            _ => {}
                        }
                    }
                    AppState::MusicPlayer => {
                        if key.code == KeyCode::Esc {
                            app.state = AppState::MainMenu;
                        } else {
                            app.handle_music_player_input(key);
                        }
                    }
                    AppState::VideoPlayer => {
                        if key.code == KeyCode::Esc {
                            app.state = AppState::MainMenu;
                        } else {
                            app.handle_video_player_input(key);
                        }
                    }
                    AppState::FileTools => {
                        if key.code == KeyCode::Esc {
                            app.state = AppState::MainMenu;
                        } else {
                            app.handle_file_tools_input(key);
                        }
                    }
                    AppState::ComingSoon => {
                        if key.code == KeyCode::Esc {
                            app.state = AppState::MainMenu;
                        }
                    }
                }
            }
        }
    }
}
