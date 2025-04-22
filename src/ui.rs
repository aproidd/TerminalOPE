use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, AppState};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    match app.state {
        AppState::MainMenu => draw_main_menu(f, app),
        AppState::MusicPlayer => draw_feature_screen(f, "🎵 Music Player", Color::Green, get_music_features()),
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
fn get_music_features() -> Vec<&'static str> {
    vec![
        "Coming soon: Music Player Implementation", "",
        "• Local playlists",
        "• Audio visualization",
        "• Volume control",
        "• Playlist management",
    ]
}

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
