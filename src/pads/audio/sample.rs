use std::fs::File;
use std::io::Read;

pub struct Sample {
    pub raw_audio: Vec<u8>
}

impl Sample {
    pub fn new(mut file: File) -> Self {
        let mut raw_audio: Vec<u8> = vec![];
        file.read_to_end(&mut raw_audio).unwrap();

        Self {
            raw_audio: raw_audio
        }
    }

    pub fn new_from_filepath(path: String) -> Self {
        let f = File::open(path).unwrap();
        Self::new(f)
    }
}
