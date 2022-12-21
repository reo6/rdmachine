use rodio::{OutputStream, OutputStreamHandle};
use crate::pads::sample::Sample;

#[allow(dead_code)]
pub struct SamplePlayer {
    output_handle: OutputStreamHandle,
    stream: OutputStream,
}

impl SamplePlayer {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Self { output_handle: stream_handle, stream: _stream }
    }

    pub fn play_sample(&self, sample: &Sample) {
        let _ = &self.output_handle.play_raw(sample.raw_audio.clone());
    }
}
