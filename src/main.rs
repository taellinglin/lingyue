extern crate rtaudio;
mod engine;


use engine::audio::AudioEngine;

fn main() {
    let sample_rate = 192000;
    let buffer_length = 2048;

    let audio_engine = AudioEngine::initialize_audio(sample_rate, buffer_length);

    if let Some(error) = audio_engine.check_for_errors() {
        println!("An error occurred: {}", error);
    } else {
        println!("Audio engine started. Press Ctrl+C to exit.");
        std::thread::sleep(std::time::Duration::from_secs(u64::max_value()));
    }

}
