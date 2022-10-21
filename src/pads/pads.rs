use gtk::{Builder, Button};
use gtk::prelude::*;
use std::sync::Arc;
use crate::pads::audio::sample::Sample;
use crate::pads::audio::sample_player::SamplePlayer;

pub struct Pad {
    id: i32,
    sample: Option<Sample>,
    btn: Button,
    sample_player: Arc<SamplePlayer>,
}

impl Pad {
    pub fn new(id: i32, btn: Button, sample: Option<Sample>, sample_player: Arc<SamplePlayer>) -> Arc<Pad> {
        let pad = Arc::new(Pad {id: id, btn: btn, sample: sample, sample_player: sample_player});
        let pad2 = Arc::clone(&pad);
        pad.btn.connect_clicked(move |_| pad2.on_click());
        Arc::clone(&pad)
    }

    fn on_click(&self) {
        match &self.sample {
            Some(s) => {
                let cl = s.file.try_clone().expect("Failed to clone.");
                &self.sample_player.play_sample(cl);
            },
            None => (),
        }
    }
}

pub fn get_btn_by_id(id: i32, builder: &Builder) -> Option<Button> {
    builder.object(format!("pad-{}", id).as_str())
}
