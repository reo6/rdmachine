use rodio::{Decoder, OutputStream, Source, OutputStreamHandle};
use std::io::BufReader;
use std::fs::File;

pub struct SamplePlayer {
    output_handle: OutputStreamHandle,
    stream: OutputStream,
}

impl SamplePlayer {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Self { output_handle: stream_handle, stream: _stream }
    }

    pub fn play_sample(&self, file: File) {
        let decoder = Decoder::new(BufReader::new(file)).unwrap();
        &self.output_handle.play_raw(decoder.convert_samples());
    }
}
