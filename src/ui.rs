use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap, Gauge},
    Frame,
};

use crate::app::{App, AppState, MusicPlayerMode};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    match app.state {
        AppState::MainMenu => draw_main_menu(f, app),
        AppState::MusicPlayer => draw_music_player(f, app),
        AppState::VideoPlayer => draw_feature_screen(f, "🎥 Video Player", Color::Magenta, get_video_features()),
        AppState::FileTools => draw_feature_screen(f, "📁 File Tools", Color::Blue, get_file_tools_features()),
        AppState::ComingSoon => draw_feature_screen(f, "🧪 Coming Soon", Color::Yellow, get_coming_soon_features()),
    }
}

// === Shared Layout Helper ===
fn get_layout(area: tui::layout::Rect) -> Vec<tui::layout::Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area)
}

// === Main Menu ===
fn draw_main_menu<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(6),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.size());

    // ASCII title - Fixed version
    let ascii_lines = vec![
        "    _______  ______   _____    __  __   _____   _   _   _    _    _____    ",
        "   |__   __||  ____| |  __ \\  |  \\/  | |_   _| | \\ | | | |  | |  / ____|   ",
        "      | |   | |__    | |__) | | \\  / |   | |   |  \\| | | |  | | | (___     ",
        "      | |   |  __|   |  _  /  | |\\/| |   | |   | . ` | | |  | |  \\___ \\    ",
        "      | |   | |____  | | \\ \\  | |  | |  _| |_  | |\\  | | |__| |  ____) |   ",
        "      |_|   |______| |_|  \\_\\ |_|  |_| |_____| |_| \\_|  \\____/  |_____/    ",
    ];

    let title_spans: Vec<Spans> = ascii_lines.into_iter().map(|line| {
        Spans::from(vec![
            Span::styled(
                line,  // Use the actual ASCII art line here
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            )
        ])
    }).collect();

    let title = Paragraph::new(title_spans)
        .alignment(Alignment::Center);

    f.render_widget(title, chunks[0]);

    // Menu items
    let items: Vec<ListItem> = app.menu_items.iter().map(|&item| {
        let title = app.get_menu_title(item);
        let desc = app.get_menu_description(item);

        let is_selected = item == app.selected_item;
        let title_style = if is_selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let desc_style = Style::default().fg(if is_selected { Color::Gray } else { Color::DarkGray });

        ListItem::new(vec![
            Spans::from(vec![
                Span::raw(if is_selected { "▶ " } else { "  " }),
                Span::styled(title, title_style),
            ]),
            Spans::from(vec![
                Span::raw("  "),
                Span::styled(desc, desc_style),
            ]),
        ])
    }).collect();

    let menu = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(Spans::from(vec![
            Span::styled(" Menu ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]))

        );

    f.render_widget(menu, chunks[1]);

    // Status bar
    let status = Paragraph::new(" Navigate with ↑ ↓  |  Press [Enter] to select  |  Press [q] to quit ")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));

    f.render_widget(status, chunks[2]);
}

// === Music Player Implementation ===
fn draw_music_player<B: Backend>(f: &mut Frame<B>, app: &App) {
    match app.mp_mode {
        MusicPlayerMode::Browse => draw_browse_mode(f, app),
        MusicPlayerMode::NowPlaying => draw_now_playing_mode(f, app),
        MusicPlayerMode::AddSong => draw_add_song_mode(f, app),
        MusicPlayerMode::EditSong => draw_edit_song_mode(f, app),
        MusicPlayerMode::DeleteConfirm => draw_delete_confirm_mode(f, app),
    }
}

fn draw_browse_mode<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(10),    // Playlist
            Constraint::Length(5),  // Controls
        ])
        .split(f.size());

    // Title
    let title = Paragraph::new("🎵 Music Player")
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Green)));

    f.render_widget(title, chunks[0]);

    // Playlist
    if app.playlist.is_empty() {
        let empty_msg = Paragraph::new("Your playlist is empty. Press 'a' to add a new song.")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .title(" Playlist "));
        
        f.render_widget(empty_msg, chunks[1]);
    } else {
        let items: Vec<ListItem> = app.playlist.iter().enumerate().map(|(i, song)| {
        let is_selected = i == app.selected_song_index;
        let prefix = if is_selected { "▶ " } else { "  " };
        let artist_text = format!("by {}", song.artist); // Simpan dalam variabel
        
        ListItem::new(vec![
            Spans::from(vec![
                Span::raw(prefix),
                Span::styled(
                    &song.title,
                    Style::default()
                        .fg(if is_selected { Color::Yellow } else { Color::White })
                        .add_modifier(if is_selected { Modifier::BOLD } else { Modifier::empty() })
                ),
            ]),
            Spans::from(vec![
                Span::raw("    "),
                Span::styled(
                    artist_text, // Gunakan variabel yang sudah dibuat
                    Style::default().fg(if is_selected { Color::Gray } else { Color::DarkGray })
                ),
            ]),
        ])
    }).collect();

        let playlist = List::new(items)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .title(" Playlist "));
        
        f.render_widget(playlist, chunks[1]);
    }

    // Controls
    let controls = vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![
            Span::styled(" [↑/↓]", Style::default().fg(Color::Yellow)),
            Span::raw(" Navigate   "),
            Span::styled(" [Enter]", Style::default().fg(Color::Yellow)),
            Span::raw(" Play   "),
            Span::styled(" [a]", Style::default().fg(Color::Yellow)),
            Span::raw(" Add song   "),
        ]),
        Spans::from(vec![
            Span::styled(" [e]", Style::default().fg(Color::Yellow)),
            Span::raw(" Edit song   "),
            Span::styled(" [d]", Style::default().fg(Color::Yellow)),
            Span::raw(" Delete song   "),
            Span::styled(" [Esc]", Style::default().fg(Color::Yellow)),
            Span::raw(" Back   "),
        ]),
    ];

    let controls_widget = Paragraph::new(controls)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title(" Controls "))
        .alignment(Alignment::Center);
    
    f.render_widget(controls_widget, chunks[2]);
}

