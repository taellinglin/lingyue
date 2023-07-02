extern crate rtaudio;
mod engine;


use engine::audio::AudioEngine;

fn main() {
    let sample_rate = 192000;
    let buffer_length = 2048;

    let audio_engine = AudioEngine::initialize_audio(sample_rate, buffer_length);

    // ... Rest of your program logic ...

    // Check for errors
    if let Some(error) = audio_engine.check_for_errors() {
        eprintln!("Audio error: {}", error);
    }
}
