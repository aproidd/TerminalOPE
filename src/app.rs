use std::{
    fs,
    process::{Child, Command},
    path::PathBuf,
};
use dirs;
use serde::{Deserialize, Serialize};
use crossterm::event::{KeyEvent, KeyCode};

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MusicPlayerMode {
    Browse,
    NowPlaying,
    AddSong,
    EditSong,
    DeleteConfirm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub title:  String,
    pub artist: String,
    pub url:    String,
    pub path:   Option<String>,
}

pub struct App {
    pub state: AppState,
    pub menu_items: Vec<MenuItem>,
    pub selected_item: MenuItem,
    pub index: usize,
    pub music_playing: bool,
    pub current_song: Option<String>,
    pub mp_mode: MusicPlayerMode,
    pub playlist: Vec<Song>,
    pub selected_song_index: usize,
    pub volume: u8,
    pub player_process: Option<std::process::Child>,
    pub form_title: String,
    pub form_artist: String,
    pub form_url: String,
    pub form_field_index: usize,
    pub video_playing: bool,
    pub current_video: Option<String>,
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

        let playlist = Self::load_playlist().unwrap_or_else(|| {
            vec![
                Song {
                    title: "Indonesian Folk Music Medley 2024 Ver. - hololive ID [Cover]".into(),
                    artist: "hololive ID".into(),
                    url: "https://youtu.be/rjhIMMSolmc?feature=shared".into(),
                    path: None,
                },
                Song {
                    title: "Terhebat - hololive ID [Cover]".into(),
                    artist: "hololive ID".into(),
                    url: "https://youtu.be/PaOMF-g1ZWU?feature=shared".into(),
                    path: None,
                },
                Song {
                    title: "Bebas - hololive ID [Cover]".into(),
                    artist: "hololive ID".into(),
                    url: "https://youtu.be/wlyRGXUwjVA?feature=shared".into(),
                    path: None,
                },
            ]
        });