fn draw_now_playing_mode<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(7),  // Now Playing
            Constraint::Min(6),     // Visualization (placeholder)
            Constraint::Length(5),  // Controls
        ])
        .split(f.size());

    // Title
    let title = Paragraph::new("🎵 Music Player - Now Playing")
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Green)));

    f.render_widget(title, chunks[0]);

    // Now Playing Information
    let song_info = if let Some(song) = app.playlist.get(app.selected_song_index) {
        vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![
                Span::styled("Title: ", Style::default().fg(Color::Gray)),
                Span::styled(&song.title, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Spans::from(vec![
                Span::styled("Artist: ", Style::default().fg(Color::Gray)),
                Span::styled(&song.artist, Style::default().fg(Color::White)),
            ]),
            Spans::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    if app.music_playing { "Playing" } else { "Paused" },
                    Style::default().fg(if app.music_playing { Color::Green } else { Color::Yellow })
                ),
            ]),
        ]
    } else {
        vec![Spans::from(vec![Span::raw("No song selected")])]
    };

    let info_widget = Paragraph::new(song_info)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title(" Now Playing "))
        .alignment(Alignment::Center);
    
    f.render_widget(info_widget, chunks[1]);

    // Volume Bar
    let volume_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(2),
        ])
        .split(chunks[2]);

    let volume_label = Paragraph::new("Volume")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    
    f.render_widget(volume_label, volume_chunks[0]);

    let volume_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green))
        .ratio(app.volume as f64 / 100.0)
        .label(format!("{}%", app.volume));
    
    f.render_widget(volume_gauge, volume_chunks[1]);

    // Visual placeholder (would be replaced with actual visualization)
    let visual_placeholder = Paragraph::new("Audio Visualization would appear here")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green)));
    
    f.render_widget(visual_placeholder, volume_chunks[2]);

    // Controls
    let controls = vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![
            Span::styled(" [Space]", Style::default().fg(Color::Yellow)),
            Span::raw(" Play/Pause   "),
            Span::styled(" [n]", Style::default().fg(Color::Yellow)),
            Span::raw(" Next   "),
            Span::styled(" [p]", Style::default().fg(Color::Yellow)),
            Span::raw(" Previous   "),
        ]),
        Spans::from(vec![
            Span::styled(" [+/-]", Style::default().fg(Color::Yellow)),
            Span::raw(" Volume   "),
            Span::styled(" [b]", Style::default().fg(Color::Yellow)),
            Span::raw(" Back to playlist   "),
            Span::styled(" [Esc]", Style::default().fg(Color::Yellow)),
            Span::raw(" Stop   "),
        ]),
    ];

    let controls_widget = Paragraph::new(controls)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title(" Controls "))
        .alignment(Alignment::Center);
    
    f.render_widget(controls_widget, chunks[3]);
}

fn draw_add_song_mode<B: Backend>(f: &mut Frame<B>, app: &App) {
    draw_song_form(f, app, "Add New Song", false);
}

fn draw_edit_song_mode<B: Backend>(f: &mut Frame<B>, app: &App) {
    draw_song_form(f, app, "Edit Song", true);
}

