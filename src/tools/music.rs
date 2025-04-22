// src/tools/music.rs
pub struct MusicPlayer {
    pub current_playlist: Vec<String>,
    pub current_index: usize,
    pub is_playing: bool,
    pub volume: u8,
}

impl MusicPlayer {
    pub fn new() -> Self {
        Self {
            current_playlist: Vec::new(),
            current_index: 0,
            is_playing: false,
            volume: 80,
        }
    }
    
    pub fn play(&mut self) {
        self.is_playing = true;
        // Will implement actual playback using rodio
    }
    
    pub fn pause(&mut self) {
        self.is_playing = false;
    }
    
    pub fn next(&mut self) {
        if !self.current_playlist.is_empty() {
            self.current_index = (self.current_index + 1) % self.current_playlist.len();
        }
    }
    
    pub fn prev(&mut self) {
        if !self.current_playlist.is_empty() {
            if self.current_index > 0 {
                self.current_index -= 1;
            } else {
                self.current_index = self.current_playlist.len() - 1;
            }
        }
    }
    
    pub fn set_volume(&mut self, volume: u8) {
        self.volume = volume.min(100);
    }
}
