use rodio::{Decoder, OutputStream, Source, OutputStreamHandle};
use crate::pads::sample::Sample;
use std::io::Cursor;

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
        let cloned_raw = sample.raw_audio.clone();
        let cursor = Cursor::new(cloned_raw);
        let source = Decoder::new(cursor).unwrap();
        let _ = &self.output_handle.play_raw(source.convert_samples());
    }
}