fn draw_song_form<B: Backend>(f: &mut Frame<B>, app: &App, title: &str, is_edit: bool) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),     // Title
            Constraint::Length(3),     // Field 1: Title
            Constraint::Length(3),     // Field 2: Artist
            Constraint::Length(3),     // Field 3: URL
            Constraint::Min(5),        // Spacer
            Constraint::Length(3),     // Controls
        ])
        .split(f.size());

    // Title
    let header = Paragraph::new(format!("🎵 Music Player - {}", title))
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Green)));

    f.render_widget(header, chunks[0]);

    // Form Fields
    let field_titles = ["Title", "Artist", "URL"];
    let field_values = [&app.form_title, &app.form_artist, &app.form_url];
    
    for i in 0..3 {
        let is_active = app.form_field_index == i;
        let field_style = Style::default()
            .fg(if is_active { Color::Yellow } else { Color::White })
            .add_modifier(if is_active { Modifier::BOLD } else { Modifier::empty() });
        
        let field = Paragraph::new(field_values[i].clone())
            .style(field_style)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(if is_active { Color::Yellow } else { Color::Green }))
                .title(format!(" {} {} ", 
                    if is_active { ">" } else { " " },
                    field_titles[i]
                )));
        
        f.render_widget(field, chunks[i + 1]);
    }

    // Controls
    let action_text = if is_edit { "Update" } else { "Add" };
    let controls = vec![
        Spans::from(vec![
            Span::styled(" [Tab]", Style::default().fg(Color::Yellow)),
            Span::raw(" Next field   "),
            Span::styled(" [Enter]", Style::default().fg(Color::Yellow)),
            Span::raw(format!(" {} song   ", action_text)),
            Span::styled(" [Esc]", Style::default().fg(Color::Yellow)),
            Span::raw(" Cancel   "),
        ]),
    ];

    let controls_widget = Paragraph::new(controls)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title(" Controls "))
        .alignment(Alignment::Center);
    
    f.render_widget(controls_widget, chunks[5]);
}

fn draw_delete_confirm_mode<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),     // Title
            Constraint::Length(5),     // Warning
            Constraint::Min(5),        // Song info
            Constraint::Length(3),     // Controls
        ])
        .split(f.size());

    // Title
    let title = Paragraph::new("🎵 Music Player - Delete Song")
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Red)));

    f.render_widget(title, chunks[0]);

    // Warning
    let warning = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![
            Span::styled("⚠ WARNING: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled("You are about to delete this song from your playlist.", 
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Spans::from(vec![Span::raw("This action cannot be undone.")]),
    ])
    .block(Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red)))
    .alignment(Alignment::Center);

    f.render_widget(warning, chunks[1]);

    // Song info
    if let Some(song) = app.playlist.get(app.selected_song_index) {
        let song_info = vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![
                Span::styled("Title: ", Style::default().fg(Color::Gray)),
                Span::styled(&song.title, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Spans::from(vec![
                Span::styled("Artist: ", Style::default().fg(Color::Gray)),
                Span::styled(&song.artist, Style::default().fg(Color::White)),
            ]),
        ];

        let info_widget = Paragraph::new(song_info)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
                .title(" Song to Delete "))
            .alignment(Alignment::Center);
        
        f.render_widget(info_widget, chunks[2]);
    }

    // Controls
    let controls = vec![
        Spans::from(vec![
            Span::styled(" [y]", Style::default().fg(Color::Yellow)),
            Span::raw(" Yes, delete   "),
            Span::styled(" [n]", Style::default().fg(Color::Yellow)),
            Span::raw(" No, cancel   "),
            Span::styled(" [Esc]", Style::default().fg(Color::Yellow)),
            Span::raw(" Cancel   "),
        ]),
    ];

    let controls_widget = Paragraph::new(controls)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red))
            .title(" Confirm Delete "))
        .alignment(Alignment::Center);
    
    f.render_widget(controls_widget, chunks[3]);
}

// === Feature Screens ===
fn draw_feature_screen<B: Backend>(f: &mut Frame<B>, title: &str, color: Color, features: Vec<&str>) {
    let chunks = get_layout(f.size());

    let title = Paragraph::new(title)
        .style(Style::default().fg(color).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(color)));

    f.render_widget(title, chunks[0]);

    let content = Paragraph::new(
        features
            .into_iter()
            .map(|line| Spans::from(Span::raw(line)))
            .collect::<Vec<_>>()
    )
    .block(Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
        .title(" Features "))
    .wrap(Wrap { trim: true });

    f.render_widget(content, chunks[1]);

    let status = Paragraph::new(" Press [Esc] to return to the main menu ")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(color)));

    f.render_widget(status, chunks[2]);
}

// === Feature Content Data ===
fn get_video_features() -> Vec<&'static str> {
    vec![
        "Coming soon: Video Player Implementation", "",
        "• Terminal video playback via mpv",
        "• Terminal ASCII art video rendering",
        "• Subtitles support",
        "• Video library management",
    ]
}

fn get_file_tools_features() -> Vec<&'static str> {
    vec![
        "Coming soon: File Tools Implementation", "",
        "• Bulk file renaming",
        "• File organization by type",
        "• Advanced search",
        "• Compression/decompression",
    ]
}

fn get_coming_soon_features() -> Vec<&'static str> {
    vec![
        "Future tools that will be added to TERMINUS:", "",
        "• Markdown viewer and editor",
        "• API tester for developers",
        "• YouTube downloader",
        "• Terminal-based chat client",
        "• System monitor dashboard",
        "• Notes and todo manager", "",
        "Have suggestions? Add them to the roadmap!",
    ]
}
