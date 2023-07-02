use rtaudio::{
    Api, Buffers, DeviceParams, SampleFormat, StreamInfo, StreamOptions, StreamStatus, RtAudioError, Stream
};
use std::sync::mpsc::{Receiver, channel};
use std::sync::{Arc, Mutex};
use std::f32::consts::PI;

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
                    num_channels: out_device_info.output_channels as u32,
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

        let mut phasor = 0.0;
        let mut phasor_inc = 440.0 / sample_rate as f32;
        let mut state = 0;
        let mut state_timer = 0.0;
        let state_duration = 10.0;
        let delta_frequency = 2.0;

        let stream_mutex_clone = stream_mutex.clone();

        stream_mutex.lock().unwrap().start(
            move |buffers: Buffers<'_>, _info: &StreamInfo, _status: StreamStatus| {
                if let Buffers::Float32 { output, input: _ } = buffers {
                    let frames = output.len() / 2;

                    for i in 0..frames {
                        let val = (phasor * PI * 2.0).sin() * 0.5;

                        // By default, buffers are interleaved.
                        output[i * 2] = val;
                        output[(i * 2) + 1] = val;

                        phasor = (phasor + phasor_inc).fract();

                        state_timer += 1.0 / sample_rate as f32;
                        if state_timer >= state_duration {
                            state_timer = 0.0;
                            state = (state + 1) % 4;
                            phasor_inc += delta_frequency;
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
