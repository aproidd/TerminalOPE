use crossterm::event::KeyEvent;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppState {
    MainMenu,
    MusicPlayer,
    VideoPlayer,
    FileTools,
    ComingSoon,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuItem {
    MusicPlayer,
    VideoPlayer,
    FileTools,
    ComingSoon,
    Quit,
}

pub struct App {
    pub state: AppState,
    pub menu_items: Vec<MenuItem>,
    pub selected_item: MenuItem,
    pub index: usize,
    
    // Music player state
    pub music_playing: bool,
    pub current_song: Option<String>,
    
    // Video player state
    pub video_playing: bool,
    pub current_video: Option<String>,
    
    // File tools state
    pub current_directory: String,
}

impl App {
    pub fn new() -> Self {
        let menu_items = vec![
            MenuItem::MusicPlayer,
            MenuItem::VideoPlayer,
            MenuItem::FileTools,
            MenuItem::ComingSoon,
            MenuItem::Quit,
        ];
        
        Self {
            state: AppState::MainMenu,
            menu_items,
            selected_item: MenuItem::MusicPlayer,
            index: 0,
            music_playing: false,
            current_song: None,
            video_playing: false,
            current_video: None,
            current_directory: String::from("~/"),
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.menu_items.len();
        self.selected_item = self.menu_items[self.index];
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.menu_items.len() - 1;
        }
        self.selected_item = self.menu_items[self.index];
    }
    
    pub fn handle_music_player_input(&mut self, _key: KeyEvent) {
        // Will be implemented in the music player module
    }
    
    pub fn handle_video_player_input(&mut self, _key: KeyEvent) {
        // Will be implemented in the video player module
    }
    
    pub fn handle_file_tools_input(&mut self, _key: KeyEvent) {
        // Will be implemented in the file tools module
    }
    
    pub fn get_menu_title(&self, item: MenuItem) -> String {
        match item {
            MenuItem::MusicPlayer => String::from("Music Player"),
            //MenuItem::VideoPlayer => String::from("🎥 Video Player"),
            //MenuItem::FileTools => String::from("📁 File Tools"),
            //MenuItem::ComingSoon => String::from("🧪 Coming Soon"),
            MenuItem::Quit => String::from("❌ Quit"),
        }
    }
    
    pub fn get_menu_description(&self, item: MenuItem) -> String {
        match item {
            MenuItem::MusicPlayer => String::from("Listen to curated or local playlists"),
            MenuItem::VideoPlayer => String::from("Watch videos in terminal (mpv-based)"),
            MenuItem::FileTools => String::from("Future toolkit: rename, organize, compress"),
            MenuItem::ComingSoon => String::from("Markdown viewer, API tester, YouTube DL?"),
            MenuItem::Quit => String::from("Exit the application"),
        }
    }
}
