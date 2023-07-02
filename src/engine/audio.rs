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

        let mut phasor: f32 = 0.0;
        let mut phasor_inc = 2.0 * PI * 220.0 / sample_rate as f32;
        let mut frequency = 220.0;
        let mut target_frequency = 880.0;
        let mut frequency_step = (target_frequency - frequency) / (sample_rate as f32 * 2.0);
        let mut direction = 1;

        let _stream_mutex_clone = stream_mutex.clone();

        stream_mutex.lock().unwrap().start(
            move |buffers: Buffers<'_>, _info: &StreamInfo, _status: StreamStatus| {
                if let Buffers::Float32 { output, input: _ } = buffers {
                    let frames = output.len() / 2;

                    for i in 0..frames {
                        let val = (phasor).sin() * 0.5;

                        // By default, buffers are interleaved.
                        output[i * 2] = val;
                        output[(i * 2) + 1] = val;

                        phasor += phasor_inc;

                        frequency += frequency_step;
                        if frequency >= target_frequency {
                            target_frequency = 220.0;
                            direction = -1;
                            frequency_step = (target_frequency - frequency) / (sample_rate as f32 * 2.0);
                        } else if frequency <= 220.0 {
                            target_frequency = 880.0;
                            direction = 1;
                            frequency_step = (target_frequency - frequency) / (sample_rate as f32 * 2.0);
                        }
                        phasor_inc = 2.0 * PI * frequency / sample_rate as f32;
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
