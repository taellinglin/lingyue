use rtaudio::{
    Api, Buffers, DeviceParams, SampleFormat, StreamInfo, StreamOptions, StreamStatus, RtAudioError, Stream
};
use std::sync::mpsc::{Receiver, channel};
use std::sync::{Arc, Mutex};
use std::f32::consts::PI;
use std::time::{Instant, Duration};

pub struct AudioEngine {
    error_rx: Receiver<RtAudioError>,
    stream_mutex: Arc<Mutex<Stream>>,
}

impl AudioEngine {
    pub fn initialize_audio(sample_rate: u32, buffer_length: u32) -> Self {
        let host = rtaudio::Host::new(Api::Unspecified).unwrap();
        let out_device = host.default_output_device().unwrap();
        let out_device_info = host.get_device_info_by_id(out_device.id).unwrap();

        let (error_tx, error_rx) = channel();
        let stream_mutex = Arc::new(Mutex::new(
            host.open_stream(
                Some(DeviceParams {
                    device_id: out_device.id,
                    num_channels: 2, // Stereo output
                    first_channel: 0,
                }),
                None,
                SampleFormat::Float32,
                sample_rate,
                buffer_length,
                StreamOptions::default(),
                move |error| error_tx.send(error).unwrap(),
            )
            .unwrap(),
        ));

        let brainwave_states = vec![
            40.0,  // Gamma
            30.0,  // Beta
            14.0,  // Alpha
            8.0,   // Theta
            2.0,   // Delta
        ];

        let mut current_state_index = 0;
        let mut current_frequency = brainwave_states[current_state_index];
        let mut next_frequency = brainwave_states[(current_state_index + 1) % brainwave_states.len()];

        let mut phasor_a: f32 = 0.0;
        let mut phasor_b: f32 = 0.5; // Phase offset of 0.5
        let mut phasor_inc_a = 0.0; // Will be updated dynamically
        let mut phasor_inc_b = 0.0; // Will be updated dynamically

        let mut start_time = Instant::now();
        let state_duration = Duration::from_secs(30); // Duration for each brainwave state

        let _stream_mutex_clone = stream_mutex.clone();

        stream_mutex.lock().unwrap().start(
            move |buffers: Buffers<'_>, _info: &StreamInfo, _status: StreamStatus| {
                if let Buffers::Float32 { output, input: _ } = buffers {
                    let frames = output.len() / 2;

                    for i in 0..frames {
                        let elapsed_time = start_time.elapsed();
                        if elapsed_time >= state_duration {
                            // Switch to the next brainwave state
                            current_state_index = (current_state_index + 1) % brainwave_states.len();
                            current_frequency = brainwave_states[current_state_index];
                            next_frequency = brainwave_states[(current_state_index + 1) % brainwave_states.len()];
                            start_time = Instant::now();
                        }

                        // Smooth transition between current and next frequency
                        let transition_ratio = elapsed_time.as_secs_f32() / state_duration.as_secs_f32();
                        let frequency = (1.0 - transition_ratio) * current_frequency + transition_ratio * next_frequency;
                        phasor_inc_a = 2.0 * PI * (110.0 + frequency) / sample_rate as f32;
                        phasor_inc_b = 2.0 * PI * (110.0 + frequency) / sample_rate as f32;

                        let left_val = phasor_a.sin();
                        let right_val = phasor_b.sin();

                        // By default, buffers are interleaved.
                        output[i * 2] = left_val;
                        output[(i * 2) + 1] = right_val;

                        phasor_a += phasor_inc_a;
                        phasor_b += phasor_inc_b;

                        if phasor_a > 2.0 * PI {
                            phasor_a -= 2.0 * PI;
                        }
                        if phasor_b > 2.0 * PI {
                            phasor_b -= 2.0 * PI;
                        }
                    }
                }
            },
        ).unwrap();

        AudioEngine {
            error_rx,
            stream_mutex,
        }
    }

    pub fn check_for_errors(&self) -> Option<RtAudioError> {
        self.error_rx.recv_timeout(std::time::Duration::from_millis(5000)).ok()
    }
}