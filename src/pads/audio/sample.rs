use std::fs::File;
use rodio::{Decoder, Source};
use rodio::source::{SamplesConverter, Buffered};

pub struct Sample {
    pub raw_audio: SamplesConverter<Buffered<Decoder<File>>, f32>,
}

impl Sample {
    pub fn new(file: File) -> Self {
        let source = Decoder::new(file).unwrap();
        let buffer = source.buffered();

        Self {
            raw_audio: buffer.convert_samples(),
        }
    }

    pub fn new_from_filepath(path: String) -> Self {
        let f = File::open(path).unwrap();
        Self::new(f)
    }
}
