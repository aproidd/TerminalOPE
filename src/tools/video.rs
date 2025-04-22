// src/tools/video.rs
pub struct VideoPlayer {
    pub current_playlist: Vec<String>,
    pub current_index: usize,
    pub is_playing: bool,
    pub volume: u8,
    pub fullscreen: bool,
}

impl VideoPlayer {
    pub fn new() -> Self {
        Self {
            current_playlist: Vec::new(),
            current_index: 0,
            is_playing: false,
            volume: 80,
            fullscreen: false,
        }
    }
    
    pub fn play(&mut self) {
        self.is_playing = true;
        // Will implement actual playback using mpv command
    }
    
    pub fn pause(&mut self) {
        self.is_playing = false;
    }
    
    pub fn toggle_fullscreen(&mut self) {
        self.fullscreen = !self.fullscreen;
    }
}
