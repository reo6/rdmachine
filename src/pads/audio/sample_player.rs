use rodio::{Decoder, OutputStream, Source, OutputStreamHandle};
use std::io::BufReader;
use std::fs::File;
use crate::pads::audio::sample::Sample;

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

// pub fn play_sample(filename: String) {
//     // Do nothing if pad has no audio file inside.

//     let (_stream, stream_handle) = OutputStream::try_default().unwrap();

//     let file = BufReader::new(File::open(filename).unwrap());
//     let source = Decoder::new(file).unwrap();

//     stream_handle.play_raw(source.convert_samples()).expect("Failed to play samples.");
// }

pub fn load_test_samples(test_sample: &str) -> Sample {
    let file = File::open(test_sample).unwrap();
    Sample::new(file)
}