        Self {
            state: AppState::MainMenu,
            menu_items,
            selected_item: MenuItem::MusicPlayer,
            index: 0,
            music_playing: false,
            current_song: None,
            mp_mode: MusicPlayerMode::Browse,
            playlist,
            selected_song_index: 0,
            volume: 80,
            player_process: None,
            form_title: String::new(),
            form_artist: String::new(),
            form_url: String::new(),
            form_field_index: 0,
            video_playing: false,
            current_video: None,
            current_directory: "~/".into(),
        }
    }

    pub fn load_playlist() -> Option<Vec<Song>> {
        let config_dir = dirs::config_dir()?;
        let playlist_path = config_dir.join("terminus").join("playlist.json");
        
        if playlist_path.exists() {
            let content = fs::read_to_string(playlist_path).ok()?;
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }
    
    pub fn save_playlist(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir().ok_or("Config directory not found")?;
        let terminus_dir = config_dir.join("terminus");
        
        if !terminus_dir.exists() {
            fs::create_dir_all(&terminus_dir)?;
        }
        
        let playlist_path = terminus_dir.join("playlist.json");
        let playlist_json = serde_json::to_string_pretty(&self.playlist)?;
        
        fs::write(playlist_path, playlist_json)?;
        Ok(())
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
    
    // Implementasi music player
    pub fn handle_music_player_input(&mut self, key: KeyEvent) {
        match self.mp_mode {
            MusicPlayerMode::Browse => self.handle_browse_mode(key),
            MusicPlayerMode::NowPlaying => self.handle_now_playing_mode(key),
            MusicPlayerMode::AddSong => self.handle_add_song_mode(key),
            MusicPlayerMode::EditSong => self.handle_edit_song_mode(key),
            MusicPlayerMode::DeleteConfirm => self.handle_delete_confirm_mode(key),
        }
    }
    
    fn handle_browse_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Down => {
                if !self.playlist.is_empty() {
                    self.selected_song_index = (self.selected_song_index + 1) % self.playlist.len();
                }
            },
            KeyCode::Up => {
                if !self.playlist.is_empty() {
                    if self.selected_song_index > 0 {
                        self.selected_song_index -= 1;
                    } else {
                        self.selected_song_index = self.playlist.len() - 1;
                    }
                }
            },
            KeyCode::Enter => {
                if !self.playlist.is_empty() {
                    self.play_selected_song();
                    self.mp_mode = MusicPlayerMode::NowPlaying;
                }
            },
            KeyCode::Char('a') => {
                // Reset form sebelum menambah lagu baru
                self.form_title = String::new();
                self.form_artist = String::new();
                self.form_url = String::new();
                self.form_field_index = 0;
                self.mp_mode = MusicPlayerMode::AddSong;
            },
            KeyCode::Char('e') => {
                if !self.playlist.is_empty() {
                    if let Some(song) = self.playlist.get(self.selected_song_index) {
                        // Pra-isi form dengan data lagu yang akan diedit
                        self.form_title = song.title.clone();
                        self.form_artist = song.artist.clone();
                        self.form_url = song.url.clone();
                        self.form_field_index = 0;
                        self.mp_mode = MusicPlayerMode::EditSong;
                    }
                }
            },
            KeyCode::Char('d') => {
                if !self.playlist.is_empty() {
                    self.mp_mode = MusicPlayerMode::DeleteConfirm;
                }
            },
            _ => {}
        }
    }
    
    fn handle_now_playing_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(' ') => {
                self.toggle_playback();
            },
            KeyCode::Char('n') => {
                self.next_song();
            },
            KeyCode::Char('p') => {
                self.prev_song();
            },
            KeyCode::Char('+') => {
                self.volume = (self.volume + 5).min(100);
            },
            KeyCode::Char('-') => {
                self.volume = self.volume.saturating_sub(5);
            },
            KeyCode::Char('b') => {
                // Kembali ke mode browse
                self.mp_mode = MusicPlayerMode::Browse;
            },
            KeyCode::Esc => {
                // Stop playback when exiting
                self.stop_playback();
                self.mp_mode = MusicPlayerMode::Browse;
            },
            _ => {}
        }
    }
    
    fn handle_add_song_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.mp_mode = MusicPlayerMode::Browse;
            },
            KeyCode::Enter => {
                if self.form_field_index < 2 {
                    // Pindah ke field berikutnya
                    self.form_field_index += 1;
                } else {
                    // Tambahkan lagu baru dan kembali ke mode browse
                    let new_song = Song {
                        title: self.form_title.clone(),
                        artist: self.form_artist.clone(),
                        url: self.form_url.clone(),
                        path: None,
                    };
                    
                    self.playlist.push(new_song);
                    let _ = self.save_playlist(); // Simpan playlist ke disk
                    self.mp_mode = MusicPlayerMode::Browse;
                }
            },
            KeyCode::Tab => {
                // Pindah ke field berikutnya dengan Tab
                self.form_field_index = (self.form_field_index + 1) % 3;
            },
            KeyCode::BackTab => {
                // Pindah ke field sebelumnya dengan Shift+Tab
                if self.form_field_index > 0 {
                    self.form_field_index -= 1;
                } else {
                    self.form_field_index = 2;
                }
            },
            KeyCode::Backspace => {
                // Hapus karakter terakhir dari field aktif
                match self.form_field_index {
                    0 => {
                        if !self.form_title.is_empty() {
                            self.form_title.pop();
                        }
                    },
                    1 => {
                        if !self.form_artist.is_empty() {
                            self.form_artist.pop();
                        }
                    },
                    2 => {
                        if !self.form_url.is_empty() {
                            self.form_url.pop();
                        }
                    },
                    _ => {}
                }
            },
            KeyCode::Char(c) => {
                // Tambahkan karakter ke field aktif
                match self.form_field_index {
                    0 => self.form_title.push(c),
                    1 => self.form_artist.push(c),
                    2 => self.form_url.push(c),
                    _ => {}
                }
            },
            _ => {}
        }
    }
    
    fn handle_edit_song_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.mp_mode = MusicPlayerMode::Browse;
            },
            KeyCode::Enter => {
                if self.form_field_index < 2 {
                    // Pindah ke field berikutnya
                    self.form_field_index += 1;
                } else {
                    // Update lagu dan kembali ke mode browse
                    if let Some(song) = self.playlist.get_mut(self.selected_song_index) {
                        song.title = self.form_title.clone();
                        song.artist = self.form_artist.clone();
                        song.url = self.form_url.clone();
                    }
                    
                    let _ = self.save_playlist(); // Simpan playlist ke disk
                    self.mp_mode = MusicPlayerMode::Browse;
                }
            },
            KeyCode::Tab => {
                // Pindah ke field berikutnya dengan Tab
                self.form_field_index = (self.form_field_index + 1) % 3;
            },
            KeyCode::BackTab => {
                // Pindah ke field sebelumnya dengan Shift+Tab
                if self.form_field_index > 0 {
                    self.form_field_index -= 1;
                } else {
                    self.form_field_index = 2;
                }
            },
            KeyCode::Backspace => {
                // Hapus karakter terakhir dari field aktif
                match self.form_field_index {
                    0 => {
                        if !self.form_title.is_empty() {
                            self.form_title.pop();
                        }
                    },
                    1 => {
                        if !self.form_artist.is_empty() {
                            self.form_artist.pop();
                        }
                    },
                    2 => {
                        if !self.form_url.is_empty() {
                            self.form_url.pop();
                        }
                    },
                    _ => {}
                }
            },
            KeyCode::Char(c) => {
                // Tambahkan karakter ke field aktif
                match self.form_field_index {
                    0 => self.form_title.push(c),
                    1 => self.form_artist.push(c),
                    2 => self.form_url.push(c),
                    _ => {}
                }
            },
            _ => {}
        }
    }
    
    fn handle_delete_confirm_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('y') => {
                // Hapus lagu dan kembali ke mode browse
                if !self.playlist.is_empty() {
                    self.playlist.remove(self.selected_song_index);
                    if self.selected_song_index >= self.playlist.len() && !self.playlist.is_empty() {
                        self.selected_song_index = self.playlist.len() - 1;
                    }
                    let _ = self.save_playlist(); // Simpan playlist ke disk
                }
                self.mp_mode = MusicPlayerMode::Browse;
            },
            KeyCode::Char('n') | KeyCode::Esc => {
                // Batal hapus
                self.mp_mode = MusicPlayerMode::Browse;
            },
            _ => {}
        }
    }
    
    // Fungsi player kontrol
     pub fn play_selected_song(&mut self) {
        // Hentikan jika kosong
        if self.playlist.is_empty() {
            return;
        }
        self.stop_playback();

        // Ambil info lagu
        let song = &self.playlist[self.selected_song_index];
        self.current_song = Some(song.title.clone());
        self.music_playing = true;

        // EXPAND TILDE kalau ada
        let source = if let Some(path) = &song.path {
            if let Some(stripped) = path.strip_prefix("~/") {
                let mut pb = dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("/"));
                pb.push(stripped);
                pb.to_string_lossy().into_owned()
            } else {
                path.clone()
            }
        } else {
            song.url.clone()
        };

        // Spawn mpv
        match Command::new("mpv")
             .args(&[
		"--no-video",
		&format!("--volume={}", self.volume),
		"--input-ipc-server=/tmp/mpvsocket",
		&source,
	    ])
	    .spawn()
        {
            Ok(child) => {
                self.player_process = Some(child);
            }
            Err(e) => {
                // Cetak error agar kelihatan di console
                eprintln!(
                    "⚠️ Gagal spawn mpv untuk `{}`: {}",
                    source, e
                );
                self.music_playing = false;
                self.current_song = None;
            }
        };
    }

    
     pub fn toggle_playback(&mut self) {
        self.music_playing = !self.music_playing;
        if self.player_process.is_some() {
            // Ini cuma dummy echo; bisa diganti dengan true IPC command
            let _ = Command::new("echo")
                .args(&["cycle", "pause"])
                .output();
        }
    }
    
    /// Stop dan kill proses mpv
    pub fn stop_playback(&mut self) {
        self.music_playing = false;
        self.current_song = None;
        if let Some(mut proc) = self.player_process.take() {
            let _ = proc.kill();
            let _ = proc.wait();
        }
    }

    /// Next / prev
    pub fn next_song(&mut self) {
        if !self.playlist.is_empty() {
            self.selected_song_index =
                (self.selected_song_index + 1) % self.playlist.len();
            self.play_selected_song();
        }
    }
    pub fn prev_song(&mut self) {
        if !self.playlist.is_empty() {
            if self.selected_song_index == 0 {
                self.selected_song_index = self.playlist.len() - 1;
            } else {
                self.selected_song_index -= 1;
            }
            self.play_selected_song();
        }
    }

    // Implementasi kosong untuk fitur yang akan diimplementasikan nanti
    pub fn handle_video_player_input(&mut self, _key: KeyEvent) {
        // Will be implemented in the video player module
    }
    
    pub fn handle_file_tools_input(&mut self, _key: KeyEvent) {
        // Will be implemented in the file tools module
    }
    
    pub fn get_menu_title(&self, item: MenuItem) -> String {
        match item {
            MenuItem::MusicPlayer => String::from("🎵 Music Player"),
            MenuItem::VideoPlayer => String::from("🎥 Video Player"),
            MenuItem::FileTools => String::from("📁 File Tools"),
            MenuItem::ComingSoon => String::from("🧪 Coming Soon"),
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
