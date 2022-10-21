use std::io::BufReader;
use std::fs::File;

pub struct Sample {
    pub file: File,
}

impl Sample {
    pub fn new(file: File) -> Self {
        Self {
            file: file
        }
    }
}
