extern crate rtaudio;
extern crate egui;
extern crate egui_tiles;
extern crate eframe;
mod engine;
mod ui;
use ui::main_panel::*;
use engine::audio::AudioEngine;

fn main() -> Result<(), eframe::Error> {
    let sample_rate = 192000;
    let buffer_length = 2048;

    let audio_engine = AudioEngine::initialize_audio(sample_rate, buffer_length);
    if let Err(err) = ui::main_panel::main_panel() {
        eprintln!("Application error: {}", err);
    }
    
    // Check for errors
    if let Some(error) = audio_engine.check_for_errors() {
        eprintln!("Audio error: {}", error);
    }

    Ok(())
}
